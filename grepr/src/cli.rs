use clap::{command, Parser};
use colored::Colorize;

use std::{
    fs::File, io::{self, BufRead, BufReader, Lines, StdinLock, StdoutLock, Write}
};

pub enum InputLinesIterator{
    FileLines(Lines<BufReader<File>>),
    StdioLines(Lines<StdinLock<'static>>)
}

impl Iterator for InputLinesIterator {
    type Item = Result<String, io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::FileLines(iter) => iter.next(),
            Self::StdioLines(iter) => iter.next()   
        }
    }
}


#[derive(Parser)]
#[command(version="0.1", about="Grepr: file pattern searcher")]
pub struct Args{
    /// Pattern to search
    pub pattern: String,
    
    /// input file path
    pub file: Option<String>,

    /// Use exact match, without regular experessions
    #[arg(short, long)]
    pub exact: bool
}

pub fn write_result(handle: &mut StdoutLock<'static>, content: &[u8], matches: &Vec<usize>, match_len: usize, line_num: Option<usize>){

    // i: content iterrator, j is match iterator
    let mut i = 0_usize;
    let mut match_iter = matches.iter();
    let mut cur_match = match_iter.next();
    
    if let Some(num) = line_num{
        let line_num = format!("line {}: ", num).red().bold();
        write!(handle, "{}", line_num).unwrap();
    }

    while i < content.len(){
        let str_to_write: String;
        
        if let Some(&match_val) = cur_match {
            if i == match_val {
                str_to_write = std::str::from_utf8(&content[match_val..match_val+match_len]).unwrap().blue().bold().to_string();
                i = match_val + match_len;
                cur_match = match_iter.next();
            } else {
                str_to_write = std::str::from_utf8(&content[i..match_val]).unwrap().to_string();
                i = match_val;
            }
        } else {
            str_to_write = std::str::from_utf8(&content[i..]).unwrap().to_string();
            i = content.len();
            
        }

        write!(handle, "{}", str_to_write).unwrap();
    }
    write!(handle, "\n").unwrap();

}

// Returns tuple with (Content, Query)
pub fn read_lines_from_file(path: &str) ->  Lines<BufReader<File>> {

    let file = File::open(path).unwrap();

    let buf = BufReader::new(file);

    buf.lines()
    
}

pub fn read_lines_from_stdio() ->  Lines<StdinLock<'static>> {
    let stdin = io::stdin();
    stdin.lines()
}

pub fn read_lines(path: Option<&str>) -> InputLinesIterator{
    match path {
        Some(path) => InputLinesIterator::FileLines(read_lines_from_file(path)),
        None => InputLinesIterator::StdioLines(read_lines_from_stdio())
    }
}
