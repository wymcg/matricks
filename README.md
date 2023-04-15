# Matricks

> _"Teach an old matrix new tricks..."_

Matricks is a WASM-based extensible LED matrix control tool intended for use on Raspberry Pi devices.
LED matrix functionality is defined by user-developed plugins, or "tricks", which can be developed in any language that 
is supported by the [Extism PDK](https://extism.org/docs/category/write-a-plug-in). 
On non-Raspberry Pi devices, Matricks will simulate a LED matrix and display the simulated matrix state in real time.

See usage details below:

```
Usage: matricks [OPTIONS] --plugins <PLUGINS> --width <WIDTH> --height <HEIGHT>

Options:
  -p, --plugins <PLUGINS>              Path to plugin or directory of plugins
  -x, --width <WIDTH>                  Width of the matrix, in number of LEDs
  -y, --height <HEIGHT>                Height of the matrix, in number of LEDs
  -f, --fps <FPS>                      Target framerate at which to drive the matrix [default: 30]
  -l, --log-dir <LOG_DIR>              Directory to write logs [default: log]
  -s, --serpentine                     Data line alternates direction between columns or rows
  -m, --magnification <MAGNIFICATION>  Magnification of the simulated matrix [default: 10]
  -h, --help                           Print help
  -V, --version                        Print version
```

## Installation
Matricks is installed using Cargo. 
In order to run a simulated matrix on non-Raspberry Pi machines, 
OpenCV and supported libraries must be installed before installing Matricks.
See platform-specific instructions below.

### Raspberry Pi
- Install Rust and Cargo from [the Rust website](https://rustup.rs)
- Run `apt install libclang-dev`
- Run `cargo install matricks`

### Windows
- Install Rust and Cargo from [the Rust website](https://rustup.rs)
- Install Chocolatey from [the Chocolatey website](https://chocolatey.org/install)
- Install vcpkg from the [the vcpkg website](https://vcpkg.io/en/getting-started.html)
- Run `choco install llvm opencv`
- Run `vcpkg install llvm opencv4[contrib,nonfree]`
- Run `cargo install matricks`

### Ubuntu
- Install Rust and Cargo from [the Rust website](https://rustup.rs)
- Run `apt install libopencv-dev clang libclang-dev`
- Run `cargo install matricks`

### Arch Linux
- Install Rust and Cargo from [the Rust website](https://rustup.rs)
- Run `pacman -S clang qt5-base opencv`
- Run `pacman -S vtk glew fmt openmpi`
- Run `cargo install matricks`

### Mac
- Install Rust and Cargo from [the Rust website](https://rustup.rs)
- Run `brew install opencv`
- Run `cargo install matricks`

Note: for installation on Mac, you will likely also need a C++ compiler and libclang (`brew install llvm`).
