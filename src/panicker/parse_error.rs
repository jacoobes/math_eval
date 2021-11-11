use crate::tokenizer::tokens::{Token, TokenType};
use std::fmt::Display;

use super::lex_error::LexErr;
#[derive(Debug, Clone, PartialEq)]
pub enum ParseErr {
    Expected(Token, Box<TokenType>),
    EOF,
    Lex(Token)
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
