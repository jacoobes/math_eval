
use super::expr::Expr;


#[derive(Debug)]
struct Grouping <V: Expr> {
    value: V
}
impl <V: Expr> Expr for Grouping <V> {}

 impl <V: Expr> Grouping <V> {
    pub fn new( value: V) -> Self {
        Self {
            value
        }
    }
}

