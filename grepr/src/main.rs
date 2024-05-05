mod cli;
mod bm;
mod regex_match;

use clap::Parser;
use cli::read_lines;
use regex_match::regex_search;
use bm::bm_search;


fn main() {
    let args = cli::Args::parse();

    let (exact, pat, file) = (args.exact, args.pattern.as_ref(), args.file.as_deref());

    let line_iter = read_lines(file);

    if exact {
        bm_search(pat, line_iter);
    } else {
        regex_search(pat, line_iter)
    }
}

