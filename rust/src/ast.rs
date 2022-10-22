#[cfg(test)]
mod tests;

use crate::tokens::{Num, Op, Token};
use std::boxed::Box;

#[derive(Debug)]
struct AST {
    left: Option<Box<AST>>,
    right: Option<Box<AST>>,
    value: Token,
}

impl AST {
    fn new(value: Token) -> Box<Self> {
        let ast = AST {
            left: None,
            right: None,
            value,
        };
        Box::new(ast)
    }

    fn set_left(&mut self, ast: Option<Box<AST>>) {
        self.left = ast;
    }

    fn set_right(&mut self, ast: Option<Box<AST>>) {
        self.right = ast;
    }

    fn evaluate(&self) -> Result<f64, String> {
        let result = match &self.value {
            Token::Number(num) => match num {
                Num::Float(x) => *x,
                Num::Integer(x) => *x as f64,
            },
            Token::Operator(oper) => self.evaluate_op(&oper)?,
            x => Err(format!("Cannot evaluate {:?}", x))?,
        };
        Ok(result)
    }

    fn evaluate_op(&self, oper: &Op) -> Result<f64, String> {
        let l = self
            .left
            .as_deref()
            .ok_or("no left".to_owned())?
            .evaluate()?;
        let r = self
            .right
            .as_deref()
            .ok_or("no right".to_owned())?
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
}
