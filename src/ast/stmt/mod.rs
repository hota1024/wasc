pub mod StmtReturn;
pub mod StmtSemi;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    StmtSemi(StmtSemi::StmtSemi),
    StmtReturn(StmtReturn::StmtReturn),
}
