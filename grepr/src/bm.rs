// Implementation on Boyer-Moore string searching algorithm

pub struct BoyerMoore;

use std::io;
use std::io::Write;
use crate::io::{write_result, InputLinesIterator};

const MAX_U8: usize = u8::MAX as usize;

impl BoyerMoore {
    pub fn find_match(data: &[u8], query: &[u8]) -> Vec<usize> {
        if query.len() == 0 || data.len() == 0 || query.len() > data.len(){
            return vec![];
        }
        
        let query_size = query.len();
        let mut i = query_size - 1;
        let mut j = i;
        let delta1 = BoyerMoore::preprocess_delta1(query);
        let mut matches:Vec<usize> = vec![];


        while i < data.len() {
            if data[i] == query[j] {
                if j == 0 {
                    matches.push(i);
                    i += 2*query_size;
                    j = query_size;
                }
                i -= 1;
                j -= 1;
            } else {
                let shift = delta1[data[i as usize] as usize];

                if shift > query_size - j - 1 {
                    i += query_size - j;
                } else {
                    i += shift + query_size - j;
                }

                j = query_size - 1;
            }
        }
        return matches;
    }

    pub fn preprocess_delta1(query: &[u8]) -> [usize; MAX_U8 + 1] {
        let query_size = query.len();
        let mut delta1 = [query_size; MAX_U8 + 1];

        for idx in 0..query_size {
            delta1[query[idx] as usize] = query_size - idx - 1;
        }

        delta1
    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    fn preprocess_delta2(query: &[u8]){
        todo!();
    }
}

pub fn bm_search(pat: &str, mut lines_iter: InputLinesIterator) {
    let pat_bytes = pat.as_bytes();
    let mut line_num = 0_usize;
    let match_len = pat.len();

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    while let Some(line) = lines_iter.next() {
        match line {
            Ok(line) => {
                let inp_bytes = line.as_bytes();
                let res = BoyerMoore::find_match(inp_bytes, pat_bytes);
                
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_match_not_found() {
        let data = b"This is a test string.";
        let query = b"xyz";
        let matches = BoyerMoore::find_match(data, query);
        assert_eq!(matches, Vec::<usize>::new());
    }

    #[test]
    fn test_find_match_beginning() {
        let data = b"abcdebcdefghijklmnopqrstuvwxyz";
        let query = b"abc";
        let matches = BoyerMoore::find_match(data, query);
        assert_eq!(matches, vec![0]);
    }

    #[test]
    fn test_find_match_end() {
        let data = b"abcdefghijklmnopqrstuvwxyz";
        let query = b"xyz";
        let matches = BoyerMoore::find_match(data, query);
        assert_eq!(matches, vec![23]);
    }

    #[test]
    fn test_find_match_multiple_times() {
        let data = b"abababababababab";
        let query = b"ab";
        let matches = BoyerMoore::find_match(data, query);
        assert_eq!(matches, vec![0, 2, 4, 6, 8, 10, 12, 14]);
    }

    #[test]
    fn test_find_match_special_characters() {
        let data = "The quick ßrown ƒox jumps over the lazy dog.".as_bytes();
        let query = "ßrown ƒox".as_bytes();
        let matches = BoyerMoore::find_match(data, query);
        assert_eq!(matches, vec![10]);
    }

    #[test]
    fn test_find_match_empty_text_and_pattern() {
        let data = b"";
        let query = b"";
        let matches = BoyerMoore::find_match(data, query);
        assert_eq!(matches, vec![]);
    }
    
}