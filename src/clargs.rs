use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};

pub const MATRICKS_DEFAULT_FPS: &str = "30";
pub const MATRICKS_DEFAULT_SERPENTINE: &str = "false";
pub const MATRICKS_DEFAULT_VERTICAL: &str = "false";
pub const MATRICKS_DEFAULT_MIRROR_HORIZONTAL: &str = "false";
pub const MATRICKS_DEFAULT_MIRROR_VERTICAL: &str = "false";
pub const MATRICKS_DEFAULT_BRIGHTNESS: &str = "255";
pub const MATRICKS_DEFAULT_LOOP: &str = "false";
pub const MATRICKS_DEFAULT_DMA_CHANNEL: &str = "10";
pub const MATRICKS_DEFAULT_GPIO_PIN: &str = "10";
pub const MATRICKS_DEFAULT_LED_SIGNAL_FREQ: &str = "800000";

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
pub struct MatricksArgs {
    #[command(subcommand)]
    pub config: MatricksSubcommand,
}

#[derive(Subcommand)]
pub enum MatricksSubcommand {
    /// Start Matricks using command line arguments
    Manual(MatricksConfigArgs),

    /// Start Matricks using a configuration file
    Auto(ConfigurationFileReadInfo),

    /// Save a command line configuration to a .toml configuration file
    Save {
        #[command(flatten)]
        info: ConfigurationFileWriteInfo,

        #[command(flatten)]
        matrix_config: MatricksConfigArgs,
    },

    /// Clear the matrix
    Clear(MatrixConfigurationArgs),
}

/// Information needed to read a configuration file
#[derive(Args, Clone)]
pub struct ConfigurationFileReadInfo {
    /// Path to a .toml configuration file
    #[arg(global = true)]
    pub config_path: String,
}

/// Information needed to write a configuration file
#[derive(Args, Clone)]
pub struct ConfigurationFileWriteInfo {
    /// Location to write configuration file
    #[arg(global = true)]
    pub config_path: String,
}

/// Options relevant to setting up a LED controller
#[derive(Serialize, Deserialize, Args, Clone)]
pub struct LEDControllerArgs {
    /// GPIO used to drive the matrix
    #[arg(short, long, default_value = MATRICKS_DEFAULT_GPIO_PIN)]
    pub gpio: u16,

    /// DMA channel used to drive the matrix
    #[arg(short, long, default_value = MATRICKS_DEFAULT_DMA_CHANNEL)]
    pub dma: u16,

    /// Signal rate of the LED controller, in Hz
    #[arg(long, default_value = MATRICKS_DEFAULT_LED_SIGNAL_FREQ)]
    pub frequency: u32,
}

/// Options relevant to setting up a LED matrix
#[derive(Serialize, Deserialize, Args, Clone)]
pub struct MatrixConfigurationArgs {
    /// Width of the matrix, in number of LEDs
    #[arg(short = 'x', long)]
    pub width: usize,

    /// Height of the matrix, in number of LEDs
    #[arg(short = 'y', long)]
    pub height: usize,

    /// Target framerate at which to drive the matrix
    #[arg(short, long, default_value = MATRICKS_DEFAULT_FPS)]
    pub fps: f32,

    /// Data line alternates direction between columns or rows
    #[arg(short, long, default_value = MATRICKS_DEFAULT_SERPENTINE)]
    pub serpentine: bool,

    /// Matrix is vertically wired
    #[arg(short, long, default_value = MATRICKS_DEFAULT_VERTICAL)]
    pub vertical: bool,

    /// Flip the matrix horizontally
    #[arg(long, default_value = MATRICKS_DEFAULT_MIRROR_HORIZONTAL)]
    pub mirror_horizontal: bool,

    /// Flip the matrix vertically
    #[arg(long, default_value = MATRICKS_DEFAULT_MIRROR_VERTICAL)]
    pub mirror_vertical: bool,

    /// Brightness of matrix, from 0-255
    #[arg(short, long, default_value = MATRICKS_DEFAULT_BRIGHTNESS)]
    pub brightness: u8,

    #[command(flatten)]
    pub controller: LEDControllerArgs,
}

/// Options relevant to setting up plugins
#[derive(Serialize, Deserialize, Args, Clone)]
pub struct PluginConfigurationArgs {
    /// Add a plugin at a given path to the playlist
    #[arg(short, long, required = true)]
    pub plugin: Vec<String>,

    /// Maximum time (in seconds) that a single plugin can run before moving on to the next one. No time limit by default.
    #[arg(short, long)]
    pub time_limit: Option<u64>,

    /// Loop plugin or set of plugins indefinitely
    #[arg(short = 'l', long = "loop", default_value = MATRICKS_DEFAULT_LOOP)]
    pub loop_plugins: bool,

    /// Allow plugins to access a particular network host
    #[arg(long)]
    pub allow_host: Option<Vec<String>>,

    /// Map a path on the host filesystem to a path on the plugin filesystem. Inputs should be of the form "DEST_PATH>HOST_PATH".
    #[arg(long)]
    pub map_path: Option<Vec<String>>,
}

/// Options relevant to setting up Matricks
#[derive(Args, Clone, Serialize, Deserialize)]
pub struct MatricksConfigArgs {
    #[command(flatten)]
    pub matrix: MatrixConfigurationArgs,

    #[command(flatten)]
    pub plugin: PluginConfigurationArgs,
}
