#[cfg(test)]
mod tests;

use crate::tokens::{Num, Op, ParenType, Token};
use std::{boxed::Box, cmp::Ordering, collections::HashMap, fmt::Display};

#[derive(Debug)]
pub struct AST {
    value: Token,
    left: Option<Box<AST>>,
    right: Option<Box<AST>>,
}

#[derive(Debug, Clone)]
struct WeightedToken {
    token: Token,
    weight: i32,
}

impl Display for WeightedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "('{}',{})", self.token, self.weight)
    }
}

impl AST {
    fn new(value: Token, left: Option<Box<AST>>, right: Option<Box<AST>>) -> Box<Self> {
        let ast = AST { left, right, value };
        Box::new(ast)
    }

    pub fn new_leaf(value: Token) -> Box<Self> {
        Self::new(value, None, None)
    }

    pub fn evaluate(ast: Box<Self>, env: &HashMap<String, f64>) -> Result<f64, String> {
        let mut stack = vec![ast];
        let mut resolve_stack: Vec<f64> = vec![];
        while !stack.is_empty() {
            let mut item = stack.pop().unwrap();
            match &item.value {
                Token::Operator(oper) => {
                    let next: Box<Self>;
                    if let Some(left) = item.left {
                        next = left;
                        item.left = None;
                        stack.push(item);
                        stack.push(next);
                    } else if let Some(right) = item.right {
                        next = right;
                        item.right = None;
                        stack.push(item);
                        stack.push(next);
                    } else {
                        let right = resolve_stack.pop();
                        let left = resolve_stack.pop();
                        if left.is_some() && right.is_some() {
                            resolve_stack.push(Self::evaluate_operator(
                                oper,
                                left.unwrap(),
                                right.unwrap(),
                            )?);
                        } else {
                            return Err("Invalid Expression".to_string());
                        }
                    }
                }
                Token::Variable(var) => {
                    resolve_stack.push(Self::evaluate_variable(var, env)?);
                }
                Token::Number(num) => {
                    resolve_stack.push(Self::evaluate_number(num)?);
                }
                x => Err(format!("Cannot evaluate {}", x))?,
            }
        }

        if resolve_stack.len() != 1 {
            Err("Invalid Expression".to_string())
        } else {
            resolve_stack.pop().ok_or("Invalid Expression".to_string())
        }
    }

    fn evaluate_operator(oper: &Op, left: f64, right: f64) -> Result<f64, String> {
        let result = match oper {
            Op::Add => left + right,
            Op::Sub => left - right,
            Op::Mult => left * right,
            Op::Div => {
                if right == 0.0 {
                    Err("Divide by Zero".to_owned())?
                }
                left / right
            }
        };
        Ok(result)
    }

    fn evaluate_number(num: &Num) -> Result<f64, String> {
        let result = match num {
            Num::Float(x) => *x,
            Num::Integer(x) => *x as f64,
        };
        Ok(result)
    }

    fn evaluate_variable(var: &String, env: &HashMap<String, f64>) -> Result<f64, String> {
        if let Some(&value) = env.get(var) {
            Ok(value)
        } else {
            Err(format!("Unknown Variable: {}", var))
        }
    }

    fn get_token_weight(token: &WeightedToken) -> Option<i32> {
        match &token.token {
            Token::Paren(_) => Some(token.weight - 2),
            Token::Number(_) => Some(token.weight - 2),
            Token::Variable(_) => Some(token.weight - 2),
            Token::Operator(oper) => match oper {
                Op::Add => Some(token.weight),
                Op::Sub => Some(token.weight),
                Op::Mult => Some(token.weight - 1),
                Op::Div => Some(token.weight - 1),
            },
            _ => None,
        }
    }

    fn find_root_index(tokens: &[WeightedToken]) -> usize {
        let mut root_index = 0;
        let mut weight = i32::MIN;
        for (i, t) in tokens.iter().enumerate() {
            if let Some(x) = Self::get_token_weight(t) {
                if x >= weight {
                    weight = x;
                    root_index = i;
                }
            }
        }
        root_index
    }

    pub fn build_tree(tokens: &[Token]) -> Result<Box<Self>, String> {
        let tokens = Self::handle_unary_minus(tokens)?;
        Self::build_tree_weighted(&Self::process_parens(&tokens)?)
    }

