use serde::Deserialize;

/// Update retrieved from the plugin every call to the update function
#[derive(Deserialize)]
pub struct PluginUpdate {
    pub state: Vec<Vec<[u8; 4]>>,
    pub done: bool,
    pub log_message: Option<String>,
}