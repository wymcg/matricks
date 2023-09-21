use matricks_plugin::{MatrixConfiguration, PluginUpdate};
use rs_ws281x::{ChannelBuilder, ControllerBuilder, StripType};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;

/// Start a new matrix control thread and return the join handle and a plugin update sender.
///
/// # Arguments
///
/// * `matrix_config` - A MatrixConfiguration struct containing information about the matrix
/// * `log_tx` - Sender channel to send logs to the log thread
///
pub fn start_matrix_control(
    matrix_config: MatrixConfiguration,
) -> (JoinHandle<()>, Sender<PluginUpdate>) {
    // Make the plugin update channel
    let (tx, rx) = channel::<PluginUpdate>();

    // Spawn a the matrix control thread
    let handle = thread::spawn(|| matrix_control(matrix_config, rx));

    // Return the matrix control thread handle and the plugin update transmit channel
    (handle, tx)
}

/// Matrix Control thread loop for real hardware (Raspberry Pi, etc.)
fn matrix_control(matrix_config: MatrixConfiguration, update_rx: Receiver<PluginUpdate>) {
    log::info!("Starting matrix control thread.");

    //// Setup the matrix controller
    let mut controller = ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            0, // channel index
            ChannelBuilder::new()
                .pin(10)
                .count((matrix_config.width * matrix_config.height) as i32)
                .strip_type(StripType::Ws2812)
                .brightness(matrix_config.brightness)
                .build(),
        )
        .build()
        .expect("Unable to start the matrix controller!");

    //// Generate matrix coord to led strip index lookup table

    // Generate non-vertical, non-serpentine matrix map
    let mut coord_to_strip_index: Vec<Vec<usize>> = vec![];
    for y in 0..matrix_config.height {
        coord_to_strip_index.push(vec![]);
        for x in 0..matrix_config.width {
            let strip_index = (y * matrix_config.width) + x;
            coord_to_strip_index[y].push(strip_index);
        }
    }

    // If this is a serpentine matrix, flip every other row
    if matrix_config.serpentine {
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
    let leds = controller.leds_mut(0);
    for led in leds {
        *led = [0, 0, 0, 0];
    }
    controller
        .render()
        .unwrap_or_else(|_| log::warn!("Failed to clear matrix on exit."));
}
