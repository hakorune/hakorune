/*!
 * Parameter Parsing Utilities
 *
 * Phase 285A1.5+: Common helper for parameter list parsing
 * - Supports `name` and `name: Type` forms
 * - Prevents infinite parser hangs
 */

use super::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

fn skip_newlines(p: &mut NyashParser) {
    while p.match_token(&TokenType::NEWLINE) {
        p.advance();
    }
}

/// Consume an optional `: Type` annotation in a parameter list.
///
/// Accepted type token shapes are intentionally syntax-only and conservative:
/// `Ident`, `Ident.Ident`, optional generic args `<...>`, and optional `[]`.
/// The parsed type is ignored at AST v0 (params remain `Vec<String>`).
pub(crate) fn maybe_consume_param_type_annotation(
    p: &mut NyashParser,
    context: &str,
) -> Result<(), ParseError> {
    if !p.match_token(&TokenType::COLON) {
        return Ok(());
    }
    p.advance(); // consume ':'
    skip_newlines(p);

    let mut consumed_any = false;
    let mut generic_depth: i32 = 0;
    let mut array_depth: i32 = 0;

    while !p.is_at_end() {
        match &p.current_token().token_type {
            TokenType::IDENTIFIER(_) => {
                consumed_any = true;
                p.advance();
            }
            TokenType::DOT => {
                consumed_any = true;
                p.advance();
            }
            TokenType::LESS => {
                consumed_any = true;
                generic_depth += 1;
                p.advance();
            }
            TokenType::GREATER => {
                if generic_depth <= 0 {
                    break;
                }
                consumed_any = true;
                generic_depth -= 1;
                p.advance();
            }
            TokenType::LBRACK => {
                consumed_any = true;
                array_depth += 1;
                p.advance();
            }
            TokenType::RBRACK => {
                if array_depth <= 0 {
                    return Err(ParseError::UnexpectedToken {
                        found: p.current_token().token_type.clone(),
                        expected: format!(
                            "balanced array suffix in {} parameter type annotation",
                            context
                        ),
                        line: p.current_token().line,
                    });
                }
                consumed_any = true;
                array_depth -= 1;
                p.advance();
            }
            TokenType::COMMA | TokenType::RPAREN => {
                if generic_depth == 0 && array_depth == 0 {
                    break;
                }
                return Err(ParseError::UnexpectedToken {
                    found: p.current_token().token_type.clone(),
                    expected: format!(
                        "balanced generic/array delimiters in {} parameter type annotation",
                        context
                    ),
                    line: p.current_token().line,
                });
            }
            TokenType::NEWLINE => {
                p.advance();
            }
            _ => {
                return Err(ParseError::UnexpectedToken {
                    found: p.current_token().token_type.clone(),
                    expected: format!("type name in {} parameter type annotation", context),
                    line: p.current_token().line,
                });
            }
        }
    }

    if !consumed_any {
        return Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: format!("type name after ':' in {} parameter list", context),
            line: p.current_token().line,
        });
    }
    if generic_depth != 0 || array_depth != 0 {
        return Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: format!(
                "closed generic/array type annotation in {} parameter list",
                context
            ),
            line: p.current_token().line,
        });
    }
    Ok(())
}

/// Parse parameter name list with Fail-Fast on unexpected tokens
///
/// Parses: IDENT (':' TYPE)? (',' IDENT (':' TYPE)? )*
/// Rejects: unexpected tokens, malformed comma sequences, malformed type annotation
///
/// # Arguments
/// * `p` - Mutable reference to NyashParser
/// * `context` - Context string for error messages ("method", "constructor", "function", etc.)
///
/// # Returns
/// * `Ok(Vec<String>)` - List of parameter names
/// * `Err(ParseError)` - Parse error with context-aware message
///
/// # Features
/// * Progress-zero detection: Tracks token position, errors if stuck
/// * Explicit token handling: All token types explicitly matched
/// * Fail-Fast: Either advances or errors (no infinite loop possible)
/// * Unified error messages: Single source of truth for error text
pub(crate) fn parse_param_name_list(
    p: &mut NyashParser,
    context: &str,
) -> Result<Vec<String>, ParseError> {
    let mut params = Vec::new();
    let mut last_token_position: Option<(usize, usize)> = None;

    while !p.match_token(&TokenType::RPAREN) && !p.is_at_end() {
        skip_newlines(p);
        if p.match_token(&TokenType::RPAREN) {
            break;
        }

        // Progress-zero detection: same position twice → infinite loop risk
        let current_position = p.current_position();
        if let Some(last_pos) = last_token_position {
            if current_position == last_pos {
                return Err(ParseError::InfiniteLoop {
                    location: format!("{} parameter list", context),
                    token: p.current_token().token_type.clone(),
                    line: p.current_token().line,
                });
            }
        }
        last_token_position = Some(current_position);

        match &p.current_token().token_type {
            TokenType::IDENTIFIER(param) => {
                params.push(param.clone());
                p.advance();
                maybe_consume_param_type_annotation(p, context)?;

                skip_newlines(p);

                if p.match_token(&TokenType::COMMA) {
                    p.advance();
                    skip_newlines(p);
                } else if !p.match_token(&TokenType::RPAREN) {
                    return Err(ParseError::UnexpectedToken {
                        found: p.current_token().token_type.clone(),
                        expected: format!("',' or ')' in {} parameter list", context),
                        line: p.current_token().line,
                    });
                }
            }
            TokenType::NEWLINE => {
                p.advance();
            }
            TokenType::COMMA => {
                return Err(ParseError::UnexpectedToken {
                    found: TokenType::COMMA,
                    expected: format!("parameter name in {} parameter list", context),
                    line: p.current_token().line,
                });
            }
            _ => {
                return Err(ParseError::UnexpectedToken {
                    found: p.current_token().token_type.clone(),
                    expected: format!("parameter name or ')' in {} parameter list", context),
                    line: p.current_token().line,
                });
            }
        }
    }

    Ok(params)
}
