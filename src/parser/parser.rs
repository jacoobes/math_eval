use peekmore::{PeekMore, PeekMoreIterator};

use super::expr::expr::*;
use crate::panicker::lex_error::LexErr;
use crate::panicker::parse_error::ParseErr;
use crate::parser::expr::function_expr::FnExpr;
use crate::parser::expr::grouping::Grouping;
use crate::parser::expr::number::Number;
use crate::tokenizer::tokens::{
    Token,
    TokenType::{self, *},
};
use std::io::BufRead;
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

    pub fn parse(&mut self) -> Vec<Box<dyn Expr>> {
        let mut parse_tree = vec![];
        while let Some(_) = self.peek() {
            if !self.had_errors {
                parse_tree.push(self.expr());
            } else {
                parse_tree.clear();
                self.had_errors = false;
                break;
            }
        }
        parse_tree
    }
    fn expr(&mut self) -> Box<dyn Expr> {
        if self.had_errors {
            return Box::new(Number::new(None));
        }
        self.fn_expr()
    }
    
    fn fn_expr(&mut self) -> Box<dyn Expr> {
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
    fn term(&mut self) ->Box<dyn Expr> {
        self.factor()
    }
    fn factor(&mut self) -> Box<dyn Expr> {
        self.power()
    }
    fn power(&mut self) -> Box<dyn Expr> {
        self.primary()
    }
    fn primary(&mut self) -> Box<dyn Expr> {
        if let Some(token) = self.consume() {
            match token.token_type {
                Paren('(') => {
                    let expr = self.expr();
                    self.consume_type(Paren(')'));
                    Box::new(Grouping::new( expr ))
                },                
                Pi(val) => Box::new(Number::new(Some(val))),
                E(val) => Box::new(Number::new(Some(val))),
                Ans(val) => Box::new(Number::new(val)),
                Literal(val) => Box::new(Number::new(Some(val))),
                _ => {
                    self.had_errors = true;
                    println!("Unknown matches or unknown expression");
                    self.sync()
                }
            }
        } else {
            self.had_errors = true;
            println!("{}", ParseErr::EOF);
            self.sync()
        }
        

    }
    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }
    fn consume(&mut self) -> Option<Token> {
        self.tokens.next()
    }
    fn consume_type(&mut self, typ: TokenType) {
      if let Some(tok) = self.consume() {
          if typ != tok.token_type { println!("{}", ParseErr::Expected((typ, tok.token_type))); self.had_errors = true}
        }  else {
            self.had_errors = true;
            println!("{}", ParseErr::EOF)
        }
    }
    fn block(&mut self) -> Box<dyn Expr> {
        self.consume_type(Curly('{'));
        let value = self.fn_expr();
        self.consume_type(Curly('{'));
        value
    }
    fn sync (&mut self) -> Box<dyn Expr> {
        while let Some(tok) = self.peek() {
            match tok.token_type {
                Sine | Cosecant | Cotangent | Secant | Cosine | Tangent | Ln | Log | ArcCosine
                | ArcCot | ArcCsc | ArcSine | ArcTangent | Degree | Rad | Root | ArcSec  | Literal(_) => {
                    break;
                }
                _ => { self.consume(); }

            }
        };
        self.expr()
    }


}
