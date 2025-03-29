use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(about = "Gensou manages your dotiles and packages.", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Create a new manifest file
    Init,

    /// Apply the manifest file
    Apply {
        /// Path to the manifest file to be used
        #[arg(short = 'm', long = "manifest")]
        manifest_path: Option<String>,
    },
}
