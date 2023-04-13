use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use crate::logging::log::Log;
use crate::logging::log_origin::LogOrigin;
use crate::logging::log_type::LogType;
use crate::matrix_configuration::MatrixConfiguration;
use crate::plugin_update::PluginUpdate;

pub fn start_matrix_control(matrix_config: MatrixConfiguration, log_tx: Sender<Log>) -> (JoinHandle<()>, Sender<PluginUpdate>) {
    // make the plugin update channel
    let (tx, rx) = channel::<PluginUpdate>();

    // spawn a the matrix control thread
    let handle = thread::spawn(|| matrix_control(matrix_config, log_tx, rx));

    // return the matrix control thread handle and the plugin update transmit channel
    (handle, tx)
}

#[cfg(not(target_arch = "arm-unknown-linux-gnueabihf"))]
fn matrix_control(matrix_config: MatrixConfiguration, log_tx: Sender<Log>, update_rx: Receiver<PluginUpdate>) {
    for update in update_rx {
        // HELP WANTED
        // Ideally, there would be some GUI-ish simulation of what a matrix would be doing
        // Kind of like what we had for vid2led with the OpenCV imshow stuff
        // I tried some simple pixel buffer libraries but wasn't able to get them going here
        // I also didn't want to put time/effort on making something in a dedicated game dev lib like bevy
        // So, if you're reading this, maybe you can!
    }
}


#[cfg(target_arch = "arm-unknown-linux-gnueabihf")]
fn matrix_control(log_tx: Sender<Log>, update_rx: Receiver<PluginUpdate>) {
    for update in update_rx {
        todo!()
    }
}