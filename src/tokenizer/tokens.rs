use crate::panicker::lex_error::LexErr;
use maplit::hashmap;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
}

impl Token {
    pub fn new(token_type: TokenType) -> Self {
        Self {
            token_type,
        }
    }

    pub fn reserved_keywords() -> HashMap<&'static str, TokenType> {
        hashmap! {
            "log" => TokenType::Log,
            "sin" => TokenType::Sine,
            "tan" => TokenType::Tangent,
            "cos" => TokenType::Cosine,
            "csc" => TokenType::Cosecant,
            "sec" => TokenType::Secant,
            "cot" => TokenType::Cotangent,
            "arccsc" => TokenType::ArcCsc,
            "arcsec" => TokenType::ArcSec,
            "arccot" => TokenType::ArcCot,
            "arcsin" => TokenType::ArcSine,
            "arccos" => TokenType::ArcCosine,
            "arctan" => TokenType::ArcTangent,
            "PI" => TokenType::Pi(std::f64::consts::PI),
            "ans" => TokenType::Ans(None),
            "ln" => TokenType::Ln,
            "e" => TokenType::E(std::f64::consts::E),
            "root" => TokenType::Root,
            "rad" => TokenType::Rad,
            "degree" => TokenType::Degree

        }
    }
    //if possible_reserved does not exist in reserved keywords, adds a poisoned token type
    pub fn get_word( possible_reserved: &str) -> TokenType {
        let keywords = Token::reserved_keywords();
        match keywords.get(possible_reserved) {
            Some(typ) => typ.to_owned(),
            None => TokenType::Poisoned(LexErr::UnknownKeyword(possible_reserved.to_string()))
        }
    }
}

#[derive(Debug, Clone, PartialEq,)]
pub enum TokenType {
    Term(char),
    Factor(char),
    FnBase(String),
    Function(String),
    Literal(f64),
    Paren(char),
    Pi(f64),
    Sine,
    Cosecant,
    ArcCsc,
    Cosine,
    Secant,
    ArcSec,
    Tangent,
    Cotangent,
    ArcCot,
    Power(char),
    ArcSine,
    ArcCosine,
    ArcTangent,
    Log,
    Base(char),
    Ans(Option<f64>),
    Ln,
    E(f64),
    Root,
    Curly(char),
    Poisoned(LexErr),
    Squiggly(char),
    Rad,
    Degree,
}
