use std::collections::BTreeMap;
use matricks_plugin::PluginUpdate;
use rs_ws281x::{ChannelBuilder, Controller, ControllerBuilder, StripType, WS2811Error};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use crate::clargs::{MATRICKS_DEFAULT_BRIGHTNESS, MATRICKS_DEFAULT_SERPENTINE};

const LED_SIGNAL_FREQUENCY: u32 = 800_000; // 800kHz
const LED_DMA_CHANNEL: i32 = 10;
const LED_GPIO_PIN: i32 = 10;


/// Start a new matrix control thread and return the join handle and a plugin update sender.
///
/// # Arguments
///
/// * `matrix_config` - A MatrixConfiguration struct containing information about the matrix
/// * `log_tx` - Sender channel to send logs to the log thread
///
pub fn start_matrix_control(
    matrix_config: BTreeMap<String, Option<String>>,
) -> (JoinHandle<()>, Sender<PluginUpdate>) {
    // Make the plugin update channel
    let (tx, rx) = channel::<PluginUpdate>();

    // Spawn a the matrix control thread
    let handle = thread::spawn(|| matrix_controller(matrix_config, rx));

    // Return the matrix control thread handle and the plugin update transmit channel
    (handle, tx)
}

fn matrix_controller(matrix_config: BTreeMap<String, Option<String>>, update_rx: Receiver<PluginUpdate>) {
    //// Pull config fields as Option<String> from the matrix config
    let width: Option<String> = matrix_config.get("width").cloned().unwrap_or(None);
    let height: Option<String> = matrix_config.get("height").cloned().unwrap_or(None);
    let brightness: Option<String> = matrix_config.get("brightness").cloned().unwrap_or_else(|| {
        log::warn!("Brightness field is not present in the matrix configuration, defaulting to {MATRICKS_DEFAULT_BRIGHTNESS}");
        Some(String::from(MATRICKS_DEFAULT_BRIGHTNESS))
    });
    let serpentine: Option<String> = matrix_config.get("serpentine").cloned().unwrap_or_else(|| {
        log::warn!("Serpentine field is not present in the matrix configuration, defaulting to {MATRICKS_DEFAULT_SERPENTINE}");
        Some(String::from(MATRICKS_DEFAULT_SERPENTINE))
    });

    //// Pull inner values from options
    let width: String = width.clone().unwrap_or_default();
    let height: String = height.clone().unwrap_or_default();
    let brightness: String = brightness.clone().unwrap_or_default();
    let serpentine: String = serpentine.clone().unwrap_or_default();

    //// Attempt to convert values to their correct types
    let width: usize = match width.parse() {
        Ok(val) => val,
        Err(e) => {
            log::error!("Failed to parse width from matrix configuration.");
            log::debug!("Received the following error while attempting to parse width: {e:?}");
            log::error!("Failed to create matrix controller.");
            return
        }
    };
    let height: usize = match height.parse() {
        Ok(val) => val,
        Err(e) => {
            log::error!("Failed to parse height from matrix configuration.");
            log::debug!("Received the following error while attempting to parse height: {e:?}");
            log::error!("Failed to create matrix controller.");
            return
        }
    };
    let brightness: u8 = match brightness.parse() {
        Ok(val) => val,
        Err(e) => {
            log::error!("Failed to parse brightness from matrix configuration.");
            log::debug!("Received the following error while attempting to parse brightness: {e:?}");
            log::error!("Failed to create matrix controller.");
            return
        }
    };
    let serpentine: bool = match serpentine.parse() {
        Ok(val) => val,
        Err(e) => {
            log::error!("Failed to parse serpentine from matrix configuration.");
            log::debug!("Received the following error while attempting to parse serpentine: {e:?}");
            log::error!("Failed to create matrix controller.");
            return
        }
    };

    //// Setup the matrix controller
    let mut controller = match make_led_controller(width as i32, height as i32, brightness) {
        Ok(controller) => {controller}
        Err(e) => {
            log::error!("Failed to start matrix controller");
            log::debug!("See error: {e:?}");
            return;
        }
    };

    //// Generate matrix coord to led strip index lookup table

    // Generate normal matrix map
    let mut coord_to_strip_index: Vec<Vec<usize>> = vec![];
    for y in 0..height {
        coord_to_strip_index.push(vec![]);
        for x in 0..width {
            let strip_index = (y * width) + x;
            coord_to_strip_index[y].push(strip_index);
        }
    }

    // If this is a serpentine matrix, flip every other row
    if serpentine {
        for (y, row) in coord_to_strip_index.iter_mut().enumerate() {
            if y % 2 == 1 {
                row.reverse()
            }
        }
    }

    //// Handle matrix updates as they come
    for update in update_rx {
        {
            let leds = controller.leds_mut(0);
            for (y, row) in update.state.iter().enumerate() {
                for (x, color) in row.iter().enumerate() {
                    leds[coord_to_strip_index[y][x]] = *color;
                }
            }
        }

        match controller.render() {
            Ok(_) => { /* Do nothing */ }
            Err(_) => {
                log::warn!("Failed to push plugin changes to matrix.");
            }
        }
    }

    // When the update channel closes, clear the LEDs
    log::info!("Clearing matrix.");
    clear_matrix(&mut controller)
        .unwrap_or_else(|_| log::warn!("Failed to clear matrix on exit."));
}

pub fn clear_matrix(led_controller: &mut Controller) -> Result<(), WS2811Error> {
    let leds = led_controller.leds_mut(0);
    for led in leds {
        *led = [0, 0, 0, 0];
    }
    led_controller.render()
}

pub fn make_led_controller(width: i32, height: i32, brightness: u8) -> Result<Controller, WS2811Error> {
    ControllerBuilder::new()
        .freq(LED_SIGNAL_FREQUENCY)
        .dma(LED_DMA_CHANNEL)
        .channel(
            0, // channel index
            ChannelBuilder::new()
                .pin(LED_GPIO_PIN)
                .count(width * height)
                .strip_type(StripType::Ws2812)
                .brightness(brightness)
                .build(),
        )
        .build()
}