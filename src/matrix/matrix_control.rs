use crate::matrix::matrix_map::{MatrixMap, MatrixMapBuilder};
use crate::matrix::matrix_state::MatrixState;
use rs_ws281x::{ChannelBuilder, Controller, ControllerBuilder, StripType, WS2811Error};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Manages the matrix update thread
pub(crate) struct MatrixController {
    /// The dimensions of the matrix, in number of LEDs
    matrix_dimensions: (usize, usize),

    /// The current state of all LEDs in the matrix, as a two-dimensional array of BGRA color values
    matrix_state: Arc<Mutex<MatrixState>>,

    /// The DMA channel to use while controlling the matrix
    dma_channel: u16,

    /// The GPIO pin to use while controlling the matrix
    gpio_pin: u16,

    /// The signal frequency to use while controlling the matrix
    signal_frequency: u32,

    /// The brightness of the matrix
    brightness: u8,

    /// Maps matrix pixels to LEDs on a strip
    matrix_map: MatrixMap,

    /// True if the matrix update thread is running
    matrix_update_thread_alive: Arc<AtomicBool>,

    /// True if the matrix update thread should continue running
    matrix_update_thread_continue: Arc<AtomicBool>,
}

impl MatrixController {
    /// Create a new matrix controller
    ///
    /// # Arguments
    ///
    /// * `matrix_dimensions` - The size of the matrix in number of LEDs (width, height)
    /// * `serpentine` - Whether or not the matrix is serpentine
    /// * `vertical` - Whether or not the matrix is vertically wired
    /// * `brighness` - The brightness of the matrix, from 0 to 255
    /// * `gpio_pin` - The GPIO pin to use to control the matrix
    /// * `dma_channel` - The DMA channel to use to control the matrix
    /// * `signal_frequency` - The signal frequency to use to control the matrix
    ///
    pub(crate) fn new(
        matrix_dimensions: (usize, usize),
        serpentine: bool,
        mirror_horizontal: bool,
        mirror_vertical: bool,
        vertical: bool,
        brightness: u8,
        gpio_pin: u16,
        dma_channel: u16,
        signal_frequency: u32,
    ) -> Self {
        // Create the matrix map
        let mut matrix_map = MatrixMapBuilder::new(matrix_dimensions.0, matrix_dimensions.1);
        if serpentine {
            matrix_map = matrix_map.serpentine();
        }
        if vertical {
            matrix_map = matrix_map.vertical();
        }
        if mirror_horizontal {
            matrix_map = matrix_map.mirror_horizontally();
        }
        if mirror_vertical {
            matrix_map = matrix_map.mirror_vertically();
        }
        let matrix_map = matrix_map.build();

        Self {
            matrix_dimensions,
            matrix_state: Arc::new(Mutex::new(vec![
                vec![[0; 4]; matrix_dimensions.0];
                matrix_dimensions.1
            ])),
            dma_channel,
            gpio_pin,
            signal_frequency,
            brightness,
            matrix_map,
            matrix_update_thread_alive: Arc::new(AtomicBool::new(false)),
            matrix_update_thread_continue: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Start the matrix controller
    pub(crate) fn start(&mut self) -> Result<(), ()> {
        if self.matrix_update_thread_alive.load(Ordering::Relaxed) {
            log::warn!("Matrix update thread already exists, ignoring this start command.");
            return Ok(());
        }

        // Set the thread continue flag
        self.matrix_update_thread_continue
            .store(true, Ordering::Relaxed);

        // Make a few copies of things for the update thread
        let thread_matrix_state = Arc::clone(&self.matrix_state);
        let thread_continue = Arc::clone(&self.matrix_update_thread_continue);
        let thread_alive = Arc::clone(&self.matrix_update_thread_alive);
        let width = self.matrix_dimensions.0;
        let height = self.matrix_dimensions.1;
        let brightness = self.brightness;
        let frequency = self.signal_frequency;
        let dma_channel = self.dma_channel;
        let gpio_pin = self.gpio_pin;
        let matrix_map = self.matrix_map.clone();

        // Start the matrix update thread
        thread::spawn(move || {
            // Create the LED controller
            let mut controller = match ControllerBuilder::new()
                .freq(frequency)
                .dma(dma_channel as i32)
                .channel(
                    0, // channel index
                    ChannelBuilder::new()
                        .pin(gpio_pin as i32)
                        .count((width * height) as i32)
                        .strip_type(StripType::Ws2812)
                        .brightness(brightness)
                        .build(),
                )
                .build()
            {
                Ok(controller) => controller,
                Err(e) => {
                    log::error!("Failed to create LED controller.");
                    log::debug!("Failed with the following error: {e}");
                    return;
                }
            };

            // Mark the thread as alive
            thread_alive.store(true, Ordering::Relaxed);

            'update: loop {
                let current_state: MatrixState = match thread_matrix_state.lock() {
                    Ok(state) => state,
                    Err(_) => {
                        log::error!("Unable to get matrix state.");
                        break 'update;
                    }
                }
                .clone();

                // Update the LEDs
                {
                    let leds = controller.leds_mut(0);
                    for (y, row) in current_state.iter().enumerate() {
                        for (x, color) in row.iter().enumerate() {
                            leds[matrix_map.get(x, y)] = *color;
                        }
                    }
                }

                // Push the update to the LEDs
                match controller.render() {
                    Ok(_) => { /* Do nothing */ }
                    Err(e) => {
                        log::error!("Failed to push plugin changes to matrix.");
                        log::debug!("Failed with the following error: {e}");

                        match e {
                            WS2811Error::SpiTransfer => {
                                log::warn!("Failed to transfer data to LEDs. It is possible that too few LEDs are connected, or the SPI buffer is too small.");
                            }
                            _ => {}
                        }
                        break 'update;
                    }
                }

                // Check if the thread has been told to stop
                if !thread_continue.load(Ordering::Relaxed) {
                    log::info!("Closing matrix update thread.");
                    break 'update;
                }
            }

            // If we are at this point, the thread is about to stop.
            // Let's clean up a bit by clearing the matrix
            match clear_matrix(&mut controller) {
                Ok(_) => { /* Do nothing */ }
                Err(_) => {
                    log::warn!("Failed to clear matrix.")
                }
            };

            // Mark the thread as dead
            thread_alive.store(false, Ordering::Relaxed);
        });

        Ok(())
    }

    /// Stop the matrix update thread
    pub(crate) fn stop(&mut self) -> Result<(), ()> {
        if !self.matrix_update_thread_alive.load(Ordering::Relaxed) {
            log::warn!("No update thread exists to stop.");
            return Err(());
        }

        // Tell the thread to stop
        self.matrix_update_thread_continue
            .store(false, Ordering::Relaxed);

        // Wait for the thread to stop
        while self.matrix_update_thread_alive.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_secs(1));
        }

        Ok(())
    }

    /// Update the state of the matrix
    ///
    /// # Arguments
    ///
    /// `new_state` - The new state for the matrix
    ///
    pub(crate) fn update(&mut self, new_state: MatrixState) -> Result<(), ()> {
        // Return an error if the thread is not alive
        if !self.matrix_update_thread_alive.load(Ordering::Relaxed) {
            return Err(());
        }

        match self.matrix_state.lock() {
            Ok(mut matrix_state) => {
                *matrix_state = new_state;
                Ok(())
            }
            Err(_) => {
                log::error!("Failed to update matrix state.");
                Err(())
            }
        }
    }
}

/// Clear all LEDs in a rs_ws281x LED controller
///
/// # Arguments
///
/// `led_controller` - The LED controller to clear
///
pub(crate) fn clear_matrix(led_controller: &mut Controller) -> Result<(), WS2811Error> {
    let leds = led_controller.leds_mut(0);
    for led in leds {
        *led = [0, 0, 0, 0];
    }
    led_controller.render()
}
