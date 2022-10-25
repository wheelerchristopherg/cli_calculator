#[cfg(test)]
mod tests;

use crate::tokens::{Num, Op, Token};
use std::{boxed::Box, fmt::Display};

#[derive(Debug)]
pub struct AST {
    value: Token,
    left: Option<Box<AST>>,
    right: Option<Box<AST>>,
}

impl AST {
    fn new(value: Token, left: Option<Box<AST>>, right: Option<Box<AST>>) -> Box<Self> {
        let ast = AST { left, right, value };
        Box::new(ast)
    }

    pub fn evaluate(&self) -> Result<f64, String> {
        let result = match &self.value {
            Token::Number(num) => self.evaluate_number(&num)?,
            Token::Operator(oper) => self.evaluate_operator(&oper)?,
            x => Err(format!("Cannot evaluate {}", x))?,
        };
        Ok(result)
    }

    fn evaluate_operator(&self, oper: &Op) -> Result<f64, String> {
        let l = self
            .left
            .as_deref()
            .ok_or("No Left Node".to_owned())?
            .evaluate()?;
        let r = self
            .right
            .as_deref()
            .ok_or("No Right Node".to_owned())?
            .evaluate()?;
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
        match (&self.left, &self.right) {
            (None, None) => (),
            _ => Err(format!("Invalid Node: Number must be leaf"))?,
        };

        let result = match num {
            Num::Float(x) => *x,
            Num::Integer(x) => *x as f64,
        };
        Ok(result)
    }

    fn get_token_weight(token: &Token) -> Option<i32> {
        match token {
            Token::Paren(_) => Some(0),
            Token::Number(_) => Some(0),
            Token::Variable(_) => Some(0),
            Token::Operator(oper) => match oper {
                Op::Add => Some(2),
                Op::Sub => Some(2),
                Op::Mult => Some(1),
                Op::Div => Some(1),
            },
            _ => None,
        }
    }

    fn find_root_index(tokens: &[Token]) -> usize {
        let mut root_index = 0;
        let mut weight = -1;
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
        let mut end = tokens.len();
        if end == 0 {
            Err("No tokens to parse")?
        }
        if let Some(e) = tokens.iter().position(|x| x == &Token::EOL) {
            end = e;
        }

        let root_index = Self::find_root_index(tokens);
        let mut left: Option<Box<Self>> = None;
        let mut right: Option<Box<Self>> = None;

        if root_index > 0 {
            left = Some(Self::build_tree(&tokens[..root_index])?)
        }
        if root_index < tokens.len() - 1 {
            right = Some(Self::build_tree(&tokens[(root_index + 1)..end])?)
        }

        Ok(Self::new(tokens[root_index].clone(), left, right))
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
