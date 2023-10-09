//! Command line arguments parser

use clap::Parser;

/// Yet another brainfuck interpreter written in Rust
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    /// The path of the source file to be interpreted
    #[arg(short, long)]
    pub src: std::path::PathBuf,

    /// The number of cells in the array
    #[arg(short, long, default_value_t = 30000)]
    pub arr: usize,
}
