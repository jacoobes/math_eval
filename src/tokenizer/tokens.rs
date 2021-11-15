use crate::panicker::lex_error::LexErr;
use maplit::hashmap;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value : Option<f64>
}

impl Token {
    pub fn new(token_type: TokenType, value : Option<f64>) -> Self {
        Self { token_type, value }
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
            "pi" => TokenType::Pi,
            "ans" => TokenType::Ans,
            "ln" => TokenType::Ln,
            "e" => TokenType::E,
            "root" => TokenType::Root,
            "rad" => TokenType::Rad,
            "degree" => TokenType::Degree

        }
    }
    //if possible_reserved does not exist in reserved keywords, adds a poisoned token type
    pub fn get_word(possible_reserved: &str) -> TokenType {
        let keywords = Token::reserved_keywords();
        match keywords.get(possible_reserved) {
            Some(typ) => typ.to_owned(),
            None => TokenType::Poisoned(LexErr::UnknownKeyword(possible_reserved.to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Term(char),
    Factor(char),
    Function(String),
    Literal,
    Paren(char),
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
    ArcSine,
    ArcCosine,
    ArcTangent,
    Log,
    Base,
    Ans,
    Ln,
    E,
    Root,
    Curly(char),
    Poisoned(LexErr),
    Squiggly(char),
    Rad,
    Degree,
    
}
