use crate::ast::Span;
use crate::parser::common::ParserUtils;
use crate::parser::declarations::box_def::members::pending_method::PendingExplicitMethodV1;
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
    let parsed =
        crate::parser::declarations::box_def::members::properties::try_parse_block_first_property(
            p,
            &mut state.source_tx,
            &mut state.birth_once_props,
        )?;
    Ok(parsed)
}

fn commit_pending_ordinary_method(
    p: &mut NyashParser,
    pending: &mut Option<PendingExplicitMethodV1>,
    state: &mut BoxMemberState,
    admit_direct_parameter_source: bool,
) -> Result<bool, ParseError> {
    let Some(method) = pending.take() else {
        return Ok(false);
    };
    let source_site = crate::parser::source_authority::SourceBoxMethodSiteV1::Direct {
        member: state.source_tx.current_member_site(),
    };
    let committed = method.commit_direct(&mut state.source_tx)?;
    let (inventory_ordinal, diagnostic_name, parameter_source) =
        p.issue_committed_instance_box_method(committed)?;
    if let Some(parameters) = parameter_source.filter(|_| admit_direct_parameter_source) {
        p.commit_callable_parameter_source(
            source_site,
            inventory_ordinal,
            crate::parser::callable_parameter_source::ParserCallableDeclarationKindV1::InstanceBoxMethod,
            diagnostic_name,
            parameters,
        )?;
    }
    state.finish_source_member()?;
    Ok(true)
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
    let delegate = crate::parser::declarations::box_def::members::delegates::parse_delegate_decl(
        p,
        state.current_source_member_ordinal(),
    )?;
    state
        .source_tx
        .record_delegate_source_at_current(&delegate)?;
    state.delegates.push(delegate);
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
        state.source_tx.commit_constructor_at_current(&key, &node)?;
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
    let parsed = crate::parser::declarations::box_def::members::fields::try_parse_visibility_block_or_single(
        p,
        visibility,
        &mut state.source_tx,
        &mut state.fields,
        &mut state.field_decls,
        &mut state.field_initializers,
        &mut state.public_fields,
        &mut state.private_fields,
        &mut state.weak_fields,
    )?;
    Ok(parsed)
}

