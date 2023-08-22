// Setup global variables
let mat_config = {};
let counter = 0;

function setup() {
    // Populate the matrix configuration
    mat_config = JSON.parse(Host.inputString());
}

function update() {
    // Fill the matrix state
    let matrix = [];
    for (let y = 0; y < mat_config.height; y++) {
        matrix.push([])
        for (let x = 0; x < mat_config.width; x++) {
            matrix[y].push([counter, counter, counter, 0]);
        }
    }

    // Increment the counter
    counter++;


    if (counter == 255) {
        // Return the plugin update
        Host.outputString(JSON.stringify({
            state: matrix,
            done: true,
            log_message: ["Done fading to white!"]
        }));
    } else {
        // Increment the counter
        counter++;

        // Return the plugin update
        Host.outputString(JSON.stringify({
            state: matrix,
            done: false,
            log_message: null
        }));
    }

}

module.exports = {setup, update}