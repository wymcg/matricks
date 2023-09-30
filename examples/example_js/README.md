# JavaScript Example Trick
This is a simple example Matricks plugin that will fade from black to white, before halting and sending a log.
**At this time, the only Mac and Linux are supported for Javascript plugin compilation.**

## Build
- Download the compiler install script with `curl -O https://raw.githubusercontent.com/extism/js-pdk/main/install.sh`
- Run the installer with `sh install.sh`
- Navigate to the `example_js` folder and run `extism-js example.js -o example_js.wasm`
- Run the plugin with [Matricks](https://github.com/wymcg/matricks) (on a Raspberry Pi) or with [Simtricks](https://github.com/wymcg/simtricks) (on other devices).
