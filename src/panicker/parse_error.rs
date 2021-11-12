use crate::tokenizer::tokens::{Token, TokenType};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseErr {
    Expected(Token, Token),
    EOF,
    Lex(Token)
}

impl Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErr::Expected(got, wanted) => {
                write!(f, "Expected : {:?} \n\nGot: {:?}\nUnexpected characters ", *wanted, *got)
            }
            ParseErr::EOF => {
                write!(f, "Unexpected end of input!")
            }
            ParseErr::Lex(tok) => {
                match &tok.token_type {
                    TokenType::Poisoned(er) => write!(f, "{}", er),
                    _ => write!(f, "Unexpected!")
                }
            },
        }
    }
}

impl std::error::Error for ParseErr {}
