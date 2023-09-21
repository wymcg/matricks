use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
pub struct MatricksArgs {
    #[command(subcommand)]
    pub config: FetchType,
}

#[derive(Subcommand)]
pub enum FetchType {
    /// Start Matricks using command line arguments
    Manual(MatrixConfigArgs),

    /// Start Matricks using a configuration file
    Auto(ConfigurationFileReadInfo),

    /// Save a command line configuration to a .toml configuration file
    Save {
        #[command(flatten)]
        info: ConfigurationFileWriteInfo,

        #[command(flatten)]
        matrix_config: MatrixConfigArgs,
    }
}

/// Information needed to read a configuration file
#[derive(Args, Clone)]
pub struct ConfigurationFileReadInfo {
    /// Path to a .toml configuration file
    #[arg(short, long)]
    path: String,
}

/// Information needed to write a configuration file
#[derive(Args, Clone)]
pub struct ConfigurationFileWriteInfo {
    /// Location to write configuration file
    #[arg(short, long)]
    path: String,
}

#[derive(Args, Clone, Serialize, Deserialize)]
pub struct MatrixConfigArgs {
    /// Path to plugin or directory of plugins
    #[arg(short, long)]
    pub plugins: String,

    /// Width of the matrix, in number of LEDs
    #[arg(short = 'x', long)]
    pub width: usize,

    /// Height of the matrix, in number of LEDs
    #[arg(short = 'y', long)]
    pub height: usize,

    /// Target framerate at which to drive the matrix
    #[arg(short, long, default_value = "30")]
    pub fps: f32,

    /// Directory to write logs
    #[arg(short = 'L', long = "log", default_value = "log")]
    pub log_dir: String,

    /// Data line alternates direction between columns or rows
    #[arg(short, long, default_value = "false")]
    pub serpentine: bool,

    /// Brightness of matrix, from 0-255
    #[arg(short, long, default_value = "255")]
    pub brightness: u8,

    /// Maximum time (in seconds) that a single plugin can run before moving on to the next one. No time limit by default.
    #[arg(short, long)]
    pub time_limit: Option<u64>,

    /// Loop plugin or set of plugins indefinitely
    #[arg(short = 'l', long = "loop", default_value = "false")]
    pub loop_plugins: bool,
}
