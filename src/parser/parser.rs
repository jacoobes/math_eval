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
use std::{f64::consts::PI, vec::IntoIter};
pub struct Parser {
    tokens: PeekMoreIterator<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().peekmore(),
        }
    }
    //anyhow::Result<T>
    pub fn parse(&mut self) -> Vec<Box<dyn Expr>> {

        
        let mut tree = vec![];
        while !self.is_at_end() {
            tree.push(self.expr())
        }
        tree
    }

    fn expr(&mut self) -> Box<dyn Expr> {
        match self.peek() {
            Some(_) => self.fn_expr(),
            None => self.literal(),
        }
    }

    fn fn_expr(&mut self) -> Box<dyn Expr> {
        let mut possible_fn_expr = self.term();
        // match self.consume() {
        //     t if matches!(&t.token_type,
        //         Ln | Rad | Degree | Sine | Cosine | Tangent | Secant | Cosecant | ArcCosine | ArcSine | ArcTangent |
        //         ArcCot | ArcCsc | ArcSec) =>    {
        //             let expr = self.expr();
        //             possible_fn_expr = Box::new( FnExpr::new(t, expr) );
        //     }
        //     t if matches!(&t.token_type, Log | Root) => {
        //         let _ = self.consume_type(&Base);
        //         let base_power = self.expr();

        //     }
        //     _ => ()
        // };
        possible_fn_expr
    }

    fn term(&mut self) -> Box<dyn Expr> {
        self.factor()
    }

    fn factor(&mut self) -> Box<dyn Expr> {
        self.literal()
    }

    fn literal(&mut self) -> Box<dyn Expr> {
        match self.peek() {
            Some(tok) if tok.token_type == LeftParen => {
                self.consume();
                let expr = self.expr();
                self.consume_type(&RightParen);
                Box::new(Grouping::new(expr))
            }
            Some(tok) if tok.token_type == Literal => {
                let literal = self.consume();
                let value = literal.value.unwrap().parse::<f64>().ok();
                Box::new(Number::new(value))
            }
            Some(tok) if tok.token_type == Pi => {
                self.consume();
                Box::new(Number::new(Some(PI)))
            }
            Some(tok) if tok.token_type == E => {
                self.consume();
                Box::new(Number::new(Some(std::f64::consts::E)))
            }
            Some(_) => {
                let other = self.consume();
                match other.token_type {
                    Poisoned(_) => {
                        println!("poison");
                        Box::new(Number::new(None))
                    }
                    _ => {
                        Box::new(Number::new(None))
                    }
                }
            }
            None => Box::new(Number::new(None))
        }
    }

    //advance the iterator
    fn consume(&mut self) -> Token {
        self.tokens.next().unwrap()
    }

    fn consume_type(&mut self, typ: &TokenType) {
        if let Some(x) = self.peek() {
            match &x.token_type {
                Poisoned(_) => {
                    self.consume();
                }
                _ if &x.token_type == typ => {
                    self.consume();
                },
                other if other == &EOF => {
                    panic!("{}", ParseErr::EOF);
                },
                other => {
                    let poison =  ParseErr::Expected(x.clone(), Box::new(other.to_owned()));
                    panic!("{}", poison);
                }
            }
        } 
    }

    //check the next value, does not advance
    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    fn peek_next(&mut self) -> Option<&Token> {
        self.tokens.peek_next()
    }

    fn is_at_end(&mut self) -> bool {
        match self.peek() {
            Some(token) => { if token.token_type == EOF { true } else { false } }
            None => true,
        }
    }
}
