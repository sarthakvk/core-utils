use clap::{command, Parser};
use colored::Colorize;

use std::{
    fs::File, io::{self, BufRead, BufReader, Lines, StdoutLock, Write}
};

use crate::bm::BoyerMoore;
use crate::regex_match::match_regex_pat;
use regex::Regex;


#[derive(Parser)]
#[command(version="0.1", about="Grepr: file pattern searcher")]
pub struct Args{
    /// Pattern to search
    pub pattern: String,
    
    /// input file path
    pub input: String,

    /// Use exact match, without regular experessions
    #[arg(short, long)]
    pub exact: bool
}

pub fn regex_search(pat: &str, path: &str) {

    let mut hay = read_lines_from_file(path);
    let re_pat = Regex::new(pat).unwrap();
    let mut line_num = 0_usize;
    let match_len = pat.len();

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    
    while let Some(line) = hay.next(){
        match line {
            Ok(line) => {
                let res = match_regex_pat(&re_pat, &line);
            
                if res.len() != 0{
                    write_result(&mut handle, line.as_bytes(), &res, match_len, Some(line_num));
                }
                line_num += 1;
            },
            Err(err) => panic!("{}", err)
        }
    }
    handle.flush().unwrap();
}

pub fn bm_search(patt: &str, input: &str) {
    let patt_bytes = patt.as_bytes();
    let mut inp_str = read_lines_from_file(input);
    let mut line_num = 0_usize;
    let match_len = patt.len();

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    while let Some(line) = inp_str.next() {
        match line {
            Ok(line) => {
                let inp_bytes = line.as_bytes();
                let res = BoyerMoore::find_match(inp_bytes, patt_bytes);
                
                if res.len() != 0 {
                    write_result(&mut handle, inp_bytes, &res, match_len, Some(line_num));
                }
                line_num += 1;
            },
            Err(err) => panic!("{}", err)            
        }
    }

    handle.flush().unwrap();
}

fn write_result(handle: &mut StdoutLock<'static>, content: &[u8], matches: &Vec<usize>, match_len: usize, line_num: Option<usize>){

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
fn read_lines_from_file(path: &str) -> Lines<BufReader<File>> {
    
    let file = File::open(path).unwrap();

    let buf = BufReader::new(file);

    buf.lines()
}
