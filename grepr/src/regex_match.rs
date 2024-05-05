
use std::io::Write;
use regex::Regex;

use std::io;
use crate::cli::{write_result, InputLinesIterator};


pub fn match_regex_pat(pat: &Regex, hay: &str) -> Vec<usize> {

    pat.find_iter(hay).map(|m: regex::Match<'_>| m.start()).collect()
}

pub fn regex_search(pat: &str, mut lines_iter: InputLinesIterator) {

    let re_pat = Regex::new(pat).unwrap();
    let mut line_num = 0_usize;
    let match_len = pat.len();

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    
    while let Some(line) = lines_iter.next(){
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