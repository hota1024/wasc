use crate::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    List(Vec<Expr>),
    ListItems(Vec<Expr>),
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

    pub fn list_items(exprs: Vec<Expr>) -> Self {
        Self::new(ExprKind::ListItems(exprs))
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

    pub fn list_append(&mut self, expr: Expr) {
        if let ExprKind::List(exprs) = &mut self.kind {
            if let ExprKind::List(mut other) = expr.kind {
                exprs.append(&mut other);
            } else {
                exprs.push(expr);
            }
        } else {
            panic!("Expr::list_append: self is not a list");
        }
    }

    pub fn encode(&self) -> String {
        let mut code = String::new();

        match &self.kind {
            ExprKind::List(exprs) => {
                let mut items = Vec::new();
                for expr in exprs {
                    if let ExprKind::ListItems(exprs) = &expr.kind {
                        if exprs.len() == 0 {
                            continue;
                        }
                    }

                    items.push(expr.encode());
                }
                code.push('(');
                code.push_str(items.join(" ").as_str());
                code.push(')');
            }
            ExprKind::ListItems(exprs) => {
                let mut items = Vec::new();
                for expr in exprs {
                    items.push(expr.encode());
                }
                code.push_str(items.join(" ").as_str());
            }
            ExprKind::Symbol(name) => {
                code.push_str(name);
            }
            ExprKind::String(value) => {
                code.push('"');
                code.push_str(value);
                code.push('"');
            }
            ExprKind::Int(value) => {
                code.push_str(&value.to_string());
            }
            ExprKind::Float(value) => {
                code.push_str(&value.to_string());
            }
        }

        return code;
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

macro_rules! s_list_items {
    ($expr:expr) => {
        $crate::sexpr::Expr::list_items($expr)
    };
    ($($expr:expr),*) => {
        $crate::sexpr::Expr::list_items(vec![$($expr),*])
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

pub(crate) use {s_float, s_int, s_list, s_list_items, s_string, s_symbol};
