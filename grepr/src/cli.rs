use clap::{command, Parser};

#[derive(Parser)]
#[command(version="0.1", about="Grepr: file pattern searcher")]
pub struct Args{
    /// Pattern to search
    pub pattern: String,
    
    /// input file path
    pub file: Option<String>,

    /// Use exact match, without regular experessions.
    /// Uses Boyer-Moore pattern matching
    #[arg(short, long)]
    pub exact: bool
}

