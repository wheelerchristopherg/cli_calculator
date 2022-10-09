use crate::tokens::Token;
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct NotAsciiError;

impl Display for NotAsciiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expression was not an ASCII string.")
    }
}

impl Error for NotAsciiError {}

#[derive(Debug)]
enum State {
    Initial,
    Integer,
    FloatStart,
    OpenParen,
    CloseParen,
    Minus,
    Plus,
    Multiply,
    Divide,
    Float,
    EOL,
    Whitespace,
    Variable,
}

enum CharType {
    Letter,
    Digit,
    Whitespace,
    Other(char),
}

#[derive(Debug)]
pub struct TokenParser {
    current_state: State,
    expression: String,
    token_offset: usize,
}

impl TokenParser {
    pub fn new(expression: String) -> Result<TokenParser, NotAsciiError> {
        if !expression.is_ascii() {
            Err(NotAsciiError)
        } else {
            Ok(TokenParser {
                current_state: State::Initial,
                expression,
                token_offset: 0,
            })
        }
    }

    pub fn get_tokens(&mut self) -> Vec<Token> {
        let mut token_vec: Vec<Token> = vec![];
        while let Some(next) = self.next_token() {
            #[cfg(test)]
            println!("resolved token: {:?}", next);
            if next != Token::Whitespace {
                token_vec.push(next.clone());
            }
        }
        token_vec
    }

    fn next_token(&mut self) -> Option<Token> {
        let mut resolved_token = None;
        for (i, b) in self.expression[self.token_offset..].bytes().enumerate() {
            let t: Option<Token> = match Self::transition(&self.current_state, &(b as char)) {
                Some(new_state) => {
                    #[cfg(test)]
                    println!(
                        "{:?} -> {:?}: {:?}",
                        self.current_state,
                        new_state,
                        &self.expression[self.token_offset..self.token_offset + i + 1]
                    );
                    self.current_state = new_state;
                    None
                }
                None => Self::token_from_state(
                    &self.current_state,
                    &self.expression[self.token_offset..self.token_offset + i],
                ),
            };
            if let Some(resolved) = t {
                self.current_state = State::Initial;
                self.token_offset = self.token_offset + i;
                resolved_token = Some(resolved.clone());
                break;
            }
        }
        resolved_token
    }

    fn token_from_state(state: &State, value: &str) -> Option<Token> {
        match state {
            State::Initial => None,
            State::FloatStart => None,
            State::EOL => Some(Token::EOL),
            State::Variable => Some(Token::new_variable(value)),
            _ => Some(Token::from(value)),
        }
    }

    fn transition(current_state: &State, next_char: &char) -> Option<State> {
        let char_type = Self::get_char_type(*next_char);
        match (current_state, char_type) {
            (State::Initial, CharType::Digit) => Some(State::Integer),
            (State::Initial, CharType::Other('.')) => Some(State::FloatStart),
            (State::Initial, CharType::Other('(')) => Some(State::OpenParen),
            (State::Initial, CharType::Other(')')) => Some(State::CloseParen),
            (State::Initial, CharType::Other('-')) => Some(State::Minus),
            (State::Initial, CharType::Other('+')) => Some(State::Plus),
            (State::Initial, CharType::Other('*')) => Some(State::Multiply),
            (State::Initial, CharType::Other('/')) => Some(State::Divide),
            (State::Initial, CharType::Other('\n')) => Some(State::EOL),
            (State::Initial, CharType::Whitespace) => Some(State::Whitespace),
            (State::Initial, CharType::Letter) => Some(State::Variable),
            (State::Initial, CharType::Other('_')) => Some(State::Variable),
            //
            (State::Integer, CharType::Digit) => Some(State::Integer),
            (State::Integer, CharType::Other('.')) => Some(State::FloatStart),
            //
            (State::FloatStart, CharType::Digit) => Some(State::Float),
            //
            (State::Float, CharType::Digit) => Some(State::Float),
            //
            (State::Whitespace, CharType::Whitespace) => Some(State::Whitespace),
            //
            (State::Variable, CharType::Letter) => Some(State::Variable),
            (State::Variable, CharType::Digit) => Some(State::Variable),
            (State::Variable, CharType::Other('_')) => Some(State::Variable),
            _ => None,
        }
    }

    fn get_char_type(c: char) -> CharType {
        if c.is_ascii_alphabetic() {
            CharType::Letter
        } else if c.is_ascii_digit() {
            CharType::Digit
        } else if Self::is_whitespace(c) {
            CharType::Whitespace
        } else {
            CharType::Other(c)
        }
    }

    fn is_whitespace(c: char) -> bool {
        c == ' ' || c == '\t'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_all_tokens() {
        let expression = String::from(" \t1.24  +9-(x0\t*(7.2  /3)) _92.19\n");
        let mut parser = TokenParser::new(expression).expect("Expression was not ascii.");
        let t: Vec<Token> = parser.get_tokens();

        let expected_tokens = vec![
            Token::from("1.24"),
            Token::from("+"),
            Token::from("9"),
            Token::from("-"),
            Token::from("("),
            Token::new_variable("x0"),
            Token::from("*"),
            Token::from("("),
            Token::from("7.2"),
            Token::from("/"),
            Token::from("3"),
            Token::from(")"),
            Token::from(")"),
            Token::new_variable("_92"),
            Token::from(".19"),
        ];

        assert_eq!(t, expected_tokens);
    }

    #[test]
    fn fail_on_non_ascii() {
        let expression = String::from(" \u{0190}\t1.24  +9-(x0\t*(7.2  /3)) _92.19\n");
        let parse_error = TokenParser::new(expression).expect_err("Expression was not ascii.");
        println!("error: {}", parse_error)
    }

    #[test]
    fn parse_failed() {
        let expression = String::from("a * 7.(");
        let mut parser = TokenParser::new(expression).expect("Expression was not ascii.");
        let t: Vec<Token> = parser.get_tokens();
        let expected = vec![];
        assert_eq!(t, expected);
    }
}
