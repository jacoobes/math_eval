use super::expr::Expr;
#[derive(Debug)]
pub struct Number {
    value: Option<f64>,
}

impl Expr for Number {}

impl Number {
    pub fn new(value: Option<f64>) -> Self {
        Self { value }
    }
}
