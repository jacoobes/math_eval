use crate::tokenizer::tokens::Token;

use super::expr::Expr;

pub struct FnExpr {
    function: Token,
    base: Option<Box<dyn Expr>>,
    value: Box<dyn Expr>,
}
impl Expr for FnExpr {}

impl FnExpr {
    pub fn new(function: Token, base: Option<Box<dyn Expr>>, value: Box<dyn Expr>) -> Self {
        Self {
            function,
            base,
            value,
        }
    }
    fn base_or_no(&self) -> String {
        if let Some(_) = self.base {
            format!("_({:?})", &self.base)
        } else {
            "".to_string()
        }
    }
}

impl std::fmt::Debug for FnExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}" , format!("\n {:?} {} {} {:?} {}\n", &self.function.token_type , self.base_or_no(), "{", *&self.value, "}"))
    }
}