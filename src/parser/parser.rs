use peekmore::{PeekMore, PeekMoreIterator};

use super::expr::expr::*;
use crate::panicker::lex_error::LexErr;
use crate::panicker::parse_error::ParseErr;
use crate::parser::expr::binary::BinaryExpr;
use crate::parser::expr::function_expr::FnExpr;
use crate::parser::expr::unary::Unary;
use crate::parser::expr::grouping::Grouping;
use crate::parser::expr::number::Number;
use crate::tokenizer::tokens::{
    Token,
    TokenType::{self, *},
};
use std::iter::Peekable;
use std::vec::{self, IntoIter};
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
        if let Ok(token) =  self.match_advance(
            vec![ Sine, Cosecant, ArcCsc, Cosine, Secant,  ArcSec,  Tangent, Cotangent, ArcCot, ArcSine,
            ArcCosine, ArcTangent, Ln, Rad,
            Degree,]
        ) {
                let block = self.fn_block()?;
                return Ok(Box::new(FnExpr::new(token, None, block)))
        }
         if let Ok(token) = self.match_advance(vec![Log,Root ]) {
                self.consume_if(&Base, "Expected Base")?;
                let base = self.expr()?;
                let block = self.fn_block()?;
                return Ok(Box::new(FnExpr::new(token, Some(base),block )))
        };
        self.term()
    }
    //left associative
    fn term(&mut self) -> Result<Box<dyn Expr>, ParseErr> {
        let mut expr = self.factor();

        while let Ok(tok) = self.match_advance(vec![Term('+'), Term('-')]) {
            let right = self.power();
            expr = Ok(Box::new(BinaryExpr::new(expr?, tok.token_type, right?)))
        }
        expr
    }
    //left associative
    fn factor(&mut self) -> Result<Box<dyn Expr>, ParseErr> {
        let mut expr = self.power();

        while let Ok(tok) = self.match_advance(vec![Factor('x'), Factor('/'), Factor('%') ]) {
            let right = self.power();
            expr = Ok(Box::new(BinaryExpr::new(expr?, tok.token_type, right?)))
        }

        expr
    }
    //right associative
    fn power(&mut self) -> Result<Box<dyn Expr>, ParseErr> {
        let mut expr = self.unary();

        match self.match_advance(vec![Power]){
            Ok(tok) => {
                let right = self.power();
                expr = Ok(Box::new(BinaryExpr::new(expr?, tok.token_type, right?)))
            }
             Err(_) => (),
        };

        expr
    }
    //right associative
    fn unary(&mut self) -> Result<Box<dyn Expr>, ParseErr> {
        match self.match_advance(vec![Term('+'), Term('-'), Squiggly]) {
            Ok(token ) => {
                let expr = self.unary();
                return Ok(Box::new(Unary::new(token, expr?)))
            },
            Err(_) => ()
        }
        self.primary()

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
            _ => Err(ParseErr::UnknownKeyword(self.next()?.token_type, "Encountered an out-of-place token while parsing.")),
        }
    }
    fn fn_block(&mut self) -> Result<Box<dyn Expr>, ParseErr> {
        
        self.consume_if(&Curly('{'), "Expected a '{' after fn_expr declaration!")?;
        let expr = self.expr()?;
        self.consume_if(&Curly('}'), "Expected '}' after fn_expr declaration!")?;
        Ok(expr)
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

    fn consume_if(
        &mut self,
        typ: &TokenType,
        message_if_fail: &'static str,
    ) -> Result<Token, ParseErr> {
        match self.peek().map_err(|_| ParseErr::EOF(message_if_fail))? {
            tok if &tok.token_type == typ => Ok(self.next())?,
            tok => Err(ParseErr::Expected((typ.clone(), tok.token_type.clone()))),
        }
    }
    fn match_advance(&mut self, types: Vec<TokenType>) -> Result<Token, ()> {
        match &self.peek()?.token_type {
            toke if types.iter().any(|tt| tt == toke )=> {
                Ok(self.next().map_err(|_|()))?
            }
            _ => Err(())
        }
    }

    fn report(&mut self, err: ParseErr) -> ParseErr {
        self.had_errors = true;
        println!("{}", err);
        err
    }
}
