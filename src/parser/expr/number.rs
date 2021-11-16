use super::expr::Expr;
pub struct Number {
    value: Option<f64>,
}

impl Expr for Number {}

impl Number {
    pub fn new(value: Option<f64>) -> Self {
        Self { value }
    }
    fn format_num(&self) -> String {
        if let Some(v) = self.value {
            v.to_string()
        } else {
            String::from("null")
        }
    }
}

impl std::fmt::Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}" , self.format_num() )
    }
}