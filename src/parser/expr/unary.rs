use super::expr::Expr;

#[derive(Debug)]
struct Unary<V: Expr> {
    value: V,
}

impl<V: Expr> Expr for Unary<V> {}

impl<V: Expr> Unary<V> {
    pub fn new(value: V) -> Self {
        Self { value }
    }
}
