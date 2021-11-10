use peekmore::{PeekMore, PeekMoreIterator};

use super::expr::expr::*;
use crate::parser::expr::function_expr::FnExpr;
use crate::parser::expr::grouping::Grouping;
use crate::parser::expr::number::Number;
use crate::parser::expr::parse_error::ParseErr;
use crate::tokenizer::tokens::{
    Token,
    TokenType::{self, *},
};
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
        match self.consume() {
            tok if tok.token_type == LeftParen => {
                let expr = self.expr();
                self.consume_type(&RightParen);
                Box::new(Grouping::new(expr))
            }
            tok if tok.token_type == Literal => {
                let value = tok.value.unwrap().parse::<f64>().ok();
                Box::new(Number::new(value))
            }
            tok if matches!(tok.token_type, Pi) => Box::new(Number::new(Some(PI))),
            tok if matches!(tok.token_type, Ans) => Box::new(Number::new(None)),

            tok => Box::new(Number::new(None)),
        }
    }

    //advance the iterator
    fn consume(&mut self) -> Token {
        self.tokens.next().unwrap()
    }

    fn consume_type(&mut self, typ: &TokenType) -> Token {
        if let Some(x) = self.peek() {
            match &x.token_type {
                Poisoned(err) => {
                    println!("{}", &err);
                }
                _ if &x.token_type == typ => {
                    self.consume();
                }
                other => {
                    let poison =
                        ParseErr::Expected(Box::new(typ.to_owned()), Box::new(other.to_owned()));
                    println!("{}", poison);
                }
            }
            self.consume()
        } else {
            Token::empty()
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
            Some(token) if matches!(token.token_type, EOF) => true,
            Some(_) => false,
            None => true,
        }
    }
}
