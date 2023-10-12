use extism_pdk::*;
use lazy_static::lazy_static;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref COUNTER: Arc<Mutex<u8>> = Arc::new(Mutex::new(0));
}

#[host_fn]
extern "ExtismHost" {
    fn matricks_debug(msg: &str);
    fn matricks_info(msg: &str);
    fn matricks_warn(msg: &str);
    fn matricks_error(msg: &str);
}

#[plugin_fn]
pub fn setup(_: ()) -> FnResult<()> {
    unsafe {
        matricks_debug("This is a debug log message!")?;
        matricks_info("This is an info log message!")?;
        matricks_warn("This is a warn log message!")?;
        matricks_error("This is an error log message!")?;
    }

    // Do setup tasks here!
    Ok(())
}

#[plugin_fn]
pub fn update(_: ()) -> FnResult<Json<Option<Vec<Vec<[u8; 4]>>>>> {
    let width: usize = config::get("width").unwrap().parse().unwrap();
    let height: usize = config::get("height").unwrap().parse().unwrap();

    // Get the static counter
    let mut counter = COUNTER.lock().unwrap();
    let counter = counter.deref_mut();

    // Make a 2D vector of all white, with intensity set by the counter value
    let led_state: Vec<Vec<[u8; 4]>> = vec![vec![[*counter; 4]; width]; height];

    // Increment counter and determine whether to stop providing updates
    if *counter == 255 {
        // If the counter is 255, let's stop the plugin and log why
        unsafe { matricks_info("Counter has reached 255, this plugin will stop providing updates")? }
        return Ok(Json(None)); // When Matricks sees this, it will stop the plugin.
    } else {
        // If the counter is less than 255, increment the counter
        *counter += 1;

        // Return the new led state
        return Ok(Json(Some(led_state)));
    }

}