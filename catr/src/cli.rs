use clap::{command, Parser};

#[derive(Parser)]
#[command(version="0.1", about="concatenate and print files")]
pub struct Cli{
    /// space seperated filenames for files to concatenate
    pub files: Vec<String>,

    /// Number the output line, starting at 1
    #[arg[short]]
    pub num: bool
}
