# Matricks

> _"Teach an old matrix new tricks..."_

Matricks is a WASM-based extensible LED matrix control tool intended for use on Raspberry Pi devices.
LED matrix functionality is defined by user-developed plugins, or "tricks", which can be developed in any language that 
is supported by the [Extism PDK](https://extism.org/docs/category/write-a-plug-in). 
To simulate plugins while you're developing them, check out [Simtricks](https://github.com/wymcg/simtricks)!

See usage details below:

```
Usage: matricks [OPTIONS] --plugins <PLUGINS> --width <WIDTH> --height <HEIGHT>

Options:
  -p, --plugins <PLUGINS>              Path to plugin or directory of plugins
  -x, --width <WIDTH>                  Width of the matrix, in number of LEDs
  -y, --height <HEIGHT>                Height of the matrix, in number of LEDs
  -f, --fps <FPS>                      Target framerate at which to drive the matrix [default: 30]
  -L, --log <LOG_DIR>                  Directory to write logs [default: log]
  -s, --serpentine                     Data line alternates direction between columns or rows
  -b, --brightness <BRIGHTNESS>        Brightness of matrix, from 0-255 [default: 255]
  -t, --time-limit <TIME_LIMIT>        Maximum time (in seconds) that a single plugin can run before moving on to the next one. No time limit by default
  -l, --loop                           Loop plugin or set of plugins indefinitely
  -h, --help                           Print help
  -V, --version                        Print version
```

## Installation
- Install 64-bit Raspbian[^1] on your Raspberry Pi[^2].
- Install Rust and Cargo from [the Rust website](https://rustup.rs)
- Run `apt install libclang-dev libssl-dev`
- Install and configure the [rpi_ws281x library](https://github.com/rpi-ws281x/rpi_ws281x).
- Run `cargo install matricks`

[^1]: At this time, Matricks can only be installed on 64-bit operating systems.
[^2]: If you are using a Raspberry Pi with less than 1GB of RAM, compiling directly on the Pi is not recommended.
