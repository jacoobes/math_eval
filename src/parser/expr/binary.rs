use crate::{parser::expr::expr::Expr, tokenizer::tokens::Token};
#[derive(Debug)]
struct  BinaryExpr <L: Expr, R: Expr> {
    left: L,
    operator: Token,
    right: R
}

impl <L: Expr, R: Expr> Expr for BinaryExpr <L, R> {}

impl <L: Expr, R: Expr> BinaryExpr <L, R> {
    pub fn new( left: L, operator: Token, right: R) -> Self {
       Self {
           left,
           operator,
           right
       }
    }
}