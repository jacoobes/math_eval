use crate::tokenizer::tokens::Token;

use super::expr::Expr;


#[derive(Debug)]
struct FnExpr <V: Expr> {
    function : Token,
    value: V
}
impl <V: Expr> Expr for FnExpr<V> {}

impl <V: Expr> FnExpr<V> {
    pub fn new(function: Token, value: V) -> Self {
        Self {
            function,
            value
        }
    }
}