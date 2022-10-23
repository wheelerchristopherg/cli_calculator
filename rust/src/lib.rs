pub mod ast;
pub mod lexical_analyzer;
pub mod tokens;

use std::io::{self, Write};

use ast::AST;
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
            let tree: Box<AST> = AST::build_tree(&parsed_tokens);
            println!("tree: {}", tree);
            match tree.evaluate() {
                Ok(x) => println!("x0 = {}", x),
                Err(e) => println!("{}", e),
            }
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
