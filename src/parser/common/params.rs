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
    let mut inactive_ownership_result_prefix = false;

    while !p.is_at_end() {
        match &p.current_token().token_type {
            TokenType::IDENTIFIER(name) => {
                if matches!(site, TypeAnnotationSite::Return)
                    && inactive_ownership_result_prefix
                    && generic_depth == 0
                    && array_depth == 0
                {
                    return Err(ParseError::UnexpectedToken {
                        found: p.current_token().token_type.clone(),
                        expected: "[freeze:contract][parser/ownership_syntax_inactive] inactive ownership result syntax is not a type annotation".to_string(),
                        line: p.current_token().line,
                    });
                }
                if matches!(site, TypeAnnotationSite::Return)
                    && consumed_any
                    && generic_depth == 0
                    && array_depth == 0
                    && (name == "requires" || name == "ensures" || name == "uses")
                {
                    break;
                }
                consumed_any = true;
                inactive_ownership_result_prefix = matches!(site, TypeAnnotationSite::Return)
                    && type_text.is_empty()
                    && (name == "view" || name == "share");
                type_text.push_str(name);
                p.advance();
            }
            TokenType::VOID => {
                consumed_any = true;
                type_text.push_str("void");
                p.advance();
            }
            TokenType::DOT => {
                consumed_any = true;
                inactive_ownership_result_prefix = false;
                type_text.push('.');
                p.advance();
            }
            TokenType::LESS => {
                consumed_any = true;
                inactive_ownership_result_prefix = false;
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
            TokenType::ShiftRight => {
                if generic_depth < 2 {
                    return Err(ParseError::UnexpectedToken {
                        found: p.current_token().token_type.clone(),
                        expected: format!(
                            "balanced generic delimiters in {} {} type annotation",
                            context,
                            site.label()
                        ),
                        line: p.current_token().line,
                    });
                }
                consumed_any = true;
                generic_depth -= 2;
                type_text.push_str(">>");
                p.advance();
            }
            TokenType::LBRACK => {
                consumed_any = true;
                inactive_ownership_result_prefix = false;
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
            TokenType::QUESTION => {
                consumed_any = true;
                type_text.push('?');
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
            TokenType::FROM if matches!(site, TypeAnnotationSite::Return)
                && generic_depth == 0 && array_depth == 0 => break,
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
    let result = parse_optional_type_annotation(p, context, TypeAnnotationSite::Return)?;
    if let Some(spelling) = &result {
        crate::ast::TypeReferenceV1::parse(spelling).map_err(|detail| ParseError::GrammarContract {
            stable_reject_tag: "parser/type-reference", detail, line: p.current_token().line,
        })?;
    }
    if p.match_token(&TokenType::FROM) {
        return Err(crate::parser::callable_parameter_source::source_contract_required(true,
            "from requires a retained callable header", p.current_token().line));
    }
    Ok(result)
}

/// Parse the return part into the same one-shot header product as parameters.
pub(crate) fn parse_callable_return_source(
    p: &mut NyashParser,
    context: &str,
    parameters: crate::parser::callable_parameter_source::ParsedCallableParameterListV1,
    receiver_available: bool,
) -> Result<(Option<String>, crate::parser::callable_parameter_source::ParsedCallableParameterListV1), ParseError> {
    use crate::parser::callable_parameter_source::{ParserBorrowedResultSyntaxV1 as ResultSyntax,
        ParserBorrowedResultSupportSyntaxV1 as Support};
    let result = parse_optional_type_annotation(p, context, TypeAnnotationSite::Return)?;
    let parameters = parameters.with_declared_result_type(result.as_deref()).map_err(|detail| ParseError::GrammarContract {
        stable_reject_tag: "parser/type-reference", detail, line: p.current_token().line,
    })?;
    if !p.match_token(&TokenType::FROM) { return Ok((result, parameters)); }
    let line = p.current_token().line;
    let fail = |detail: &str| ParseError::GrammarContract {
        stable_reject_tag: "parser/from-support-unavailable", detail: detail.into(), line,
    };
    let declared_type = result.clone().ok_or_else(|| fail("from requires a declared result type"))?;
    p.advance();
    skip_newlines(p);
    let support = match &p.current_token().token_type {
        TokenType::ME if receiver_available => Support::Receiver,
        TokenType::IDENTIFIER(name) => {
            let mut matches = parameters.neutral().iter().enumerate().filter(|(_, parameter)| parameter.name == *name);
            let (ordinal, _) = matches.next().ok_or_else(|| fail("from must name the receiver or one declared parameter"))?;
            if matches.next().is_some() { return Err(fail("from parameter is ambiguous")); }
            if parameters.parameter_is_take(ordinal) { return Err(fail("a take parameter cannot support a returned read borrow")); }
            Support::Parameter { ordinal: ordinal as u32, name: name.clone().into_boxed_str() }
        }
        _ => return Err(fail("from must name an available receiver or one read parameter")),
    };
    p.advance();
    skip_newlines(p);
    if matches!(p.current_token().token_type, TokenType::COMMA | TokenType::FROM
        | TokenType::DOT | TokenType::LPAREN) {
        return Err(fail("from requires exactly one receiver or parameter, not a projection or call"));
    }
    Ok((result, parameters.with_borrowed_result(Some(ResultSyntax::parsed(declared_type, support)))))
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
    let product = parse_param_decl_list_product(p, context)?;
    if product.has_modified_parameters() {
        return Err(ParseError::GrammarContract {
            stable_reject_tag: "parser/take-source-contract-required",
            detail: format!("take requires a retained callable parameter source in {context}"),
            line: p.current_token().line,
        });
    }
    Ok(product.neutral().to_vec())
}

/// Parse once into a neutral AST projection plus one-shot transfer source
/// rows. Callers outside the admitted direct-method cohort discard the source
/// rows; they may not reconstruct `Ordinary` from the neutral projection.
pub(in crate::parser) fn parse_param_decl_list_product(
    p: &mut NyashParser,
    context: &str,
) -> Result<crate::parser::callable_parameter_source::ParsedCallableParameterListV1, ParseError> {
    let mut params = Vec::new();
    let mut transfers = Vec::new();
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
                let modified = matches!(param.as_str(), "take" | "edit") && p.tokens.get(p.current + 1).is_some_and(|next| {
                    next.line == p.current_token().line && matches!(next.token_type, TokenType::IDENTIFIER(_))
                });
                let transfer = if modified {
                    if param == "edit" { crate::parser::callable_parameter_source::ParserParameterTransferSyntaxV1::edit() }
                    else { crate::parser::callable_parameter_source::ParserParameterTransferSyntaxV1::from_take(true) }
                } else { crate::parser::callable_parameter_source::ParserParameterTransferSyntaxV1::ordinary() };
                if modified { p.advance(); }
                let TokenType::IDENTIFIER(name) = p.current_token().token_type.clone() else {
                    unreachable!("parameter head checked above");
                };
                let name_line = p.current_token().line;
                let typed_head = !modified || p.tokens.get(p.current + 1).is_some_and(|next| {
                    next.line == name_line && matches!(next.token_type, TokenType::COLON)
                });
                p.advance();
                if !typed_head {
                    return Err(ParseError::GrammarContract {
                        stable_reject_tag: if transfer.is_take() { "parser/take-declared-type-required" } else { "parser/edit-declared-type-required" },
                        detail: "ownership parameter requires a same-line declared type".into(),
                        line: name_line,
                    });
                }
                let declared_type_name = parse_optional_param_type_annotation(p, context)?;
                if modified && declared_type_name.is_none() {
                    return Err(ParseError::GrammarContract {
                        stable_reject_tag: if transfer.is_take() { "parser/take-declared-type-required" } else { "parser/edit-declared-type-required" },
                        detail: "ownership parameter requires an explicit declared type".into(),
                        line: p.current_token().line,
                    });
                }
                transfers.push(transfer);
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

    crate::parser::callable_parameter_source::ParsedCallableParameterListV1::from_parsed(params, transfers)
        .map_err(|error| ParseError::GrammarContract {
            stable_reject_tag: "parser/callable-parameter-list",
            detail: format!("callable parameter list rejected: {error:?}"),
            line: p.current_token().line,
        })
}
