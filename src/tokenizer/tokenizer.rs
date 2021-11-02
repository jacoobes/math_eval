use super::tokens::TokenType::*;
use super::tokens::*;

pub struct Tokenizer {
     chars : Vec<char>   
}

impl Tokenizer {

    pub fn new (expr: String) -> Self {
        Self { 
            chars : expr.chars().collect()
        }
    }

    pub fn run(&self) -> Vec<Token> {
        let chars= &self.chars;
        let mut tokens: Vec<Token> = vec![];
    
        for mut i in 0..chars.len() {
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
                'x' => tokens.push(Token::new(Multiply, Some(String::from(ch)), i)),
                '%' => tokens.push(Token::new(Modulus, Some(String::from(ch)), i)),
    
                '.' => tokens.push(Token::new(Dot, Some(String::from(ch)), i)),
                '_' => tokens.push(Token::new(Base, Some(String::from(ch)), i)),
    
                '\'' => tokens.push(Token::new(Power, Some(String::from(ch)), i)),
    
                _ => {
                    if matches!(ch, '0'..='9') {

                    }
    
                }
            }
        }
        println!("{:?}", tokens);
        vec![]
    }

    fn is_at_end (&self, current_location : usize) -> bool {
        self.chars.len() == current_location
    }
    


}



