use serde::Serialize;

/// Matrix configuration information to be passed to the plugin's setup function
#[derive(Serialize)]
pub struct MatrixConfiguration {
    /// Width of the matrix, in number of LEDs
    pub width: usize,

    /// Height of the matrix, in number of LEDs
    pub height: usize,

    /// FPS that the program will attempt to drive the LEDs
    pub target_fps: f32,

    /// Data line alternates direction between columns or rows
    /// In other words, every other row or column is reversed
    pub serpentine: bool,

    #[cfg(not(target_arch = "aarch64"))]
    /// Magnification of the simulated matrix
    pub magnification: f32,
}
