use super::*;

#[test]
fn parse_all_tokens() {
    let expression = String::from(" \t1.24  +9-(x0\t*(7.2  /3)) _92.19\n");
    let mut parser = TokenParser::new(&expression).unwrap();
    let t: Vec<Token> = parser.get_tokens().expect("should parse successfully");
    let expected_tokens = vec![
        Token::new_number("1.24"),
        Token::new_op("+"),
        Token::new_number("9"),
        Token::new_op("-"),
        Token::new_paren("("),
        Token::new_variable("x0"),
        Token::new_op("*"),
        Token::new_paren("("),
        Token::new_number("7.2"),
        Token::new_op("/"),
        Token::new_number("3"),
        Token::new_paren(")"),
        Token::new_paren(")"),
        Token::new_variable("_92"),
        Token::new_number(".19"),
        Token::EOL,
    ];
    assert_eq!(t, expected_tokens);
}

#[test]
fn fail_on_non_ascii() {
    let expression = String::from(" \u{0190}\t1.24  +9-(x0\t*(7.2  /3)) _92.19\n");
    let parse_error = TokenParser::new(&expression).expect_err("should throw a NotAsciiError");
    assert_eq!(parse_error, NotAsciiError);
}

#[test]
fn parse_failed() {
    let expression = String::from("a * 7.(8821) _+ ");
    let mut parser = TokenParser::new(&expression).unwrap();
    let result = parser.get_tokens().expect_err("");
    let expected = InvalidTokenError {
        position: 6,
        value: "7.".to_owned(),
    };
    assert_eq!(result, expected);
}

#[test]
fn invalid_char() {
    let expression = String::from("15 ===  &@3.4 + ^%$#12 \n");
    let mut parser = TokenParser::new(&expression).unwrap();
    let result = parser
        .get_tokens()
        .expect_err("should return InvalidTokenError");
    let expected = InvalidTokenError {
        position: 4,
        value: "=".to_owned(),
    };
    assert_eq!(result, expected);
}

#[test]
fn no_new_line() {
    let expression = String::from("15");
    let mut parser = TokenParser::new(&expression).unwrap();
    let t: Vec<Token> = parser.get_tokens().expect("should parse successfully");
    let expected = vec![Token::new_number("15")];
    assert_eq!(t, expected);
}

#[test]
fn empty_expression() {
    let expression = String::from("");
    let mut parser = TokenParser::new(&expression).unwrap();
    let t: Vec<Token> = parser.get_tokens().expect("should parse successfully");
    let expected = Vec::<Token>::new();
    assert_eq!(t, expected);
}

#[test]
fn multiple_eol() {
    let expression = String::from("1.3 +  9/2 \n (8.2  9) / 3\n 89\n");
    let mut parser = TokenParser::new(&expression).unwrap();
    let t: Vec<Token> = parser.get_tokens().expect("should parse successfully");
    let expected = vec![
        Token::new_number("1.3"),
        Token::new_op("+"),
        Token::new_number("9"),
        Token::new_op("/"),
        Token::new_number("2"),
        Token::EOL,
        Token::new_paren("("),
        Token::new_number("8.2"),
        Token::new_number("9"),
        Token::new_paren(")"),
        Token::new_op("/"),
        Token::new_number("3"),
        Token::EOL,
        Token::new_number("89"),
        Token::EOL,
    ];
    assert_eq!(t, expected);
}
