use std::io::BufRead;

use crate::{scanner::Scanner, token_type::TokenType};

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn run_file(mut self, path: &str) -> Result<(), std::io::Error> {
        let bytes = std::fs::read(path)?;
        self.run(&String::from_utf8(bytes).unwrap());
        Ok(())
    }

    pub fn run_prompt(mut self) -> Result<(), std::io::Error> {
        let stdin = std::io::stdin();
        let mut reader = std::io::BufReader::new(stdin);
        loop {
            print!("> ");
            let mut line = String::new();
            reader.read_line(&mut line)?;
            if line.is_empty() {
                break;
            }
            self.run(&line);
            self.had_error = false;
        }
        Ok(())
    }

    pub fn run(&mut self, source: &str) {
        // Indicate an error in the exit code.
        if self.had_error {
            std::process::exit(65);
        }

        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        // For now, just print the tokens.
        for token in tokens {
            println!("{}", token.to_string());
            if token.token_type == TokenType::Unknown {
                self.error(token.line, "Unexpected character.");
            }
        }
    }

    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, where_: &str, message: &str) {
        eprintln!("[line {line}] Error{where_}: {message}");

        self.had_error = true;
    }
}
