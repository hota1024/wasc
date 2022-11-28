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

    pub fn encode(expr: &Self) -> String {
        let mut code = String::new();

        match &expr.kind {
            ExprKind::List(exprs) => {
                let mut items = Vec::new();
                for expr in exprs {
                    items.push(Self::encode(expr));
                }
                code.push('(');
                code.push_str(items.join(" ").as_str());
                code.push(')');
            }
            ExprKind::ListItems(exprs) => {
                let mut items = Vec::new();
                for expr in exprs {
                    items.push(Self::encode(expr));
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
