use std::fmt::Display;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Clone)]
pub enum ParenType {
    OpenParen,
    CloseParen,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Num {
    Float(f64),
    Integer(i64),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Add,
    Sub,
    Mult,
    Div,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Paren(ParenType),
    Number(Num),
    Operator(Op),
    Variable(String),
    Whitespace,
    EOL,
    InvalidToken(String),
}

impl Token {
    pub fn new_number(value: &str) -> Self {
        if let Some(num) = Self::parse_number(value) {
            Token::Number(num)
        } else {
            Token::InvalidToken(value.to_owned())
        }
    }

    pub fn new_paren(value: &str) -> Self {
        if let Some(paren) = Self::parse_paren(value) {
            Token::Paren(paren)
        } else {
            Token::InvalidToken(value.to_owned())
        }
    }

    pub fn new_op(value: &str) -> Self {
        if let Some(op) = Self::parse_operator(value) {
            Token::Operator(op)
        } else {
            Token::InvalidToken(value.to_owned())
        }
    }

    pub fn new_variable(value: &str) -> Self {
        Token::Variable(value.to_owned())
    }

    fn parse_paren(value: &str) -> Option<ParenType> {
        match value {
            "(" => Some(ParenType::OpenParen),
            ")" => Some(ParenType::CloseParen),
            _ => None,
        }
    }

    fn parse_number(value: &str) -> Option<Num> {
        if let Ok(int_val) = value.parse::<i64>() {
            Some(Num::Integer(int_val))
        } else if let Ok(float_val) = value.parse::<f64>() {
            Some(Num::Float(float_val))
        } else {
            None
        }
    }

    fn parse_operator(value: &str) -> Option<Op> {
        match value {
            "+" => Some(Op::Add),
            "-" => Some(Op::Sub),
            "*" => Some(Op::Mult),
            "/" => Some(Op::Div),
            _ => None,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Paren(ParenType::OpenParen) => write!(f, "("),
            Token::Paren(ParenType::CloseParen) => write!(f, ")"),
            Token::Number(Num::Float(x)) => write!(f, "{}", x),
            Token::Number(Num::Integer(x)) => write!(f, "{}", x),
            Token::Operator(Op::Add) => write!(f, "+"),
            Token::Operator(Op::Sub) => write!(f, "-"),
            Token::Operator(Op::Mult) => write!(f, "*"),
            Token::Operator(Op::Div) => write!(f, "/"),
            Token::Variable(x) => write!(f, "{}", x),
            Token::Whitespace => write!(f, "''"),
            Token::EOL => write!(f, "EoL"),
            Token::InvalidToken(_) => write!(f, "Invalid"),
        }
    }
}
