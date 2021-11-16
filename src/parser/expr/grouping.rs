use std::fmt::{format, write};

use super::expr::Expr;

pub struct Grouping {
    value: Box<dyn Expr>,
}
impl Expr for Grouping {}

impl Grouping {
    pub fn new(value: Box<dyn Expr>) -> Self {
        Self { value }
    }
}

impl std::fmt::Debug for Grouping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "grouping< {} >", format!("( {:?} )", &self.value) )
    }
}