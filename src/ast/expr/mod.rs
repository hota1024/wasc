use super::lit::Lit;

pub mod expr_binary;
pub mod expr_block;
pub mod expr_unary;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    ExprBinary(expr_binary::ExprBinary),
    ExprUnary(expr_unary::ExprUnary),
    ExprBlock(expr_block::ExprBlock),
    Lit(Lit),
}
