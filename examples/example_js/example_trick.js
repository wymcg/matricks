// Make a counter, and start it at zero
let counter = 0;

function setup() {
    // Nothing to do here!
}

function update() {
    // Get the width and height from the config
    width = Config.get("width")
    height = Config.get("height")

    // Create a matrix state of all white, with intesity set by the counter value
    let matrix_state = [];
    for (let y = 0; y < height; y++){
        matrix_state.push([])
        for (let x = 0; x < width; x++){
            matrix_state[y].push([counter, counter, counter, counter]);
        }
    }

    if (counter >= 255) {
        // Stop the plugin if the counter is at the max value
        Host.outputString(JSON.stringify(null));
    } else {
        // Otherwise, increment the counter and return the new matrix state
        counter++;
        Host.outputString(JSON.stringify(matrix_state));
    }
}

module.exports = {setup, update}