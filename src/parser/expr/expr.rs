use std::{any::Any, fmt::Debug};

pub trait Expr
where
    Self: Debug + Any,
{
}
