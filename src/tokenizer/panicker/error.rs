use std::num::ParseFloatError;

#[derive(Debug)]
pub enum CalcErr {
    ParseFloat(ParseFloatError)
}