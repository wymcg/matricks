use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct Args {
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
    #[arg(short, long, default_value = "log")]
    pub log_dir: String,

    /// Data line alternates direction between columns or rows
    /// In other words, every other row or column is reversed
    #[arg(short, long, default_value = "false")]
    pub serpentine: bool,

    #[cfg(not(target_arch = "aarch64"))]
    /// Magnification of the simulated matrix
    #[arg(short, long, default_value = "10")]
    pub magnification: f32,
}
