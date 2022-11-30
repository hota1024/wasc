use std::option;

use super::{Expr, ExprKind};

pub struct EncoderOptions {
    pub indent_str: String,
    pub new_line_str: String,
}

pub struct Encoder {
    pub indent_str: String,
    pub new_line_str: String,
}

impl Encoder {
    pub fn default() -> Self {
        Self {
            indent_str: "  ".to_string(),
            new_line_str: "\n".to_string(),
        }
    }

    pub fn new(options: EncoderOptions) -> Self {
        Self {
            indent_str: options.indent_str,
            new_line_str: options.new_line_str,
        }
    }

    pub fn encode(self: &Self, expr: &Expr) -> String {
        self.encode_expr(&expr, 0, false)
    }

    fn encode_expr(self: &Self, expr: &Expr, level: usize, inline: bool) -> String {
        let indent = if inline {
            String::new()
        } else {
            self.indent_str.repeat(level)
        };
        let new_line = if inline {
            ""
        } else {
            self.new_line_str.as_str()
        };

        match &expr.kind {
            ExprKind::List(ref exprs) => {
                if exprs.iter().any(|e| matches!(e.kind, ExprKind::List(_))) {
                    let first = exprs.first().unwrap();
                    let (_, rest) = exprs.split_first().unwrap();

                    let mut items = vec![format!(
                        "{}({}",
                        indent,
                        self.encode_expr(&first, level, true)
                    )];

                    for expr in rest {
                        items.push(self.encode_expr(&expr, level + 1, false));
                    }

                    items.push(format!("{})", indent));

                    items.join(new_line)
                } else {
                    format!(
                        "{}({})",
                        indent,
                        exprs
                            .iter()
                            .map(|e| self.encode_expr(&e, level + 1, true))
                            .collect::<Vec<_>>()
                            .join(" ")
                    )
                }
            }
            ExprKind::Symbol(value) => format!("{}{}", indent, value),
            ExprKind::String(value) => format!("{}\"{}\"", indent, value),
            ExprKind::Int(value) => format!("{}{}", indent, value),
            ExprKind::Float(value) => format!("{}{}", indent, value),
        }
    }
}
