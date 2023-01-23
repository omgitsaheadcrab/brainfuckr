use clap::Parser;

#[derive(Parser)]
pub struct Arguments {
    /// The path to the source file to interpret
    pub src: std::path::PathBuf,
}
