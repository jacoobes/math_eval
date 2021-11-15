use peekmore::{PeekMore, PeekMoreIterator};

use super::expr::expr::*;
use crate::panicker::lex_error::LexErr;
use crate::panicker::parse_error::ParseErr;
use crate::parser::expr::binary::BinaryExpr;
use crate::parser::expr::function_expr::FnExpr;
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

    pub fn parse(&mut self) -> Result<Vec<Box<dyn Expr>>, ParseErr> {
        let mut parse_tree = vec![];

        while let Ok(_) = self.peek() {
            parse_tree.push(self.expr()?)
        }

        Ok(parse_tree)
    }
    fn expr(&mut self) -> Result<Box<dyn Expr>, ParseErr> {
        if self.had_errors {
            Err(ParseErr::EOF("Ended evaluation"))
        } else {
            self.fn_expr()
        }
    }

    fn fn_expr(&mut self) -> Result<Box<dyn Expr>, ParseErr> {
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
    fn term(&mut self) -> Result<Box<dyn Expr>, ParseErr> {
        self.factor()
    }
    fn factor(&mut self) -> Result<Box<dyn Expr>, ParseErr> {
        let mut expr = self.power();

        
        

        expr
    }
    fn power(&mut self) -> Result<Box<dyn Expr>, ParseErr> {
        let mut expr = self.primary();
        
        match self.peek() {
            Ok(tok) if tok.token_type == Power => {
               self.next()?;
               let right = self.power();
               expr = Ok(Box::new(BinaryExpr::new(expr?, Power, right?))) 
            },
            Ok(_) | Err(_) => ()
        };
        
        expr  
    }
    fn primary(&mut self) -> Result<Box<dyn Expr>, ParseErr> {
        match &self.peek().map_err(|_| ParseErr::EOF("END"))?.token_type {
            Paren('(') => {
                self.next()?;
                let expr = self.expr();
                self.consume_if(
                    &Paren(')'),
                    "Expected RIGHT_PAREN, parsed until end of line",
                )?;
                Ok(Box::new(Grouping::new(expr.unwrap())))
            }
            Pi => Ok(Box::new(Number::new(Some(
                self.next().map(|_| std::f64::consts::PI)?,
            )))),
            E => Ok(Box::new(Number::new(Some(
                self.next().map(|_| std::f64::consts::E)?,
            )))),
            Ans => Ok(Box::new(Number::new(
                self.next().and_then(|tok| Ok(tok.value))?,
            ))),
            Literal => Ok(Box::new(Number::new(
                self.next().and_then(|c| Ok(c.value))?,
            ))),
            _ => Err(ParseErr::UnknownKeyword(self.next()?.token_type)),
        }
    }

    fn peek(&mut self) -> Result<&Token, ()> {
        if let None = self.tokens.peek() {
            Err(())
        } else {
            Ok(self.tokens.peek().unwrap())
        }
    }
    fn next(&mut self) -> Result<Token, ParseErr> {
        if let None = self.tokens.peek() {
            Err(ParseErr::EOF("Tried consuming, nothing left to consume!"))
        } else {
            Ok(self.tokens.next().unwrap())
        }
    }
    fn previous(&self) -> &Token {
        todo!()
    }

    fn consume_if( &mut self, typ: &TokenType,  message_if_fail: &'static str,)
     -> Result<Token, ParseErr> {
        match self.peek().map_err(|_| ParseErr::EOF(message_if_fail))? {
            tok if &tok.token_type == typ => Ok(self.next())?,
            tok => Err(ParseErr::Expected((typ.clone(), tok.token_type.clone()))),
        }
    }

    fn report(&mut self, err: ParseErr) -> ParseErr {
        self.had_errors = true;
        println!("{}", err);
        err
    }
    // fn sync (&mut self)  {
    //     while let Some(tok) = self.peek() {
    //         match tok.token_type {
    //             Sine | Cosecant | Cotangent | Secant | Cosine | Tangent | Ln | Log | ArcCosine
    //             | ArcCot | ArcCsc | ArcSine | ArcTangent | Degree | Rad | Root | ArcSec  | Literal(_) => {
    //                 break;
    //             }
    //             _ => { self.next(); }

    //         }
    //     };
    // }
}
