use crate::{parser::expr::expr::Expr, tokenizer::tokens::Token};
#[derive(Debug)]
pub struct BinaryExpr {
    left: Box<dyn Expr>,
    operator: Token,
    right: Box<dyn Expr>,
}

impl Expr for BinaryExpr {}

impl BinaryExpr {
    pub fn new(left: Box<dyn Expr>, operator: Token, right: Box<dyn Expr>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}
