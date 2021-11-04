use crate::panicker::error::CalcErr;
use maplit::hashmap;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    value: Option<String>,
    start: usize,
    end: usize,
}

impl Token {
    pub fn new(token_type: TokenType, value: Option<String>, start: usize, end: usize) -> Self {
        Self {
            token_type,
            value,
            start,
            end,
        }
    }

    pub fn reserved_keywords(&self) -> HashMap<String, TokenType> {
        hashmap! {
            String::from("log") => TokenType::Log,
            String::from("sin") => TokenType::Sine,
            String::from("tan") => TokenType::Tangent,
            String::from("cos") => TokenType::Cosine,
            String::from("arcsin") => TokenType::Cosine,
            String::from("arccos") => TokenType::ArcCosine,
            String::from("arctan") => TokenType::ArcTangent,
            String::from("PI") => TokenType::Pi,
            String::from("ans") => TokenType::Ans,
            String::from("ln") => TokenType::Ln,
            String::from("e") => TokenType::E
        }
    }
    pub fn get_word() -> TokenType {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum TokenType {
    Literal,
    LeftParen,
    RightParen,
    Pi,
    Sine,
    Cosine,
    Tangent,
    Power,
    Minus,
    Plus,
    Divide,
    Multiply,
    Modulus,
    ArcSine,
    ArcCosine,
    ArcTangent,
    LeftBracket,
    RightBracket,
    Log,
    Base,
    Ans,
    Ln,
    E,
    Root,
    LeftCurly,
    RightCurly,
    Poisoned(CalcErr),
    EOF,
}
