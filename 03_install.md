---
layout: page
title: Installation
permalink: /install/
---

## Matricks
Matricks is intended only to be run on Raspberry Pi devices, and can be installed using Cargo after installing a few dependencies.

# Installation
- Install 64-bit Raspbian[^1] on your Raspberry Pi[^2].
- Install Rust and Cargo from [the Rust website](https://rustup.rs)
- Run `apt install libclang-dev libssl-dev`
- Install and configure the [rpi_ws281x library](https://github.com/rpi-ws281x/rpi_ws281x).
- Run `cargo install matricks`

  [^1]: At this time, Matricks can only be installed on 64-bit operating systems.
  [^2]: If you are using a Raspberry Pi with less than 1GB of RAM, compiling directly on the Pi is not recommended.

## Simtricks
Simtricks is a LED matrix simulator specifically for Matricks plugin development.

# Installation
- Install Rust and Cargo from [the Rust website](https://rustup.rs)
- Run `cargo install --git https://github.com/wymcg/simtricks`

---