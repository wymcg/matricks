mod clargs;
mod matrix_control;
mod plugin_iterator;
mod matricks;

use crate::clargs::{MatricksArgs, MatricksSubcommand};
use crate::matricks::core;

use std::fs;
use clap::Parser;

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

    core(config);
}
