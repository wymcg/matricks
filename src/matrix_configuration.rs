use serde::Serialize;

/// Matrix configuration information to be passed to the plugin's setup function
#[derive(Serialize)]
pub struct MatrixConfiguration {
    pub width: usize,
    pub height: usize,
    pub target_fps: u32,
}
