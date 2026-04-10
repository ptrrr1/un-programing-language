use std::{
    env,
    fs::File,
    io::{self, BufReader, Write},
    path::Path,
    process::exit,
};

use parser::{Parser, stmt::TypedStmt};
use scanner::Scanner;
use tokens::TokenType;

pub mod enviroment;
pub mod errors;
pub mod parser;
pub mod scanner;
pub mod tokens;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.len() {
        0 => run_prompt()?, // Interactive
        1 => {
            let file_path = Path::new(&args[0]);
            let mut buffer = read_file(file_path).unwrap_or_else(|err| {
                eprintln!("{}", err);
                exit(66);
            });

            let scanner_result = Scanner::scan_file(&mut buffer);
            let tokens = scanner_result
                .into_tokens() // Destroys Scanner
                .into_iter()
                .filter(|t| {
                    !matches!(
                        t.token_type,
                        TokenType::Space | TokenType::CommentStarter | TokenType::Comment(_)
                    )
                });
            dbg!(&tokens);
            let parser_result = Parser::parse_tokens(tokens);
            dbg!(&parser_result);

            if parser_result.has_err() {
                dbg!(&parser_result);
                dbg!(parser_result.into_err());
                exit(70);
            }

            let stmt = parser_result.into_stmt();

            let i = stmt.iter().map(|s| TypedStmt::try_from(s.clone()));
            i.for_each(|v| println!("{:#?}", v.is_ok().then(|| v.unwrap().eval())));
        } // File
        _ => {
            eprintln!("Usage: un [script]");
            exit(64);
        }
    };

    Ok(())
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

fn run_prompt() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!(">> ");
        stdout.flush()?;

        let mut buf: String = String::new();
        let bytes_read = stdin.read_line(&mut buf);

        match bytes_read {
            Ok(s) => {
                if s == 1 {
                    break;
                } else {
                    let scanner_result = Scanner::scan_line(buf, 0);

                    if scanner_result.has_err() {
                        println!("{:#?}", scanner_result.into_err());
                        continue;
                    }

                    let tokens = scanner_result
                        .into_tokens()
                        .into_iter()
                        .filter(|t| !matches!(t.token_type, TokenType::Space));
                    // println!(":: {:#?}", &tokens);

                    let parser_result = Parser::parse_tokens(tokens);
                    // println!(":: {:#?}", &parser_result);

                    if parser_result.has_err() {
                        println!("{:#?}", parser_result.into_err());
                        continue;
                    }

                    let stmt = parser_result.into_stmt().pop().unwrap();
                    // println!(":: {:#?}", &expr);

                    let typed_stmt = TypedStmt::try_from(stmt);
                    // // println!(":: {:#?}", &typed_expr);

                    match typed_stmt {
                        Ok(typed_expr) => typed_expr.eval(),
                        Err(err) => println!(":: Err: {:?}", err),
                    }
                }
            }
            Err(e) if e.kind() == io::ErrorKind::Interrupted => break,
            Err(e) => return Err(e),
        }
    }

    Ok(())
}
