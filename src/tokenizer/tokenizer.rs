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
    fn consume(&mut self) -> Option<char> {
        self.chars.next()
    }
    //check the next value, does not advance
    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }
    //consumes nums
    fn consume_num(&mut self, start: char) -> String {
        let mut parseable = String::from(start);
        while let Some(possible_ch) = self.peek() {
            if matches!(possible_ch, '.' | '0'..='9') {
                parseable.push(
                    self.chars
                        .next_if(|ch| matches!(ch, '0'..='9' | '.'))
                        .unwrap(),
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
        while let Some(_) = self.peek() {
            let ch = self.consume().unwrap();
            match ch {
                ' ' => (),
                '{' | '}' => self.add_token(Token::new(Curly(ch))),
                '(' | ')' => self.add_token(Token::new(Paren(ch))),
                '+' | '-' => self.add_token(Token::new(Term(ch))),
                '/' | 'x' | '%' => self.add_token(Token::new(Factor(ch))),
                '_' => self.add_token(Token::new(Base(ch))),
                '^' => self.add_token(Token::new(Power(ch))),
                '~' => self.add_token(Token::new(Squiggly(ch))),
                '0'..='9' | '.' => {
                    let parseable = self.consume_num(ch);
                    //if token is not parseable, push a poisoned token (erroring in parsing stage), else push a number
                    self.add_token(match parseable.parse::<f64>() {
                        Ok(val) => Token::new(Literal(val)),
                        Err(e) => Token::new(Poisoned(LexErr::NumParseErr(e, parseable))),
                    })
                }

                'a'..='z' | 'A'..='Z' => {
                    let parseable = self.consume_str(ch);
                    let token_type = Token::get_word(&parseable);
                    self.add_token(Token::new(token_type))
                }
                //anything not picked up by lexer will be poisoned
                _ => self.add_token(Token::new(Poisoned(LexErr::UnknownKeyword(ch.to_string())))),
            }
        }
    }
}
