#[cfg(test)]
mod tests;

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
    pub fn new(expression: &String) -> Result<TokenParser, NotAsciiError> {
        if !expression.is_ascii() {
            Err(NotAsciiError)
        } else {
            Ok(TokenParser {
                current_state: State::Initial,
                expression: expression.clone(),
                token_offset: 0,
            })
        }
    }

    pub fn get_tokens(&mut self) -> Vec<Token> {
        let mut token_vec: Vec<Token> = vec![];
        while self.token_offset < self.expression.len() {
            let next = match self.next_token() {
                Some(n) => n,
                None => break,
            };
            #[cfg(test)]
            println!("resolved token: {:?}", next);
            if next != Token::Whitespace {
                token_vec.push(next.clone());
            }
        }
        token_vec
    }

    fn next_token(&mut self) -> Option<Token> {
        self.current_state = State::Initial;
        let mut resolved_token = None;
        for (i, b) in self.expression[self.token_offset..].bytes().enumerate() {
            let next_state = Self::transition(&self.current_state, &(b as char));
            match next_state {
                None => {
                    #[cfg(test)]
                    println!("transition failed, resolving");
                    let start = self.token_offset;
                    let end = if i == 0 { 1 } else { i };
                    resolved_token = Some(Self::token_from_state(
                        &self.current_state,
                        &self.expression[start..start + end],
                    ));
                    self.token_offset += end;
                    break;
                }
                Some(state) => {
                    #[cfg(test)]
                    println!("{:?} -> {:?}", self.current_state, state);
                    self.current_state = state;
                }
            }

            if self.token_offset + i + 1 == self.expression.len() {
                #[cfg(test)]
                println!(
                    "no more characters, resolving, state = {:?}",
                    self.current_state
                );
                resolved_token = Some(Self::token_from_state(
                    &self.current_state,
                    &self.expression[self.token_offset..self.token_offset + i + 1],
                ));
                self.token_offset += i + 1;
            }
        }
        resolved_token
    }

    fn token_from_state(state: &State, value: &str) -> Token {
        match state {
            State::Initial => Token::InvalidToken(value.to_owned()),
            State::FloatStart => Token::InvalidToken(value.to_owned()),
            State::EOL => Token::EOL,
            State::Variable => Token::new_variable(value),
            State::Integer => Token::new_number(value),
            State::Float => Token::new_number(value),
            State::OpenParen => Token::new_paren(value),
            State::CloseParen => Token::new_paren(value),
            State::Minus => Token::new_op(value),
            State::Plus => Token::new_op(value),
            State::Multiply => Token::new_op(value),
            State::Divide => Token::new_op(value),
            State::Whitespace => Token::Whitespace,
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