use super::tokens::TokenType::*;
use super::tokens::*;

pub fn tokenizer(expr: String) -> Vec<Token> {
    let chars: Vec<char> = expr.chars().collect();
    let mut tokens: Vec<Token> = vec![];

    for i in 0..chars.len() {
        let ch = chars[i];
        match ch {
            '{' => tokens.push(Token::new(LeftCurly, Some(String::from(ch)), i)),
            '}' => tokens.push(Token::new(RightCurly, Some(String::from(ch)), i)),

            '[' => tokens.push(Token::new(LeftBracket, Some(String::from(ch)), i)),
            ']' => tokens.push(Token::new(RightBracket, Some(String::from(ch)), i)),

            '(' => tokens.push(Token::new(LeftParen, Some(String::from(ch)), i)),
            ')' => tokens.push(Token::new(RightParen, Some(String::from(ch)), i)),

            '+' => tokens.push(Token::new(Plus, Some(String::from(ch)), i)),
            '-' => tokens.push(Token::new(Minus, Some(String::from(ch)), i)),

            '/' => tokens.push(Token::new(Divide, Some(String::from(ch)), i)),
            '%' => tokens.push(Token::new(Modulus, Some(String::from(ch)), i)),

            '.' => tokens.push(Token::new(Dot, Some(String::from(ch)), i)),
            '_' => tokens.push(Token::new(Base, Some(String::from(ch)), i)),

            '\'' => tokens.push(Token::new(Power, Some(String::from(ch)), i)),

            _ => ()
        }
    }
    vec![]
}


