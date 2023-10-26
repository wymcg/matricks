use crate::matrix_state::MatrixState;
use rs_ws281x::{ChannelBuilder, Controller, ControllerBuilder, StripType, WS2811Error};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use crate::matrix_map::MatrixMap;


pub struct MatrixController {
    matrix_dimensions: (usize, usize),
    matrix_state: Arc<Mutex<Vec<Vec<[u8; 4]>>>>,
    serpentine: bool,
    dma_channel: u16,
    gpio_pin: u16,
    signal_frequency: u32,
    brightness: u8,
    matrix_update_thread_alive: Arc<AtomicBool>,
    matrix_update_thread_continue: Arc<AtomicBool>,
}

impl MatrixController {
    pub fn new(
        matrix_dimensions: (usize, usize),
        serpentine: bool,
        brightness: u8,
        gpio_pin: u16,
        dma_channel: u16,
        signal_frequency: u32,
    ) -> Self {
        Self {
            matrix_dimensions,
            matrix_state: Arc::new(Mutex::new(vec![
                vec![[0; 4]; matrix_dimensions.0];
                matrix_dimensions.1
            ])),
            serpentine,
            dma_channel,
            gpio_pin,
            signal_frequency,
            brightness,
            matrix_update_thread_alive: Arc::new(AtomicBool::new(false)),
            matrix_update_thread_continue: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&mut self) -> Result<(), ()> {
        if self.matrix_update_thread_alive.load(Ordering::Relaxed) {
            log::warn!("Matrix update thread already exists, ignoring this start command.");
            return Ok(());
        }

        // Set the thread continue flag
        self.matrix_update_thread_continue.store(true, Ordering::Relaxed);

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

        // Make a matrix map
        let mut matrix_map = MatrixMap::new(self.matrix_dimensions.0, self.matrix_dimensions.1);
        if self.serpentine {
            matrix_map = matrix_map.serpentine();
        }

        // Start the matrix update thread
        thread::spawn(move || {
            // Mark the thread as alive
            thread_alive.store(true, Ordering::Relaxed);

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
                .build() {
                Ok(controller) => {controller}
                Err(e) => {
                    log::error!("Failed to create LED controller.");
                    log::debug!("Failed with the following error: {e}");
                    return;
                }
            };

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
                    Err(_) => {
                        log::warn!("Failed to push plugin changes to matrix.");
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

    pub fn stop(&mut self) -> Result<(), ()> {
        if !self.matrix_update_thread_alive.load(Ordering::Relaxed) {
            log::warn!("No update thread exists to stop.");
            return Err(());
        }

        // Tell the thread to stop
        self.matrix_update_thread_continue.store(false, Ordering::Relaxed);

        // Wait for the thread to stop
        while self.matrix_update_thread_alive.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_secs(1));
        }

        Ok(())
    }

    pub fn update(&mut self, new_state: MatrixState) -> Result<(), ()> {
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

pub fn clear_matrix(led_controller: &mut Controller) -> Result<(), WS2811Error> {
    let leds = led_controller.leds_mut(0);
    for led in leds {
        *led = [0, 0, 0, 0];
    }
    led_controller.render()
}
