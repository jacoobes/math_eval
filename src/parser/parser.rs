use peekmore::{PeekMore, PeekMoreIterator};

use super::expr::expr::*;
use crate::parser::expr::function_expr::FnExpr;
use crate::parser::expr::grouping::Grouping;
use crate::parser::expr::number::Number;
use crate::tokenizer::tokens::{
    Token,
    TokenType::{self, *},
};
use crate::panicker::parse_error::ParseErr;
use std::vec::IntoIter;
pub struct Parser {
    tokens: PeekMoreIterator<IntoIter<Token>>,
    had_errors: bool
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        
        Self {
            tokens: tokens.into_iter().peekmore(),
            had_errors: false
        }
    }

    pub fn check_lexer_errors(tokens: &Vec<Token>) -> bool {
        let mut had_err = false;
        for token in tokens {
            match &token.token_type {
                Poisoned(e) =>  {
                    println!("{}", &e);
                    had_err = true;
                },
                _ => ()
            }
        };
        had_err
    }

    pub fn parse(&mut self) -> Vec<Box<dyn Expr>> {
        let mut parse_tree = vec![];
        while let Some(_) = self.peek() {
            if self.had_errors {
                self.synchronize();
            } 
            parse_tree.push(self.expr());
        }
        parse_tree 
    }

    fn expr(&mut self) -> Box<dyn Expr> {
        self.fn_expr()
    }
    fn fn_expr(&mut self) -> Box<dyn Expr> {
        if let Some(token) = self.peek() {
            match &token.token_type {
                Sine | Cosecant | Cotangent | Secant | Cosine | Tangent 
                | Ln | Log | ArcCosine | ArcCot | ArcCsc | ArcSine | ArcTangent | Degree 
                | Rad | Root | ArcSec => {
                  let fn_name = self.consume();
                  if let Some(token) = self.peek() {
                      if &token.token_type == &Base { 
                        self.consume();  
                        let base = Some(self.expr());
                        let fn_block = self.block();
                        Box::new(FnExpr::new(fn_name, base, fn_block))
                      } else {
                         self.consume();
                         let fn_block = self.block();
                         Box::new(FnExpr::new(fn_name, None,fn_block))
                      }
                  } else {
                      self.term()
                  }
                }
                _ => self.term()
            }
        } else {
            self.term()
        }
    }
    fn term(&mut self) -> Box<dyn Expr> {
        self.factor()   
    }
    fn factor(&mut self) -> Box<dyn Expr> {
        self.power()
    }
    fn power(&mut self) -> Box<dyn Expr> {
        self.primary()
    }
    fn primary(&mut self) -> Box<dyn Expr> {
        Box::new(Number::new(None))
    }
    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }
    fn consume(&mut self) -> Token {
        self.tokens.next().unwrap()
    }
    fn consume_type(&mut self, typ : TokenType) -> Option<Token> {
        let result = match self.peek() {
            Some(token) if &token.token_type == &typ =>  {
                Ok(self.consume())
            }
            Some(_) => {
                self.had_errors = true;
                Err(ParseErr::Expected(Box::new(typ), self.consume()))
            },
            None => Err(ParseErr::EOF)
        };
        match result {
            Ok(t) => Some(t),
            Err(e) => {
                println!("{}", &e);
                None
            }
        }
    }
    fn block(&mut self) -> Box<dyn Expr> {
        self.consume_type(Curly('{')); 
        let value = self.expr();
        self.consume_type(Curly('{')); 
        value
    }




    fn synchronize(&mut self) {
        while let Some(token) = self.peek() {
            match &token.token_type {
                Literal(_) | Sine | Cosecant | Cotangent | Secant | Cosine | Tangent 
                | Ln | Log | ArcCosine | ArcCot | ArcCsc | ArcSine | ArcTangent | Degree 
                | Rad | Root | ArcSec =>  break,
                _ => () 
            }
            self.consume();
        }
    }
}
