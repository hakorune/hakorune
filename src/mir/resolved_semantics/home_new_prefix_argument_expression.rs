//! Connect the retained source loan at the same real pre-evaluation Prefix point.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1;

fn carrier_array_append_holder<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    call_site: &OwnedExprSiteV1,
    actual_site: &SourceExprSiteV1,
    locals: &PrefixLocalFlow<'_>,
    port: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<Option<crate::mir::normal_callable_semantic_package::NamedArrayHolderSourceLoanV1>, E> {
    let Some(call) = input.function().method_call_source_v1(call_site.site()) else {
        return Ok(None);
    };
    let crate::mir::resolved_semantics::ResolvedMethodCallReceiverSourceV1::Lexical(
        crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding),
    ) = call.receiver()
    else {
        return Ok(None);
    };
    let Some(source) = locals.available_named_array_holder(call.receiver_site()) else {
        return Ok(None);
    };
    // Select this family from the original holder and exact append actual
    // before demanding an append loan. Other named-array uses are not appends.
    if !source.is_dynamic_carrier_append_only_v1()
        || !source.append_candidate(call_site)
            .is_some_and(|candidate| candidate.site() == actual_site)
    {
        return Ok(None);
    }
    let Some(BorrowedCallArgumentsV1::NamedArrayHolderSource(verified)) = port(
        call_site,
        BorrowedCallActualRequestV1::NamedArrayAppendStatement {
            binding,
            source: &source,
        },
    )? else {
        return Ok(None);
    };
    if verified.binding() != binding
        || !verified.is_dynamic_carrier_append_only_v1()
        || !verified
            .append_candidate(call_site)
            .is_some_and(|candidate| candidate.site() == actual_site)
    {
        return Ok(None);
    }
    Ok(Some(verified))
}

pub(super) fn text_reference(
    input: ResolvedFunctionLoweringInputV1<'_>,
    locals: &PrefixLocalFlow<'_>,
    site: &SourceExprSiteV1,
) -> Option<(BindingRefV1, BindingRefV1)> {
    let ResolvedLexicalRefV1::Local(binding) = input.function().variable_ref(site)? else {
        return None;
    };
    let OrdinaryObservation::Handle(root) = locals.observe(site)? else {
        return None;
    };
    let super::super::BindingKindV1::Parameter { index } = input.function().binding(root)?.kind()
    else {
        return None;
    };
    (locals.is_self_rooted_handle(root)
        && input
            .function()
            .declaration_binding(&SourceBindingSiteV1::Parameter { index })
            == Some(root))
    .then_some((binding, root))
}

/// An entry-formal read: a Text entry reference, else an untyped (opaque) entry formal.
pub(super) fn entry_formal_reference(
    input: ResolvedFunctionLoweringInputV1<'_>,
    locals: &PrefixLocalFlow<'_>,
    site: &SourceExprSiteV1,
) -> Option<(BindingRefV1, BindingRefV1)> {
    text_reference(input, locals, site).or_else(|| dynamic_formal_reference(input, locals, site))
}

pub(super) fn dynamic_formal_reference(
    input: ResolvedFunctionLoweringInputV1<'_>,
    locals: &PrefixLocalFlow<'_>,
    site: &SourceExprSiteV1,
) -> Option<(BindingRefV1, BindingRefV1)> {
    let binding = locals.opaque_formal_binding(site)?;
    let crate::mir::resolved_semantics::BindingKindV1::Parameter { index } =
        input.function().binding(binding)?.kind()
    else {
        return None;
    };
    (input
        .function()
        .declaration_binding(&SourceBindingSiteV1::Parameter { index })
        == Some(binding))
    .then_some((binding, binding))
}

