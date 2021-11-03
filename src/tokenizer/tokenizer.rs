use std::str::CharIndices;
use super::tokens::*;
use crate::tokenizer::tokens::TokenType::*;
pub struct Tokenizer  {
     expr : String,
}

impl Tokenizer {

    pub fn new (expr: String) -> Self {
        Self { 
            expr :  expr,
        }
    }
    fn get_iterator(&self) -> CharIndices {
        self.expr.char_indices()
    }

    pub fn run(&self) -> Vec<Token> {
        let chars= self.get_iterator();
      chars.map(|  (i, ch) | {
    
            match ch {
                '{' => Token::new(LeftCurly, Some(String::from(ch)), i),
                '}' => Token::new(RightCurly, Some(String::from(ch)), i),
                '[' => Token::new(LeftBracket, Some(String::from(ch)), i),
                ']' => Token::new(RightBracket, Some(String::from(ch)), i),
                '(' =>Token::new(LeftParen, Some(String::from(ch)), i),
                ')' => Token::new(RightParen, Some(String::from(ch)), i),
    
                '+' =>Token::new(Plus, Some(String::from(ch)), i),
                '-' => Token::new(Minus, Some(String::from(ch)), i),
    
                '/' => Token::new(Divide, Some(String::from(ch)), i),
                'x' =>Token::new(Multiply, Some(String::from(ch)), i),
                '%' => Token::new(Modulus, Some(String::from(ch)), i),
    
                '_' => Token::new(Base, Some(String::from(ch)), i),
    
                '\'' => Token::new(Power, Some(String::from(ch)), i),

                _ => {
                    if matches!(ch, '0'..='9' | '.') {

                    }
                    Token::new(Pi, Some(String::from(ch)), i)
                }


            }
            


        } ).collect::<Vec<Token>>()
    }
}