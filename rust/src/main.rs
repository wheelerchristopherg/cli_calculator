mod lexical_analyzer;
mod tokens;

use std::env;
use std::io::{self, Write};

use lexical_analyzer::TokenParser;
use tokens::Token;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        main_loop();
    } else if args.len() == 3 && args.get(1).unwrap_or(&"".to_owned()) == "--expression" {
        let default = "".to_owned();
        let e = args.get(2).unwrap_or(&default);
        parse_expression(e);
    } else {
        println!("invalid arguments");
    }
}

fn main_loop() {
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

fn parse_expression(expression: &String) {
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
