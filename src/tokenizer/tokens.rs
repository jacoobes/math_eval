use maplit::hashmap;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    value: Option<String>,
    location: usize,
}

impl Token {
    pub fn new(token_type: TokenType, value: Option<String>, location: usize) -> Self {
        Self {
            token_type,
            value,
            location,
        }
    }

    pub fn literal () -> HashMap<String, String> {
        hashmap! {
            


        }
    }
}

#[derive(Debug)]
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
    Dot,
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
}
