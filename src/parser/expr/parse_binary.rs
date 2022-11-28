use crate::{
    ast::expr::{
        expr_binary::{BinaryOp, ExprBinary},
        Expr,
    },
    parser::{
        parser_result::{ParseErr, ParseResult},
        token_walker::TokenWalker,
    },
};

pub fn parse_binary_expr(
    walker: &mut TokenWalker,
    sub_parser: fn(&mut TokenWalker) -> ParseResult<Expr>,
    op_parser: fn(&mut TokenWalker) -> Result<BinaryOp, ParseErr>,
) -> ParseResult<Expr> {
    let mut expr = sub_parser(walker)?;

    loop {
        let op_result = op_parser(walker);

        if let Ok(op) = op_result {
            let right = sub_parser(walker)?;

            expr = Expr::ExprBinary(ExprBinary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            });
        } else {
            break;
        }
    }

    Ok(expr)
}
