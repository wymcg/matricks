# Matricks

> _"Teach an old matrix new tricks..."_

Matricks is a WASM-based extensible LED matrix control tool intended for use on Raspberry Pi devices.
LED matrix functionality is defined by user-developed plugins, or "tricks", which can be developed in any language that 
is supported by the [Extism PDK](https://extism.org/docs/category/write-a-plug-in). 
To simulate plugins while you're developing them, check out [Simtricks](https://github.com/wymcg/simtricks)!

## Run Matricks

### Installation on Raspberry Pi
- Install 64-bit Raspbian[^1] on your Raspberry Pi[^2]
- Install Rust and Cargo from [the Rust website](https://rustup.rs)
- Run `apt install libclang-dev libssl-dev`
- Run `cargo install matricks`
- [Configure your Raspberry Pi](#raspberry-pi-configuration) and reboot

### Use a pre-compiled binary
For convenience, pre-compiled binaries are available in the releases tab.
- [Configure your Raspberry Pi](#raspberry-pi-configuration) and reboot
- Run the following command to download and run Matricks:

```
MATRICKS_VERSION=0.3.1 && \
wget https://github.com/wymcg/matricks/releases/download/v$MATRICKS_VERSION/matricks_$MATRICKS_VERSION && \ 
chmod +x matricks_$MATRICKS_VERSION && \
./matricks_$MATRICKS_VERSION 
```

### Cross-compilation
- On another device,
  - Install Rust and Cargo from [the Rust website](https://rustup.rs)
  - Run `rustup target add aarch64-unknown-linux-musl`
  - Run `cargo install cross`
  - Clone this repository and build with `cross build --release --target aarch64-unknown-linux-musl`
  - Transfer the produced executable to your Raspberry Pi
- On your Raspberry Pi,
  - Install 64-bit Raspbian[^1]
  - [Configure your Raspberry Pi](#raspberry-pi-configuration) and reboot
  - Run the executable

## Usage
This section describes basic usage of Matricks. For general usage information, run `matricks help`.
For a list of plugins to try, there are several example plugins listed in the [examples README](./examples/README.md).

### Manual configuration
You may manually provide a configuration to Matricks using `matricks manual`.
To run a plugin (or a set of plugins in a directory), Matricks can be invoked as follows:

```
matricks manual [OPTIONS] --path <PLUGIN_PATH> --width <WIDTH> --height <HEIGHT>
```

This will run the plugin(s) at the given path on the connected matrix.
Other matrix and plugin configuration options are also available; See `matricks help manual` for more information.

### Saving a configuration
Once you have confirmed that everything is working with `matricks manual`, you can save your configuration to a file using the `matricks save` command.
To save your configuration, Matricks can be invoked as follows:

```
matricks save <NEW_CONFIG_PATH> [OPTIONS] --path <PLUGIN_PATH> --width <WIDTH> --height <HEIGHT>
```

This is similar to `matricks manual`, but instead of running the plugin, Matricks will save the configuration information to a new TOML file at the given path.
`matricks save` has the same matrix and plugin configuration options as `matricks manual`. 
See `matricks help save` for more information.

### Automatic configuration
If you have a TOML configuration file (created either by hand or by running `matricks save`), you can use it using `matricks auto`.
To run Matricks with a configuration file, Matricks can be invoked as follows:

```
matricks auto <CONFIG_PATH>
```

This command will use the configuration information in the given file to drive the matrix.
See `matricks help auto` for more information.

### Clearing the matrix
If for any reason you need to clear all LEDs on the matrix, Matricks can be invoked as follows:

```
matricks clear --width <WIDTH> --height <HEIGHT>
```

See `matricks help clear` for more information.

### View Logs
To see logs from Matricks, prepend your command with `RUST_LOG=matricks=info`.
For example:
```
RUST_LOG=matricks=info matricks auto your_config.toml
```

[^1]: At this time, Matricks can only be installed and run on 64-bit operating systems.
[^2]: If you are using a Raspberry Pi with less than 1GB of RAM, installation using this method is not recommended.

## Raspberry Pi Configuration
Matricks requires some configuration before it can be used to drive a LED matrix. 
If these instructions are not followed, Matricks may not work as expected.
This section paraphrases the instructions from the [rpi_ws281x README](https://github.com/jgarff/rpi_ws281x#spi).

### Enable SPI
The easiest way to enable SPI on Raspberry Pi is with the `raspi-config` command line tool.
Run `sudo raspi-config` and navigate to the SPI activation dialog by selecting `Interface Options > SPI`.

### Change GPU Core Frequency
Add the following lines to `/boot/config.txt`:

| Device         | Lines to Add                                    |
|----------------|-------------------------------------------------|
| Raspberry Pi 3 | ```core_freq=250```                             |
| Raspberry Pi 4 | ```core_freq=500```<br/>```core_freq_min=500``` |

### Change SPI Buffer Size (optional)
On some distributions, it may be necessary to increase the maximum SPI transfer size by editing `/boot/cmdline.txt` and adding the following line:

```
spidev.bufsiz=32768
```