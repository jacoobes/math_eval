use crate::tokenizer::tokens::{Token, TokenType};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseErr {
    Expected((TokenType, TokenType)),
    EOF(&'static str),
    UnknownKeyword(TokenType, &'static str),
}

impl Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErr::Expected((expected, got)) => {
                write!(f, "Expected : {:?} \n\nGot: {:?}\n ", expected, got)
            }
            ParseErr::EOF(extra_msg) => {
                write!(f, "Unexpected end of input! {}", extra_msg)
            }
            ParseErr::UnknownKeyword(tok, extra_msg) => {
                write!(f, "Unexpected token_type {:?}. {}", &tok, extra_msg)
            }
        }
    }
}

impl std::error::Error for ParseErr {}
