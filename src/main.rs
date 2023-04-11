mod clargs;
mod logging;
mod plugin_iterator;

use std::sync::mpsc::Sender;
use clargs::Args;
use plugin_iterator::PluginIterator;

use crate::logging::log_thread::LoggingThread;
use clap::Parser;
use extism::{Context, Plugin};
use crate::logging::log::Log;
use crate::logging::log_origin::LogOrigin;
use crate::logging::log_type::LogType;
use crate::plugin_iterator::PluginIteratorError;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

fn main() {
    // parse command line arguments
    let args = Args::parse();

    // start the logging thread
    let (log_thread_handle, log_tx) = LoggingThread::new(args.log_dir).start();

    // make an initial version log
    log_main(
        &log_tx,
        LogType::Normal,
        format!("Starting Matricks v{}", VERSION.unwrap_or("unknown")));

    // make the plugin iterator
    let plugin_data_list = match PluginIterator::new(args.plugins)
    {
        Ok(plugin_iterator) => {plugin_iterator}
        Err(e) => {
            log_main(&log_tx, LogType::Error, format!("Failed to instantiate plugin list. See error: <{e:?}>"));
            log_main(&log_tx, LogType::Normal, format!("Quitting Matricks."));
            return;
        }
    };

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
                        return
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
        log_main(&log_tx, LogType::Normal, format!("Starting plugin '{plugin_path}'"));
        let mut plugin = match Plugin::new(&context, plugin_data, [], true) {
            Ok(plugin) => plugin,
            Err(_) => {
                log_main(&log_tx, LogType::Warning, format!("Unable to instantiate plugin '{plugin_path}'. This plugin will be skipped."));
                continue;
            }
        };

        // call setup and update here

    }

    log_main(&log_tx, LogType::Normal, format!("Quitting Matricks."));

    // close channels
    drop(log_tx);

    // join logging and matrix control threads
    log_thread_handle
        .join()
        .expect("Unable to join log thread!");

}

/// Send a log from the main thread
fn log_main(log_tx: &Sender<Log>, log_type: LogType, description: String) {
    log_tx.send(
        Log::new(
            LogOrigin::MainThread,
            log_type,
            description
        )).expect("Unable to send log from main thread!");
}
