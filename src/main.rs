mod clargs;
mod matrix_control_thread;
mod plugin_iterator;

use crate::clargs::{MatricksArgs, MatricksSubcommand};
use crate::plugin_iterator::{PluginIterator, PluginIteratorError};
use crate::matrix_control_thread::start_matrix_control;

use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::str::from_utf8;
use std::time::{Duration, Instant};

use clap::Parser;
use extism::{Context, Plugin};
use matricks_plugin::{MatrixConfiguration, PluginUpdate};
use serde_json::from_str;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

fn main() {
    ////// SETUP

    // parse command line arguments
    let args = MatricksArgs::parse();

    // Start the logger
    env_logger::init();

    // Make an initial version log
    log::info!("Starting Matricks v{}.", VERSION.unwrap_or("unknown"));

    // Pull config from command line argument
    let config = match args.config {
        MatricksSubcommand::Manual(config) => {
            log::info!("Matrix configuration has been manually supplied.");
            config
        }
        MatricksSubcommand::Auto(file_info) => {
            // Read the file to a string
            let matrix_config_string_toml = match fs::read_to_string(&file_info.path) {
                Ok(string) => {string}
                Err(e) => {
                    log::error!("Failed to read config file at path \"{}\".", file_info.path);
                    log::debug!("Received the following error while attempting to read file: {e:?}");
                    log::info!("Quitting Matricks.");
                    return;
                }
            };

            match toml::from_str(&matrix_config_string_toml) {
                Ok(config) => {
                    log::info!("Matrix configuration has been supplied from a configuration file.");
                    config
                }
                Err(e) => {
                    log::error!("Failed to parse config file at path \"{}\".", file_info.path);
                    log::debug!("Received the following error while attempting to parse file: {e:?}");
                    log::info!("Quitting Matricks.");
                    return;
                }
            }

        }
        MatricksSubcommand::Save {
            info,
            matrix_config,
        } => {
            // Serialize the matrix configuration to a string
            let config_string_toml = match toml::to_string(&matrix_config) {
                Ok(string) => {string}
                Err(e) => {
                    log::error!("Failed to serialize matrix configuration");
                    log::debug!("Received the following error while attempting to serialize matrix configuration: {e:?}");
                    log::info!("Quitting Matricks.");
                    return;
                }
            };

            match fs::write(&info.path, config_string_toml) {
                Ok(_) => {
                    log::info!("Successfully wrote matrix configuration to configuration file at path \"{}\"", info.path);
                    log::info!("Quitting Matricks.");
                    return;
                }
                Err(e) => {
                    log::error!("Failed to write matrix configuration to configuration file at path \"{}\"", info.path);
                    log::debug!("Received the following error while attempting to write matrix configuration to file: {e:?}");
                    log::info!("Quitting Matricks.");
                    return;
                }
            };
        }
    };

    // Calculate the frame time from the FPS option
    let target_frame_time_ms = Duration::from_nanos((1_000_000_000.0 / config.fps).round() as u64);

    // Make the matrix configuration string
    let mat_config = MatrixConfiguration {
        width: config.width,
        height: config.height,
        target_fps: config.fps,
        serpentine: config.serpentine,
        brightness: config.brightness,
        ..Default::default()
    };
    let mat_config_string = match serde_json::to_string(&mat_config) {
        Ok(s) => s,
        Err(_) => {
            log::error!("Unable to generate matrix configuration information!");
            log::info!("Quitting Matricks.");
            return;
        }
    };

    // Start the matrix control thread
    log::info!("Starting the matrix control thread.");
    let (matrix_control_handle, matrix_control_tx) = start_matrix_control(mat_config);

    // The main loop, which is run infinitely if the loop command line flag is set
    'main_loop: loop {
        // make the plugin iterator
        let plugin_data_list = match PluginIterator::new(config.plugins.clone()) {
            Ok(plugin_iterator) => plugin_iterator,
            Err(e) => {
                log::error!("Failed to instantiate plugin list.");
                log::debug!("See error: {e:?}");
                log::info!("Quitting Matricks.");
                return;
            }
        };

        ////// PLUGIN LOOP
        for plugin_result in plugin_data_list {
            // Check if the plugin data was successfully read
            let (plugin_path, plugin_data) = match plugin_result {
                Ok(data) => data,
                Err(error) => {
                    match error {
                        PluginIteratorError::InvalidSeedPath(path) => {
                            // The seed path is invalid, meaning that no plugins can be read.
                            // This should never ever happen
                            log::error!("Could not read plugin data due to an invalid or missing path\"{path}\".");
                            log::info!("Quitting Matricks.");
                            return;
                        }
                        PluginIteratorError::InvalidPluginPath(path) => {
                            // Something went wrong with the path to the active plugin
                            // We can't run this plugin, but we might be able to run others
                            log::error!("Could not read plugin data due to an invalid or missing path \"{path}\".");
                            log::warn!("This plugin will be skipped.");
                        }
                    }
                    continue;
                }
            };

            // Pull the filename out from the plugin path
            let plugin_name = Path::new(&plugin_path)
                .file_name()
                .unwrap_or(OsStr::new(&plugin_path))
                .to_str()
                .unwrap_or(&plugin_path);

            // Make a new context for the plugin
            let context = Context::new();

            // Make a new instance of the plugin
            log::info!("Starting plugin \"{plugin_name}\".");
            let mut plugin = match Plugin::new(&context, plugin_data, [], true) {
                Ok(plugin) => plugin,
                Err(e) => {
                    log::error!("Unable to instantiate plugin \"{plugin_name}\".");
                    log::debug!("Received the following error while attempting to instantiate the plugin: {e:?}");
                    log::warn!("This plugin will be skipped.");
                    continue;
                }
            };

            // Call setup function of current active plugin
            let _setup_result = match plugin.call("setup", &mat_config_string) {
                Ok(result) => {
                    log::info!("Successfully set up plugin \"{plugin_name}\".");
                    result
                }
                Err(e) => {
                    log::warn!("Unable to set up plugin \"{plugin_name}\".");
                    log::debug!("Received the following error while setting up the plugin: {e:?}");
                    &[]
                }
            };

            // Mark the time when this plugin started its update loop
            let plugin_start_time = Instant::now();

            // Setup the last frame time variable
            let mut last_frame_time = Instant::now();

            // Run an update every frame
            'update_loop: loop {
                // Move on to the next plugin if the plugin time limit has been exceeded
                match config.time_limit {
                    None => { /* There is no time limit, so do nothing */ }
                    Some(time_limit) => {
                        // Move on to the next plugin if this plugin has been running longer than the time limit
                        if Instant::now() - plugin_start_time > Duration::from_secs(time_limit) {
                            break 'update_loop;
                        }
                    }
                }

                // Call the update function if a frame has passed
                if (Instant::now() - last_frame_time) >= target_frame_time_ms {
                    // Reset the last frame time
                    last_frame_time = Instant::now();

                    // Call the update function
                    match plugin.call("update", "") {
                        Ok(json_result_utf8) => {
                            // Convert the result form utf8 to &str
                            let json_result_str = match from_utf8(json_result_utf8) {
                                Ok(s) => s,
                                Err(_) => {
                                    log::warn!("Received invalid UTF-8 result from plugin \"{plugin_name}\"");
                                    log::warn!("This plugin will be skipped.");
                                    break 'update_loop;
                                }
                            };

                            // Make a matrix state object from the string
                            let new_update = match from_str::<PluginUpdate>(json_result_str) {
                                Ok(matrix_state) => matrix_state,
                                Err(_) => {
                                    log::warn!(
                                        "Received malformed update from plugin \"{plugin_name}\""
                                    );
                                    log::warn!("This plugin will be skipped.");
                                    break 'update_loop;
                                }
                            };

                            // Send matrix state to the matrix control thread
                            match matrix_control_tx.send(new_update.clone()) {
                                Ok(_) => { /* do nothing if it sent ok */ }
                                Err(_) => {
                                    log::error!("Failed to send state update to matrix control.");
                                    log::info!("Quitting Matricks.");
                                }
                            };

                            // Send plugin logs
                            match new_update.log_message {
                                None => { /* Plugin didn't send us anything, so don't do anything */
                                }
                                Some(logs) => {
                                    for log in logs {
                                        // Send a log message, identifying as the plugin
                                        log::info!("<{plugin_name}> {log}");
                                    }
                                }
                            }

                            // Go to the next plugin if the plugin says it is done
                            if new_update.done {
                                log::info!("Halting plugin \"{plugin_name}\" on plugin request.");
                                break 'update_loop;
                            }
                        }
                        Err(e) => {
                            log::error!(
                                "Unable to retrieve state update from plugin \"{plugin_name}\""
                            );
                            log::debug!("Received the following error while retrieving state update from plugin: {e:?}");
                            log::warn!("This plugin will be skipped.");
                            break 'update_loop;
                        }
                    };
                }
            }
        }

        // Break if the loop flag is not set
        if !config.loop_plugins {
            break 'main_loop;
        }
    }

    log::info!("Quitting Matricks.");

    // Close the connection to the matrix control thread, which allows the matrix control thread to stop
    drop(matrix_control_tx);

    // Join logging and matrix control threads
    matrix_control_handle
        .join()
        .unwrap_or_else(|_| log::warn!("Unable to join matrix control thread."));

    log::info!("Done.");
}
