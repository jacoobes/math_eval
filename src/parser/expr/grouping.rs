use super::expr::Expr;

#[derive(Debug)]
pub struct Grouping {
    value: Box<dyn Expr>,
}
impl Expr for Grouping {}

impl Grouping {
    pub fn new(value: Box<dyn Expr>) -> Self {
        Self { value }
    }
}
