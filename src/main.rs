mod clargs;
mod control;
mod plugin_iterator;
mod core;

use crate::clargs::{MatricksArgs, MatricksSubcommand};
use crate::core::matricks_core;

use std::fs;
use clap::Parser;
use crate::control::make_led_controller;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

fn main() {
    // Parse command line arguments
    let args = MatricksArgs::parse();

    // Start the logger
    env_logger::init();

    // Make an initial version log
    log::info!("Starting Matricks v{}.", VERSION.unwrap_or("unknown"));

    // Pull config from command line argument
    match args.config {
        MatricksSubcommand::Manual(config) => {
            log::info!("Matrix configuration has been manually supplied.");

            // Run the Matricks core using the manually supplied config
            matricks_core(config);
        }
        MatricksSubcommand::Auto(file_info) => {
            log::info!("Matrix configuration has been supplied via a configuration file.");

            // Read the file to a string
            let matrix_config_string_toml = match fs::read_to_string(&file_info.config_path) {
                Ok(string) => {string}
                Err(e) => {
                    log::error!("Failed to read config file at path \"{}\".", file_info.config_path);
                    log::debug!("Received the following error while attempting to read file: {e:?}");
                    log::info!("Quitting Matricks.");
                    return;
                }
            };

            // Pull the matrix config out of the string
            let config = match toml::from_str(&matrix_config_string_toml) {
                Ok(config) => {
                    log::info!("Matrix configuration has been supplied from a configuration file.");
                    config
                }
                Err(e) => {
                    log::error!("Failed to parse config file at path \"{}\".", file_info.config_path);
                    log::debug!("Received the following error while attempting to parse file: {e:?}");
                    log::info!("Quitting Matricks.");
                    return;
                }
            };

            // Run the Matricks core using the retrieved config
            matricks_core(config);
        }
        MatricksSubcommand::Save {
            info,
            matrix_config,
        } => {
            log::info!("Saving the matrix configuration.");

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

            match fs::write(&info.config_path, config_string_toml) {
                Ok(_) => {
                    log::info!("Successfully wrote matrix configuration to configuration file at path \"{}\"", info.config_path);
                    log::info!("Quitting Matricks.");
                    return;
                }
                Err(e) => {
                    log::error!("Failed to write matrix configuration to configuration file at path \"{}\"", info.config_path);
                    log::debug!("Received the following error while attempting to write matrix configuration to file: {e:?}");
                    log::info!("Quitting Matricks.");
                    return;
                }
            };
        }
        MatricksSubcommand::Clear => {
            log::info!("Clearing the matrix.");

            todo!("Clear the matrix here");
        }
    };

}
