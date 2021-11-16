use std::fmt::write;

use crate::{parser::expr::expr::Expr, tokenizer::tokens::TokenType};

pub struct BinaryExpr {
    left: Box<dyn Expr>,
    operator: TokenType,
    right: Box<dyn Expr>,
}

impl Expr for BinaryExpr {}

impl BinaryExpr {
    pub fn new(left: Box<dyn Expr>, operator: TokenType, right: Box<dyn Expr>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
    fn token_type(&self) -> char {
        match &self.operator {
            &TokenType::Term(c) | &TokenType::Factor(c) => c,
            &TokenType::Power => '^',
            _ => ' ' 
        }
    }
}

impl std::fmt::Debug for BinaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}" , format!("\nbinary< {:?} {} {:?} >\n", &self.left, self.token_type(), &self.right))
    }
}