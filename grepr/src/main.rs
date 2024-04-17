use std::{
    env,
    fs::File,
    io::{self, Read},
};

use grepr_lib::search::bm::BoyerMoore;

fn main() {
    match grepr() {
        Ok(idx) => match idx {
            Some(idx) => println!("Found at index: {}", idx),
            None => println!("Not Found"),
        },
        Err(err) => println!("{}", err),
    }
}

fn grepr() -> Result<Option<usize>, String> {
    let query_data = extract_query_data();
    let (content, query): (&[u8], &[u8]);

    match query_data {
        Ok((val1, val2)) => {
            content = val1.as_bytes();
            query = val2.as_bytes();
            Ok(BoyerMoore::find_match(&content, &query))
        }
        Err(err) => Err(err),
    }
}

// Returns tuple with (Content, Query)
fn extract_query_data() -> Result<(String, String), String> {
    let args = parse_args();
    let (path, query, mut content): (String, String, String);

    match args {
        Ok((val1, val2)) => {
            path = val1;
            query = val2;
        }
        Err(err) => {
            return Err(err);
        }
    }

    content = String::new();

    match read_search_data(&path, &mut content) {
        Ok(_) => Ok((content, query)),
        Err(err) => Err(err.to_string()),
    }
}

fn read_search_data(path: &str, content: &mut String) -> io::Result<()> {
    let mut file = File::open(path)?;

    if let Err(err) = file.read_to_string(content) {
        return Err(err);
    }
    Ok(())
}

fn parse_args() -> Result<(String, String), String> {
    let mut args = env::args();

    match args.len() {
        3 => {
            let _ = args.next();
            let filename = args.next().unwrap();
            let query = args.next().unwrap();
            return Ok((filename, query));
        }

        0..=2 => {
            let err = format!("Too few arguments!").to_string();
            Err(err)
        }

        _ => {
            let err = format!("Too many arguments: {}, expected 2", args.len() - 1).to_string();
            Err(err)
        }
    }
}
