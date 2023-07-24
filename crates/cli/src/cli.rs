use clap::{Parser, Subcommand};

/// Default path for output folder.
pub const DEFAULT_FOLDER: &str = "";

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    WritePrompt {
        /// Path to the directory where openAI output is written
        #[clap(default_value = DEFAULT_FOLDER)]
        output_dir: String,
    },
    WriteSequence {},
}
