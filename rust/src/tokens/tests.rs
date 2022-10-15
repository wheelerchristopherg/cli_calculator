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
