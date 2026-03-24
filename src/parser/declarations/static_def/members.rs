//! Members helpers for static box (staged)
use crate::ast::ASTNode;
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;
use std::collections::HashMap;

/// Parse a `static { ... }` initializer if present, honoring STRICT gate behavior.
/// Returns Ok(Some(body)) when consumed; Ok(None) otherwise.
pub(crate) fn parse_static_initializer_if_any(
    p: &mut NyashParser,
) -> Result<Option<Vec<ASTNode>>, ParseError> {
    if !p.match_token(&TokenType::STATIC) {
        return Ok(None);
    }
    let strict = std::env::var("NYASH_PARSER_STATIC_INIT_STRICT")
        .ok()
        .as_deref()
        == Some("1");
    if strict {
        match p.peek_token() {
            TokenType::LBRACE => {
                p.advance(); // consume 'static'
                let body = p.parse_block_statements()?;
                return Ok(Some(body));
            }
            TokenType::BOX | TokenType::FUNCTION => {
                // top-level seam: do not consume, let caller close the box
                return Ok(None);
            }
            _ => {
                // backward-compatible fallback: treat as initializer
                p.advance();
                let body = p.parse_block_statements()?;
                return Ok(Some(body));
            }
        }
    } else {
        p.advance(); // consume 'static'
        let body = p.parse_block_statements()?;
        Ok(Some(body))
    }
}

/// Parse a method body and apply optional postfix catch/cleanup inline (Stage‑3 gate).
/// Caller must have consumed `(` and collected `params` and parsed `body`.
pub(crate) fn wrap_method_body_with_postfix_if_any(
    p: &mut NyashParser,
    body: Vec<ASTNode>,
) -> Result<Vec<ASTNode>, ParseError> {
    crate::parser::declarations::box_def::members::postfix::wrap_with_optional_postfix(p, body)
}

/// Parse either a method or a field in static box after consuming an identifier `name`.
/// - If next token is `(`, parses a method with optional postfix and inserts into `methods`.
/// - Otherwise, treats as a field name and pushes into `fields`.
pub(crate) fn try_parse_method_or_field(
    p: &mut NyashParser,
    name: String,
    methods: &mut HashMap<String, ASTNode>,
    fields: &mut Vec<String>,
    last_method_name: &mut Option<String>,
) -> Result<bool, ParseError> {
    let trace = std::env::var("NYASH_PARSER_TRACE_STATIC").ok().as_deref() == Some("1");
    // Allow NEWLINE(s) between identifier and '('
    if !p.match_token(&TokenType::LPAREN) {
        // Lookahead skipping NEWLINE to see if a '(' follows → treat as method head
        let mut k = 0usize;
        while matches!(p.peek_nth_token(k), TokenType::NEWLINE) {
            k += 1;
        }
        if matches!(p.peek_nth_token(k), TokenType::LPAREN) {
            // Consume intervening NEWLINEs so current becomes '('
            while p.match_token(&TokenType::NEWLINE) {
                p.advance();
            }
        } else {
            p.ensure_no_pending_runes("field")?;
            if trace {
                crate::runtime::get_global_ring0()
                    .log
                    .debug(&format!("[parser][static-box] field detected: {}", name));
            }
            // Field
            fields.push(name);
            return Ok(true);
        }
    }
    if trace {
        crate::runtime::get_global_ring0().log.debug(&format!(
            "[parser][static-box] method head detected: {}(..)",
            name
        ));
    }
    // Method
    let attrs = p.take_pending_runes_for_static_box_method()?;
    p.advance(); // consume '('
    let mut params = Vec::new();
    while !p.match_token(&TokenType::RPAREN) && !p.is_at_end() {
        // Peek at the current token to ensure forward progress
        match p.current_token().token_type {
            TokenType::IDENTIFIER(ref param) => {
                params.push(param.clone());
                p.advance();
                crate::parser::common::params::maybe_consume_param_type_annotation(
                    p,
                    "static box method",
                )?;
            }
            TokenType::COMMA => {
                p.advance();
            }
            TokenType::NEWLINE => {
                // Stage-3 tolerant mode: allow newlines in parameter list
                p.advance();
            }
            TokenType::RPAREN => {
                break;
            }
            _ => {
                // Unexpected token handling
                if std::env::var("NYASH_PARSER_METHOD_PARAM_STRICT")
                    .ok()
                    .as_deref()
                    == Some("1")
                {
                    return Err(ParseError::UnexpectedToken {
                        found: p.current_token().token_type.clone(),
                        expected:
                            "parameter identifier (optional ': Type'), comma, newline, or ')'"
                                .to_string(),
                        line: p.current_token().line,
                    });
                } else {
                    // Tolerant mode: skip unexpected token to avoid infinite loop
                    p.advance();
                }
            }
        }
    }
    p.consume(TokenType::RPAREN)?;
    // Allow NEWLINE(s) between ')' and '{' of method body
    while p.match_token(&TokenType::NEWLINE) {
        p.advance();
    }
    // Parse method body; optionally use strict method-body guard when enabled
    let body = if std::env::var("NYASH_PARSER_METHOD_BODY_STRICT")
        .ok()
        .as_deref()
        == Some("1")
    {
        p.parse_method_body_statements()?
    } else {
        p.parse_block_statements()?
    };
    let body = wrap_method_body_with_postfix_if_any(p, body)?;
    // Construct method node
    let method = ASTNode::FunctionDeclaration {
        name: name.clone(),
        params,
        body,
        // Methods inside a static box are semantically static
        is_static: true,
        is_override: false,
        attrs,
        span: crate::ast::Span::unknown(),
    };
    let mut method = method;
    p.attach_pending_runes_to_declaration(&mut method)?;
    *last_method_name = Some(name.clone());
    methods.insert(name, method);
    Ok(true)
}
