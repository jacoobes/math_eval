use std::num::ParseFloatError;
use std::fmt::Display;
#[derive(Debug, Clone)]
pub enum CalcErr {
    NumParseErr(ParseFloatError, String),
    UnknownKeyword(String)
}

impl Display for CalcErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalcErr::NumParseErr(parse_float_error, weird_strang) => 
                write!(f, "Bad parse : {} \n\n {}", weird_strang, parse_float_error),
             CalcErr::UnknownKeyword(weird_string) => 
                write!(f, "Unknown keyword : {}",  weird_string),
        }
    }
}

impl std::error::Error for CalcErr {}