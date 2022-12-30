use super::lit::Lit;

pub mod expr_as;
pub mod expr_binary;
pub mod expr_block;
pub mod expr_call;
pub mod expr_unary;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    ExprBinary(expr_binary::ExprBinary),
    ExprUnary(expr_unary::ExprUnary),
    ExprBlock(expr_block::ExprBlock),
    ExprCall(expr_call::ExprCall),
    ExprAs(expr_as::ExprAs),
    Lit(Lit),
}
