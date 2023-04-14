use crate::logging::log::Log;
use crate::logging::log_origin::LogOrigin;
use crate::logging::log_type::LogType;
use crate::matrix_configuration::MatrixConfiguration;
use crate::plugin_update::PluginUpdate;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;

// import matrix control stuff only if we're compiling for a raspberry pi
#[cfg(target_arch = "aarch64")]
use rs_ws281x::{ChannelBuilder, Controller, ControllerBuilder, StripType, WS2811Error};

// import opencv highgui stuff if we're compiling for anything else
#[cfg(not(target_arch = "aarch64"))]
use opencv::{
    core::{Mat_AUTO_STEP, CV_8UC4},
    highgui, imgproc,
    prelude::*,
};

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

#[cfg(target_arch = "aarch64")]
fn matrix_control(
    matrix_config: MatrixConfiguration,
    log_tx: Sender<Log>,
    update_rx: Receiver<PluginUpdate>,
) {
    //// setup the matrix controller
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

    //// generate matrix coord to led strip index lookup table

    // generate non-vertical, non-serpentine matrix map
    let mut coord_to_strip_index: Vec<Vec<usize>> = vec![];
    for y in 0..matrix_config.height {
        coord_to_strip_index.push(vec![]);
        for x in 0..matrix_config.width {
            let strip_index = (y * matrix_config.width) + x;
            coord_to_strip_index[y].push(strip_index);
        }
    }

    // if this is a serpentine matrix, flip every other row
    if matrix_config.serpentine {
        for (y, row) in coord_to_strip_index.iter_mut().enumerate() {
            if y % 2 == 1 {
                row.reverse()
            }
        }
    }

    println!("{coord_to_strip_index:?}");

    let leds = controller.leds_mut(0);

    //// handle matrix updates as they come
    for update in update_rx {
        for (y, row) in update.state.iter().enumerate() {
            for (x, color) in row.iter().enumerate() {
                leds[coord_to_strip_index[y][x]] = color.clone();
            }
        }
    }
}

#[cfg(not(target_arch = "aarch64"))]
fn matrix_control(
    matrix_config: MatrixConfiguration,
    log_tx: Sender<Log>,
    update_rx: Receiver<PluginUpdate>,
) {
    /*
        This really sucks. There is no reason this should use something as big as OpenCV just
        to render a few pixels on a screen. For everyone's sake, this should be rewritten at some
        point to use something lighter and FFI-less
    */

    log_tx.send(Log::new(
        LogOrigin::MatrixControlThread,
        LogType::Warning,
        "Starting matrix simulation...".to_string()
    )).expect("Unable to send log from matrix thread!");

    // make a mat to hold the resized mat in the update loop
    let mut resized_mat = unsafe {
        Mat::new_rows_cols(
            (matrix_config.height as f32 * matrix_config.magnification) as i32,
            (matrix_config.width as f32 * matrix_config.magnification) as i32,
            CV_8UC4,
        )
            .expect("Failed to make Mat to hold resized image!")
    };
    let resized_size = resized_mat.size().unwrap();

    // the update loop
    for update in update_rx {
        // flatten the state so that we can make an opencv mat with it
        let state: Vec<u8> = update
            .state
            .iter()
            .flatten()
            .flatten()
            .map(|val| val.clone())
            .collect();

        // create the mat
        let mut mat = Mat::from_slice(&state).expect("Failed make Mat from slice!");

        // size the mat and set the color channel info
        unsafe {
            mat.create_rows_cols(
                matrix_config.height as i32,
                matrix_config.width as i32,
                CV_8UC4,
            )
            .expect("Failed to assign size and channel information to Mat!")
        };

        // resize the mat
        imgproc::resize(
            &mat,
            &mut resized_mat,
            resized_size,
            0.0,
            0.0,
            imgproc::INTER_LINEAR,
        )
        .expect("Failed to resize the Mat!");

        highgui::imshow("Test", &resized_mat).expect("Couldn't display image");

        let key = highgui::wait_key(1).expect("Couldn't get key");
        if key == 113 {
            // quit with q
            break;
        }
    }
}
