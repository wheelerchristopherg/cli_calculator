mod lexical_analyzer;
mod tokens;
use lexical_analyzer::TokenParser;
use std::io::{self, Write};
use tokens::Token;

fn main() {
    loop {
        if let Ok(expression) = read_line("> ") {
            let exp = expression.trim();
            if exp == "" || exp == "q" {
                break;
            }
            match TokenParser::new(expression) {
                Ok(mut parser) => {
                    let parsed_tokens: Vec<Token> = parser.get_tokens();
                    println!("Tokens: {:?}", parsed_tokens)
                }
                Err(e) => println!("Error: {}", e),
            }
        }
    }
}

fn read_line(prompt: &str) -> Result<String, io::Error> {
    let mut user_input = String::new();
    print!("{}", prompt);
    io::stdout().flush()?;
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => Ok(user_input),
        Err(e) => Err(e),
    }
}
