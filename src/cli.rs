use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "Leet")]
#[clap(author = "Swapnil Kumbhar <https://github.com/SwapnilKumbhar>")]
#[clap(about = "Custom starter templates for Leetcode.")]
pub struct Args {
    /// Provide a custom config file
    #[clap(short, long, global = true)]
    pub config: Option<String>,

    /// Toggles verbose logs (INFO level)
    #[clap(short, long, global = true)]
    pub verbose: bool,

    /// Command to run
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new project
    New { language: String, name: String },

    /// Show currently supported languages
    ShowLanguages {},
}
