use super::tokens::*;
use crate::panicker::error::CalcErr;
use crate::tokenizer::tokens::TokenType::*;
use std::iter::FromIterator;
/**
*
   Each value in Rust has a variable that’s called its owner.
   There can only be one owner at a time.
   When the owner goes out of scope, the value will be dropped.

*/
pub struct Tokenizer {
    expr: String,
    pub tokens: Vec<Token>
}

impl Tokenizer {
    pub fn new(expr: &String) -> Self {
        Self {
            expr: expr.to_owned(),
            tokens : vec![]
        }
    }

    fn get_split(&self) -> Vec<(usize, char)> {
        Vec::from_iter(self.expr.char_indices().map(|(usize, ch)| (usize, ch)))
    }

    pub fn run(&mut self) {
        let mut chars = self.get_split().into_iter().peekable();

        while let Some(_) = chars.peek() {
            let (loc, ch) = chars.next().unwrap();
            match ch {
                ' ' => (),
                '{' => self.tokens.push(Token::new(LeftCurly, Some(String::from(ch)), loc, loc)),
                '}' => self.tokens.push(Token::new(RightCurly, Some(String::from(ch)), loc, loc)),
                '[' => self.tokens.push(Token::new(LeftBracket, Some(String::from(ch)), loc, loc)),
                ']' => self.tokens.push(Token::new(RightBracket, Some(String::from(ch)), loc, loc)),
                '(' => self.tokens.push(Token::new(LeftParen, Some(String::from(ch)), loc, loc)),
                ')' => self.tokens.push(Token::new(RightParen, Some(String::from(ch)), loc, loc)),

                '+' => self.tokens.push(Token::new(Plus, Some(String::from(ch)), loc, loc)),
                '-' => self.tokens.push(Token::new(Minus, Some(String::from(ch)), loc, loc)),

                '/' => self.tokens.push(Token::new(Divide, Some(String::from(ch)), loc, loc)),
                'x' => self.tokens.push(Token::new(Multiply, Some(String::from(ch)), loc, loc)),
                '%' => self.tokens.push(Token::new(Modulus, Some(String::from(ch)), loc, loc)),

                '_' => self.tokens.push(Token::new(Base, Some(String::from(ch)), loc, loc)),

                '\'' => self.tokens.push(Token::new(Power, Some(String::from(ch)), loc, loc)),

                '0'..='9' | '.' => {
                    let mut parseable = String::from(ch);

                    while let Some((_, parse_char)) = chars.peek() {
                        if matches!(parse_char, '.' | '0'..='9') {
                            parseable.push(*parse_char);
                            chars.next_if(|(_, ch)| ch.is_numeric() || *ch == '.');
                        } else {
                            break;
                        }
                    }
                    self.tokens.push(if let Err(e) = parseable.parse::<f64>() {
                        Token::new(
                            Poisoned(CalcErr::NumParseErr(e, parseable.clone())),
                            None,
                            loc,
                            parseable.len(),
                        )
                    } else {
                        Token::new(Literal, Some(parseable.clone()), loc, parseable.len())
                    })
                }
                'a'..='z' | 'A'..='Z' => {}
                _ => (),
            }
        }
    }
}
