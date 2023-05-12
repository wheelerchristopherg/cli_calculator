use std::collections::HashMap;

use super::*;

#[test]
fn simple_evaluate() {
    let left = AST::new(Token::new_number("19"), None, None);
    let right = AST::new(Token::new_number("2"), None, None);
    let root = AST::new(Token::new_op("/"), Some(left), Some(right));

    let env = HashMap::new();
    let val = AST::evaluate(root, &env).expect("should be 9.5");
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

    let env = HashMap::new();
    let val = AST::evaluate(root, &env).expect("should be 7.0");
    assert_eq!(val, -3.5);
}

#[test]
fn no_left_node() {
    let root = AST::new(Token::new_op("/"), None, None);
    let env = HashMap::new();
    let error = AST::evaluate(root, &env).expect_err("should throw 'Invalid Expression'");
    assert_eq!(error, "Invalid Expression");
}

#[test]
fn no_right_node() {
    let left = AST::new(Token::new_number("19"), None, None);
    let root = AST::new(Token::new_op("/"), Some(left), None);
    let env = HashMap::new();
    let error = AST::evaluate(root, &env).expect_err("should throw 'Invalid Expression'");
    assert_eq!(error, "Invalid Expression");
}

#[test]
fn divide_by_zero() {
    let left = AST::new(Token::new_number("19"), None, None);
    let right = AST::new(Token::new_number("0"), None, None);
    let root = AST::new(Token::new_op("/"), Some(left), Some(right));
    let env = HashMap::new();
    let error = AST::evaluate(root, &env).expect_err("should throw 'Divide by Zero'");
    assert_eq!(error, "Divide by Zero");
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
    let env = HashMap::new();
    let root: Box<AST> = AST::build_tree(&v).expect("The tree should build successfully.");
    println!("AST: {}", root);
    let result = AST::evaluate(root, &env).expect("the result should be a real value");
    assert_eq!(result, -4.609999999999999);
}

#[test]
fn evaluate_single_number() {
    let v = vec![Token::new_number("1.24"), Token::EOL];
    let env = HashMap::new();
    let root: Box<AST> = AST::build_tree(&v).expect("the tree should build successfully");
    let result = AST::evaluate(root, &env).expect("should be a real value");
    assert_eq!(result, 1.24);
}

#[test]
fn invalid_tree() {
    let v = vec![
        Token::new_number("1"),
        Token::new_op("-"),
        Token::new_op("+"),
        Token::EOL,
    ];
    let error_msg = AST::build_tree(&v).expect_err("Tree should fail to build");
    assert_eq!(error_msg, "Invalid Expression");
}

#[test]
fn evaluate_paren() {
    let v = vec![
        Token::new_paren("("),
        Token::new_number("10"),
        Token::new_op("+"),
        Token::new_paren("("),
        Token::new_number("9"),
        Token::new_op("-"),
        Token::new_number("19"),
        Token::new_paren(")"),
        Token::new_op("*"),
        Token::new_paren("("),
        Token::new_number("10"),
        Token::new_op("+"),
        Token::new_number("5"),
        Token::new_paren(")"),
        Token::new_op("-"),
        Token::new_number("120"),
        Token::new_paren(")"),
        Token::new_op("/"),
        Token::new_number("10"),
    ];
    let env = HashMap::new();
    let root: Box<AST> = AST::build_tree(&v).expect("The tree should build successfully.");
    println!("AST: {}", root);
    let result = AST::evaluate(root, &env).expect("the result should be a real value");
    assert_eq!(result, -26.0);
}

#[test]
fn evaluate_variables() {
    let mut env = HashMap::new();
    env.insert("x0".to_owned(), 9.2);
    let v = vec![
        Token::new_variable("x0"),
        Token::new_op("+"),
        Token::new_number("10"),
    ];
    let root: Box<AST> = AST::build_tree(&v).expect("The tree should build successfully.");
    let result = AST::evaluate(root, &env).expect("the result should be a real value");
    assert_eq!(result, 19.2);
}

#[test]
fn evaluate_paren_missing_close_paren() {
    let v = vec![
        Token::new_paren("("),
        Token::new_number("10"),
        Token::new_op("+"),
        Token::new_paren("("),
        Token::new_number("9"),
        Token::new_op("-"),
        Token::new_number("19"),
        Token::new_paren(")"),
        Token::new_op("*"),
        Token::new_paren("("),
        Token::new_number("10"),
        Token::new_op("+"),
        Token::new_number("5"),
        Token::new_paren(")"),
        Token::new_op("-"),
        Token::new_number("120"),
        Token::new_op("/"),
        Token::new_number("10"),
    ];
    let error: String = AST::build_tree(&v).expect_err("The tree should fail to build.");
    assert_eq!(error, "Missing )");
}

#[test]
fn evaluate_paren_too_many_close_paren() {
    let v = vec![
        Token::new_number("10"),
        Token::new_op("+"),
        Token::new_paren("("),
        Token::new_number("9"),
        Token::new_op("-"),
        Token::new_number("19"),
        Token::new_paren(")"),
        Token::new_op("*"),
        Token::new_paren("("),
        Token::new_number("10"),
        Token::new_op("+"),
        Token::new_number("5"),
        Token::new_paren(")"),
        Token::new_op("-"),
        Token::new_number("120"),
        Token::new_paren(")"),
        Token::new_op("/"),
        Token::new_number("10"),
    ];
    let error: String = AST::build_tree(&v).expect_err("The tree should fail to build.");
    assert_eq!(error, "Extra )");
}

