pub mod stmt_return;
pub mod stmt_semi;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    StmtSemi(stmt_semi::StmtSemi),
    StmtReturn(stmt_return::StmtReturn),
}
