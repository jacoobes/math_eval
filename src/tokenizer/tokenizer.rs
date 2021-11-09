use super::tokens::*;
use crate::panicker::lex_error::LexErr;
use crate::tokenizer::tokens::TokenType::*;
use std::iter::{FromIterator, Peekable};
use std::vec::IntoIter;

pub struct Tokenizer {
    pub tokens: Vec<Token>,
    chars: Peekable<IntoIter<(usize, char)>>,
}

impl Tokenizer {
    pub fn new(expr: &String) -> Self {
        Self {
            tokens: vec![],
            chars: Vec::from_iter(expr.char_indices()).into_iter().peekable(),
        }
    }
    //advance the iterator
    fn consume(&mut self) -> Option<(usize, char)> {
        self.chars.next()
    }
    //check the next value, does not advance
    fn peek(&mut self) -> Option<&(usize, char)> {
        self.chars.peek()
    }
    //consumes nums
    fn consume_num(&mut self, start: char) -> String {
        let mut parseable = String::from(start);
        while let Some((_, possible_ch)) = self.peek() {
            if matches!(possible_ch, '.' | '0'..='9') {
                parseable.push(
                    self.chars.next_if(|(_, ch)| matches!(ch, '0'..='9' | '.')).unwrap().1,
                );
            } else {
                break;
            }
        }
        parseable
    }
    //consumes keywords
    fn consume_str(&mut self, start: char) -> String {
        let mut parseable = String::from(start);
        while let Some((_, ch)) = self.peek() {
            if ch.is_ascii_alphabetic() {
                parseable.push(self.chars.next_if(|(_, ch)| ch.is_ascii_alphabetic()).unwrap().1,
                );
            } else {
                break;
            }
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
                '(' => self.tokens.push(Token::new(LeftParen, Some(String::from(ch)), loc, loc)),
                ')' => self.tokens.push(Token::new(RightParen, Some(String::from(ch)), loc, loc)),
                '+' => self.tokens.push(Token::new(Plus, Some(String::from(ch)), loc, loc)),
                '-' => self.tokens.push(Token::new(Minus, Some(String::from(ch)), loc, loc)),
                '/' => self.tokens.push(Token::new(Divide, Some(String::from(ch)), loc, loc)),
                'x' => self.tokens.push(Token::new(Multiply, Some(String::from(ch)), loc, loc)),
                '%' => self.tokens.push(Token::new(Modulus, Some(String::from(ch)), loc, loc)),
                '_' => self.tokens.push(Token::new(Base, Some(String::from(ch)), loc, loc)),
                '^' => self.tokens.push(Token::new(Power, Some(String::from(ch)), loc, loc)),
                '~' => self.tokens.push(Token::new(Squiggly, Some(String::from(ch)), loc, loc)),
                '0'..='9' | '.' => {
                    let parseable = self.consume_num(ch);
                    let len = parseable.len();

                    //if token is not parseable, push a poisoned token (erroring in parsing stage), else push a number
                    self.tokens.push(if let Err(e) = parseable.parse::<f64>() {
                        Token::new(Poisoned(LexErr::NumParseErr(e, parseable)), None, loc, len)
                    } else {
                        Token::new(Literal, Some(parseable), loc, len)
                    })
                }

                'a'..='z' | 'A'..='Z' => {
                    let parseable = self.consume_str(ch);
                    let len = parseable.len();
                    let token_type = Token::get_word(&parseable);
                    self.tokens.push(Token::new(token_type, Some(parseable), loc, len))
                }
                //anything not picked up by lexer will be poisoned
                _ => self.tokens.push(Token::new(
                    Poisoned(LexErr::UnknownKeyword(ch.to_string())),
                    Some(ch.to_string()),
                    loc,
                    loc,
                )),
            }
        }
        self.tokens.push(Token::new(EOF, None, 0, self.chars.len()))
    }
}
