use super::lit::Lit;

pub mod ExprBinary;
pub mod ExprBlock;
pub mod ExprUnary;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    ExprBinary(ExprBinary::ExprBinary),
    ExprUnary(ExprUnary::ExprUnary),
    ExprBlock(ExprBlock::ExprBlock),
    Lit(Lit),
}
