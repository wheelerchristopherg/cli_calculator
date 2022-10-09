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
}

impl Token {
    pub fn new_variable(value: &str) -> Self {
        Token::Variable(value.to_owned())
    }

    fn is_paren(value: &str) -> Option<ParenType> {
        match value {
            "(" => Some(ParenType::OpenParen),
            ")" => Some(ParenType::CloseParen),
            _ => None,
        }
    }

    fn is_number(value: &str) -> Option<Num> {
        if let Ok(int_val) = value.parse::<i64>() {
            Some(Num::Integer(int_val))
        } else if let Ok(float_val) = value.parse::<f64>() {
            Some(Num::Float(float_val))
        } else {
            None
        }
    }

    fn is_operator(value: &str) -> Option<Op> {
        match value {
            "+" => Some(Op::Add),
            "-" => Some(Op::Sub),
            "*" => Some(Op::Mult),
            "/" => Some(Op::Div),
            _ => None,
        }
    }

    fn is_whitespace(value: &str) -> bool {
        let trimmed = value.trim();
        trimmed == ""
    }
}

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        if let Some(paren_type) = Self::is_paren(value) {
            Token::Paren(paren_type)
        } else if let Some(number) = Self::is_number(value) {
            Token::Number(number)
        } else if let Some(op) = Self::is_operator(value) {
            Token::Operator(op)
        } else if Self::is_whitespace(value) {
            Token::Whitespace
        } else {
            Token::EOL
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_paren() {
        assert_eq!(Token::from("("), Token::Paren(ParenType::OpenParen))
    }
}
