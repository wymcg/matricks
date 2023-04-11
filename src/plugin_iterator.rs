use glob::glob;
use regex::Regex;
use std::fs;
use std::fs::metadata;

/// Error types for PluginIterator
#[derive(Debug)]
pub enum PluginIteratorError {
    InvalidSeedPath(String),   // The seed path is not valid
    InvalidPluginPath(String), // The given plugin path is not valid
}

/// Generate Extism plugin environments given a path
/// If the path is a plugin on its own
pub struct PluginIterator {
    pub seed_path: String,
    plugin_paths: Vec<String>,
    current_plugin_idx: usize,
}

impl PluginIterator {
    pub fn new(seed_path: String) -> Result<Self, PluginIteratorError> {
        // get path metadata
        let md = match metadata(&seed_path) {
            Ok(md) => md,
            Err(_) => return Err(PluginIteratorError::InvalidSeedPath(seed_path)),
        };

        if md.is_dir() {
            // get a list of paths to all .wasm files in the directory
            match glob(format!("{seed_path}/**/*.wasm").as_str()) {
                Ok(paths) => {
                    // make the list of plugin paths
                    let mut plugin_paths: Vec<String> = vec![];
                    for entry in paths {
                        match entry {
                            Ok(path) => {
                                plugin_paths.push(String::from(path.to_str().unwrap()));
                            }
                            Err(_) => return Err(PluginIteratorError::InvalidSeedPath(seed_path)),
                        }
                    }

                    // return the result
                    Ok(PluginIterator {
                        seed_path,
                        plugin_paths,
                        current_plugin_idx: 0,
                    })
                }
                Err(_) => Err(PluginIteratorError::InvalidSeedPath(seed_path)),
            }
        } else if md.is_file() && Regex::new(r"^.+\.wasm").unwrap().is_match(&seed_path) {
            // this path is already a valid wasm plugin path, so return an iterator with just that path in it
            Ok(PluginIterator {
                seed_path: seed_path.clone(),
                plugin_paths: vec![seed_path],
                current_plugin_idx: 0,
            })
        } else {
            Err(PluginIteratorError::InvalidSeedPath(seed_path))
        }
    }
}

impl Iterator for PluginIterator {
    type Item = Result<(String, Vec<u8>), PluginIteratorError>;

    /// Iterate through each plugin and return the data in the .wasm file, along with the plugin path
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_plugin_idx >= self.plugin_paths.len() {
            return None;
        }

        // grab the plugin path
        let plugin_path = self.plugin_paths[self.current_plugin_idx].clone();

        // read the wasm from the path
        let wasm = match fs::read(&plugin_path) {
            Ok(data) => data,
            Err(_) => return Some(Err(PluginIteratorError::InvalidPluginPath(plugin_path))),
        };

        // increment the current plugin index
        self.current_plugin_idx += 1;

        Some(Ok((plugin_path, wasm)))
    }
}
