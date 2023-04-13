use crate::logging::log::Log;
use crate::logging::log_origin::LogOrigin;
use crate::logging::log_type::LogType;
use crate::matrix_configuration::MatrixConfiguration;
use crate::plugin_update::PluginUpdate;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;

// import matrix control stuff only if we're compiling for a raspberry pi
#[cfg(any(target_os = "arm-unknown-linux-gnueabihf", target_os = "armv6-unknown-linux-gnueabihf", target_os = "armv7-unknown-linux-gnueabihf"))]
use rs_ws281x::{ChannelBuilder, Controller, ControllerBuilder, StripType, WS2811Error};


pub fn start_matrix_control(
    matrix_config: MatrixConfiguration,
    log_tx: Sender<Log>,
) -> (JoinHandle<()>, Sender<PluginUpdate>) {
    // make the plugin update channel
    let (tx, rx) = channel::<PluginUpdate>();

    // spawn a the matrix control thread
    let handle = thread::spawn(|| matrix_control(matrix_config, log_tx, rx));

    // return the matrix control thread handle and the plugin update transmit channel
    (handle, tx)
}

#[cfg(any(target_os = "arm-unknown-linux-gnueabihf", target_os = "armv6-unknown-linux-gnueabihf", target_os = "armv7-unknown-linux-gnueabihf"))]
fn matrix_control(
    matrix_config: MatrixConfiguration,
    log_tx: Sender<Log>,
    update_rx: Receiver<PluginUpdate>,
) {
    // setup the matrix controller
    let mut controller = ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            0, // channel index
            ChannelBuilder::new()
                .pin(10)
                .count(64)
                .strip_type(StripType::Ws2812)
                .brightness(20)
                .build(),
        )
        .build()
        .expect("Unable to start the matrix controller!");

    for update in update_rx {}

}

#[cfg(all(not(target_os = "arm-unknown-linux-gnueabihf"), not(target_os = "armv6-unknown-linux-gnueabihf"), not(target_os = "armv7-unknown-linux-gnueabihf")))]

fn matrix_control(
    matrix_config: MatrixConfiguration,
    log_tx: Sender<Log>,
    update_rx: Receiver<PluginUpdate>,
) {
    log_tx.send(Log::new(
        LogOrigin::MatrixControlThread,
        LogType::Warning,
        "Matrix simulation is not yet implemented for non-Raspberry Pi targets. The matrix control thread will act as a dummy thread.".to_string()
    )).expect("Unable to send log from matrix thread!");

    for update in update_rx {}

}
