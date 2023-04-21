---
layout: page
title: Installation
permalink: /install/
---

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