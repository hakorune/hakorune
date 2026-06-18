use crate::ast::Span;
use crate::parser::common::ParserUtils;
use crate::parser::declarations::box_def::state::BoxMemberState;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

/// Thin wrappers to keep the main loop tidy (behavior-preserving)
fn box_try_block_first_property(
    p: &mut NyashParser,
    state: &mut BoxMemberState,
) -> Result<bool, ParseError> {
    if !p.match_token(&TokenType::LBRACE) {
        return Ok(false);
    }
    p.ensure_no_pending_runes("block-first property")?;
    crate::parser::declarations::box_def::members::properties::try_parse_block_first_property(
        p,
        &mut state.methods,
        &mut state.birth_once_props,
    )
}

fn box_try_method_postfix_after_last(
    p: &mut NyashParser,
    state: &mut BoxMemberState,
) -> Result<bool, ParseError> {
    if state.last_method_name.is_none()
        || !(p.match_token(&TokenType::CATCH) || p.match_token(&TokenType::CLEANUP))
    {
        return Ok(false);
    }
    p.ensure_no_pending_runes("method postfix")?;
    crate::parser::declarations::box_def::members::postfix::try_parse_method_postfix_after_last_method(
        p,
        &mut state.methods,
        &state.last_method_name,
    )
}

fn box_try_init_block(p: &mut NyashParser, state: &mut BoxMemberState) -> Result<bool, ParseError> {
    if !(p.match_token(&TokenType::INIT) && p.peek_token() != &TokenType::LPAREN) {
        return Ok(false);
    }
    p.ensure_no_pending_runes("init block")?;
    crate::parser::declarations::box_def::members::fields::parse_init_block_if_any(
        p,
        &mut state.init_fields,
        &mut state.weak_fields,
    )
}

fn box_try_delegate(p: &mut NyashParser, state: &mut BoxMemberState) -> Result<bool, ParseError> {
    if !p.match_token(&TokenType::DELEGATE) {
        return Ok(false);
    }
    p.ensure_no_pending_runes("delegate declaration")?;
    state
        .delegates
        .push(crate::parser::declarations::box_def::members::delegates::parse_delegate_decl(p)?);
    Ok(true)
}

fn box_try_transition(p: &mut NyashParser, state: &mut BoxMemberState) -> Result<bool, ParseError> {
    if let Some(transition) =
        crate::parser::declarations::box_def::members::transitions::try_parse_transition_decl(p)?
    {
        p.ensure_no_pending_runes("transition declaration")?;
        state.transitions.push(transition);
        return Ok(true);
    }
    Ok(false)
}

fn box_try_constructor(
    p: &mut NyashParser,
    is_override: bool,
    state: &mut BoxMemberState,
) -> Result<bool, ParseError> {
    if let Some((key, node)) =
        crate::parser::declarations::box_def::members::constructors::try_parse_constructor(
            p,
            is_override,
        )?
    {
        let mut node = node;
        p.attach_pending_runes_to_declaration(&mut node)?;
        state.constructors.insert(key, node);
        return Ok(true);
    }
    Ok(false)
}

fn box_try_visibility(
    p: &mut NyashParser,
    visibility: &str,
    state: &mut BoxMemberState,
) -> Result<bool, ParseError> {
    if visibility != "public" && visibility != "private" {
        return Ok(false);
    }
    p.ensure_no_pending_runes("visibility field/property")?;
    crate::parser::declarations::box_def::members::fields::try_parse_visibility_block_or_single(
        p,
        visibility,
        &mut state.methods,
        &mut state.fields,
        &mut state.field_decls,
        &mut state.field_initializers,
        &mut state.public_fields,
        &mut state.private_fields,
        &mut state.last_method_name,
        &mut state.weak_fields,
    )
}

