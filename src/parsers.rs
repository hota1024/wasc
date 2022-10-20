use crate::{
    ast::{Ast, AstKind, BinOp, Type},
    lexer::{lex, remove_whitespace_tokens},
    parse_err::ParseErr,
    parser_common::{parse_binary_expr, ParseResult},
    span::Span,
    token_walker::TokenWalker,
    tokens::{Token, TokenKind},
};

pub fn parse(walker: &mut TokenWalker) -> ParseResult {
    parse_program(walker)
}

pub fn parse_program(walker: &mut TokenWalker) -> ParseResult {
    let mut items = Vec::new();

    while walker.peek().kind != TokenKind::Eof {
        let item = parse_item(walker)?;

        items.push(item);
    }

    Ok(Ast::program(items))
}

pub fn parse_item(walker: &mut TokenWalker) -> ParseResult {
    let peek = walker.peek();

    match peek.kind {
        TokenKind::KeywordLet => parse_item_let(walker),
        TokenKind::KeywordFn => parse_item_fn(walker),
        _ => Err(ParseErr::unexpected_token(peek.clone(), format!("global"))),
    }
}

pub fn parse_item_let(walker: &mut TokenWalker) -> ParseResult {
    walker.expect_next_token(TokenKind::KeywordLet)?;
    let let_token = walker.current().clone();
    let is_mut = if walker.peek().kind == TokenKind::KeywordMut {
        walker.next();
        true
    } else {
        false
    };
    let ident_token = walker.next();
    let name = Ast::ident_from_token(ident_token)?;
    walker.expect_next_token(TokenKind::Colon)?;
    let ty = parse_ty(walker)?;
    walker.expect_next_token(TokenKind::Eq)?;
    let init = parse_expr(walker)?;
    let semi_token = walker.expect_next_token(TokenKind::Semi)?;

    Ok(Ast::item_let(
        &let_token,
        name,
        is_mut,
        ty,
        init,
        &semi_token,
    ))
}

pub fn parse_item_fn(walker: &mut TokenWalker) -> ParseResult {
    walker.expect_next_token(TokenKind::KeywordFn)?;
    let fn_token = walker.current().clone();
    let ident_token = walker.next();
    let name = Ast::ident_from_token(ident_token)?;
    walker.expect_next_token(TokenKind::OpenParen)?;
    walker.expect_next_token(TokenKind::CloseParen)?;
    walker.expect_next_token(TokenKind::Colon)?;
    let ret_type = parse_ty(walker)?;
    let block = parse_block(walker)?;

    Ok(Ast::item_fn(&fn_token, name, ret_type, block))
}

pub fn parse_block(walker: &mut TokenWalker) -> ParseResult {
    walker.expect_next_token(TokenKind::OpenBrace)?;
    let open_token = walker.current().clone();
    let mut stmts = Vec::new();

    while walker.peek().kind != TokenKind::CloseBrace {
        let item = parse_stmt(walker)?;

        stmts.push(item);
    }

    let close_token = walker.expect_next_token(TokenKind::CloseBrace)?;

    Ok(Ast::block(&open_token, stmts, &close_token))
}

pub fn parse_stmt(walker: &mut TokenWalker) -> ParseResult {
    let peek = walker.peek();

    match peek.kind {
        TokenKind::KeywordReturn => parse_stmt_return(walker),
        _ => {
            let expr = parse_expr(walker)?;
            let semi_token = walker.expect_next_token(TokenKind::Semi)?;

            Ok(Ast::stmt_semi(expr, semi_token))
        }
    }
}

pub fn parse_stmt_return(walker: &mut TokenWalker) -> ParseResult {
    walker.expect_next_token(TokenKind::KeywordReturn)?;
    let return_token = walker.current().clone();
    let expr = parse_expr(walker)?;
    let semi_token = walker.expect_next_token(TokenKind::Semi)?;

    Ok(Ast::stmt_return(&return_token, expr, &semi_token))
}

pub fn parse_ty(walker: &mut TokenWalker) -> ParseResult {
    let token = walker.next();

    Ast::ty_from_token(&token)
}

/// parse expressions
pub fn parse_expr(walker: &mut TokenWalker) -> ParseResult {
    parse_add(walker)
}

/// parse addition and subtraction
pub fn parse_add(walker: &mut TokenWalker) -> ParseResult {
    parse_binary_expr(walker, parse_mul, |walker| -> Result<BinOp, ParseErr> {
        let token = walker.peek();
        match token {
            Token {
                kind: TokenKind::Plus,
                ..
            } => {
                walker.next();
                Ok(BinOp::Add)
            }
            Token {
                kind: TokenKind::Minus,
                ..
            } => {
                walker.next();
                Ok(BinOp::Sub)
            }
            _ => Err(ParseErr::unexpected_token(token.clone(), format!("+, -"))),
        }
    })
}

/// parse multiplication and division
pub fn parse_mul(walker: &mut TokenWalker) -> ParseResult {
    parse_binary_expr(walker, parse_atom, |walker| -> Result<BinOp, ParseErr> {
        let token = walker.peek();
        match token {
            Token {
                kind: TokenKind::Star,
                ..
            } => {
                walker.next();
                Ok(BinOp::Mul)
            }
            Token {
                kind: TokenKind::Slash,
                ..
            } => {
                walker.next();
                Ok(BinOp::Div)
            }
            _ => Err(ParseErr::unexpected_token(token.clone(), format!("*, /"))),
        }
    })
}

pub fn parse_atom(walker: &mut TokenWalker) -> ParseResult {
    let token = walker.next();

    if let TokenKind::UnsignedInt(_) = token.kind {
        Ast::lit_from_token(token)
    } else {
        Err(ParseErr::unexpected_token(token.clone(), "int".to_string()))
    }
}

#[test]
fn test_parse_item_let() {
    use pretty_assertions::assert_eq;

    let tokens = remove_whitespace_tokens(lex("let x: i64 = 64;"));
    let mut walker = TokenWalker::new(tokens);

    // assert_eq!(
    //     parse_program(&mut walker).unwrap(),
    //     Ast::program(vec![Ast::item_let(
    //         &Token::new(TokenKind::KeywordLet, "let".to_string(), Span::new(0, 3)),
    //             Ast::new(AstKind::LitIdent("x".to_string()), Span::new(4, 5)),
    //             false,
    //             Ast::new(AstKind::Ty { ty: Type::Int64 }, Span::new(7, 10)),
    //             Ast::new(AstKind::LitUnsignedInt(64), Span::new(13, 15))
    //         // Ast::new(AstKind::KeywordLet, "let".to_string(), Span::new(4, 5)),
    //         // false,
    //         // Ast::ty(Type::I64, Span::new(7, 10)),
    //         // Ast::lit(64, Span::new(12, 14)),
    //     )]) // Ast::program(vec![Ast::item_let(
    //         //     &Token::new(TokenKind::KeywordLet, "let".to_string(), Span::new(0, 3)),
    //         //     Ast::new(AstKind::LitIdent("x".to_string()), Span::new(7, 8)),
    //         //     false,
    //         //     Ast::new(AstKind::Ty { ty: Type::Int64 }, Span::new(10, 13)),
    //         //     Ast::new(AstKind::LitUnsignedInt(64), Span::new(16, 18))
    //         // )])
    // );
}
