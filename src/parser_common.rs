use crate::{
    ast::{Ast, AstKind, BinOp},
    parse_err::ParseErr,
    token_walker::TokenWalker,
};

pub type ParseResult = Result<Ast, ParseErr>;

pub fn parse_binary_expr(
    walker: &mut TokenWalker,
    sub_parser: fn(&mut TokenWalker) -> ParseResult,
    op_parser: fn(&mut TokenWalker) -> Result<BinOp, ParseErr>,
) -> ParseResult {
    let mut expr = sub_parser(walker)?;

    loop {
        let op_result = op_parser(walker);

        if let Ok(op) = op_result {
            let right = sub_parser(walker)?;

            expr = Ast::expr_bin(op, expr, right);
        } else {
            break;
        }
    }

    Ok(expr)
}