fn box_try_build_gate_members(
    p: &mut NyashParser,
    state: &mut BoxMemberState,
) -> Result<bool, ParseError> {
    if !p.is_build_gate_head() {
        return Ok(false);
    }

    let line = p.current_token().line;
    p.consume_build_gate_head()?;
    let predicate = p.parse_build_predicate()?;

    let then_state = parse_box_member_gate_block(p)?;
    let else_state = if p.match_token(&TokenType::ELSE) {
        p.advance();
        Some(if p.is_build_gate_head() {
            parse_box_member_gate_group(p)?
        } else {
            parse_box_member_gate_block(p)?
        })
    } else {
        None
    };

    let selected_then = p.eval_build_predicate(&predicate, Span::new(0, 0, line, 1))?;
    let then_signature = then_state.signature();
    let else_signature = else_state.as_ref().map(BoxMemberState::signature);

    match &else_signature {
        Some(sig) if then_signature != *sig => {
            return Err(ParseError::BuildCfg {
                message: "member-level gate branches must preserve the same public signature"
                    .to_string(),
                line,
            });
        }
        None if !then_signature.is_empty() => {
            return Err(ParseError::BuildCfg {
                message:
                    "member-level gate inside box bodies requires an else branch with the same public signature"
                        .to_string(),
                line,
            });
        }
        _ => {}
    }

    let selected_state = if selected_then {
        then_state
    } else if let Some(else_state) = else_state {
        else_state
    } else {
        BoxMemberState::default()
    };

    state.merge_from(selected_state);
    state.last_method_name = None;
    Ok(true)
}

fn parse_box_member_gate_block(p: &mut NyashParser) -> Result<BoxMemberState, ParseError> {
    p.consume(TokenType::LBRACE)?;
    let mut state = BoxMemberState::default();
    parse_box_member_body(p, &mut state)?;
    p.consume(TokenType::RBRACE)?;
    Ok(state)
}

fn parse_box_member_gate_group(p: &mut NyashParser) -> Result<BoxMemberState, ParseError> {
    if !p.is_build_gate_head() {
        return Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: "gate".to_string(),
            line: p.current_token().line,
        });
    }
    let line = p.current_token().line;
    p.consume_build_gate_head()?;
    let predicate = p.parse_build_predicate()?;
    let then_state = parse_box_member_gate_block(p)?;
    let else_state = if p.match_token(&TokenType::ELSE) {
        p.advance();
        Some(if p.is_build_gate_head() {
            parse_box_member_gate_group(p)?
        } else {
            parse_box_member_gate_block(p)?
        })
    } else {
        None
    };
    let selected_then = p.eval_build_predicate(&predicate, Span::new(0, 0, line, 1))?;
    let then_signature = then_state.signature();
    let else_signature = else_state.as_ref().map(BoxMemberState::signature);

    match &else_signature {
        Some(sig) if then_signature != *sig => {
            return Err(ParseError::BuildCfg {
                message: "member-level gate branches must preserve the same public signature"
                    .to_string(),
                line,
            });
        }
        None if !then_signature.is_empty() => {
            return Err(ParseError::BuildCfg {
                message:
                    "member-level gate inside box bodies requires an else branch with the same public signature"
                        .to_string(),
                line,
            });
        }
        _ => {}
    }

    Ok(if selected_then {
        then_state
    } else if let Some(else_state) = else_state {
        else_state
    } else {
        BoxMemberState::default()
    })
}

