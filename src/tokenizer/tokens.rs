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

    pub fn reserved_keywords() -> HashMap<String, TokenType> {
        hashmap! {
            String::from("log") => TokenType::Log,
            String::from("sin") => TokenType::Sine,
            String::from("tan") => TokenType::Tangent,
            String::from("cos") => TokenType::Cosine,
            String::from("csc") => TokenType::Cosecant,
            String::from("sec") => TokenType::Secant,
            String::from("cot") => TokenType::Cotangent,
            String::from("arccsc") => TokenType::ArcCsc,
            String::from("arcsec") => TokenType::ArcSec,
            String::from("arccot") => TokenType::ArcCot,
            String::from("arcsin") => TokenType::ArcSine,
            String::from("arccos") => TokenType::ArcCosine,
            String::from("arctan") => TokenType::ArcTangent,
            String::from("PI") => TokenType::Pi,
            String::from("ans") => TokenType::Ans,
            String::from("ln") => TokenType::Ln,
            String::from("e") => TokenType::E,
            String::from("root") => TokenType::Root,
            String::from("rad") => TokenType::Rad,
            String::from("degree") => TokenType::Rad

        }
    }
    //if possible_reserved does not exist in reserved keywords, adds a poisoned token type
    pub fn get_word( possible_reserved: &String) -> TokenType {
        let keywords = Token::reserved_keywords();
        match keywords.get(possible_reserved) {
            Some(typ) => typ.to_owned(),
            None => TokenType::Poisoned(CalcErr::UnknownKeyword(possible_reserved.clone()))
        }
    }
}

#[derive(Debug, Clone)]
pub enum TokenType {
    Literal,
    LeftParen,
    RightParen,
    Pi,
    Sine,
    Cosecant,
    ArcCsc,
    Cosine,
    Secant,
    ArcSec,
    Tangent,
    Cotangent,
    ArcCot,
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
    Squiggly,
    Rad,
    Degree,
    EOF,
}
