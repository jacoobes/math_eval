use std::fmt::Display;
use std::num::ParseFloatError;

#[derive(Debug, Clone, PartialEq)]
pub enum LexErr {
    NumParseErr(ParseFloatError, String),
    UnknownKeyword(String),
}
impl Display for LexErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexErr::NumParseErr(parse_float_error, weird_strang) => {
                write!(f, "Bad parse : {} \n\n {}", weird_strang, parse_float_error)
            }
            LexErr::UnknownKeyword(weird_string) => {
                write!(f, "Unknown keyword : {}", weird_string)
            }
        }
    }
}
impl std::error::Error for LexErr {}
