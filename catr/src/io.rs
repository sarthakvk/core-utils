use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines, Write},
    process,
};

pub trait MultipleFilesReader {
    fn create_reader(paths: Vec<String>) -> Files {
        let mut out = vec![];

        for path in paths{
            let f = File::open(path).unwrap_or_else(|err| {
                let mut stderr = io::stderr();
                writeln!(stderr, "{}", err.to_string()).unwrap();
                process::exit(1);
            });
            out.push(
                BufReader::new(f).lines()
            )
        }
        out
    }
}

pub type Files = Vec<Lines<BufReader<File>>>;
pub struct FilesReader;

impl MultipleFilesReader for FilesReader {}


pub fn write_files_to_stdout(files: Files, mark_line: bool) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    
    for file_iter in files {
        let mut line_num = 1_usize;
        for line in file_iter {
            match line {
                Ok(line) => {
                    if mark_line {
                        write!(handle, "\t{}  {}\n", line_num, line).unwrap();
                    } else {
                        write!(handle, "{}\n", line).unwrap();
                    }
                    line_num += 1;
                }
                Err(err) => {
                    let mut stderr = io::stderr();
                    write!(stderr, "{}", err.to_string()).unwrap();
                    process::exit(1);
                }
            }
        }
    }
    handle.flush().unwrap();
}
