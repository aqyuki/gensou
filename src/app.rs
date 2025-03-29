use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(about = "Gensou manages your dotiles and packages.", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Path to the manifest file to be used
    #[arg(short = 'm', long = "manifest")]
    pub manifest_path: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Create a new manifest file
    Init,

    /// Apply the manifest file
    Apply,
}
