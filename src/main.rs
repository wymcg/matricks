mod clargs;
mod core;
mod path_map;
mod plugin_iterator;
mod plugin_logs;
mod matrix;

use crate::clargs::{MatricksArgs, MatricksSubcommand};
use crate::core::matricks_core;

use crate::matrix::matrix_control::clear_matrix;
use clap::Parser;
use rs_ws281x::{ChannelBuilder, ControllerBuilder, StripType};
use std::{env, fs};

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
const DEFAULT_LOG_LEVEL: &str = "matricks=info";

fn main() {
    // Parse command line arguments
    let args = MatricksArgs::parse();

    // If the user has not setup the logger themselves, set the log level to the default
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", DEFAULT_LOG_LEVEL);
    }

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
                Ok(string) => string,
                Err(e) => {
                    log::error!(
                        "Failed to read config file at path \"{}\".",
                        file_info.config_path
                    );
                    log::debug!(
                        "Received the following error while attempting to read file: {e:?}"
                    );
                    log::info!("Quitting Matricks.");
                    return;
                }
            };

            // Pull the matrix config out of the string
            let config = match toml::from_str(&matrix_config_string_toml) {
                Ok(config) => config,
                Err(e) => {
                    log::error!(
                        "Failed to parse config file at path \"{}\".",
                        file_info.config_path
                    );
                    log::debug!(
                        "Received the following error while attempting to parse file: {e:?}"
                    );
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
                Ok(string) => string,
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
                    log::error!(
                        "Failed to write matrix configuration to configuration file at path \"{}\"",
                        info.config_path
                    );
                    log::debug!("Received the following error while attempting to write matrix configuration to file: {e:?}");
                    log::info!("Quitting Matricks.");
                    return;
                }
            };
        }
        MatricksSubcommand::Clear(matrix_config) => {
            log::info!("Clearing the matrix.");

            // Make an LED controller
            let mut controller = match ControllerBuilder::new()
                .freq(matrix_config.controller.frequency)
                .dma(matrix_config.controller.dma as i32)
                .channel(
                    0, // channel index
                    ChannelBuilder::new()
                        .pin(matrix_config.controller.gpio as i32)
                        .count((matrix_config.width * matrix_config.height) as i32)
                        .strip_type(StripType::Ws2812)
                        .brightness(matrix_config.brightness)
                        .build(),
                )
                .build()
            {
                Ok(c) => c,
                Err(e) => {
                    log::error!("Failed to create LED controller.");
                    log::debug!("Received the following error while attempting to create LED controller: {e:?}");
                    log::info!("Quitting Matricks.");
                    return;
                }
            };

            // Clear the matrix using the controller we just made
            match clear_matrix(&mut controller) {
                Ok(_) => {
                    log::info!("Successfully cleared matrix.");
                    log::info!("Quitting Matricks.");
                }
                Err(e) => {
                    log::error!("Failed to clear matrix.");
                    log::debug!("Received the following error while clear matrix: {e:?}");
                    log::info!("Quitting Matricks.");
                }
            }
        }
    };
}
