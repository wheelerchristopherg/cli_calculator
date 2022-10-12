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
        if let Some(num) = Self::is_number(value) {
            Token::Number(num)
        } else {
            Token::InvalidToken(value.to_owned())
        }
    }

    pub fn new_paren(value: &str) -> Self {
        if let Some(paren) = Self::is_paren(value) {
            Token::Paren(paren)
        } else {
            Token::InvalidToken(value.to_owned())
        }
    }

    pub fn new_op(value: &str) -> Self {
        if let Some(op) = Self::is_operator(value) {
            Token::Operator(op)
        } else {
            Token::InvalidToken(value.to_owned())
        }
    }

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_number() {
        let t = Token::new_number("1");
        assert_eq!(t, Token::Number(Num::Integer(1)));

        let t = Token::new_number("1937");
        assert_eq!(t, Token::Number(Num::Integer(1937)));

        let t = Token::new_number("-909082");
        assert_eq!(t, Token::Number(Num::Integer(-909082)));

        let t = Token::new_number("15.281");
        assert_eq!(t, Token::Number(Num::Float(15.281)));

        let t = Token::new_number("0.237");
        assert_eq!(t, Token::Number(Num::Float(0.237)));

        let t = Token::new_number("10.");
        assert_eq!(t, Token::Number(Num::Float(10.0)));
    }

    #[test]
    fn test_new_number_invalid() {
        let t = Token::new_number("10.e");
        assert_eq!(t, Token::InvalidToken("10.e".to_owned()));

        let t = Token::new_number("10ef");
        assert_eq!(t, Token::InvalidToken("10ef".to_owned()));

        let t = Token::new_number("(");
        assert_eq!(t, Token::InvalidToken("(".to_owned()));

        let t = Token::new_number("+");
        assert_eq!(t, Token::InvalidToken("+".to_owned()));
    }

    #[test]
    fn test_new_op() {
        let t = Token::new_op("+");
        assert_eq!(t, Token::Operator(Op::Add));

        let t = Token::new_op("-");
        assert_eq!(t, Token::Operator(Op::Sub));

        let t = Token::new_op("/");
        assert_eq!(t, Token::Operator(Op::Div));

        let t = Token::new_op("*");
        assert_eq!(t, Token::Operator(Op::Mult));
    }

    #[test]
    fn test_new_op_invalid() {
        let t = Token::new_op("10.e");
        assert_eq!(t, Token::InvalidToken("10.e".to_owned()));

        let t = Token::new_op("10ef");
        assert_eq!(t, Token::InvalidToken("10ef".to_owned()));

        let t = Token::new_op("(");
        assert_eq!(t, Token::InvalidToken("(".to_owned()));

        let t = Token::new_op("+1");
        assert_eq!(t, Token::InvalidToken("+1".to_owned()));
    }

    #[test]
    fn test_new_paren() {
        let t = Token::new_paren("(");
        assert_eq!(t, Token::Paren(ParenType::OpenParen));

        let t = Token::new_paren(")");
        assert_eq!(t, Token::Paren(ParenType::CloseParen));
    }

    #[test]
    fn test_new_paren_invalid() {
        let t = Token::new_paren("10.e");
        assert_eq!(t, Token::InvalidToken("10.e".to_owned()));

        let t = Token::new_paren("10ef");
        assert_eq!(t, Token::InvalidToken("10ef".to_owned()));

        let t = Token::new_paren("(_");
        assert_eq!(t, Token::InvalidToken("(_".to_owned()));

        let t = Token::new_paren("+1");
        assert_eq!(t, Token::InvalidToken("+1".to_owned()));
    }

    #[test]
    fn test_new_variable() {
        let t = Token::new_variable("dkainas;kf;jkaspfkjsfj");
        assert_eq!(t, Token::Variable("dkainas;kf;jkaspfkjsfj".to_owned()));
    }
}
