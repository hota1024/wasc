pub mod stmt_let;
pub mod stmt_return;
pub mod stmt_semi;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    StmtSemi(stmt_semi::StmtSemi),
    StmtReturn(stmt_return::StmtReturn),
    StmtLet(stmt_let::StmtLet),
}
