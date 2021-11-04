use super::tokens::*;
use crate::tokenizer::tokens::TokenType::*;
use std::iter::{FromIterator};

/**
*
   Each value in Rust has a variable that’s called its owner.
   There can only be one owner at a time.
   When the owner goes out of scope, the value will be dropped.

*/
pub struct Tokenizer {
    expr: String,
}

impl Tokenizer {
    pub fn new(expr: &String) -> Self {
        Self {
            expr: expr.to_owned(),
        }
    }

    fn get_split(&self) -> Vec<(usize, char)> {
        Vec::from_iter(self.expr.char_indices().map(|(usize, ch)| (usize, ch)))
    }

    pub fn run(&self) {
        let mut chars = self.get_split().into_iter().peekable();
        let mut tokens: Vec<Token> = vec![];

        while let Some(_) = chars.peek() {
            let (loc, ch) = chars.next().unwrap();
            match ch {
                ' ' => (),
                '{' => tokens.push(Token::new(LeftCurly, Some(String::from(ch)), loc)),
                '}' => tokens.push(Token::new(RightCurly, Some(String::from(ch)), loc)),
                '[' => tokens.push(Token::new(LeftBracket, Some(String::from(ch)), loc)),
                ']' => tokens.push(Token::new(RightBracket, Some(String::from(ch)), loc)),
                '(' => tokens.push(Token::new(LeftParen, Some(String::from(ch)), loc)),
                ')' => tokens.push(Token::new(RightParen, Some(String::from(ch)), loc)),

                '+' => tokens.push(Token::new(Plus, Some(String::from(ch)), loc)),
                '-' => tokens.push(Token::new(Minus, Some(String::from(ch)), loc)),

                '/' => tokens.push(Token::new(Divide, Some(String::from(ch)), loc)),
                'x' => tokens.push(Token::new(Multiply, Some(String::from(ch)), loc)),
                '%' => tokens.push(Token::new(Modulus, Some(String::from(ch)), loc)),

                '_' => tokens.push(Token::new(Base, Some(String::from(ch)), loc)),

                '\'' => tokens.push(Token::new(Power, Some(String::from(ch)), loc)),

                '0'..='9' => {
                    let mut parseable = String::from(ch);

                    while let Some( (_,parse_char)) = chars.next() {
                      if matches!(parse_char, '0'..='9' | '.') {
                        parseable.push(parse_char)
                      } else {
                         panic!("Not a parseable number"); 
                      }
                    } 

                    if let Err(e) = parseable.parse::<f64>() {
                        
                    } else {}
                        
                    
                },
                _ => ()
            }
        }
    }
}
