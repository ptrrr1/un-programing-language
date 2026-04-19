use interpreter::Interpreter;
use std::{
    env,
    io::{self},
    process::exit,
};

pub mod enviroment;
pub mod errors;
pub mod expr;
pub mod interpreter;
pub mod parser;
pub mod scanner;
pub mod stmt;
pub mod tokens;
pub mod types;

fn main() -> io::Result<()> {
    let mut interpreter = Interpreter::default();

    let args: Vec<String> = env::args().skip(1).collect();
    match args.len() {
        0 => interpreter.run_prompt()?,       // Interactive
        1 => interpreter.run_file(&args[0])?, // File
        _ => {
            eprintln!("Usage: un [script]");
            exit(64);
        }
    };

    Ok(())
}
