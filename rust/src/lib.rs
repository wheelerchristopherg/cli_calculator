pub mod ast;
pub mod lexical_analyzer;
pub mod tokens;

use std::{
    collections::HashMap,
    io::{self, Write},
};

use ast::AST;
use lexical_analyzer::TokenParser;
use tokens::Token;

pub fn main_loop() {
    let mut env = HashMap::new();
    env.insert("pi".to_owned(), AST::new_leaf(Token::new_number("3.14159")));
    let mut result_index = 0;
    loop {
        if let Ok(expression) = read_line("> ") {
            let exp = expression.trim();
            if exp.is_empty() || exp == "q" {
                break;
            }

            let result = evaluate_string_expression(&expression, &mut env, result_index);
            match result {
                Ok(value) => {
                    println!("{}", value);
                    result_index += 1;
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
    }
}

pub fn evaluate_string_expression(
    expression: &str,
    env: &mut HashMap<String, Box<AST>>,
    index: i32,
) -> Result<String, String> {
    let parsed_tokens: Vec<Token> = match parse_tokens(expression) {
        Ok(parsed) => parsed,
        Err(e) => return Err(e),
    };

    let tree: Box<AST> = match AST::build_tree(&parsed_tokens) {
        Ok(parsed) => parsed,
        Err(e) => return Err(e),
    };
    println!("tree: {}", tree);

    match tree.evaluate(env) {
        Ok(result) => {
            let result_ast = AST::new_leaf(Token::new_number(&result.to_string()));
            env.insert(format!("x{}", index), result_ast);
            if result == result.floor() {
                Ok(format!("x{} = {}.0", index, result))
            } else {
                Ok(format!("x{} = {}", index, result))
            }
        }
        Err(e) => Err(e),
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
