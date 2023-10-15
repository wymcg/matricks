---
layout: page
title: Developing Plugins
permalink: /plugins/
---

This page describes the basics of Matricks plugins.
Plugins may be developed in any language that is supported by the [Extism PDK](https://extism.org/docs/category/write-a-plug-in).
Much of the information on this page applies to plugins in all compatible languages, but all example code given here will be in Rust.
***If you're interested in examples of plugin development in other languages, check out the examples on the [GitHub](https://github.com/wymcg/matricks).***

## Prerequisites
### Install Matricks
To run your plugins on a Raspberry Pi, download and install Matricks. 
See installation instructions for Matricks [here](/03_install.md).

### Install Simtricks (Optional)
To run your plugins on non-Raspberry Pi devices, download and install Simtricks, a simulator for Matricks plugin testing and development. 
For installation instructions and usage information, check out [Simtricks](https://github.com/wymcg/simtricks).

### Install Rust and the `wasm32-wasi` target
If you don't have Rust installed already, you can do so by visiting the [rustup website](https://rustup.rs/). 
Once you have Rust installed, use `rustup` to install the `wasm32-wasi` toolchain:
```
rustup target add wasm32-wasi
```

### Create a new plugin from a template (Optional)
You can use `cargo-generate` to create a new empty Matricks plugin.
The [Matricks plugin template](https://github.com/wymcg/matricks_plugin_template.git) sets up the `update` and `setup` functions, as well as the Matricks logging functions.
First, install `cargo-generate`:
```
cargo install cargo-generate
```

Next, use `cargo-generate` to make a new project:
```
cargo generate --git https://github.com/wymcg/matricks_plugin_template.git
```

Once you have generated your new project, you can navigate to the new project and confirm that it builds:
```
cargo build --release --target wasm32-wasi
```

## Plugin structure
Matricks plugins have two parts: the `setup` function, and the `update` function. 

### The `setup` function
The `setup` function is called once when the plugin is instantiated. 
In this function, you may perform any initialization your plugin needs. 
In simple plugins, this function may be empty.

In Rust, the setup function is declared as follows:
```rust
#[plugin_fn]
pub fn setup(_: ()) -> FnResult<()> {
    // Setup your plugin here!
    
    Ok(())
}
```

### The `update` function
The `update` function is called once per frame, and provides the next state of the matrix to Matricks.
The next matrix state is returned as a JSON string containing a two-dimensional array of BGRA color values.
For example, a 3x3 matrix showing the color blue on all LEDs would be represented as:
```json
[
  [[255, 0, 0, 255], [255, 0, 0, 255], [255, 0, 0, 255]],
  [[255, 0, 0, 255], [255, 0, 0, 255], [255, 0, 0, 255]],
  [[255, 0, 0, 255], [255, 0, 0, 255], [255, 0, 0, 255]]
]
```
If the `update` function returns `null` instead, Matricks will stop requesting new updates from this plugin. 

In Rust, a setup function that returns a blue 3x3 matrix state as shown above might look like the following:
```rust
#[plugin_fn]
pub fn update(_: ()) -> FnResult<Json<Option<Vec<Vec<[u8; 4]>>>>> {
    // --snip--
    
    // Return a JSON-format string containing 3x3 matrix of all blue
    Ok(Json(Some(vec![vec![[255, 0, 0, 255]; 3]; 3])))
    
    // Or, if you want to stop this plugin, return JSON null instead
    Ok(Json(None))
}
```

### Retrieving configuration info
Matricks provides information about the connected LED matrix using a key-value store. 
The following keys are available:

| Key      | Description                           |
|----------|---------------------------------------|
| `width`  | Width of the LED matrix, in number of LEDs |
| `height` | Height of the LED matrix, in number of LEDs |
| `target_fps` | The target framerate of the matrix    |
| `brightness` | The brightness of the matrix, from 0-255 |

The values stored in this key-value store are strings, and must be converted to the desired type before use.

The method for accessing the config key-value store differs between [supported languages](https://extism.org/docs/category/write-a-plug-in).
Consult the Extism PDK documentation for information on your desired language.
In Rust, pulling the width of the matrix from the config might look like this:
```rust
let width: usize = config::get("width")     // Option<String>
    .unwrap()                               // String
    .parse()                                // Result<usize, Err>
    .unwrap();                              // usize
```

### Logging from a plugin
Several host functions are available which allow plugins to make logs through Matricks:

| Name             | Description                        |
|------------------|------------------------------------|
| `matricks_debug` | Make a debug log through Matricks  |
| `matricks_info`  | Make an info log through Matricks  |
| `matricks_warn`  | Make a warn log through Matricks   |
| `matricks_error` | Make an error log through Matricks |

The method for calling host functions differs between [supported languages](https://extism.org/docs/category/write-a-plug-in).
Consult the Extism PDK documentation for information on your desired language.
In Rust, declaring and calling the logging functions might look like this:
```rust
// Declare the debug functions
#[host_fn]
extern "ExtismHost" {
    fn matricks_debug(msg: &str);
    fn matricks_info(msg: &str);
    fn matricks_warn(msg: &str);
    fn matricks_error(msg: &str);
}

// --snip--

// Call the log functions
unsafe { // Foreign functions in Rust must be called within "unsafe" blocks!
    matricks_debug("This is a debug message!")?;
    matricks_info("This is an info message!")?;
    matricks_warn("This is a warning message!")?;
    matricks_error("This is an error message!")?;
}
```

### Maintaining plugin state
It is very common to want to maintain some sort of plugin state between calls to the `setup` and `update` functions. 
For example, you might want to initialize a counter in the `setup` function, and increment it every time the `update` function is called.

In some supported languages, it is possible to declare state variables globally so that they can be accessed from the
`setup` and `update` functions. For example, in Rust, you may use the `lazy_static` crate to do the following:
```rust
use lazy_static::lazy_static;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref COUNTER: Arc<Mutex<u8>> = Arc::new(Mutex::new(0usize));
}

// --snip--

#[plugin_fn]
pub fn update(_: ()) -> FnResult<Json<Option<Vec<Vec<[u8; 4]>>>>> {
    // --snip--
    
    // Grab a mutable reference to the counter
    let mut counter = COUNTER.lock().unwrap();
    let counter = counter.deref_mut();
    
    // Increment the counter
    *counter += 1;
    
    // --snip--
}
```

Alternatively, Extism provides a key-value store for plugins.
The method for accessing this key-value store differs between [supported languages](https://extism.org/docs/category/write-a-plug-in).
Complex data structures can be difficult to use with this key-value store, so it is generally recommended to avoid this method if possible.

### A note on WASM/WASI compatibility
You may find that when using a Rust crate in your plugin project, features may be broken or your project may have compilation issues. 
WASM (and especially WASI) are still very new, and so certain libraries may have compatibility issues that may not be addressed. 
In these cases, seek out alternative crates; some crates may specifically advertise WASM compatibility.

WASM/WASI compatibility issues are becoming rarer as the platform matures, but issues do occasionally appear. 
Because of this, it is recommended to make a small test plugin to confirm that the libraries you would like to use in your plugin work as expected.

## Conclusion
Hopefully, you've been able to get your hands dirty with some Matricks plugin development! If you have a question that hasn't been answered by this document, here's a few links that might help.

- [Matricks GitHub](https://github.com/wymcg/matricks)
- [Example Plugins](https://github.com/wymcg/matricks/issues/39)
- [Extism PDK for Rust](https://extism.org/docs/write-a-plugin/rust-pdk/)
- [Extism PDK for other languages](https://extism.org/docs/category/write-a-plug-in)



---