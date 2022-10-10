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
        loop {
            let next = match self.next_token() {
                Err(s) => Token::InvalidToken(s),
                Ok(Some(n)) => n,
                Ok(None) => break,
            };
            #[cfg(test)]
            println!("resolved token: {:?}", next);
            if next != Token::Whitespace {
                token_vec.push(next.clone());
            }
        }
        token_vec
    }

    fn next_token(&mut self) -> Result<Option<Token>, String> {
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
                None => {
                    #[cfg(test)]
                    println!("transition failed");
                    let value = &self.expression[self.token_offset..self.token_offset + i];
                    Some(Self::token_from_state(&self.current_state, value))
                }
            };
            if let Some(resolved) = t {
                self.current_state = State::Initial;
                if i > 0 {
                    self.token_offset = self.token_offset + i;
                    if let Token::InvalidToken(e) = resolved {
                        return Err(e);
                    } else {
                        resolved_token = Some(resolved.clone());
                        break;
                    }
                } else {
                    resolved_token = Some(Token::InvalidToken(
                        self.expression[self.token_offset..self.token_offset + 1].to_owned(),
                    ));
                    self.token_offset += 1;
                    break;
                }
            }
        }
        Ok(resolved_token)
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
        let expression = String::from("a * 7.(8821) _+ ");
        let mut parser = TokenParser::new(expression).expect("Expression was not ascii.");
        let t: Vec<Token> = parser.get_tokens();
        let expected = vec![
            Token::new_variable("a"),
            Token::new_op("*"),
            Token::InvalidToken("7.".to_owned()),
            Token::new_paren("("),
            Token::new_number("8821"),
            Token::new_paren(")"),
            Token::new_variable("_"),
            Token::new_op("+"),
        ];
        assert_eq!(t, expected);
    }

    #[test]
    fn invalid_char() {
        let expression = String::from("15 ===  &@3.4 + ^%$#12 \n");
        let mut parser = TokenParser::new(expression).expect("Expression was not ascii.");
        let t: Vec<Token> = parser.get_tokens();
        let expected = vec![
            Token::new_number("15"),
            Token::InvalidToken("=".to_owned()),
            Token::InvalidToken("=".to_owned()),
            Token::InvalidToken("=".to_owned()),
            Token::InvalidToken("&".to_owned()),
            Token::InvalidToken("@".to_owned()),
            Token::new_number("3.4"),
            Token::new_op("+"),
            Token::InvalidToken("^".to_owned()),
            Token::InvalidToken("%".to_owned()),
            Token::InvalidToken("$".to_owned()),
            Token::InvalidToken("#".to_owned()),
            Token::new_number("12"),
        ];
        assert_eq!(t, expected);
    }
}
