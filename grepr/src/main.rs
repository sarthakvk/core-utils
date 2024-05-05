mod cli;
mod bm;
mod regex_match;

use clap::Parser;
use cli::{bm_search, regex_search};

fn main() {
    let args = cli::Args::parse();

    let (exact, patt, path) = (args.exact, args.pattern.as_ref(), args.input.as_ref());

    if exact {
        bm_search(patt, path);
    } else {
        regex_search(patt, path)
    }
}

