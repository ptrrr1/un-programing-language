use std::{
    env,
    fs::File,
    io::{self, BufReader},
    path::Path,
    process::exit,
};

use scanner::Scanner;

pub mod parser;
pub mod scanner;
pub mod tokens;

fn main() {
    let mut scanner = Scanner::default();

    let args: Vec<String> = env::args().skip(1).collect();
    match args.len() {
        0 => unimplemented!(), // Interactive
        1 => {
            let file_path = Path::new(&args[0]);
            let mut buffer = read_file(file_path).unwrap_or_else(|err| {
                eprintln!("{}", err);
                exit(66);
            });

            scanner.scan_file(&mut buffer);

            dbg!(scanner);
        } // File
        _ => {
            eprintln!("Usage: un [script]");
            exit(64);
        }
    };
}

fn read_file(path: &Path) -> io::Result<BufReader<File>> {
    let file_ext: &str = "un"; // Don't have a name yet for the language

    if path.is_file() && path.extension().is_some_and(|ext| ext == file_ext) {
        let f = File::open(path)?;
        Ok(BufReader::new(f))
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "File doesn't exist or has wrong extension",
        ))
    }
}
