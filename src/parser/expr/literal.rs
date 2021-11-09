use super::expr::Expr;
#[derive(Debug)]
struct Literal {
    value : Option<f64>
}

impl Expr for Literal {}

impl Literal {
    pub fn new(value: Option<f64>) -> Self {
        Self {
            value
        }
    }
}