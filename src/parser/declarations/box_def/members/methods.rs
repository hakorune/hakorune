//! Methods parsing (name(params){ body })
use crate::ast::{ASTNode, Span};
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

/// Try to parse a method declaration starting at `method_name` (already consumed identifier).
/// Returns Some(method_node) when parsed; None when not applicable (i.e., next token is not '(').
pub(crate) fn try_parse_method(
    p: &mut NyashParser,
    method_name: String,
    is_override: bool,
) -> Result<Option<ASTNode>, ParseError> {
    if !p.match_token(&TokenType::LPAREN) {
        return Ok(None);
    }
    let attrs = p.take_pending_runes_for_instance_method()?;
    p.advance(); // consume '('

    // Phase 285A1.5: Use shared helper to prevent parser hangs on unsupported type annotations
    let params = crate::parser::common::params::parse_param_name_list(p, "method")?;
    p.consume(TokenType::RPAREN)?;
    let body = p.parse_block_statements()?;

    let method = ASTNode::FunctionDeclaration {
        name: method_name.clone(),
        params,
        body,
        is_static: false,
        is_override,
        attrs,
        span: Span::unknown(),
    };
    Ok(Some(method))
}