    fn build_tree_weighted(tokens: &[WeightedToken]) -> Result<Box<Self>, String> {
        let mut end = tokens.len();
        if end == 0 {
            Err("No tokens to parse")?
        }
        if let Some(e) = tokens.iter().position(|x| x.token == Token::EOL) {
            end = e;
        }

        let mut tree: Vec<Token> = vec![];
        let mut stack: Vec<(usize, usize)> = vec![];
        stack.push((0, end));

        while !stack.is_empty() {
            let (s, e) = stack.pop().unwrap();
            let root_index = s + Self::find_root_index(&tokens[s..e]);
            let t = tokens[root_index].token.clone();
            tree.push(t);

            if root_index > s && root_index < e - 1 {
                stack.push(((root_index + 1), e));
            }
            if root_index > s {
                stack.push((s, root_index));
            }
        }

        let mut resolved: Vec<Box<Self>> = vec![];
        while !tree.is_empty() {
            let t = tree.pop().expect("should contain valid token");
            match t {
                Token::Operator(_) => {
                    let left = resolved.pop();
                    let right = resolved.pop();
                    if left.is_none() || right.is_none() {
                        return Err("Invalid Expression".to_string());
                    }
                    resolved.push(Self::new(t, left, right));
                }
                _ => {
                    resolved.push(Self::new_leaf(t));
                }
            }
        }

        if resolved.len() > 1 {
            Err("Invalid Expression".to_string())
        } else {
            resolved.pop().ok_or("Invalid Expression".to_string())
        }
    }

    fn process_parens(tokens: &[Token]) -> Result<Vec<WeightedToken>, String> {
        let mut level = 0;
        let mut weighted_tokens: Vec<WeightedToken> = vec![];
        let mut paren_stack: Vec<i32> = vec![];
        let mut previous_token: Option<Token> = None;
        for token in tokens.iter() {
            const WEIGHT_OFFSET: i32 = 3;
            let weighted_token: Option<WeightedToken> = match token {
                Token::Paren(ParenType::OpenParen) => {
                    level -= WEIGHT_OFFSET;
                    paren_stack.push(level);
                    None
                }
                Token::Paren(ParenType::CloseParen) => {
                    if let Some(popped) = paren_stack.pop() {
                        if popped != level {
                            return Err("Extra )".to_string());
                        }
                    } else {
                        return Err("Extra )".to_string());
                    }

                    level += WEIGHT_OFFSET;
                    None
                }
                _ => Some(WeightedToken {
                    token: token.clone(),
                    weight: level,
                }),
            };

            let (implied_multiply, implied_level) = match (previous_token, token) {
                (Some(Token::Number(_)), Token::Paren(ParenType::OpenParen)) => {
                    (true, level + WEIGHT_OFFSET)
                }
                (Some(Token::Variable(_)), Token::Paren(ParenType::OpenParen)) => {
                    (true, level + WEIGHT_OFFSET)
                }
                (Some(Token::Paren(ParenType::CloseParen)), Token::Paren(ParenType::OpenParen)) => {
                    (true, level + WEIGHT_OFFSET)
                }
                (Some(Token::Paren(ParenType::CloseParen)), Token::Number(_)) => (true, level),
                (Some(Token::Paren(ParenType::CloseParen)), Token::Variable(_)) => (true, level),
                _ => (false, 0),
            };

            if implied_multiply {
                weighted_tokens.push(WeightedToken {
                    token: Token::Operator(Op::Mult),
                    weight: implied_level,
                });
            }

            if let Some(t) = weighted_token {
                weighted_tokens.push(t.clone());
            }

            previous_token = Some(token.clone());
        }

        match level.cmp(&0) {
            Ordering::Greater => Err("Extra )".to_string()),
            Ordering::Less => Err("Missing )".to_string()),
            Ordering::Equal => Ok(weighted_tokens),
        }
    }

