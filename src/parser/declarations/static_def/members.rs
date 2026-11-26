//! Members helpers for static box (staged)
use crate::ast::{ASTNode, Span};
use crate::parser::common::ParserUtils;
use crate::parser::declarations::box_def::members::pending_method::PendingExplicitMethodV1;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

/// `method` is a member modifier, not a token kind. Called after the member
/// loop consumed an identifier `method`; adopts the following `name (` head as
/// the real method name. Returns None when that head is absent so a lone
/// `method` keeps its existing field classification.
pub(crate) fn take_method_modifier_name(p: &mut NyashParser) -> Option<String> {
    let mut name_index = 0usize;
    while matches!(p.peek_nth_token(name_index), TokenType::NEWLINE) {
        name_index += 1;
    }
    let TokenType::IDENTIFIER(real_name) = p.peek_nth_token(name_index) else {
        return None;
    };
    let mut paren_index = name_index + 1;
    while matches!(p.peek_nth_token(paren_index), TokenType::NEWLINE) {
        paren_index += 1;
    }
    if !matches!(p.peek_nth_token(paren_index), TokenType::LPAREN) {
        return None;
    }
    let real_name = real_name.clone();
    while p.match_token(&TokenType::NEWLINE) {
        p.advance();
    }
    p.advance();
    Some(real_name)
}

/// Parse a method in a static box after consuming its name.
/// The caller owns publication into the ordered inventory.
pub(super) fn parse_method(
    p: &mut NyashParser,
    name: String,
    declaration_span: Span,
) -> Result<PendingExplicitMethodV1, ParseError> {
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
            return Err(super::static_box_state(declaration_span.line));
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
    let (return_type_name, parameter_source) = crate::parser::common::params::parse_callable_return_source(
        p, "static box method", parameter_source, false,
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
    Ok(PendingExplicitMethodV1::with_parameter_source(
        name,
        method,
        declaration_span,
        parameter_source,
    ))
}
