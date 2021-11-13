use peekmore::{PeekMore, PeekMoreIterator};

use super::tokens::*;
use crate::panicker::lex_error::LexErr;
use crate::tokenizer::tokens::TokenType::*;
use std::iter::{FromIterator, Peekable};
use std::vec::IntoIter;

pub struct Tokenizer {
    pub tokens: Vec<Token>,
    chars: Peekable<IntoIter<char>>,
}

impl Tokenizer {
    pub fn new(expr: &String) -> Self {
        Self {
            tokens: vec![],
            chars: Vec::from_iter(expr.chars()).into_iter().peekable(),
        }
    }
    //advance the iterator
    fn consume(&mut self) -> char {
        self.chars.next().unwrap()
    }
    //check the next value, does not advance
    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }
    //consumes nums
    fn consume_num(&mut self) -> String {
        let mut parseable = String::from(self.consume());
        while let Some(possible_ch) = self.peek() {
            if matches!(possible_ch, '.' | '0'..='9') {
                parseable.push(
                    self.chars
                        .next_if(|ch| matches!(ch, '0'..='9' | '.'))
                        .unwrap(),
                );
            } else {
                break
            }
        }
        parseable
    }
    //consumes keywords
    fn consume_str(&mut self) -> String {
        let mut parseable = String::from(self.consume());
        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphabetic() {
                parseable.push(self.chars.next_if(|ch| ch.is_ascii_alphabetic()).unwrap());
            } else {
                break;
            }
        }
        parseable
    }

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token)
    }

    pub fn run(&mut self) {
        while let Some(&cha) = self.peek() {
            match cha {
                ' ' => (),
                '{' | '}' => self.add_token(Token::new(Curly(cha))),
                '(' | ')' => self.add_token(Token::new(Paren(cha))),
                '+' | '-' => self.add_token(Token::new(Term(cha))),
                '/' | 'x' | '%' => self.add_token(Token::new(Factor(cha))),
                '_' => self.add_token(Token::new(Base)),
                '^' => self.add_token(Token::new(Power(cha))),
                '~' => self.add_token(Token::new(Squiggly(cha))),
                '0'..='9' | '.' => {
                    let parseable = self.consume_num();
                    //if token is not parseable, push a poisoned token (erroring in parsing stage), else push a number
                    self.add_token(match parseable.parse::<f64>() {
                        Ok(val) => Token::new(Literal(val)),
                        Err(e) => Token::new(Poisoned(LexErr::NumParseErr(e, parseable))),
                    })
                }

                'a'..='z' | 'A'..='Z' => {
                    let parseable = self.consume_str();
                    let token_type = Token::get_word(&parseable);
                    self.add_token(Token::new(token_type))
                }
                //anything not picked up by lexer will be poisoned
                _ => self.add_token(Token::new(Poisoned(LexErr::UnknownKeyword(cha.to_string())))),
            }
            if let None = self.peek() {break} else {self.consume()}; 
        }
    }
}
