use crate::enviroment::Enviroment;
use crate::stmt::resolver::Resolver;
use crate::types::value::Value;
use crate::{parser::Parser, scanner::Scanner, stmt::signal::Signal, tokens::TokenType};
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};
use std::{
    fs::File,
    io::{self, BufReader, Write},
    path::Path,
    process::exit,
};

#[derive(Debug, Default)]
pub struct Interpreter {
    pub env: Rc<RefCell<Enviroment>>,
    pub locals: HashMap<String, usize>,
}

impl Interpreter {
    pub fn look_up_var(&mut self, env: Rc<RefCell<Enviroment>>, identifier: &str) -> Value {
        match self.locals.get(identifier) {
            Some(depth) => Enviroment::get_at(env, identifier, *depth),
            None => self
                .env
                .borrow()
                .get_var_val(&identifier.to_string())
                .unwrap(), // TODO: More todo.. fix this or think about it
        }
    }

    pub fn run_file(&mut self, file_path: &String) -> io::Result<()> {
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

        let stmts = parser_result.into_stmt();

        // dbg!(&parser_result);

        // TODO: Try to understand it better
        let mut resolver = Resolver::default();
        resolver.resolve_stmts(&stmts);
        let locals = resolver.into_locals();
        self.locals = locals;

        for stmt in stmts {
            // dbg!(&stmt);
            let s = stmt.accept(self.env.clone(), self);
            // dbg!(&s);

            match s {
                Signal::Normal => continue,
                Signal::Return(_val) => {
                    // TODO: Improve
                    panic!("Return outside of function")
                }
                Signal::Break => panic!("Break outside of loop"),
                // Signal::Continue => eprintln!("Continue outside of loop"),
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
    pub fn run_prompt(&mut self) -> io::Result<()> {
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
                        println!(":: {:#?}", &parser_result);

                        if parser_result.has_err() {
                            // println!("{:#?}", parser_result.into_err());
                            parser_result
                                .into_err()
                                .into_iter()
                                .for_each(|e| println!("{}", e));
                            continue;
                        }

                        // TODO: In case no err, add stmt to vec! and run them
                    }
                }
                Err(e) if e.kind() == io::ErrorKind::Interrupted => break,
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }
}