fn box_try_build_gate_members(
    p: &mut NyashParser,
    state: &mut BoxMemberState,
) -> Result<bool, ParseError> {
    if !p.is_build_gate_head() {
        return Ok(false);
    }

    let line = p.current_token().line;
    let gate_site = state.current_gate_site();
    p.consume_build_gate_head()?;
    let predicate = p.parse_build_predicate()?;

    let then_state = parse_box_member_gate_block(
        p,
        &state.source_tx,
        gate_site,
        crate::parser::source_authority::SourceBuildGateBranchV1::Then,
    )?;
    let else_state = if p.match_token(&TokenType::ELSE) {
        p.advance();
        Some(if p.is_build_gate_head() {
            let else_source = state.source_tx.branch_at(
                gate_site,
                crate::parser::source_authority::SourceBuildGateBranchV1::Else,
            );
            parse_box_member_gate_group(p, &else_source)?
        } else {
            parse_box_member_gate_block(
                p,
                &state.source_tx,
                gate_site,
                crate::parser::source_authority::SourceBuildGateBranchV1::Else,
            )?
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
        BoxMemberState::with_source_transaction(state.source_tx.branch())
    };

    state.try_merge_selected_gate(selected_state, gate_site)?;
    Ok(true)
}

fn parse_box_member_gate_block(
    p: &mut NyashParser,
    source_tx: &crate::parser::source_authority::OpenBoxMethodSourceTransactionV1,
    gate_site: crate::ast::BoxMemberGateSiteV1,
    branch: crate::parser::source_authority::SourceBuildGateBranchV1,
) -> Result<BoxMemberState, ParseError> {
    p.mark_callable_parameter_member_gate_unsupported();
    p.consume(TokenType::LBRACE)?;
    let mut state = BoxMemberState::with_source_transaction(source_tx.branch_at(gate_site, branch));
    parse_box_member_body(p, &mut state, false)?;
    p.consume(TokenType::RBRACE)?;
    Ok(state)
}

fn parse_box_member_gate_group(
    p: &mut NyashParser,
    source_tx: &crate::parser::source_authority::OpenBoxMethodSourceTransactionV1,
) -> Result<BoxMemberState, ParseError> {
    if !p.is_build_gate_head() {
        return Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: "gate".to_string(),
            line: p.current_token().line,
        });
    }
    let line = p.current_token().line;
    let gate_site = source_tx.current_gate_site();
    p.consume_build_gate_head()?;
    let predicate = p.parse_build_predicate()?;
    let then_state = parse_box_member_gate_block(
        p,
        source_tx,
        gate_site,
        crate::parser::source_authority::SourceBuildGateBranchV1::Then,
    )?;
    let else_state = if p.match_token(&TokenType::ELSE) {
        p.advance();
        Some(if p.is_build_gate_head() {
            let else_source = source_tx.branch_at(
                gate_site,
                crate::parser::source_authority::SourceBuildGateBranchV1::Else,
            );
            parse_box_member_gate_group(p, &else_source)?
        } else {
            parse_box_member_gate_block(
                p,
                source_tx,
                gate_site,
                crate::parser::source_authority::SourceBuildGateBranchV1::Else,
            )?
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

    let selected = if selected_then {
        then_state
    } else if let Some(else_state) = else_state {
        else_state
    } else {
        BoxMemberState::with_source_transaction(source_tx.branch())
    };
    let mut merged = BoxMemberState::with_source_transaction(source_tx.branch());
    merged.try_merge_selected_gate(selected, gate_site)?;
    Ok(merged)
}

pub(crate) fn parse_box_member_body(
    p: &mut NyashParser,
    state: &mut BoxMemberState,
    admit_direct_parameter_source: bool,
) -> Result<(), ParseError> {
    let mut pending_method: Option<PendingExplicitMethodV1> = None;
    while !p.match_token(&TokenType::RBRACE) && !p.is_at_end() {
        if let Some(method) = pending_method.as_mut() {
            if p.match_token(&TokenType::CATCH) || p.match_token(&TokenType::CLEANUP) {
                p.ensure_no_pending_runes("method postfix")?;
                method.try_apply_postfix(p)?;
                continue;
            }
        }
        commit_pending_ordinary_method(
            p,
            &mut pending_method,
            state,
            admit_direct_parameter_source,
        )?;

        if p.maybe_parse_opt_annotation_noop(
            crate::parser::statements::helpers::AnnotationSite::Member,
        )? {
            continue;
        }

        if box_try_block_first_property(p, state)? {
            state.finish_source_member()?;
            continue;
        }

        if p.match_token(&TokenType::RBRACE) {
            break;
        }

        if box_try_init_block(p, state)? {
            state.finish_source_member()?;
            continue;
        }

        if box_try_delegate(p, state)? {
            state.finish_source_member()?;
            continue;
        }

        if box_try_transition(p, state)? {
            state.finish_source_member()?;
            continue;
        }

        if let Some(invariant) = p.try_parse_invariant_clause()? {
            state.invariants.push(invariant);
            state.finish_source_member()?;
            continue;
        }

        if box_try_build_gate_members(p, state)? {
            state.finish_source_member()?;
            continue;
        }

        let mut is_override = false;
        if p.match_token(&TokenType::OVERRIDE) {
            is_override = true;
            p.advance();
        }

        if box_try_constructor(p, is_override, state)? {
            state.finish_source_member()?;
            continue;
        }

        if p.match_token(&TokenType::WEAK) {
            p.ensure_no_pending_runes("weak field")?;
            crate::parser::grammar_contract::require_semantic_entry(
                "weak_stored_field",
                p.build_config.grammar_profile,
                p.current_token().token_type.clone(),
                p.current_token().line,
            )?;
            p.advance();
            if let TokenType::IDENTIFIER(field_name) = &p.current_token().token_type {
                let field_name = field_name.clone();
                p.advance();
                crate::parser::declarations::box_def::members::fields::parse_weak_field(
                    p,
                    field_name,
                    &mut state.fields,
                    &mut state.field_decls,
                    &mut state.weak_fields,
                )?;
                state.finish_source_member()?;
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
            let declaration_span = p.current_span();
            p.advance();

            if box_try_visibility(p, &field_or_method, state)? {
                state.finish_source_member()?;
                continue;
            }

            if crate::parser::env::unified_members() && field_or_method == "get" {
                if let Some(_property_name) =
                    crate::parser::declarations::box_def::members::fields::try_parse_get_computed_property(
                        p,
                        field_or_method_line,
                        &mut state.source_tx,
                    )?
                {
                    p.ensure_no_pending_runes("get property")?;
                    state.finish_source_member()?;
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
                    &mut state.source_tx,
                    &mut state.birth_once_props,
                )? {
                    state.finish_source_member()?;
                    continue;
                }
            }

            if box_try_method_or_field(
                p,
                field_or_method,
                declaration_span,
                is_override,
                state,
                &mut pending_method,
            )? {
                if pending_method.is_none() {
                    state.finish_source_member()?;
                }
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
    commit_pending_ordinary_method(p, &mut pending_method, state, admit_direct_parameter_source)?;
    Ok(())
}

/// Parse either a method or a header-first field/property starting with `name`.
/// Updates `methods`/`fields` and `last_method_name` as appropriate.
fn box_try_method_or_field(
    p: &mut NyashParser,
    name: String,
    declaration_span: crate::ast::Span,
    is_override: bool,
    state: &mut BoxMemberState,
    pending_method: &mut Option<PendingExplicitMethodV1>,
) -> Result<bool, ParseError> {
    if let Some((method, parameter_source)) =
        crate::parser::declarations::box_def::members::methods::try_parse_method(
            p,
            name.clone(),
            is_override,
        )?
    {
        let mut method = method;
        p.attach_pending_runes_to_declaration(&mut method)?;
        *pending_method = Some(PendingExplicitMethodV1::with_parameter_source(
            name,
            method,
            declaration_span,
            parameter_source,
        ));
        return Ok(true);
    }
    let parsed = crate::parser::declarations::box_def::members::fields::try_parse_header_first_field_or_property(
        p,
        name,
        declaration_span,
        &mut state.source_tx,
        &mut state.fields,
        &mut state.field_decls,
        &mut state.field_initializers,
        false,
    )?;
    if parsed {
        p.ensure_no_pending_runes("field/property")?;
    }
    Ok(parsed)
}
