use crate::tokenizer::tokens::Token;

use super::expr::Expr;

#[derive(Debug)]
struct Unary {
    sign: Token,
    value: Box<dyn Expr>,
}

impl Expr for Unary {}

impl Unary {
    pub fn new(sign : Token, value: Box<dyn Expr>) -> Self {
        Self { sign, value }
    }
}
