mod clargs;
mod logging;
mod matrix_control_thread;
mod plugin_iterator;

use clargs::Args;
use plugin_iterator::PluginIterator;
use std::str::from_utf8;
use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};

use crate::logging::log::Log;
use crate::logging::log_origin::LogOrigin;
use crate::logging::log_thread::LoggingThread;
use crate::logging::log_type::LogType;
use crate::matrix_control_thread::start_matrix_control;
use crate::plugin_iterator::PluginIteratorError;
use clap::Parser;
use extism::{Context, Plugin};
use matricks_plugin::{MatrixConfiguration, PluginUpdate};
use serde_json::from_str;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

fn main() {
    ////// SETUP

    // parse command line arguments
    let args = Args::parse();

    // start the logging thread
    let (log_thread_handle, log_tx) = LoggingThread::new(args.log_dir).start();

    // make an initial version log
    log_main(
        &log_tx,
        LogType::Normal,
        format!("Starting Matricks v{}", VERSION.unwrap_or("unknown")),
    );

    // calculate the frame time from the fps option
    let target_frame_time_ms = Duration::from_nanos((1_000_000_000.0 / args.fps).round() as u64);

    // make the matrix configuration string
    let mat_config = MatrixConfiguration {
        width: args.width,
        height: args.height,
        target_fps: args.fps,
        serpentine: args.serpentine,
        magnification: args.magnification,
    };
    let mat_config_string = match serde_json::to_string(&mat_config) {
        Ok(s) => s,
        Err(_) => {
            log_main(
                &log_tx,
                LogType::Error,
                "Unable to generate matrix configuration information!".to_string(),
            );
            log_main(&log_tx, LogType::Normal, "Quitting Matricks.".to_string());
            return;
        }
    };

    // start the matrix control thread
    log_main(
        &log_tx,
        LogType::Normal,
        "Starting the matrix control thread.".to_string(),
    );
    let (matrix_control_handle, matrix_control_tx) =
        start_matrix_control(mat_config, log_tx.clone());

    // make the plugin iterator
    let plugin_data_list = match PluginIterator::new(args.plugins) {
        Ok(plugin_iterator) => plugin_iterator,
        Err(e) => {
            log_main(
                &log_tx,
                LogType::Error,
                format!("Failed to instantiate plugin list. See error: <{e:?}>"),
            );
            log_main(&log_tx, LogType::Normal, format!("Quitting Matricks."));
            return;
        }
    };

    ////// PLUGIN LOOP
    for plugin_result in plugin_data_list {
        // check if the plugin data was successfully read
        let (plugin_path, plugin_data) = match plugin_result {
            Ok(data) => data,
            Err(error) => {
                match error {
                    PluginIteratorError::InvalidSeedPath(path) => {
                        // the seed path is invalid, meaning that no plugins can be read.
                        // this should never ever happen
                        log_main(&log_tx,
                                 LogType::Error,
                                 format!("Could not read plugin data due to an invalid or missing path '{path}', resulting in a catastrophic error.")
                        );
                        log_main(&log_tx, LogType::Normal, format!("Quitting Matricks."));
                        return;
                    }
                    PluginIteratorError::InvalidPluginPath(path) => {
                        // something went wrong with the path to the active plugin
                        // we can't run this plugin, but we might be able to run others
                        log_main(&log_tx,
                                 LogType::Warning,
                                 format!("Could not read plugin data due to an invalid or missing path '{path}'. This plugin will be skipped.")
                        );
                    }
                }
                continue;
            }
        };

        // make a new context for the plugin
        let context = Context::new();

        // make a new instance of the plugin
        log_main(
            &log_tx,
            LogType::Normal,
            format!("Starting plugin '{plugin_path}'"),
        );
        let mut plugin = match Plugin::new(&context, plugin_data, [], true) {
            Ok(plugin) => plugin,
            Err(e) => {
                log_main(&log_tx, LogType::Warning, format!("Extism reported the following error while attempting to instantiate the plugin: {e:?}"));
                log_main(&log_tx, LogType::Warning, format!("Unable to instantiate plugin '{plugin_path}'. This plugin will be skipped."));
                continue;
            }
        };

        // call setup function of current active plugin
        let _setup_result = match plugin.call("setup", &mat_config_string) {
            Ok(result) => {
                log_main(
                    &log_tx,
                    LogType::Normal,
                    "Plugin setup complete. Starting update loop...".to_string(),
                );
                result
            }
            Err(_) => {
                log_main(
                    &log_tx,
                    LogType::Warning,
                    "Unable to complete setup! Starting update loop anyway...".to_string(),
                );
                &[]
            }
        };

        // setup the last frame time variable
        let mut last_frame_time = Instant::now();
        'update_loop: loop {
            // only call the update function if a frame has passes
            if (Instant::now() - last_frame_time) >= target_frame_time_ms {
                // reset the last frame time
                last_frame_time = Instant::now();

                // call the update function
                match plugin.call("update", "") {
                    Ok(json_result_utf8) => {
                        // convert the result form utf8 to &str
                        let json_result_str = match from_utf8(json_result_utf8) {
                            Ok(s) => s,
                            Err(_) => {
                                log_main(
                                    &log_tx,
                                    LogType::Warning,
                                    "Invalid UTF-8 result from plugin! Skipping this plugin..."
                                        .to_string(),
                                );
                                break 'update_loop;
                            }
                        };

                        // make a matrix state object from the string
                        let new_update = match from_str::<PluginUpdate>(json_result_str) {
                            Ok(matrix_state) => matrix_state,
                            Err(_) => {
                                log_main(
                                    &log_tx,
                                    LogType::Warning,
                                    "Unable to deserialize result from plugin! Skipping this plugin...".to_string()
                                );
                                break 'update_loop;
                            }
                        };

                        // send matrix state to the matrix control thread
                        match matrix_control_tx.send(new_update.clone()) {
                            Ok(_) => { /* do nothing if it sent ok */ }
                            Err(_) => {
                                log_main(
                                    &log_tx,
                                    LogType::Error,
                                    "Unable to send matrix state update to matrix control thread!"
                                        .to_string(),
                                );
                                log_main(&log_tx, LogType::Normal, format!("Quitting Matricks."));
                            }
                        };

                        // send plugin logs
                        match new_update.log_message {
                            None => { /* plugin didn't send us anything, so don't do anything */ }
                            Some(logs) => {
                                for log in logs {
                                    // send a log message, identifying as the plugin
                                    log_tx
                                        .send(Log::new(
                                            LogOrigin::Plugin(plugin_path.clone()),
                                            LogType::Normal, // assume the plugin log is part of normal operation
                                            log,
                                        ))
                                        .expect("Unable to send plugin log to log thread!");
                                }
                            }
                        }

                        // go to the next plugin if the plugin says it is done
                        if new_update.done {
                            log_main(
                                &log_tx,
                                LogType::Normal,
                                "Plugin signalled that it is done. Moving on to the next plugin..."
                                    .to_string(),
                            );
                            break 'update_loop;
                        }
                    }
                    Err(_) => {
                        log_main(
                            &log_tx,
                            LogType::Warning,
                            "Unable to call update function! Skipping this plugin...".to_string(),
                        );
                        break 'update_loop;
                    }
                };
            }
        }
    }

    log_main(&log_tx, LogType::Normal, format!("Quitting Matricks."));

    // close channels
    drop(matrix_control_tx);
    drop(log_tx);

    // join logging and matrix control threads
    matrix_control_handle
        .join()
        .expect("Unable to join matrix control thread!");
    log_thread_handle
        .join()
        .expect("Unable to join log thread!");
}

/// Send a log from the main thread
fn log_main(log_tx: &Sender<Log>, log_type: LogType, description: String) {
    log_tx
        .send(Log::new(LogOrigin::MainThread, log_type, description))
        .expect("Unable to send log from main thread!");
}
