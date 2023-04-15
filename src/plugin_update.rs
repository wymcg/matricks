use serde::Deserialize;

/// Update retrieved from the plugin every call to the update function
#[derive(Deserialize, Clone)]
pub struct PluginUpdate {
    /// State of each LED in the matrix, as a two-dimensional matrix of BGRA values
    pub state: Vec<Vec<[u8; 4]>>,

    /// Whether or not the plugin is done providing updates.
    ///
    /// If this is ever set to true, the main thread will move on to the next available plugin
    pub done: bool,

    /// Logs made by the plugin
    ///
    /// If this is not None, the main thread will log the strings in the list on behalf of the plugin
    pub log_message: Option<Vec<String>>,
}