#[test]
fn no_operation_in_expression() {
    let v = vec![Token::new_number("10.2"), Token::new_number("0.3")];
    let error: String = AST::build_tree(&v).expect_err("The tree should fail to build.");
    assert_eq!(error, "Invalid Expression");
}

#[test]
fn negative_numbers_at_start_of_expression() {
    let v = vec![Token::new_op("-"), Token::new_number("10")];
    let tree = AST::build_tree(&v).expect("expect tree with '-10' to build");
    let env = HashMap::new();
    let result = AST::evaluate(tree, &env).expect("expect result -10");
    assert_eq!(result, -10.0);

    let v = vec![Token::new_op("-"), Token::new_variable("var")];
    let tree = AST::build_tree(&v).expect("expect tree with '-var' to build");
    let mut env = HashMap::new();
    env.insert("var".to_string(), 24.1);
    let result = AST::evaluate(tree, &env).expect("expect result -24.1");
    assert_eq!(result, -24.1);

    let v = vec![
        Token::new_op("-"),
        Token::new_paren("("),
        Token::new_number("267"),
        Token::new_paren(")"),
    ];
    let tree = AST::build_tree(&v).expect("expect tree with '-(267)' to build");
    let env = HashMap::new();
    let result = AST::evaluate(tree, &env).expect("expect result -267");
    assert_eq!(result, -267.0);

    let v = vec![
        Token::new_op("-"),
        Token::new_paren("("),
        Token::new_number("25"),
        Token::new_op("*"),
        Token::new_number("4"),
        Token::new_paren(")"),
    ];
    let tree = AST::build_tree(&v).expect("expect tree with '-(25 * 4)' to build");
    let env = HashMap::new();
    let result = AST::evaluate(tree, &env).expect("expect result -100");
    assert_eq!(result, -100.0);
}

#[test]
fn negative_numbers_after_operator() {
    for (i, operator) in [
        Token::new_op("+"),
        Token::new_op("-"),
        Token::new_op("*"),
        Token::new_op("/"),
    ]
    .iter()
    .enumerate()
    {
        let v = vec![
            Token::new_number("7"),
            operator.clone(),
            Token::new_op("-"),
            Token::new_number("10"),
        ];
        let tree = AST::build_tree(&v).expect("expect tree to build for 7 _ -10");
        let env = HashMap::new();
        let result0 = AST::evaluate(tree, &env).expect("expect numerical result");

        let v = vec![
            Token::new_number("2"),
            operator.clone(),
            Token::new_op("-"),
            Token::new_variable("var"),
        ];
        let tree = AST::build_tree(&v).expect("expect tree for 2 _ -var to build");
        let mut env = HashMap::new();
        env.insert("var".to_string(), 24.1);
        let result1 = AST::evaluate(tree, &env).expect("expect numerical result");

        let v = vec![
            Token::new_number("5"),
            operator.clone(),
            Token::new_op("-"),
            Token::new_paren("("),
            Token::new_number("267"),
            Token::new_paren(")"),
        ];
        let tree = AST::build_tree(&v).expect("expect tree for 5 _ -(267) to build");
        let env = HashMap::new();
        let result2 = AST::evaluate(tree, &env).expect("expect numerical result");

        let v = vec![
            Token::new_number("3"),
            operator.clone(),
            Token::new_op("-"),
            Token::new_paren("("),
            Token::new_number("25"),
            Token::new_op("*"),
            Token::new_number("4"),
            Token::new_paren(")"),
        ];
        let tree = AST::build_tree(&v).expect("expect tree for 3 _ -(25 * 4) to build");
        let env = HashMap::new();
        let result3 = AST::evaluate(tree, &env).expect("expect numerical result");

        match i {
            0 => {
                println!("Addition");
                assert_eq!(result0, 7.0 + -10.0);
                println!("result0 passed");
                assert_eq!(result1, 2.0 + -24.1);
                println!("result1 passed");
                assert_eq!(result2, 5.0 + -(267.0));
                println!("result2 passed");
                assert_eq!(result3, 3.0 + -(25.0 * 4.0));
                println!("result3 passed");
            }
            1 => {
                println!("Subtraction");
                assert_eq!(result0, 7.0 - -10.0);
                println!("result0 passed");
                assert_eq!(result1, 2.0 - -24.1);
                println!("result1 passed");
                assert_eq!(result2, 5.0 - -(267.0));
                println!("result2 passed");
                assert_eq!(result3, 3.0 - -(25.0 * 4.0));
                println!("result3 passed");
            }
            2 => {
                println!("Multiplication");
                assert_eq!(result0, 7.0 * -10.0);
                println!("result0 passed");
                assert_eq!(result1, 2.0 * -24.1);
                println!("result1 passed");
                assert_eq!(result2, 5.0 * -(267.0));
                println!("result2 passed");
                assert_eq!(result3, 3.0 * -(25.0 * 4.0));
                println!("result3 passed");
            }
            3 => {
                println!("Division");
                assert_eq!(result0, 7.0 / -10.0);
                println!("result0 passed");
                assert_eq!(result1, 2.0 / -24.1);
                println!("result1 passed");
                assert_eq!(result2, 5.0 / -(267.0));
                println!("result2 passed");
                assert_eq!(result3, 3.0 / -(25.0 * 4.0));
                println!("result3 passed");
            }
            _ => (),
        };
    }
}
