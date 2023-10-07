---
layout: page
title: Developing Plugins
permalink: /plugins/
---

This page describes the basic structure of Matricks plugins, and how to develop a simple plugin in Rust. 
***If you're interested in examples of plugin development in other languages, check out the examples on the [GitHub](https://github.com/wymcg/matricks).***

## Prerequisites
### Install Matricks
See installation instructions for Matricks [here](/03_install.md).

### Install the Simulator (Optional)
Simtricks is a simulator for Matricks plugin development. For installation instructions and usage information, check out [Simtricks](https://github.com/wymcg/simtricks).

### Install Rust and the `wasm32-wasi` Target
If you don't have Rust installed already, you can do so by running the command listed on the [rustup website](https://rustup.rs/). 
Once you have Rust installed, use `rustup` to install the `wasm32-wasi` toolchain:
```
rustup target add wasm32-wasi
```

## Plugin Structure

### The Setup Function
The setup function is called once when the plugin is initialized by Matricks. 
It takes one parameter, a string, and returns nothing. 
The only input parameter of this function is a JSON-format string containing information about how the LED matrix is configured (i.e. width, height, target FPS, etc.). 
The setup function is intended to be used to save this configuration information, and for any initial setup the plugin may need.

### The Update Function
The update function is called once per frame to get matrix state information from the plugin. 
It takes no parameters, and returns information about the matrix state, including the color of each LED in the matrix and whether the plugin is done providing updates to the matrix.

## Project Setup

### Project Creation
First, make a new Rust library using Cargo.
```
cargo new --lib <NAME OF YOUR PLUGIN>
```
For the sake of this document, we'll use the name `example_trick`.

### `Cargo.toml`
We need to add a few things to our `Cargo.toml`. 
Add the following lines to specify that this library should be compiled as a C/C++-style dynamic library:
```toml
[lib]
crate-type = ["cdylib"]
```
There are a few required dependencies. Add the following dependencies to your `Cargo.toml`:
```toml
[dependencies]
extism-pdk = "0.3.1"
matricks_plugin = "0.1.1"
serde_json = "1.0.96"
lazy_static = "1.4.0"
```
Here's a table explaining what each of them are for:

| Dependency        | Purpose                                                                                                                                                   |
| ----------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `extism-pdk`      | Gives us the `#[plugin_fn]` macro, which lets us tell the compiler which functions we want to expose to Matricks (just the setup and update functions)    |
| `matricks_plugin` | A convenience crate that gives us structs for data coming into and out of the plugin                                                                      |
| `serde_json`      | An extension to `serde` that lets us turn our data to and from strings so that we can communicate with Matricks                                           |
| `lazy_static`     | Allows us to make static variables at runtime. This is required to remember state and configuration data between calls to the setup and update functions. |

### `lib.rs`
Delete the contents of `lib.rs` and import your dependencies:
```rust
use std::ops::DerefMut; 
use std::sync::{Mutex, Arc};
use extism_pdk::*;
use matricks_plugin::{MatrixConfiguration, PluginUpdate};
use serde_json::from_str;
use lazy_static::lazy_static;
```

#### The Setup Function
Define the setup function.
```rust
#[plugin_fn]
pub fn setup(Json(cfg_json): Json<MatrixConfiguration>) -> FnResult<()> {
	
	/* Setup tasks (i.e. saving config information) */
	
    Ok(())
}
```
Although the setup function doesn't return anything, the Extism PDK for Rust requires that all plugins require a `FnResult`, so we'll return an empty `FnResult`.

Also, note the `#[plugin_fn]` macro at the top. This tells the Extism PDK to prepare this function for communication with Matricks.

#### The Update Function
Define the update function.
```rust
#[plugin_fn]
pub fn update(_: ()) -> FnResult<Json<PluginUpdate>> {
	/* Make a new PluginUpdate with information about the matrix state */
	
	// Using default values for example here
    Ok(Json(PluginUpdate::default()))
}
```
Here, we would normally make a new `PluginUpdate` with the new LED state information (among other things). 
For now, we'll use the default values. 
Again, note the `#[plugin_fn]` macro from the Extism PDK at the top.

Now, you are able to build the plugin and run it with Matricks. 
As is, the plugin isn't very interesting (and will behave erratically since the `PluginUpdate` we return from the update function contains no LED state information by default), but it can be built and run by Matricks without issue.

## Building the Plugin
To build the plugin, use the Cargo build command and specify `wasm32-wasi` as the target:
```
cargo build --release --target wasm32-wasi
```
This will create a .wasm file in `target/wasm32-wasi/release/<NAME OF YOUR CRATE>.wasm`. 
This is your new plugin, ready to be used by Matricks.

## Running Plugins
To run your plugin on a Raspberry Pi, transfer the `.wasm` file to your Raspberry Pi and run the following command
```
matricks manual --width <WIDTH> --height <HEIGHT> --path <PATH TO PLUGIN/DIRECTORY>
```
To try out your plugin on non-Raspberry Pi devices, try [Simtricks](https://github.com/wymcg/simtricks).

If Matricks is given a path to a plugin (i.e. a .wasm file), it will run only that plugin. 
If it is given a path to a directory, Matricks will recursively find all plugins in the directory and run them all, one after another.

If Matricks detects multiple plugins (i.e. a directory full of plugins), Matricks will run each of them each once, one after another.
Matricks will move on to the next plugin when the currently running plugin reports that it will not provide any further updates via the `done` field in `PluginResult`, or when the plugin has been running for longer than the user-specified limit.

Matricks offers other configuration options, including FPS, brightness, plugin time limit, and more. For more information, run `matricks help manual`.

## Adding Plugin Functionality

### Plugin State
In order to preserve state between calls to the update and setup functions. you may use the `lazy_static!` macro to create structs that will persist between calls. 
For plugins written in Rust, it is common to put the reference behind a `Arc<Mutex<T>>`. 
For example, to make a place to store config information[^configplan] from setup you may do the following:
```rust
lazy_static! {  
    static ref CONFIG: Arc<Mutex<MatrixConfiguration>> =  
        Arc::new(Mutex::new(MatrixConfiguration::default()));  
}
```
To to set this `MatrixConfiguration` struct from setup might look like this:
```rust
#[plugin_fn]  
pub fn setup(Json(cfg_json): Json<MatrixConfiguration>) -> FnResult<()> {  
    let mut config = CONFIG.lock().unwrap();  
    let config = config.deref_mut();  
    *config = cfg_json;  
	  
    Ok(())  
}
```
You may make any custom struct persistent between calls in a similar way. 
See the basic example [here](https://github.com/wymcg/matricks/tree/main/examples/example_rust) for more information.

[^configplan]: The mechanism for giving the plugin matrix configuration information will be changed in Matricks v0.3.0. See [this issue](https://github.com/wymcg/matricks/tree/main/examples/example_rust) for more information.

### Setting LED Color
LED color information is sent to Matricks via the `state` field in the `PluginUpdate` struct returned from the update function. 
The `state` field is a two-dimensional vector of LED colors, where `state[y][x]` is the LED at coordinate `(x,y)`. 
Each piece of LED color information is an array of 4 `u8` values, in BGRA order.

Below is a basic update function which sets every LED to be red each frame. It uses the  
technique from before to save the matrix configuration in the setup function, and uses  
the width and height information to construct the LED state.

```rust
#[plugin_fn]
pub fn update(_: ()) -> FnResult<Json<PluginUpdate>> {
	// get the static config object
	let config = CONFIG.lock().unwrap();

	// make a 2D vector of all red
	let mut led_state: Vec<Vec<[u8; 4]>> = vec![];
	for y in 0..config.width {
		led_state.push(vec![]);
		for _x in 0..config.height {
			led_state[y].push([0, 0, 255, 0]);
		}
	}

	Ok(Json(PluginUpdate {
		state: led_state,
		..Default::default()
	}))
}
```

### Logging from Plugins
One or more logs can be made from plugins by populating the `log_message` field in the `PluginUpdate` struct returned from the update function[^logplan]. 
If the field is left as `None`, no logs will be made. 
If it is populated with a list of strings, Matricks will make one normal plugin log for the plugin per string in the list, with each string as a description for one of the plugins.

[^logplan]: The plugin logging mechanism will be changed in Matricks v0.3.0. See [this issue](https://github.com/wymcg/matricks/issues/39) for more information.

### Stopping Plugins
If you would like to have a plugin stop providing updates to Matricks, you can do so by setting the `done` field in the `PluginUpdate` struct returned from the update function to `true`.
When Matricks sees that this field is true, it will make one final update to the matrix using the contents of the `state` field, and then move on to the next available plugin.

### A Note on WASM/WASI Compatibility
You may find that when using a Rust crate in your plugin project, features may be broken or your project may have compilation issues. 
WASM (and especially WASI) are still very new, and so certain libraries may have compatibility issues that may not be addressed. 
In these cases, seek out alternative crates; some crates may specifically advertise WASM compatibility.

WASM/WASI compatibility issues are becoming rarer as the platform matures, but issues do occasionally appear. 
Because of this, it is recommended to make a small test plugin to confirm that the libraries you would like to use in your plugin work as expected.

## Conclusion
Hopefully, you've been able to get your hands dirty with some Matricks plugin development! If you have a question that hasn't been answered by this document, here's a few links that might help.

- [Matricks GitHub](https://github.com/wymcg/matricks)
- [Example Plugins](https://github.com/wymcg/matricks/issues/39)
- [matricks_plugin docs.rs](https://docs.rs/matricks_plugin/latest/matricks_plugin/)
- [Extism PDK for Rust](https://extism.org/docs/write-a-plugin/rust-pdk/)
- [Extism PDK for other languages](https://extism.org/docs/category/write-a-plug-in)

If you like this project, contribute on [GitHub](https://github.com/wymcg/matricks/issues)!

---