use std::{
    cell::RefCell,
    fs::File,
    io::{self, BufReader, Write},
    path::Path,
    process::exit,
    rc::Rc,
};

use crate::{enviroment::Enviroment, parser::Parser, scanner::Scanner, tokens::TokenType};

#[derive(Default)]
pub struct Interpreter {
    env: Rc<RefCell<Enviroment>>,
}

impl Interpreter {
    pub fn run_file(&self, file_path: &String) -> io::Result<()> {
        let file_path = Path::new(file_path);
        let mut buffer = Self::read_file(file_path).unwrap_or_else(|err| {
            eprintln!("{}", err);
            exit(66);
        });

        let scanner_result = Scanner::scan_file(&mut buffer);
        if scanner_result.has_err() {
            // println!("{:#?}", scanner_result.into_err());
            scanner_result
                .into_err()
                .into_iter()
                .for_each(|e| println!("{}", e));
            exit(70);
        }

        let tokens = scanner_result.into_tokens().into_iter().filter(|t| {
            !matches!(
                t.token_type,
                TokenType::Space | TokenType::CommentStarter | TokenType::Comment(_)
            )
        });
        // println!(":: {:#?}", &tokens);

        let parser_result = Parser::parse_tokens(tokens);
        // println!(":: {:#?}", &parser_result);

        if parser_result.has_err() {
            // println!("{:#?}", parser_result.into_err());
            parser_result
                .into_err()
                .into_iter()
                .for_each(|e| println!("{}", e));
            exit(70);
        }

        // dbg!(&parser_result);

        for stmt in parser_result.into_stmt() {
            let r = stmt.eval(self.env.clone());
            match r {
                Ok(_) => {}
                Err(e) => {
                    dbg!(e);
                }
            }
        }

        Ok(())
    }

    fn read_file(path: &Path) -> io::Result<BufReader<File>> {
        let file_ext: &str = "un";

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

    pub fn run_prompt() -> io::Result<()> {
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
                            // println!("{:#?}", scanner_result.into_err());
                            scanner_result
                                .into_err()
                                .into_iter()
                                .for_each(|e| println!("{}", e));
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
                            // println!("{:#?}", parser_result.into_err());
                            parser_result
                                .into_err()
                                .into_iter()
                                .for_each(|e| println!("{}", e));
                            continue;
                        }
                    }
                }
                Err(e) if e.kind() == io::ErrorKind::Interrupted => break,
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }
}
