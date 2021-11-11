use super::expr::Expr;

#[derive(Debug)]
struct Unary {
    value: Box<dyn Expr>,
}

impl Expr for Unary {}

impl Unary {
    pub fn new(value: Box<dyn Expr>) -> Self {
        Self { value }
    }
}
