use clap::Parser;
use io::{write_files_to_stdout, Files, FilesReader, MultipleFilesReader};

mod io;
mod cli;

fn main() {
    let args = cli::Cli::parse();

    let reader: Files = FilesReader::create_reader(args.files);

    write_files_to_stdout(reader, args.num);
}
