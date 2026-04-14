use std::{
    env,
    io::{self},
    process::exit,
};

use interpreter::Interpreter;
use un_std::math_globals;

pub mod enviroment;
pub mod errors;
pub mod interpreter;
pub mod parser;
pub mod scanner;
pub mod tokens;
pub mod un_std;

fn main() -> io::Result<()> {
    let interpreter = Interpreter::with_exposed(math_globals());

    let args: Vec<String> = env::args().skip(1).collect();
    match args.len() {
        0 => Interpreter::run_prompt()?,      // Interactive
        1 => interpreter.run_file(&args[0])?, // File
        _ => {
            eprintln!("Usage: un [script]");
            exit(64);
        }
    };

    Ok(())
}
