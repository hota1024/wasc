use super::lit::Lit;

pub mod ExprBinary;
pub mod ExprBlock;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    ExprBin(ExprBinary::ExprBinary),
    ExprBlock(ExprBlock::ExprBlock),
    Lit(Lit),
}
