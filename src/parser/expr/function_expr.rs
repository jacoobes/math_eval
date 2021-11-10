use crate::tokenizer::tokens::Token;

use super::expr::Expr;

#[derive(Debug)]
pub struct FnExpr {
    function: Token,
    value: Box<dyn Expr>,
}
impl Expr for FnExpr {}

impl FnExpr {
    pub fn new(function: Token, value: Box<dyn Expr>) -> Self {
        Self { function, value }
    }
}
