use peekmore::{PeekMore, PeekMoreIterator};

use super::expr::expr::*;
use crate::panicker::lex_error::LexErr;
use crate::panicker::parse_error::ParseErr;
use crate::parser::expr::function_expr::FnExpr;
use crate::parser::expr::binary::BinaryExpr;
use crate::parser::expr::grouping::Grouping;
use crate::parser::expr::number::Number;
use crate::tokenizer::tokens::{
    Token,
    TokenType::{self, *},
};
use std::iter::Peekable;
use std::vec::IntoIter;
pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
    had_errors: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
            had_errors: false,
        }
    }

    pub fn check_lexer_errors(tokens: &Vec<Token>) -> bool {
        let mut had_err = false;
        println!("{:?}", tokens);
        for token in tokens {
            match &token.token_type {
                Poisoned(e) => {
                    println!("{}", &e);
                    had_err = true;
                }
                _ => (),
            }
        }
        had_err
    }

    pub fn had_err(&self) -> bool {
        self.had_errors
    }

    pub fn parse(&mut self) -> Result<Vec<Box<dyn Expr>>, ()> {
        let mut parse_tree = vec![];
        while let Some(_) = self.peek() {
            if !self.had_errors {
                parse_tree.push(self.expr()?);
            } else {
                parse_tree.clear();
                self.had_errors = false;
                break;
            }
        }
        Ok(parse_tree)
    }
    fn expr(&mut self) ->  Result<Box<dyn Expr>,()> {
        if self.had_errors {
            Err(())
        } else {
            self.fn_expr()
        }
    }
    
    fn fn_expr(&mut self) -> Result<Box<dyn Expr>,()> {
        // let peek = self.peek().and_then(|_| Ok(self.consume()));

        // match peek {
        //     Ok(tok) => 
        //         match tok.token_type {
        //         Sine | Cosecant | Cotangent | Secant | Cosine | Tangent | Ln | Log | ArcCosine
        //         | ArcCot | ArcCsc | ArcSine | ArcTangent | Degree | Rad | Root | ArcSec => {
        //             let name = tok;
        //             let base = match self.consume_type(Base) {
        //                 Ok(token) => {

        //                 },
        //                 Err(e) => () 
        //             }
        //         }
        //         _=> self.term()    
        //     },
        //     Err(e) => Err(e)
        // }
        self.term()
    }
    fn term(&mut self) -> Result<Box<dyn Expr>,()> {
        self.factor()
    }
    fn factor(&mut self) -> Result<Box<dyn Expr>,()> {
        self.power()
    }
    fn power(&mut self) ->  Result<Box<dyn Expr>,()> {
        if let None = self.peek() {
            self.sync();
        }
        self.primary()
    }
    fn primary(&mut self) -> Result<Box<dyn Expr>,()> {
        if let None = self.peek() {
            self.sync();
        }
        
       let token = self.consume().unwrap(); 
            match token.token_type {
                Paren('(') => {
                    let expr = self.expr();
                    self.consume_type(Paren(')'))?;
                    Ok(Box::new(Grouping::new( expr.unwrap() )))
                },                
                Pi(val) => Ok(Box::new(Number::new(Some(val)))),
                E(val) => Ok(Box::new(Number::new(Some(val)))),
                Ans(val) => Ok(Box::new(Number::new(val))),
                Literal(val) => Ok(Box::new(Number::new(Some(val)))),
                _ => {
                    self.report(ParseErr::UnknownKeyword(token));
                    Err(())
                }
            }
        } 

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }
    fn consume(&mut self) -> Option<Token> {
        match self.tokens.next() {
            Some(c) => Some(c),
            None => {
                self.report(ParseErr::EOF);
                None
            }
        }
    }
    fn consume_type(&mut self, typ: TokenType,) -> Result<Token, ()>  {
      if let None = self.peek() {
         return Err(self.report(ParseErr::EOF));
      } 
        let tok = self.peek().unwrap().to_owned(); 
          if typ != tok.token_type {
               Err( self.report(ParseErr::Expected((typ, tok.token_type))))
            } else {
                Ok(self.consume().unwrap())
            }
        
    }

    fn report(&mut self, err : ParseErr)  {
        self.had_errors = true;
        println!("{}", err);
        self.sync();
    }
    fn block(&mut self) -> Result<Box<dyn Expr>, ()> {
        self.consume_type(Curly('{'))?;
        let value = self.fn_expr();
        self.consume_type(Curly('}'))?;
        value
    }
    fn sync (&mut self)  {
        while let Some(tok) = self.peek() {
            match tok.token_type {
                Sine | Cosecant | Cotangent | Secant | Cosine | Tangent | Ln | Log | ArcCosine
                | ArcCot | ArcCsc | ArcSine | ArcTangent | Degree | Rad | Root | ArcSec  | Literal(_) => {
                    break;
                }
                _ => { self.consume(); }

            }
        };
    }


}
