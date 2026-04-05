use std::{
    env,
    fs::File,
    io::{self, BufReader, Write},
    path::Path,
    process::exit,
};

use parser::{Parser, expr::Expr, typed_expr::TypedExpr};
use scanner::Scanner;
use tokens::TokenType;

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

            let mut scanner = Scanner::default();
            scanner.scan_file(&mut buffer);

            //dbg!(&scanner);

            let tokens = scanner
                .into_tokens() // Destroys Scanner
                .into_iter()
                .filter(|t| !matches!(t.token_type, TokenType::Space));

            let mut parser = Parser::new(tokens);
            parser.parse_tokens();

            //dbg!(&parser);

            let expr = parser.into_expr();

            let i = expr.iter().map(|expr| TypedExpr::try_from(expr.clone()));
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
                    let mut scanner = Scanner::default();
                    scanner.scan_line(buf, 0);
                    // println!(":: {:#?}", &scanner);

                    let tokens = scanner
                        .into_tokens()
                        .into_iter()
                        .filter(|t| !matches!(t.token_type, TokenType::Space));
                    // println!(":: {:#?}", &tokens);

                    let mut parser = Parser::new(tokens);
                    parser.parse_tokens();
                    // println!(":: {:#?}", &parser);

                    let expr = parser.into_expr().pop().unwrap();
                    // println!(":: {:#?}", &expr);

                    let typed_expr = TypedExpr::try_from(expr);
                    // println!(":: {:#?}", &typed_expr);

                    match typed_expr {
                        Ok(texpr) => println!(":: {:?}", texpr.eval()),
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
