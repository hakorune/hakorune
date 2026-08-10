//! Members helpers for static box (staged)
use crate::ast::{ASTNode, Span};
use crate::parser::common::ParserUtils;
use crate::parser::declarations::box_def::members::pending_method::PendingExplicitMethodV1;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

pub(crate) enum ParsedStaticMemberV1 {
    Field(String),
    Method(PendingExplicitMethodV1),
}

/// Parse a `static { ... }` initializer if present, honoring STRICT gate behavior.
/// Returns Ok(Some(body)) when consumed; Ok(None) otherwise.
pub(crate) fn parse_static_initializer_if_any(
    p: &mut NyashParser,
) -> Result<Option<Vec<ASTNode>>, ParseError> {
    if !p.match_token(&TokenType::STATIC) {
        return Ok(None);
    }
    let strict = crate::parser::env::parser_static_init_strict_enabled();
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

/// Parse either a method or a field in static box after consuming an identifier `name`.
/// The caller owns publication into the ordered inventory.
pub(crate) fn try_parse_method_or_field(
    p: &mut NyashParser,
    name: String,
    declaration_span: Span,
) -> Result<ParsedStaticMemberV1, ParseError> {
    let trace = crate::parser::env::parser_static_trace_enabled();
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
                crate::parser::log::debug(&format!(
                    "[parser][static-box] field detected: {}",
                    name
                ));
            }
            return Ok(ParsedStaticMemberV1::Field(name));
        }
    }
    if trace {
        crate::parser::log::debug(&format!(
            "[parser][static-box] method head detected: {}(..)",
            name
        ));
    }
    // Method
    let attrs = p.take_pending_runes_for_static_box_method()?;
    p.advance(); // consume '('
    let parameter_source =
        crate::parser::common::params::parse_param_decl_list_product(p, "static box method")?;
    let param_decls = parameter_source.neutral().to_vec();
    let params = crate::ast::ParamDecl::names(parameter_source.neutral());
    p.consume(TokenType::RPAREN)?;
    let return_type_name = crate::parser::common::params::parse_optional_return_type_annotation(
        p,
        "static box method",
    )?;
    let (uses, contracts) = p.parse_signature_metadata_until_body()?;
    // Allow NEWLINE(s) between ')' and '{' of method body
    while p.match_token(&TokenType::NEWLINE) {
        p.advance();
    }
    // Parse method body; optionally use strict method-body guard when enabled
    let body = if crate::parser::env::parser_method_body_strict_enabled() {
        p.parse_method_body_statements()?
    } else {
        p.parse_block_statements()?
    };
    // Construct method node
    let method = ASTNode::FunctionDeclaration {
        name: name.clone(),
        params,
        param_decls,
        return_type_name,
        body,
        contracts,
        uses,
        // Methods inside a static box are semantically static
        is_static: true,
        is_override: false,
        attrs,
        span: declaration_span,
    };
    let mut method = method;
    p.attach_pending_runes_to_declaration(&mut method)?;
    Ok(ParsedStaticMemberV1::Method(
        PendingExplicitMethodV1::with_parameter_source(
            name,
            method,
            declaration_span,
            parameter_source,
        ),
    ))
}