pub(super) fn observe<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    call_site: &OwnedExprSiteV1,
    actuals: &[BorrowedCallActualCandidateV1],
    locals: &PrefixLocalFlow<'_>,
    prefix_known: bool,
    homes: &[LiveHomeV1],
    local_calls: &mut Vec<LocalCallObservationV1>,
    path_calls: &mut BTreeSet<OwnedExprSiteV1>,
    fields: &mut impl FnMut(
        &[LocalFieldReadRequestV1],
        LocalFieldReadDemandV1,
    ) -> Result<Option<Vec<LocalFieldReadResultV1>>, E>,
    port: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<(), E> {
    if !prefix_known {
        if crate::config::env::builder_debug_enabled() {
            eprintln!("[argument-expression] call={call_site:?} prefix unavailable");
        }
        return Ok(());
    }
    for actual in actuals {
        match &actual.value {
            // A Dynamic invocation retains every operand through its ordered
            // argument-expression source, including literal i64 operands.
            // Static calls may return no expression-source loan for a literal.
            BorrowedCallActualValueV1::Integer(_)
            | BorrowedCallActualValueV1::Expression
            | BorrowedCallActualValueV1::FieldAccess
            | BorrowedCallActualValueV1::MethodCall(_)
            | BorrowedCallActualValueV1::String(_)
            | BorrowedCallActualValueV1::SelfRooted { .. }
            | BorrowedCallActualValueV1::Scalar(_, SourceScalarKind::Integer) => {}
            BorrowedCallActualValueV1::Binding(_)
                if locals.static_text_reference(&actual.site).is_some()
                    || locals.text_reference(&actual.site).is_some()
                    || locals
                        .current_scalar_read_v1(&actual.site, local_calls)
                        .is_some() => {}
            _ => continue,
        }
        let append_holder = carrier_array_append_holder(
            input,
            call_site,
            &actual.site,
            locals,
            port,
        )?;
        let source = if let Some(holder) = append_holder.as_ref() {
            let appended = port(
                call_site,
                BorrowedCallActualRequestV1::ArgumentExpressionCarrierArrayAppendSource {
                    input,
                    ordinal: actual.ordinal,
                    holder,
                },
            )?;
            if appended.is_some() {
                appended
            } else {
                port(
                    call_site,
                    BorrowedCallActualRequestV1::ArgumentExpressionSource(
                        input,
                        actual.ordinal,
                    ),
                )?
            }
        } else {
            port(
                call_site,
                BorrowedCallActualRequestV1::ArgumentExpressionSource(input, actual.ordinal),
            )?
        };
        let Some(BorrowedCallArgumentsV1::ArgumentExpressionSource(loan)) = source else {
            if crate::config::env::builder_debug_enabled() {
                eprintln!(
                    "[argument-expression] call={call_site:?} ordinal={} source loan unavailable",
                    actual.ordinal
                );
            }
            continue;
        };
        if crate::config::env::builder_debug_enabled() {
            eprintln!(
                "[argument-expression] call={call_site:?} ordinal={} source loan retained",
                actual.ordinal
            );
        }
        if loan.plan().root().statement() != statement.site()
            || loan
                .plan()
                .nodes()
                .first()
                .is_none_or(|node| node.site() != &actual.site)
        {
            continue;
        }
        if matches!(actual.value, BorrowedCallActualValueV1::FieldAccess) {
            let source = std::rc::Rc::clone(loan.plan());
            let Some(request) = field_read::expression_field_read_request(input, &source, &actual.site, locals) else { continue; };
            let Some(candidate) = HomeExpressionPlanV1::borrowed_argument_reads_v1(
                std::rc::Rc::clone(&source), homes, std::slice::from_ref(&request),
            ) else { continue; };
            let mut installation = locals.expression_installation();
            if !installation.approve(&candidate) { continue; }
            if fields(std::slice::from_ref(&request), LocalFieldReadDemandV1::SourceExpression {
                probe: false, plan: source,
            })?.is_some() { installation.commit(); }
            continue;
        }
        let mut reads = Vec::new();
        let mut finite_reads = Vec::new();
        let mut dynamic_reads = Vec::new();
        let mut loop_dynamic_reads = Vec::new();
        let mut ready = true;
        for node in loan.plan().nodes() {
            if !matches!(node.source(), ExpressionNodeSourceV1::Reference(_)) {
                continue;
            }
            if loan.text_operand_is(node.site()) {
                if locals.static_text_reference(node.site()).is_some()
                    || locals.text_reference(node.site()).is_some()
                {
                    continue;
                }
                let Some((binding, root)) = text_reference(input, locals, node.site()) else {
                    if crate::config::env::builder_debug_enabled() {
                        eprintln!("[argument-expression] call={call_site:?} ordinal={} text operand={:?} has no formal entry", actual.ordinal, node.site());
                    }
                    ready = false;
                    break;
                };
                let Some(BorrowedCallArgumentsV1::TextEntryRead(read)) = port(
                    call_site,
                    BorrowedCallActualRequestV1::ArgumentTextEntryRead {
                        source: &loan,
                        read: node.site(),
                        binding,
                        root,
                    },
                )?
                else {
                    if crate::config::env::builder_debug_enabled() {
                        eprintln!("[argument-expression] call={call_site:?} ordinal={} text operand={:?} entry read unavailable", actual.ordinal, node.site());
                    }
                    ready = false;
                    break;
                };
                reads.push(read);
            } else if loan.dynamic_operand_contains(node.site())
                && locals.loop_dynamic_reference_v1(node.site()).is_some()
            {
                let Some(read) = locals.loop_dynamic_reference_v1(node.site()) else {
                    ready = false;
                    break;
                };
                loop_dynamic_reads.push(read);
            } else if loan.signed_operand_contains(node.site())
                && locals
                    .current_scalar_read_v1(node.site(), local_calls)
                    .is_none()
            {
                let Some((binding, root)) = text_reference(input, locals, node.site()) else {
                    if crate::config::env::builder_debug_enabled() {
                        eprintln!("[argument-expression] call={call_site:?} ordinal={} integer operand={:?} has no formal entry", actual.ordinal, node.site());
                    }
                    ready = false;
                    break;
                };
                if let Some(BorrowedCallArgumentsV1::FiniteEntryRead(read)) = port(
                    call_site,
                    BorrowedCallActualRequestV1::FiniteEntryRead {
                        input,
                        plan: loan.plan(),
                        read: node.site(),
                        binding,
                        root,
                    },
                )? {
                    if read.signed_only() {
                        finite_reads.push(read);
                        continue;
                    }
                    ready = false;
                    break;
                }
                if !loan.dynamic_operand_contains(node.site()) {
                    ready = false;
                    break;
                }
                let Some(BorrowedCallArgumentsV1::DynamicEntryRead(read)) = port(
                    call_site,
                    BorrowedCallActualRequestV1::DynamicEntryRead {
                        input,
                        plan: loan.plan(),
                        read: node.site(),
                        binding,
                        root,
                    },
                )?
                else {
                    if crate::config::env::builder_debug_enabled() {
                        eprintln!("[argument-expression] call={call_site:?} ordinal={} dynamic operand={:?} entry read unavailable", actual.ordinal, node.site());
                    }
                    ready = false;
                    break;
                };
                dynamic_reads.push(read);
            }
        }
        if !ready {
            continue;
        }
        let source = std::rc::Rc::clone(loan.plan());
        let mut installation = None;
        let qualified = qualify_argument_expression_v1(
            loan,
            statement.site(),
            homes,
            &reads,
            &finite_reads,
            &dynamic_reads,
            &loop_dynamic_reads,
            &mut |site| text_reference(input, locals, site),
            &mut |site| locals.static_text_reference(site),
            &mut |site| locals.text_reference(site),
            &mut |site| locals.current_scalar_read_v1(site, local_calls),
            &mut |site| field_read::expression_field_read_request(input, &source, site, locals),
            &mut |site| {
                local_call_flow::issue_source_expression_call_v1(
                    input,
                    &source,
                    site,
                    homes,
                    &mut |binding| locals.field_alias_call_path(binding),
                    port,
                )
            },
            &mut |plan| {
                let Some(frame) = plan.argument_frame() else { return false; };
                // Preflight before completing the pending actual or installing
                // the frame: a same-site disagreement must publish neither.
                if frame.evaluation().calls().iter().any(|call| local_calls.iter()
                    .any(|old| old.site() == call.site() && old != call)) {
                    return false;
                }
                let mut candidate = locals.expression_installation();
                if !candidate.approve(plan) {
                    return false;
                }
                installation = Some(candidate);
                true
            },
            &mut |requests, demand| Ok(fields(requests, demand)?.is_some()),
        )?;
        if crate::config::env::builder_debug_enabled() {
            eprintln!(
                "[argument-expression] qualified call={call_site:?} ordinal={} present={}",
                actual.ordinal,
                qualified.is_some()
            );
        }
        if qualified.is_none() && crate::config::env::builder_debug_enabled() {
            eprintln!(
                "[argument-expression] call={call_site:?} ordinal={} qualification incomplete",
                actual.ordinal
            );
        }
        if let Some(candidate) = qualified {
            let frame = candidate
                .argument_frame()
                .expect("argument qualifier retains its frame");
            if crate::config::env::builder_debug_enabled() {
                eprintln!(
                    "[argument-expression] completing call={call_site:?} ordinal={} root={:?} owned={:?}",
                    actual.ordinal,
                    frame.evaluation().root_class(),
                    frame.evaluation().owned_result_nodes()
                );
            }
            let Some(BorrowedCallArgumentsV1::ArgumentExpressionFrame(frame)) = port(
                call_site,
                BorrowedCallActualRequestV1::CompleteArgumentExpression { input, frame },
            )?
            else {
                unreachable!("successful argument completion must return its canonical frame");
            };
            if crate::config::env::builder_debug_enabled() {
                eprintln!(
                    "[argument-expression] completed call={call_site:?} ordinal={}",
                    actual.ordinal
                );
            }
            // The approved candidate must use the same canonical frame that
            // the pending Call now owns for this current Home walk.
            let mut installation = installation.expect("approved argument installation");
            assert!(
                installation.retain_approved_argument(std::rc::Rc::clone(&frame)),
                "completed argument must match the approved Home frame"
            );
            installation.commit();
            // Only the canonical committed argument contributes calls to this
            // reached path, just like Print/initializer evaluation. No second
            // call inventory or speculative observation is published.
            for call in frame.evaluation().calls() {
                path_calls.insert(call.site().clone());
                if !local_calls.iter().any(|old| old.site() == call.site()) {
                    local_calls.push(call.clone());
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
pub(crate) fn observe_for_test<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    parameters: Vec<(
        u32,
        BindingRefV1,
        crate::mir::callable_parameter_contract::CallableParameterContractKindV1,
    )>,
    prefix_known: bool,
    unavailable: Option<BindingRefV1>,
    fields: &mut impl FnMut(
        &[LocalFieldReadRequestV1],
        LocalFieldReadDemandV1,
    ) -> Result<Option<Vec<LocalFieldReadResultV1>>, E>,
    port: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<Vec<HomeExpressionPlanV1>, E> {
    let mut locals = PrefixLocalFlow::new(input);
    assert!(locals.install_parameters(parameters));
    if let Some(binding) = unavailable {
        locals.install_uninitialized(binding);
    }
    local_call_flow::stage_statement_actuals(
        input,
        statement,
        &locals,
        prefix_known,
        &[],
        &mut Vec::new(),
        &mut BTreeSet::new(),
        fields,
        port,
    )?;
    Ok(locals.expression_plans())
}
