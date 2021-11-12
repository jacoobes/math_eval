use peekmore::{PeekMore, PeekMoreIterator};

use super::expr::expr::*;
use crate::parser::expr::fn_expr_w_base::FnWithBase;
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
    errors: Vec<ParseErr>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        
        Self {
            tokens: tokens.into_iter().peekmore(),
            errors: vec![]
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
    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }
    fn consume(&mut self) -> Token {
        self.tokens.next().unwrap()
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
