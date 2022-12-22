use crate::lox::Lox;

mod lox;
mod scanner;
mod token;
mod token_type;

fn main() -> Result<(), std::io::Error> {
    println!("Hello, Lox!");
    let lox = Lox::new();

    lox.run_prompt()
}
