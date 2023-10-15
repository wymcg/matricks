---
layout: page
title: Installation
permalink: /install/
---

Matricks is intended only to be run on Raspberry Pi devices, and can be installed using Cargo after installing a few dependencies.
To run plugins on non-Raspberry Pi devices, check out [Simtricks](https://github.com/wymcg/simtricks).

## Installation on Raspberry Pi
- Install 64-bit Raspbian[^1] on your Raspberry Pi[^2]
- Install Rust and Cargo from [the Rust website](https://rustup.rs)
- Run `apt install libclang-dev libssl-dev`
- Install and configure the [rpi_ws281x library](https://github.com/rpi-ws281x/rpi_ws281x).
- Run `cargo install matricks`

## Cross-compilation for Raspberry Pi
- On another device,
  - Install Rust and Cargo from [the Rust website](https://rustup.rs)
  - Run `rustup target add aarch64-unknown-linux-musl`
  - Run `cargo install cross`
  - Clone this repository and build with `cross build --release --target aarch64-unknown-linux-musl`
  - Transfer the produced executable to your Raspberry Pi
- On your Raspberry Pi,
  - Install 64-bit Raspbian[^1]
  - Install and configure the [rpi_ws281x library](https://github.com/rpi-ws281x/rpi_ws281x).
  - Run the executable

## Use a pre-compiled binary
For convenience, pre-compiled binaries are available in the [releases](https://github.com/wymcg/matricks/releases) tab.
- Run `apt install libclang-dev libssl-dev`
- Install and configure the [rpi_ws281x library](https://github.com/rpi-ws281x/rpi_ws281x).

Then, you may run the following command to download and run Matricks:
```
MATRICKS_VERSION=0.3.0 && \
wget https://github.com/wymcg/matricks/releases/download/v$MATRICKS_VERSION/matricks_$MATRICKS_VERSION && \ 
chmod +x matricks_$MATRICKS_VERSION && \
./matricks_$MATRICKS_VERSION 
```

[^1]: At this time, Matricks can only be installed and run on 64-bit operating systems.
[^2]: If you are using a Raspberry Pi with less than 1GB of RAM, installation using this method is not recommended.

---