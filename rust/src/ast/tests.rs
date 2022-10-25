use super::*;

#[test]
fn simple_evaluate() {
    let left = AST::new(Token::new_number("19"), None, None);
    let right = AST::new(Token::new_number("2"), None, None);
    let root = AST::new(Token::new_op("/"), Some(left), Some(right));

    let val = root.evaluate().expect("should be 9.5");
    assert_eq!(val, 9.5);
}

#[test]
fn complex_evaluate() {
    let left = AST::new(Token::new_number("19"), None, None);
    let right = AST::new(Token::new_number("2"), None, None);
    let root = AST::new(Token::new_op("+"), Some(left), Some(right));

    let left = root;
    let right = AST::new(Token::new_number("3"), None, None);
    let root = AST::new(Token::new_op("/"), Some(left), Some(right));

    let left = root;
    let right = AST::new(Token::new_number("10.5"), None, None);
    let root = AST::new(Token::new_op("-"), Some(left), Some(right));

    let val = root.evaluate().expect("should be 7.0");
    assert_eq!(val, -3.5);
}

#[test]
fn no_left_node() {
    let root = AST::new(Token::new_op("/"), None, None);

    let error = root.evaluate().expect_err("should throw 'No Left Node'");
    assert_eq!(error, "No Left Node");
}

#[test]
fn no_right_node() {
    let left = AST::new(Token::new_number("19"), None, None);
    let root = AST::new(Token::new_op("/"), Some(left), None);

    let error = root.evaluate().expect_err("should throw 'No Right Node'");
    assert_eq!(error, "No Right Node");
}

#[test]
fn divide_by_zero() {
    let left = AST::new(Token::new_number("19"), None, None);
    let right = AST::new(Token::new_number("0"), None, None);
    let root = AST::new(Token::new_op("/"), Some(left), Some(right));

    let error = root.evaluate().expect_err("should throw 'Divide by Zero'");
    assert_eq!(error, "Divide by Zero");
}

#[test]
fn cannot_evaluate_paren() {
    let left = AST::new(Token::new_number("19"), None, None);
    let right = AST::new(Token::new_paren("("), None, None);
    let root = AST::new(Token::new_op("/"), Some(left), Some(right));

    let error = root
        .evaluate()
        .expect_err("should throw 'Cannot evaluate ('");
    assert_eq!(error, "Cannot evaluate (");
}

#[test]
fn build_tree() {
    let v = vec![
        Token::new_number("1.24"),
        Token::new_op("+"),
        Token::new_number("9"),
        Token::new_op("-"),
        Token::new_number(".19"),
        Token::new_op("*"),
        Token::new_number("15"),
        Token::new_op("-"),
        Token::new_number("120"),
        Token::new_op("/"),
        Token::new_number("10"),
    ];

    let root: Box<AST> = AST::build_tree(&v).expect("The tree should build successfully.");
    let result = root.evaluate().expect("the result should be a real value");
    assert_eq!(result, -4.609999999999999);
}

#[test]
fn build_tree1() {
    let v = vec![
        Token::new_number("1.24"),
        Token::new_op("+"),
        Token::new_number("9"),
        Token::new_op("-"),
        Token::new_number(".19"),
        Token::new_op("*"),
        Token::new_number("15"),
        Token::new_op("-"),
        Token::new_number("120"),
        Token::new_op("/"),
        Token::new_number("10"),
        Token::EOL,
    ];

    let root: Box<AST> = AST::build_tree(&v).expect("the tree should build successfully");
    let result = root.evaluate().expect("should be a real value");
    assert_eq!(result, -4.609999999999999);
}
