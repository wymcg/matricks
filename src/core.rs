use crate::clargs::MatricksConfigArgs;
use crate::matrix::matrix_control::MatrixController;
use std::collections::BTreeMap;

use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::str::from_utf8;
use std::time::{Duration, Instant};

use crate::path_map::PathMap;
use crate::plugin::plugin_logs;
use extism::manifest::Wasm;
use extism::{Function, Manifest, Plugin, ValType};
use serde_json::from_str;

/// Core Matricks functionality
///
/// # Arguments
///
/// * `config` - Information about how Matricks should run
///
pub fn matricks_core(config: MatricksConfigArgs) {
    // Calculate the frame time from the FPS option
    let target_frame_time_ms =
        Duration::from_nanos((1_000_000_000.0 / config.matrix.fps).round() as u64);

    // Create the config
    let mut matricks_config: BTreeMap<String, Option<String>> = BTreeMap::new();
    matricks_config.insert(
        String::from("width"),
        Some(format!("{}", config.matrix.width)),
    );
    matricks_config.insert(
        String::from("height"),
        Some(format!("{}", config.matrix.height)),
    );
    matricks_config.insert(
        String::from("target_fps"),
        Some(format!("{}", config.matrix.fps)),
    );
    matricks_config.insert(
        String::from("serpentine"),
        Some(format!("{}", config.matrix.serpentine)),
    );
    matricks_config.insert(
        String::from("brightness"),
        Some(format!("{}", config.matrix.brightness)),
    );

    // Setup the host functions
    let plugin_debug_log_function = Function::new(
        "matricks_debug",
        [ValType::I64],
        [],
        None,
        plugin_logs::plugin_debug_log,
    );
    let plugin_info_log_function = Function::new(
        "matricks_info",
        [ValType::I64],
        [],
        None,
        plugin_logs::plugin_info_log,
    );
    let plugin_warn_log_function = Function::new(
        "matricks_warn",
        [ValType::I64],
        [],
        None,
        plugin_logs::plugin_warn_log,
    );
    let plugin_error_log_function = Function::new(
        "matricks_error",
        [ValType::I64],
        [],
        None,
        plugin_logs::plugin_error_log,
    );
    let plugin_functions = [
        plugin_debug_log_function,
        plugin_info_log_function,
        plugin_warn_log_function,
        plugin_error_log_function,
    ];

    // Process user-supplied path mappings
    let mut path_mappings: Vec<PathMap> = vec![];
    match config.plugin.map_path {
        None => { /* Do nothing */ }
        Some(path_map_strings) => {
            for path_map_string in path_map_strings {
                match PathMap::from_string(path_map_string.clone()) {
                    Ok(path_map) => {
                        log::info!(
                            "Mapping local filesystem path \"{}\" to plugin filesystem path \"{}\"",
                            path_map.from,
                            path_map.to
                        );
                        path_mappings.push(path_map);
                    }
                    Err(_) => {
                        log::warn!("Unable to process path mapping \"{path_map_string}\". This mapping will be ignored.");
                    }
                };
            }
        }
    }

    // Create a new matrix controller object
    let mut matrix = MatrixController::new(
        (config.matrix.width, config.matrix.height),
        config.matrix.serpentine,
        config.matrix.mirror_horizontal,
        config.matrix.mirror_vertical,
        config.matrix.vertical,
        config.matrix.brightness,
        config.matrix.controller.gpio,
        config.matrix.controller.dma,
        config.matrix.controller.frequency,
    );

    // Start the matrix controller
    match matrix.start() {
        Ok(_) => {}
        Err(_) => {
            log::error!("Failed to start new matrix controller.");
        }
    }

    // The main loop, which is run infinitely if the loop command line flag is set
    'main_loop: loop {
        ////// PLUGIN LOOP
        for plugin_path in &config.plugin.plugin {
            // Get the plugin data at the given path
            let plugin_data = match fs::read(plugin_path) {
                Ok(data) => {data}
                Err(e) => {
                    log::error!("Failed to read plugin data at path '{plugin_path}'");
                    log::debug!("Failed with error: {e}");
                    log::warn!("This plugin will be skipped.");
                    continue;
                }
            };

            // Pull the filename out from the plugin path
            let plugin_name = Path::new(plugin_path)
                .file_name()
                .unwrap_or(OsStr::new(plugin_path))
                .to_str()
                .unwrap_or(plugin_path);

            // Make a new manifest for the plugin
            let mut manifest = Manifest::new([Wasm::data(plugin_data)]);

            // Add the allowed hosts to the manifest
            for host in config.plugin.allow_host.clone().unwrap_or(vec![]) {
                log::debug!("Adding host \"{host}\" to the manifest.");
                manifest = manifest.with_allowed_host(host);
            }

            // Add the path mappings to the manifest
            for path_map in path_mappings.clone() {
                log::debug!(
                    "Adding mapping from \"{}\" to \"{}\" to the manifest.",
                    path_map.from,
                    path_map.to
                );
                manifest = manifest.with_allowed_path(path_map.from, path_map.to);
            }

            // Make a new instance of the plugin
            log::info!("Starting plugin \"{plugin_name}\".");
            let plugin = match Plugin::create_with_manifest(
                &manifest,
                plugin_functions.clone(),
                true,
            ) {
                Ok(plugin) => plugin,
                Err(e) => {
                    log::error!("Unable to instantiate plugin \"{plugin_name}\".");
                    log::debug!("Received the following error while attempting to instantiate the plugin: {e:?}");
                    log::warn!("This plugin will be skipped.");
                    continue;
                }
            };

            // Apply the config to the plugin
            let mut plugin = match plugin.with_config(&matricks_config) {
                Ok(plugin) => plugin,
                Err(e) => {
                    log::error!("Unable to apply configuration to plugin \"{plugin_name}\".");
                    log::debug!("Received the following error while attempting to instantiate the plugin: {e:?}");
                    log::warn!("This plugin will be skipped.");
                    continue;
                }
            };

            // Call setup function of current active plugin
            let _setup_result = match plugin.call("setup", "") {
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
                match config.plugin.time_limit {
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

                            // Pull the next matrix state from the plugin's response
                            let new_matrix_state =
                                match from_str::<Option<Vec<Vec<[u8; 4]>>>>(json_result_str) {
                                    Ok(matrix_state) => matrix_state,
                                    Err(_) => {
                                        log::warn!(
                                        "Received malformed update from plugin \"{plugin_name}\""
                                    );
                                        log::warn!("This plugin will be skipped.");
                                        break 'update_loop;
                                    }
                                };

                            let new_matrix_state = match new_matrix_state {
                                None => {
                                    log::info!("Done with plugin \"{plugin_name}\".");
                                    break 'update_loop;
                                }
                                Some(new_matrix_state) => {new_matrix_state}
                            };

                            match matrix.update(new_matrix_state) {
                                Ok(_) => { /* Do nothing, the new state sent without issue */
                                }
                                Err(_) => {
                                    log::error!("Failed to update matrix controller.");
                                    break 'main_loop;
                                }
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
        if !config.plugin.loop_plugins {
            break 'main_loop;
        }
    }

    log::info!("Quitting Matricks.");

    // Stop the update thread
    match matrix.stop() {
        Ok(_) => {}
        Err(_) => {
            log::error!("Failed to stop matrix controller.");
        }
    }

    log::info!("Done.");
}
