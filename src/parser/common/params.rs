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

#[derive(Clone, Copy)]
enum TypeAnnotationSite {
    Parameter,
    Return,
}

impl TypeAnnotationSite {
    fn label(self) -> &'static str {
        match self {
            TypeAnnotationSite::Parameter => "parameter",
            TypeAnnotationSite::Return => "return",
        }
    }

    fn empty_expected(self, context: &str) -> String {
        match self {
            TypeAnnotationSite::Parameter => {
                format!("type name after ':' in {} parameter list", context)
            }
            TypeAnnotationSite::Return => {
                format!("type name after ':' in {} return annotation", context)
            }
        }
    }

    fn closed_expected(self, context: &str) -> String {
        match self {
            TypeAnnotationSite::Parameter => {
                format!(
                    "closed generic/array type annotation in {} parameter list",
                    context
                )
            }
            TypeAnnotationSite::Return => {
                format!("closed generic/array return type annotation in {}", context)
            }
        }
    }
}

fn parse_optional_type_annotation(
    p: &mut NyashParser,
    context: &str,
    site: TypeAnnotationSite,
) -> Result<Option<String>, ParseError> {
    if !p.match_token(&TokenType::COLON) {
        return Ok(None);
    }
    p.advance(); // consume ':'
    skip_newlines(p);

    let mut consumed_any = false;
    let mut generic_depth: i32 = 0;
    let mut array_depth: i32 = 0;
    let mut type_text = String::new();

    while !p.is_at_end() {
        match &p.current_token().token_type {
            TokenType::IDENTIFIER(name) => {
                if matches!(site, TypeAnnotationSite::Return)
                    && consumed_any
                    && generic_depth == 0
                    && array_depth == 0
                    && (name == "requires" || name == "ensures")
                {
                    break;
                }
                consumed_any = true;
                type_text.push_str(name);
                p.advance();
            }
            TokenType::DOT => {
                consumed_any = true;
                type_text.push('.');
                p.advance();
            }
            TokenType::LESS => {
                consumed_any = true;
                generic_depth += 1;
                type_text.push('<');
                p.advance();
            }
            TokenType::GREATER => {
                if generic_depth <= 0 {
                    break;
                }
                consumed_any = true;
                generic_depth -= 1;
                type_text.push('>');
                p.advance();
            }
            TokenType::LBRACK => {
                consumed_any = true;
                array_depth += 1;
                type_text.push('[');
                p.advance();
            }
            TokenType::RBRACK => {
                if array_depth <= 0 {
                    return Err(ParseError::UnexpectedToken {
                        found: p.current_token().token_type.clone(),
                        expected: format!(
                            "balanced array suffix in {} {} type annotation",
                            context,
                            site.label()
                        ),
                        line: p.current_token().line,
                    });
                }
                consumed_any = true;
                array_depth -= 1;
                type_text.push(']');
                p.advance();
            }
            TokenType::COMMA => {
                if generic_depth == 0 && array_depth == 0 {
                    match site {
                        TypeAnnotationSite::Parameter => break,
                        TypeAnnotationSite::Return => {
                            return Err(ParseError::UnexpectedToken {
                                found: p.current_token().token_type.clone(),
                                expected: format!(
                                    "method body after {} return type annotation",
                                    context
                                ),
                                line: p.current_token().line,
                            });
                        }
                    }
                }
                consumed_any = true;
                type_text.push(',');
                p.advance();
            }
            TokenType::RPAREN => {
                if matches!(site, TypeAnnotationSite::Parameter)
                    && generic_depth == 0
                    && array_depth == 0
                {
                    break;
                }
                return Err(ParseError::UnexpectedToken {
                    found: p.current_token().token_type.clone(),
                    expected: format!(
                        "balanced generic/array delimiters in {} {} type annotation",
                        context,
                        site.label()
                    ),
                    line: p.current_token().line,
                });
            }
            TokenType::LBRACE | TokenType::RBRACE | TokenType::SEMICOLON
                if matches!(site, TypeAnnotationSite::Return) =>
            {
                if generic_depth == 0 && array_depth == 0 {
                    break;
                }
                return Err(ParseError::UnexpectedToken {
                    found: p.current_token().token_type.clone(),
                    expected: format!(
                        "balanced generic/array delimiters in {} return type annotation",
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
                    expected: format!("type name in {} {} type annotation", context, site.label()),
                    line: p.current_token().line,
                });
            }
        }
    }

    if !consumed_any {
        return Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: site.empty_expected(context),
            line: p.current_token().line,
        });
    }
    if generic_depth != 0 || array_depth != 0 {
        return Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: site.closed_expected(context),
            line: p.current_token().line,
        });
    }
    Ok(Some(type_text))
}

pub(crate) fn parse_optional_param_type_annotation(
    p: &mut NyashParser,
    context: &str,
) -> Result<Option<String>, ParseError> {
    parse_optional_type_annotation(p, context, TypeAnnotationSite::Parameter)
}

pub(crate) fn parse_optional_return_type_annotation(
    p: &mut NyashParser,
    context: &str,
) -> Result<Option<String>, ParseError> {
    parse_optional_type_annotation(p, context, TypeAnnotationSite::Return)
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
/// * `Ok(Vec<ParamDecl>)` - Parameter names plus preserved source type metadata
/// * `Err(ParseError)` - Parse error with context-aware message
///
/// # Features
/// * Progress-zero detection: Tracks token position, errors if stuck
/// * Explicit token handling: All token types explicitly matched
/// * Fail-Fast: Either advances or errors (no infinite loop possible)
/// * Unified error messages: Single source of truth for error text
pub(crate) fn parse_param_decl_list(
    p: &mut NyashParser,
    context: &str,
) -> Result<Vec<crate::ast::ParamDecl>, ParseError> {
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
                let name = param.clone();
                p.advance();
                let declared_type_name = parse_optional_param_type_annotation(p, context)?;
                params.push(crate::ast::ParamDecl {
                    name,
                    declared_type_name,
                });

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
