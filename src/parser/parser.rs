use crate::{ tokenizer::tokens::{Token, TokenType::{*, self}}};
use crate::parser::expr::parse_error::ParseErr;
use super::expr::expr::*;
use std::{iter::{ Peekable}, num};
use std::vec::IntoIter;
use crate::parser::expr::number::Number;
pub struct Parser
 {
    tokens : Vec<Token>,
}


impl Parser {
    fn expr(&mut self) -> Box<dyn Expr> { 
       match self.peek() { 
           Some(_)  => self.fn_expr(),                 
           None => self.literal()
           
       }
    }

    fn fn_expr(&self) -> Box<dyn Expr> {
        todo!()
    }

    fn term(&self) -> Box<dyn Expr> {
        todo!()
    }
    
    fn factor(&self) -> Box<dyn Expr> {
        todo!()
    }

    fn literal(&mut self) -> Box<dyn Expr> {
        match self.peek() {
            Some(C) => Box::new(Number::new(Some(1.0))),
            None => Box::new(Number::new(None))
        }
    }

     //advance the iterator
     fn consume(&mut self) -> Token {
        
        todo!()
    }

    fn consume_type(&mut self, typ: &TokenType) -> Token {
        if let Some(x) = self.peek() {
            match &x.token_type {
                Poisoned(err) =>{ 
                    println!("{}", &err); 
                    self.consume()
                }
                _ if &x.token_type == typ => {
                    self.consume()
                }
                other => {
                   let poison = ParseErr::Expected(Box::new(typ.to_owned()), Box::new(other.to_owned()));
                   println!("{}",poison);
                    self.consume()
                }
            }
        } else {
            Token::empty()
        }
    }
    //check the next value, does not advance
    fn peek(&mut self) -> Option<Token> {
        todo!()
    }

     fn is_at_end(&mut self) -> bool {
        todo!()
     }
}

