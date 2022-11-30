pub mod encoder;

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    List(Vec<Expr>),
    Symbol(String),
    String(String),
    Int(i64),
    Float(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
}

impl Expr {
    pub fn new(kind: ExprKind) -> Self {
        Self { kind }
    }

    pub fn list(exprs: Vec<Expr>) -> Self {
        Self::new(ExprKind::List(exprs))
    }

    pub fn symbol(name: String) -> Self {
        Self::new(ExprKind::Symbol(name))
    }

    pub fn string(value: String) -> Self {
        Self::new(ExprKind::String(value))
    }

    pub fn int(value: i64) -> Self {
        Self::new(ExprKind::Int(value))
    }

    pub fn float(value: f64) -> Self {
        Self::new(ExprKind::Float(value))
    }
}

macro_rules! s_list {
    ($expr:expr) => {
        $crate::sexpr::Expr::list($expr)
    };
    ($($expr:expr),*) => {
        $crate::sexpr::Expr::list(vec![$($expr),*])
    };

}

macro_rules! s_symbol {
    ($name:tt) => {
        $crate::sexpr::Expr::symbol($name.to_string())
    };
    ($name:expr) => {
        $crate::sexpr::Expr::symbol($name)
    };
}

macro_rules! s_string {
    ($value:expr) => {
        $crate::sexpr::Expr::string($value.to_string())
    };
}

macro_rules! s_int {
    ($value:expr) => {
        $crate::sexpr::Expr::int($value)
    };
}

macro_rules! s_float {
    ($value:expr) => {
        $crate::sexpr::Expr::float($value)
    };
    () => {};
}

pub(crate) use {s_float, s_int, s_list, s_string, s_symbol};
