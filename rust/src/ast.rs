#[cfg(test)]
mod tests;

use crate::tokens::{Num, Op, ParenType, Token};
use std::{boxed::Box, collections::HashMap, fmt::Display};

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

impl AST {
    fn new(value: Token, left: Option<Box<AST>>, right: Option<Box<AST>>) -> Box<Self> {
        let ast = AST { left, right, value };
        Box::new(ast)
    }

    pub fn new_leaf(value: Token) -> Box<Self> {
        Self::new(value, None, None)
    }

    pub fn evaluate(&self, env: &HashMap<String, Box<AST>>) -> Result<f64, String> {
        let result = match &self.value {
            Token::Number(num) => self.evaluate_number(num)?,
            Token::Operator(oper) => self.evaluate_operator(oper, env)?,
            Token::Paren(_) => Err("Parenthetical expressions are not yet implemented")?,
            Token::Variable(var) => self.evaluate_variable(var, env)?,
            x => Err(format!("Cannot evaluate {}", x))?,
        };
        Ok(result)
    }

    fn evaluate_operator(&self, oper: &Op, env: &HashMap<String, Box<AST>>) -> Result<f64, String> {
        let l = self
            .left
            .as_deref()
            .ok_or_else(|| "Invalid Expression".to_owned())?
            .evaluate(env)?;
        let r = self
            .right
            .as_deref()
            .ok_or_else(|| "Invalid Expression".to_owned())?
            .evaluate(env)?;
        let result = match oper {
            Op::Add => l + r,
            Op::Sub => l - r,
            Op::Mult => l * r,
            Op::Div => {
                if r == 0.0 {
                    Err("Divide by Zero".to_owned())?
                }
                l / r
            }
        };
        Ok(result)
    }

    fn evaluate_number(&self, num: &Num) -> Result<f64, String> {
        if self.left.is_some() || self.right.is_some() {
            return Err("Invalid Expression".to_string());
        }

        let result = match num {
            Num::Float(x) => *x,
            Num::Integer(x) => *x as f64,
        };
        Ok(result)
    }

    fn evaluate_variable(
        &self,
        var: &String,
        env: &HashMap<String, Box<AST>>,
    ) -> Result<f64, String> {
        if self.left.is_some() || self.right.is_some() {
            return Err("Invalid Expression".to_string());
        }

        if let Some(value) = env.get(var) {
            value.evaluate(env)
        } else {
            Err(format!("Unknown Variable: {}", var))
        }
    }

    fn get_token_weight(token: &WeightedToken) -> Option<i32> {
        match &token.token {
            Token::Paren(_) => Some(token.weight),
            Token::Number(_) => Some(token.weight),
            Token::Variable(_) => Some(token.weight),
            Token::Operator(oper) => match oper {
                Op::Add => Some(token.weight + 2),
                Op::Sub => Some(token.weight + 2),
                Op::Mult => Some(token.weight + 1),
                Op::Div => Some(token.weight + 1),
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
        let weighted_tokens: &[WeightedToken] = &Self::adjust_weights(tokens).unwrap();
        Self::build_tree_weighted(weighted_tokens)
    }

    fn build_tree_weighted(tokens: &[WeightedToken]) -> Result<Box<Self>, String> {
        let mut end = tokens.len();
        if end == 0 {
            Err("No tokens to parse")?
        }
        if let Some(e) = tokens.iter().position(|x| x.token == Token::EOL) {
            end = e;
        }

        let root_index = Self::find_root_index(tokens);
        let mut left: Option<Box<Self>> = None;
        let mut right: Option<Box<Self>> = None;

        if root_index > 0 {
            left = Some(
                Self::build_tree_weighted(&tokens[..root_index])
                    .map_err(|_| "Invalid Expression".to_string())?,
            )
        }
        if root_index > 0 && root_index < tokens.len() - 1 {
            right = Some(
                Self::build_tree_weighted(&tokens[(root_index + 1)..end])
                    .map_err(|_| "Invalid Expression".to_string())?,
            )
        }

        Ok(Self::new(tokens[root_index].token.clone(), left, right))
    }

    fn adjust_weights(tokens: &[Token]) -> Result<Vec<WeightedToken>, String> {
        let mut level = 0;
        let mut weighted_tokens: Vec<WeightedToken> = vec![];
        for token in tokens.iter() {
            let weighted_token: Option<WeightedToken> = match token {
                Token::Paren(ParenType::OpenParen) => {
                    level -= 10;
                    None
                }
                Token::Paren(ParenType::CloseParen) => {
                    level += 10;
                    None
                }
                _ => Some(WeightedToken {
                    token: token.clone(),
                    weight: level,
                }),
            };
            if let Some(t) = weighted_token {
                weighted_tokens.push(t.clone());
            }
        }

        if level != 0 {
            Err("Mismatched parenthases".to_string())
        } else {
            Ok(weighted_tokens)
        }
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