pub(crate) fn parse_box_member_body(
    p: &mut NyashParser,
    state: &mut BoxMemberState,
) -> Result<(), ParseError> {
    while !p.match_token(&TokenType::RBRACE) && !p.is_at_end() {
        if p.maybe_parse_opt_annotation_noop(
            crate::parser::statements::helpers::AnnotationSite::Member,
        )? {
            continue;
        }

        if box_try_block_first_property(p, state)? {
            continue;
        }

        if box_try_method_postfix_after_last(p, state)? {
            continue;
        }

        if p.match_token(&TokenType::RBRACE) {
            break;
        }

        if box_try_init_block(p, state)? {
            continue;
        }

        if box_try_delegate(p, state)? {
            state.last_method_name = None;
            continue;
        }

        if box_try_transition(p, state)? {
            state.last_method_name = None;
            continue;
        }

        if let Some(invariant) = p.try_parse_invariant_clause()? {
            state.invariants.push(invariant);
            state.last_method_name = None;
            continue;
        }

        if box_try_build_gate_members(p, state)? {
            continue;
        }

        let mut is_override = false;
        if p.match_token(&TokenType::OVERRIDE) {
            is_override = true;
            p.advance();
        }

        if box_try_constructor(p, is_override, state)? {
            continue;
        }

        if p.match_token(&TokenType::WEAK) {
            p.ensure_no_pending_runes("weak field")?;
            p.advance();
            if let TokenType::IDENTIFIER(field_name) = &p.current_token().token_type {
                let field_name = field_name.clone();
                p.advance();
                crate::parser::declarations::box_def::members::fields::parse_weak_field(
                    p,
                    field_name,
                    &mut state.methods,
                    &mut state.fields,
                    &mut state.field_decls,
                    &mut state.weak_fields,
                )?;
                continue;
            } else {
                return Err(ParseError::UnexpectedToken {
                    expected: "field name after 'weak'".to_string(),
                    found: p.current_token().token_type.clone(),
                    line: p.current_token().line,
                });
            }
        }

        if let TokenType::IDENTIFIER(field_or_method) = &p.current_token().token_type {
            let field_or_method = field_or_method.clone();
            let field_or_method_line = p.current_token().line;
            p.advance();

            if box_try_visibility(p, &field_or_method, state)? {
                continue;
            }

            if crate::parser::env::unified_members() && field_or_method == "get" {
                if let Some(_property_name) =
                    crate::parser::declarations::box_def::members::fields::try_parse_get_computed_property(
                        p,
                        field_or_method_line,
                        &mut state.methods,
                    )?
                {
                    p.ensure_no_pending_runes("get property")?;
                    state.last_method_name = None;
                    continue;
                }
            }

            if crate::parser::env::unified_members()
                && (field_or_method == "once" || field_or_method == "birth_once")
            {
                p.ensure_no_pending_runes("unified property")?;
                if crate::parser::declarations::box_def::members::properties::try_parse_unified_property(
                    p,
                    &field_or_method,
                    &mut state.methods,
                    &mut state.birth_once_props,
                )? {
                    state.last_method_name = None;
                    continue;
                }
            }

            if box_try_method_or_field(p, field_or_method, is_override, state)? {
                continue;
            }
        } else {
            return Err(ParseError::UnexpectedToken {
                expected: "method or field name".to_string(),
                found: p.current_token().token_type.clone(),
                line: p.current_token().line,
            });
        }
    }
    Ok(())
}

/// Parse either a method or a header-first field/property starting with `name`.
/// Updates `methods`/`fields` and `last_method_name` as appropriate.
fn box_try_method_or_field(
    p: &mut NyashParser,
    name: String,
    is_override: bool,
    state: &mut BoxMemberState,
) -> Result<bool, ParseError> {
    if let Some(method) = crate::parser::declarations::box_def::members::methods::try_parse_method(
        p,
        name.clone(),
        is_override,
    )? {
        let mut method = method;
        p.attach_pending_runes_to_declaration(&mut method)?;
        state.last_method_name = Some(name.clone());
        state.methods.insert(name, method);
        return Ok(true);
    }
    let parsed = crate::parser::declarations::box_def::members::fields::try_parse_header_first_field_or_property(
        p,
        name,
        &mut state.methods,
        &mut state.fields,
        &mut state.field_decls,
        &mut state.field_initializers,
        &mut state.weak_fields,
        false,
    )?;
    if parsed {
        p.ensure_no_pending_runes("field/property")?;
    }
    Ok(parsed)
}
