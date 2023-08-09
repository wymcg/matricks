# Rust Example Trick
This is a simple example Matricks plugin that will fade from black to white, before halting and sending a log.

## Build
- Install the `wasm32-wasi` toolchain by running `rustup target add wasm32-wasi`
- Navigate to the `example_rust` folder and run `cargo build --release --target wasm32-wasi`
- Run the plugin with [Matricks](https://github.com/wymcg/matricks) (on a Raspberry Pi) or with [Simtricks](https://github.com/wymcg/simtricks) (on other devices).