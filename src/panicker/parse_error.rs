use crate::tokenizer::tokens::{Token, TokenType};
use std::fmt::Display;
#[derive(Debug, Clone, PartialEq)]
pub enum ParseErr {
    Expected(Token, Box<TokenType>),
    EOF,
}

impl Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErr::Expected(got, wanted) => {
                write!(f, "Expected : {:?} \n\nGot: {:?}\nUnexpected characters starting at {}, ending at {} ", *wanted, *got, &got.start, &got.end)
            }
            ParseErr::EOF => {
                write!(f, "Unexpected end of input!")
            }
        }
    }
}

impl std::error::Error for ParseErr {}
