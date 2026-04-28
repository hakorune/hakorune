//! Postfix handlers (catch/cleanup) utilities for unified members
use crate::ast::{ASTNode, Span};
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;
use std::collections::HashMap;

fn has_postfix(p: &NyashParser) -> bool {
    p.match_token(&TokenType::CATCH) || p.match_token(&TokenType::CLEANUP)
}

fn parse_postfix_trycatch(
    p: &mut NyashParser,
    body: Vec<ASTNode>,
    duplicate_catch_expected: &'static str,
) -> Result<Vec<ASTNode>, ParseError> {
    let mut catch_clauses: Vec<crate::ast::CatchClause> = Vec::new();
    if p.match_token(&TokenType::CATCH) {
        p.advance();
        p.consume(TokenType::LPAREN)?;
        let (exc_ty, exc_var) = p.parse_catch_param()?;
        p.consume(TokenType::RPAREN)?;
        let catch_body = p.parse_block_statements()?;
        catch_clauses.push(crate::ast::CatchClause {
            exception_type: exc_ty,
            variable_name: exc_var,
            body: catch_body,
            span: Span::unknown(),
        });
        if p.match_token(&TokenType::CATCH) {
            let line = p.current_token().line;
            return Err(ParseError::UnexpectedToken {
                found: p.current_token().token_type.clone(),
                expected: duplicate_catch_expected.to_string(),
                line,
            });
        }
    }
    let finally_body = if p.match_token(&TokenType::CLEANUP) {
        p.advance();
        Some(p.parse_block_statements()?)
    } else {
        None
    };
    Ok(vec![ASTNode::TryCatch {
        try_body: body,
        catch_clauses,
        finally_body,
        span: Span::unknown(),
    }])
}

/// If Stage-3 gate allows, parse optional catch/cleanup after a block body and wrap it.
/// Returns a (possibly) wrapped body.
pub(crate) fn wrap_with_optional_postfix(
    p: &mut NyashParser,
    body: Vec<ASTNode>,
) -> Result<Vec<ASTNode>, ParseError> {
    if !(crate::config::env::parser_stage3_enabled() && has_postfix(p)) {
        return Ok(body);
    }

    parse_postfix_trycatch(p, body, "single catch only after member body")
}

pub(crate) fn wrap_with_required_postfix(
    p: &mut NyashParser,
    body: Vec<ASTNode>,
    duplicate_catch_expected: &'static str,
) -> Result<Vec<ASTNode>, ParseError> {
    parse_postfix_trycatch(p, body, duplicate_catch_expected)
}

/// Try to parse method-level postfix catch/cleanup after the last parsed method.
/// Attaches a TryCatch wrapper around the last method body.
pub(crate) fn try_parse_method_postfix_after_last_method(
    p: &mut NyashParser,
    methods: &mut HashMap<String, ASTNode>,
    last_method_name: &Option<String>,
) -> Result<bool, ParseError> {
    if !has_postfix(p) || last_method_name.is_none() {
        return Ok(false);
    }
    let mname = last_method_name.clone().unwrap();
    let Some(mnode) = methods.get_mut(&mname) else {
        let line = p.current_token().line;
        return Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: "method body before postfix catch/cleanup".to_string(),
            line,
        });
    };
    let crate::ast::ASTNode::FunctionDeclaration { body, .. } = mnode else {
        let line = p.current_token().line;
        return Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: "function declaration before postfix catch/cleanup".to_string(),
            line,
        });
    };
    let already = body
        .iter()
        .any(|n| matches!(n, crate::ast::ASTNode::TryCatch { .. }));
    if already {
        let line = p.current_token().line;
        return Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: "duplicate postfix catch/cleanup after method".to_string(),
            line,
        });
    }
    let old = std::mem::take(body);
    *body = parse_postfix_trycatch(p, old, "single catch only after method body")?;
    Ok(true)
}
