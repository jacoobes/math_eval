use crate::tokenizer::tokens::{Token, TokenType};
use super::expr::expr::*;
use std::iter::{FromIterator, Peekable};
use std::vec::IntoIter;
pub struct Parser
 {
    tokens : Peekable<IntoIter<Token>>,
}


impl Parser {
    fn expr<T:Expr>(&self) -> Box<dyn Expr> {
       todo!()
    }

    fn fn_expr<T: Expr>(&self) -> Box<dyn Expr> {
        todo!()
    }

    fn term<T:Expr>(&self) -> Box<dyn Expr> {
        todo!()
    }
    
    fn factor<T:Expr>(&self) -> Box<dyn Expr> {
        todo!()
    }


     //advance the iterator
     fn consume(&mut self) -> Token {
        self.tokens.next().unwrap()
    }
    //check the next value, does not advance
    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

     fn is_at_end(&mut self) -> bool {
        self.tokens.peek().unwrap().token_type == TokenType::EOF
     }
}

