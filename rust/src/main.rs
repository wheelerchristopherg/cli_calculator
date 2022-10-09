mod tokens;
use std::io;
use tokens::Token;

fn main() {
    loop {
        println!("Enter Expression:");
        match read_line() {
            Ok(expression) => {
                let exp = expression.trim();
                if exp == "q" {
                    break;
                }
                let str_tokens = exp.split(" ");
                let mut parsed_tokens: Vec<Token> = Vec::new();
                for t in str_tokens {
                    parsed_tokens.push(Token::from(t))
                }
                println!("Tokens: {:?}", parsed_tokens)
            }
            Err(_) => {
                println!("Oops! Something went wrong!");
            }
        }
    }
}

fn read_line() -> Result<String, io::Error> {
    let mut user_input = String::new();
    let stdin = io::stdin();
    match stdin.read_line(&mut user_input) {
        Ok(_) => Ok(user_input),
        Err(e) => Err(e),
    }
}
