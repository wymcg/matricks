use extism_pdk::*;
use lazy_static::lazy_static;
use matricks_plugin::{MatrixConfiguration, PluginUpdate};
use serde_json::from_str;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref CONFIG: Arc<Mutex<MatrixConfiguration>> =
        Arc::new(Mutex::new(MatrixConfiguration::default()));
    static ref COUNTER: Arc<Mutex<u8>> = Arc::new(Mutex::new(0));
}

#[plugin_fn]
pub fn setup(cfg_json: String) -> FnResult<()> {
    // Set the matrix configuration struct
    let mut config = CONFIG.lock().unwrap();
    let config = config.deref_mut();
    *config = from_str(&*cfg_json)
        .expect("Unable to deserialize matrix config!");

    Ok(())
}

#[plugin_fn]
pub fn update(_: ()) -> FnResult<Json<PluginUpdate>> {
    // Get the static config object
    let config = CONFIG.lock().unwrap();

    // Get the static counter
    let mut counter = COUNTER.lock().unwrap();
    let counter = counter.deref_mut();

    // Make a 2D vector of all white, with intensity set by the counter value
    let led_state: Vec<Vec<[u8; 4]>> = vec![vec![[*counter; 4]; config.width]; config.height];

    // Increment counter and determine whether to stop providing updates
    if *counter == 255 {
        // If the counter is 255, let's stop the plugin and log why
        Ok(Json(PluginUpdate {
            state: led_state,
            done: true,
            log_message: Some(vec!["Done fading to white!".to_string()]),
            ..Default::default()
        }))
    } else {
        // If the counter is less than 255, increment the counter
        *counter += 1;

        Ok(Json(PluginUpdate {
            state: led_state,
            done: false,
            ..Default::default()
        }))
    }

}