pub mod lexical_analyzer;
pub mod tokens;
pub mod ast;

use std::io::{self, Write};

use lexical_analyzer::TokenParser;
use tokens::Token;

pub fn main_loop() {
    loop {
        if let Ok(expression) = read_line("> ") {
            let exp = expression.trim();
            if exp == "" || exp == "q" {
                break;
            }
            parse_expression(&expression);
        }
    }
}

pub fn parse_expression(expression: &String) {
    match TokenParser::new(expression) {
        Ok(mut parser) => {
            let parsed_tokens: Vec<Token> = parser.get_tokens();
            println!("Tokens: {:?}", parsed_tokens)
        }
        Err(e) => println!("Error: {}", e),
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
