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
            if exp.is_empty() || exp == "q" {
                break;
            }
            println!("{}", evaluate_string_expression(&expression));
        }
    }
}

pub fn evaluate_string_expression(expression: &str) -> String {
    let parsed_tokens: Vec<Token> = match parse_tokens(expression) {
        Ok(parsed) => parsed,
        Err(e) => return e,
    };

    let tree: Box<AST> = match AST::build_tree(&parsed_tokens) {
        Ok(parsed) => parsed,
        Err(e) => return e,
    };

    match tree.evaluate() {
        Ok(result) => {
            if result == result.floor() {
                format!("x0 = {}.0", result)
            } else {
                format!("x0 = {}", result)
            }
        }
        Err(e) => e,
    }
}

fn parse_tokens(expression: &str) -> Result<Vec<Token>, String> {
    let mut parser =
        TokenParser::new(expression).map_err(|_| "Expression contains non-ascii characters.")?;

    parser.get_tokens().map_err(|e| {
        let end = expression
            .chars()
            .position(|c| c == '\n')
            .unwrap_or(expression.len());
        format!("{}\n{}\n{}", e, &expression[..end], e.arrow())
    })
}

fn read_line(prompt: &str) -> Result<String, io::Error> {
    let mut user_input = String::new();
    print!("{}", prompt);
    io::stdout().flush()?;
    io::stdin().read_line(&mut user_input).map(|_| user_input)
}
