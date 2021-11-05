use super::tokens::*;
use crate::panicker::error::CalcErr;
use crate::tokenizer::tokens::TokenType::*;
use std::iter::FromIterator;
use std::iter::Peekable;
use std::vec::IntoIter;
/**
*
   Each value in Rust has a variable that’s called its owner.
   There can only be one owner at a time.
   When the owner goes out of scope, the value will be dropped.

*/
pub struct Tokenizer {
    expr: String,
    pub tokens: Vec<Token>,
    chars : Peekable<IntoIter<(usize, char)>>
}

impl Tokenizer {
    pub fn new(expr: &String) -> Self {
        Self {
            expr: expr.to_owned(),
            tokens : vec![],
            chars: Vec::from_iter(expr.char_indices()).into_iter().peekable()
        }
    }

    fn consume(&mut self) -> Option<(usize, char)> {
        self.chars.next()
    }

    fn peek(&mut self) -> Option<&(usize, char)> {
        self.chars.peek()
    }

    fn consume_num(&mut self, start: char) -> String {
        let mut parseable = String::from(start);

        while let Some((_, possible_ch)) = self.peek() {
            if matches!(possible_ch, '.' | '0'..='9') {
                parseable.push(
                    self.chars.next_if(|(_, ch)| matches!(ch, '0'..='9' | '.')).unwrap().1
                );
            } else {  break;  }
        }
        parseable
    }

    fn consume_str(&mut self, start: char) -> String {
        let mut parseable = String::from(start);
        while let Some((_, ch)) = self.peek()  { 
            if ch.is_ascii_alphabetic() {
                parseable.push(
                    self.chars.next_if(|(_, ch)| ch.is_ascii_alphabetic()).unwrap().1
                );
            } else { break; }
        } 
        parseable
    }

    pub fn run(&mut self) {

        while let Some(_) = self.peek() {
            let (loc, ch) = self.consume().unwrap();
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
                    let parseable = self.consume_num(ch);
                    let len = parseable.len();
                    self.tokens.push(if let Err(e) = parseable.parse::<f64>() {
                        Token::new(
                            Poisoned(CalcErr::NumParseErr(e, parseable)),
                            None,
                            loc,
                            len,
                        )
                    } else {
                        Token::new(Literal, Some(parseable), loc, len)
                    })
                }

                'a'..='z' | 'A'..='Z' => {
                    let parseable = self.consume_str(ch);
                    let len = parseable.len();
                    let token_type = Token::get_word(&parseable);
                    self.tokens.push(Token::new(token_type,Some(parseable), loc, len))
                }
                _ => self.tokens.push(Token::new(Poisoned(CalcErr::UnknownKeyword(ch.to_string())),Some(ch.to_string()), loc, loc))
            }
        }
    }
}