    fn handle_unary_minus(tokens: &[Token]) -> Result<Vec<Token>, String> {
        let mut tokens_vec: Vec<Token> = tokens.to_vec();
        if tokens.len() == 1 {
            return Ok(tokens_vec);
        } else if tokens.len() == 2 {
            tokens_vec.insert(0, Token::InvalidToken("padding".to_string()));
        }

        let mut new_tokens: Vec<Token> = vec![];

        let mut skip_next = false;
        for (i, token_window) in tokens_vec.windows(3).enumerate() {
            if skip_next {
                skip_next = false;
                continue;
            }

            match (
                i,
                token_window[0].clone(),
                token_window[1].clone(),
                token_window[2].clone(),
            ) {
                (0, Token::Operator(Op::Sub), Token::Paren(ParenType::OpenParen), _) => {
                    new_tokens.push(Token::new_number("-1"));
                    new_tokens.push(token_window[1].clone())
                }
                (0, Token::Operator(Op::Sub), Token::Number(Num::Float(n)), _) => {
                    new_tokens.push(Token::new_number(&format!("-{}", n)));
                }
                (0, Token::Operator(Op::Sub), Token::Number(Num::Integer(n)), _) => {
                    new_tokens.push(Token::new_number(&format!("-{}", n)));
                }
                (0, Token::Operator(Op::Sub), v @ Token::Variable(_), _) => {
                    new_tokens.push(Token::new_paren("("));
                    new_tokens.push(Token::new_number("-1"));
                    new_tokens.push(Token::new_op("*"));
                    new_tokens.push(v);
                    new_tokens.push(Token::new_paren(")"));
                }
                (
                    0,
                    Token::InvalidToken(_),
                    Token::Operator(Op::Sub),
                    Token::Paren(ParenType::OpenParen),
                ) => {
                    new_tokens.push(Token::new_number("-1"));
                }
                (
                    0,
                    Token::InvalidToken(_),
                    Token::Operator(Op::Sub),
                    Token::Number(Num::Float(n)),
                ) => {
                    skip_next = true;
                    new_tokens.push(Token::new_number(&format!("-{}", n)));
                }
                (
                    0,
                    Token::InvalidToken(_),
                    Token::Operator(Op::Sub),
                    Token::Number(Num::Integer(n)),
                ) => {
                    skip_next = true;
                    new_tokens.push(Token::new_number(&format!("-{}", n)));
                }
                (0, Token::InvalidToken(_), Token::Operator(Op::Sub), v @ Token::Variable(_)) => {
                    skip_next = true;
                    new_tokens.push(Token::new_paren("("));
                    new_tokens.push(Token::new_number("-1"));
                    new_tokens.push(Token::new_op("*"));
                    new_tokens.push(v);
                    new_tokens.push(Token::new_paren(")"));
                }
                (0, Token::InvalidToken(_), _, _) => {
                    new_tokens.push(token_window[1].clone());
                }
                (0, _, _, _) => {
                    new_tokens.push(token_window[0].clone());
                    new_tokens.push(token_window[1].clone());
                }
                (
                    _,
                    Token::Operator(_) | Token::Paren(ParenType::OpenParen),
                    Token::Operator(Op::Sub),
                    Token::Number(Num::Float(n)),
                ) => {
                    skip_next = true;
                    new_tokens.push(Token::new_number(&format!("-{}", n)));
                }
                (
                    _,
                    Token::Operator(_) | Token::Paren(ParenType::OpenParen),
                    Token::Operator(Op::Sub),
                    Token::Number(Num::Integer(n)),
                ) => {
                    skip_next = true;
                    new_tokens.push(Token::new_number(&format!("-{}", n)));
                }
                (
                    _,
                    Token::Operator(_) | Token::Paren(ParenType::OpenParen),
                    Token::Operator(Op::Sub),
                    Token::Paren(ParenType::OpenParen),
                ) => {
                    new_tokens.push(Token::new_number("-1"));
                }
                (
                    _,
                    Token::Operator(_) | Token::Paren(ParenType::OpenParen),
                    Token::Operator(Op::Sub),
                    v @ Token::Variable(_),
                ) => {
                    skip_next = true;
                    new_tokens.push(Token::new_paren("("));
                    new_tokens.push(Token::new_number("-1"));
                    new_tokens.push(Token::new_op("*"));
                    new_tokens.push(v);
                    new_tokens.push(Token::new_paren(")"));
                }
                (_, _, t, _) => new_tokens.push(t),
            }
        }
        if !skip_next {
            if let Some(last) = tokens_vec.last() {
                new_tokens.push(last.clone());
            }
        }
        Ok(new_tokens)
    }
}

impl Display for AST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.left, &self.right) {
            (None, None) => write!(f, "'{}'", self.value),
            (None, Some(x)) => write!(f, "'{}' (None, {})", self.value, (**x)),
            (Some(x), None) => write!(f, "'{}' ({}, None)", self.value, (**x)),
            (Some(x), Some(y)) => write!(f, "'{}' ({}, {})", self.value, (**x), (**y)),
        }
    }
}
