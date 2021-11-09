use crate::tokenizer::tokens::TokenType;
use std::fmt::Display;
#[derive(Debug, Clone, PartialEq)]
pub enum ParseErr {
    Expected( Box<TokenType>,  Box<TokenType>)
}

impl Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErr::Expected(wanted, got) => {
                write!(f, "Expected : {:?} \n\n Got: {:?}", *wanted, *got)
            }

        }
    }
}

impl crate::parser::expr::expr::Expr for ParseErr {}
impl std::error::Error for ParseErr {}