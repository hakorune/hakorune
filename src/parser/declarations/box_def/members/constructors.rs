//! Constructors parsing (init/pack/birth)
use crate::ast::{ASTNode, Span};
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

/// Try to parse a constructor at current position.
/// Supported: `init(...) {}`, `pack(...) {}`, `birth(...) {}`.
/// Returns Ok(Some((key, node))) when a constructor was parsed and consumed.
pub(crate) fn try_parse_constructor(
    p: &mut NyashParser,
    is_override: bool,
) -> Result<Option<(String, ASTNode)>, ParseError> {
    // init(...)
    if p.match_token(&TokenType::INIT) && p.peek_token() == &TokenType::LPAREN {
        if is_override {
            return Err(ParseError::UnexpectedToken {
                expected: "method definition, not constructor after override keyword".to_string(),
                found: TokenType::INIT,
                line: p.current_token().line,
            });
        }
        let name = "init".to_string();
        let attrs = p.take_pending_runes_for_constructor()?;
        p.advance(); // consume 'init'
        p.consume(TokenType::LPAREN)?;

        // Phase 285A1.5: Use shared helper to prevent parser hangs on unsupported type annotations
        let params = crate::parser::common::params::parse_param_name_list(p, "constructor (init)")?;

        p.consume(TokenType::RPAREN)?;
        let mut body = p.parse_block_statements()?;
        // Optional postfix catch/cleanup (method-level gate)
        if p.match_token(&TokenType::CATCH) || p.match_token(&TokenType::CLEANUP) {
            body = super::postfix::wrap_with_required_postfix(
                p,
                body,
                "single catch only after method body",
            )?;
        }
        let node = ASTNode::FunctionDeclaration {
            name: name.clone(),
            params: params.clone(),
            body,
            is_static: false,
            is_override: false,
            attrs,
            span: Span::unknown(),
        };
        let key = format!("{}/{}", name, params.len());
        return Ok(Some((key, node)));
    }

    // pack(...)
    if p.match_token(&TokenType::PACK) && p.peek_token() == &TokenType::LPAREN {
        if is_override {
            return Err(ParseError::UnexpectedToken {
                expected: "method definition, not constructor after override keyword".to_string(),
                found: TokenType::PACK,
                line: p.current_token().line,
            });
        }
        let name = "pack".to_string();
        let attrs = p.take_pending_runes_for_constructor()?;
        p.advance(); // consume 'pack'
        p.consume(TokenType::LPAREN)?;

        // Phase 285A1.5: Use shared helper to prevent parser hangs on unsupported type annotations
        let params = crate::parser::common::params::parse_param_name_list(p, "constructor (pack)")?;

        p.consume(TokenType::RPAREN)?;
        let body = p.parse_block_statements()?;
        let node = ASTNode::FunctionDeclaration {
            name: name.clone(),
            params: params.clone(),
            body,
            is_static: false,
            is_override: false,
            attrs,
            span: Span::unknown(),
        };
        let key = format!("{}/{}", name, params.len());
        return Ok(Some((key, node)));
    }

    // birth(...)
    if p.match_token(&TokenType::BIRTH) && p.peek_token() == &TokenType::LPAREN {
        if is_override {
            return Err(ParseError::UnexpectedToken {
                expected: "method definition, not constructor after override keyword".to_string(),
                found: TokenType::BIRTH,
                line: p.current_token().line,
            });
        }
        let name = "birth".to_string();
        let attrs = p.take_pending_runes_for_constructor()?;
        p.advance(); // consume 'birth'
        p.consume(TokenType::LPAREN)?;

        // Phase 285A1.5: Use shared helper to prevent parser hangs on unsupported type annotations
        let params =
            crate::parser::common::params::parse_param_name_list(p, "constructor (birth)")?;

        p.consume(TokenType::RPAREN)?;
        let body = p.parse_block_statements()?;
        let node = ASTNode::FunctionDeclaration {
            name: name.clone(),
            params: params.clone(),
            body,
            is_static: false,
            is_override: false,
            attrs,
            span: Span::unknown(),
        };
        let key = format!("{}/{}", name, params.len());
        return Ok(Some((key, node)));
    }

    Ok(None)
}
