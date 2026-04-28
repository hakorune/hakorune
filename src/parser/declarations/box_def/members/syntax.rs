//! Shared syntax helpers for box members.
//!
//! This module owns syntax-level parsing that is common to stored fields and
//! unified member properties. Emission stays in `property_emit.rs`.

use crate::ast::{ASTNode, Span};
use crate::parser::common::ParserUtils;
use crate::parser::declarations::box_def::members::postfix;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

pub(crate) struct TypedMemberHeader {
    pub name: String,
    pub declared_type_name: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PropertyBodyPostfix {
    BlockOnly,
    ArrowOrBlock,
}

pub(crate) fn parse_optional_declared_type_name(p: &mut NyashParser) -> Option<String> {
    if let TokenType::IDENTIFIER(ty) = &p.current_token().token_type {
        let ty = Some(ty.clone());
        p.advance();
        ty
    } else {
        None
    }
}

pub(crate) fn parse_required_typed_member_header(
    p: &mut NyashParser,
    expected_name: &'static str,
    expected_colon: &'static str,
    expected_type: &'static str,
) -> Result<TypedMemberHeader, ParseError> {
    let name = parse_required_identifier(p, expected_name)?;
    let declared_type_name = parse_required_type_after_colon(p, expected_colon, expected_type)?;
    Ok(TypedMemberHeader {
        name,
        declared_type_name,
    })
}

pub(crate) fn parse_required_type_after_colon(
    p: &mut NyashParser,
    expected_colon: &'static str,
    expected_type: &'static str,
) -> Result<Option<String>, ParseError> {
    if !p.match_token(&TokenType::COLON) {
        return Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: expected_colon.to_string(),
            line: p.current_token().line,
        });
    }
    p.advance();
    Ok(Some(parse_required_declared_type_name(p, expected_type)?))
}

pub(crate) fn parse_optional_type_after_colon(
    p: &mut NyashParser,
    expected_colon: &'static str,
) -> Result<Option<String>, ParseError> {
    if !p.match_token(&TokenType::COLON) {
        return Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: expected_colon.to_string(),
            line: p.current_token().line,
        });
    }
    p.advance();
    Ok(parse_optional_declared_type_name(p))
}

pub(crate) fn try_parse_property_body(
    p: &mut NyashParser,
    postfix_policy: PropertyBodyPostfix,
) -> Result<Option<Vec<ASTNode>>, ParseError> {
    if p.match_token(&TokenType::FatArrow) {
        let body = parse_arrow_return_body(p)?;
        return match postfix_policy {
            PropertyBodyPostfix::BlockOnly => Ok(Some(body)),
            PropertyBodyPostfix::ArrowOrBlock => {
                Ok(Some(postfix::wrap_with_optional_postfix(p, body)?))
            }
        };
    }

    if p.match_token(&TokenType::LBRACE) {
        let body = p.parse_block_statements()?;
        return Ok(Some(postfix::wrap_with_optional_postfix(p, body)?));
    }

    Ok(None)
}

pub(crate) fn parse_required_property_body(
    p: &mut NyashParser,
    postfix_policy: PropertyBodyPostfix,
    expected_body: &'static str,
) -> Result<Vec<ASTNode>, ParseError> {
    if let Some(body) = try_parse_property_body(p, postfix_policy)? {
        return Ok(body);
    }
    Err(ParseError::UnexpectedToken {
        found: p.current_token().token_type.clone(),
        expected: expected_body.to_string(),
        line: p.current_token().line,
    })
}

fn parse_required_identifier(
    p: &mut NyashParser,
    expected: &'static str,
) -> Result<String, ParseError> {
    if let TokenType::IDENTIFIER(name) = &p.current_token().token_type {
        let name = name.clone();
        p.advance();
        Ok(name)
    } else {
        Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: expected.to_string(),
            line: p.current_token().line,
        })
    }
}

fn parse_required_declared_type_name(
    p: &mut NyashParser,
    expected: &'static str,
) -> Result<String, ParseError> {
    parse_optional_declared_type_name(p).ok_or_else(|| ParseError::UnexpectedToken {
        found: p.current_token().token_type.clone(),
        expected: expected.to_string(),
        line: p.current_token().line,
    })
}

fn parse_arrow_return_body(p: &mut NyashParser) -> Result<Vec<ASTNode>, ParseError> {
    p.advance();
    let expr = p.parse_expression()?;
    Ok(vec![ASTNode::Return {
        value: Some(Box::new(expr)),
        span: Span::unknown(),
    }])
}
