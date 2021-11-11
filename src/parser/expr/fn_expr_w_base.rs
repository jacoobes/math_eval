use crate::tokenizer::tokens::Token;

use super::expr::Expr;

#[derive(Debug)]
pub struct FnWithBase {
    function: Token,
    base: Box<dyn Expr>,
    value: Box<dyn Expr>,
}
impl Expr for FnWithBase {}

impl FnWithBase {
    pub fn new(function: Token, base : Box<dyn Expr>, value: Box<dyn Expr>) -> Self {
        Self { function, base,  value }
    }
}
