use super::*;

#[test]
fn simple_evaluate() {
    let left = AST::new(Token::new_number("19"));
    let right = AST::new(Token::new_number("2"));
    let mut root = AST::new(Token::new_op("/"));
    root.set_left(Some(left));
    root.set_right(Some(right));

    let val = root.evaluate().expect("should be 9.5");
    assert_eq!(val, 9.5);
}

#[test]
fn complex_evaluate() {
    let left = AST::new(Token::new_number("19"));
    let right = AST::new(Token::new_number("2"));
    let mut root = AST::new(Token::new_op("+"));
    root.set_left(Some(left));
    root.set_right(Some(right));

    let left = root;
    let right = AST::new(Token::new_number("3"));
    let mut root = AST::new(Token::new_op("/"));
    root.set_left(Some(left));
    root.set_right(Some(right));

    let left = root;
    let right = AST::new(Token::new_number("10.5"));
    let mut root = AST::new(Token::new_op("-"));
    root.set_left(Some(left));
    root.set_right(Some(right));

    let val = root.evaluate().expect("should be 7.0");
    assert_eq!(val, -3.5);
}

#[test]
fn no_left_node() {
    let root = AST::new(Token::new_op("/"));

    let error = root.evaluate().expect_err("should throw 'No Left Node'");
    assert_eq!(error, "No Left Node");
}

#[test]
fn no_right_node() {
    let left = AST::new(Token::new_number("19"));
    let mut root = AST::new(Token::new_op("/"));
    root.set_left(Some(left));

    let error = root.evaluate().expect_err("should throw 'No Right Node'");
    assert_eq!(error, "No Right Node");
}

#[test]
fn divide_by_zero() {
    let left = AST::new(Token::new_number("19"));
    let right = AST::new(Token::new_number("0"));
    let mut root = AST::new(Token::new_op("/"));
    root.set_left(Some(left));
    root.set_right(Some(right));

    let error = root.evaluate().expect_err("should throw 'Divide by Zero'");
    assert_eq!(error, "Divide by Zero");
}

#[test]
fn cannot_evaluate_paren() {
    let left = AST::new(Token::new_number("19"));
    let right = AST::new(Token::new_paren("("));
    let mut root = AST::new(Token::new_op("/"));
    root.set_left(Some(left));
    root.set_right(Some(right));

    let error = root
        .evaluate()
        .expect_err("should throw 'Cannot evaluate Paren(OpenParen)'");
    assert_eq!(error, "Cannot evaluate Paren(OpenParen)");
}
