//! Original argument binding, observation and frame completion.
use super::*;

/// Original outer Call and ordinal identity, independent of the value's class.
fn corroborates_argument_plan_v1(
    plan: &ExpressionSourcePlanV1,
    source: &StaticIncomingSourceV1,
    ordinal: u32,
    site: &SourceExprSiteV1,
) -> bool {
    matches!(plan.root(), ExpressionPlanRootV1::CallArgument { call, ordinal: original, .. }
        if *original == ordinal && std::sync::Arc::ptr_eq(call, source.method_source()))
        && plan.owner() == source.call_site().owner()
        && source.argument_sites().get(ordinal as usize) == Some(site)
        && plan.nodes().first().is_some_and(|node| node.site() == site)
}

fn corroborates_argument_source_v1(
    plan: &ExpressionSourcePlanV1,
    source: &StaticIncomingSourceV1,
    ordinal: u32,
    method: &std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>,
) -> bool {
    corroborates_argument_plan_v1(plan, source, ordinal, method.site())
        && method.owner() == plan.owner()
        && plan.nodes().first().is_some_and(|node| {
            matches!(
            node.source(),
            crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::MethodCall
        )
        })
}

/// Re-observation borrows every original argument plan Rc. This is source
/// coverage only: literal, local and Core inputs share no execution shortcut.
pub(in crate::mir::normal_callable_semantic_package) fn bind_argument_sources_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    prepared: &PreparedBorrowedFormalIngressV1,
    contracts: Option<&[OwnedCallableParameterContractDeclarationV1]>,
    site: &OwnedExprSiteV1,
    previous: Option<&Result<PreparedBorrowedCallActualsV1, String>>,
    rows: Option<&mut PreparedBorrowedCallActualsV1>,
) -> Result<(), String> {
    let Some(rows) = rows else { return Ok(()) };
    if let BorrowedCallActualEvidencePhaseV1::SourceObject(identity) = &mut rows.phase {
        let reject = || freeze("borrowed-object/source-actual-identity");
        let mut originals = prepared
            .source_incoming
            .exact_rows()
            .filter(|row| &row.call == site);
        let original = originals.next().ok_or_else(reject)?;
        if originals.next().is_some()
            || original.source.require_instance()? != &identity.source
            || original.arguments != identity.incoming_arguments
        {
            return Err(reject());
        }
        let old = previous
            .map(|row| {
                let row = row
                    .as_ref()
                    .map_err(|_| freeze("borrowed-actual/repeated-walk-drift"))?;
                let BorrowedCallActualEvidencePhaseV1::SourceObject(old) = &row.phase else {
                    return Err(reject());
                };
                if !old.same_original_source_v1(identity) {
                    return Err(reject());
                }
                Ok((!old.argument_sources.is_empty()).then_some(&old.argument_sources))
            })
            .transpose()?
            .flatten();
        let method = input
            .function()
            .method_call_source_v1(site.site())
            .ok_or_else(reject)?;
        identity.argument_sources = model::bind_original_argument_plans_v1(
            input,
            super::super::super::super::borrowed_formal_uses::BorrowedCallSourceLoanV1::Instance(
                &identity.source,
            ),
            method,
            &identity.candidates,
            old,
        )?;
        return Ok(());
    }
    if let BorrowedCallActualEvidencePhaseV1::SourceInstance(identity) = &mut rows.phase {
        let reject = || freeze("borrowed-instance/argument-source-identity");
        let contracts = contracts.ok_or_else(reject)?;
        let original = crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1::original_instance_input_call_v1(prepared, contracts, site)?.ok_or_else(reject)?;
        if original.source.require_instance()? != &identity.source
            || original.arguments != identity.incoming_arguments
        {
            return Err(reject());
        }
        let old = previous
            .map(|row| {
                let row = row
                    .as_ref()
                    .map_err(|_| freeze("borrowed-actual/repeated-walk-drift"))?;
                let BorrowedCallActualEvidencePhaseV1::SourceInstance(old) = &row.phase else {
                    return Err(reject());
                };
                if !old.same_original_source_v1(identity) {
                    return Err(reject());
                }
                Ok((!old.argument_sources.is_empty()).then_some(&old.argument_sources))
            })
            .transpose()?
            .flatten();
        let method = input
            .function()
            .method_call_source_v1(site.site())
            .ok_or_else(reject)?;
        identity.argument_sources = model::bind_original_argument_plans_v1(
            input,
            super::super::super::super::borrowed_formal_uses::BorrowedCallSourceLoanV1::Instance(
                &identity.source,
            ),
            method,
            &identity.candidates,
            old,
        )?;
        return Ok(());
    }
    let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &mut rows.phase else {
        return Ok(());
    };
    let reject = || freeze("borrowed-static/core-source-identity");
    if !original_static_source_v1(prepared, site)?
        .is_some_and(|source| Rc::ptr_eq(source, &identity.source))
    {
        return Err(reject());
    }
    let old = previous
        .map(|rows| {
            let rows = rows
                .as_ref()
                .map_err(|_| freeze("borrowed-actual/repeated-walk-drift"))?;
            let BorrowedCallActualEvidencePhaseV1::SourceStatic(old) = &rows.phase else {
                return Err(reject());
            };
            if !Rc::ptr_eq(&old.source, &identity.source)
                || old.candidates != identity.candidates
                || old.integer_evidence != identity.integer_evidence
                || old.core_arguments.len() != identity.core_arguments.len()
                || old.argument_sources.len() != identity.candidates.len()
            {
                return Err(reject());
            }
            Ok(old)
        })
        .transpose()?;
    if input.owner() != site.owner()
        || identity.source.call_site() != site
        || identity.candidates.len() != identity.source.argument_sites().len()
        || !input
            .function()
            .method_call_source_v1(site.site())
            .is_some_and(|call| std::sync::Arc::ptr_eq(call, identity.source.method_source()))
    {
        return Err(reject());
    }
    for (ordinal, candidate) in identity.candidates.iter().enumerate() {
        let ordinal = ordinal as u32;
        if candidate.ordinal != ordinal {
            return Err(reject());
        }
        if let BorrowedCallActualValueV1::SelfRooted { binding, root } = candidate.value {
            if !prepared
                .static_arguments
                .contains_key(&(site.clone(), ordinal))
            {
                let flow = input
                    .function()
                    .expression_source()
                    .value_flow()
                    .ok_or_else(reject)?;
                if flow.read(&candidate.site, binding).and_then(|value| {
                    flow.alias_entry_origin(value, |site| input.function().variable_ref(site))
                }) != Some(root)
                {
                    return Err(reject());
                }
            }
        }
        if let BorrowedCallActualValueV1::MethodCall(method) = &candidate.value {
            if let Some(source) = ordinary_argument_call_source_v1(
                prepared,
                &OwnedExprSiteV1::new(site.owner(), candidate.site.clone()),
                method,
            )? {
                source.validate_input_v1(input, prepared, contracts)?;
                if identity.core_arguments.contains_key(&ordinal) {
                    return Err(reject());
                }
                continue;
            }
            let receipt = identity.core_arguments.get(&ordinal).ok_or_else(reject)?;
            if !std::sync::Arc::ptr_eq(method, receipt.source().source())
                || !input
                    .function()
                    .method_call_source_v1(method.site())
                    .is_some_and(|call| std::sync::Arc::ptr_eq(call, method))
                || old.is_some_and(|old| {
                    !old.core_arguments
                        .get(&ordinal)
                        .is_some_and(|original| Rc::ptr_eq(original, receipt))
                })
            {
                return Err(reject());
            }
        } else if identity.core_arguments.contains_key(&ordinal) {
            return Err(reject());
        }
    }
    if identity
        .core_arguments
        .keys()
        .any(|ordinal| *ordinal as usize >= identity.candidates.len())
    {
        return Err(reject());
    }
    identity.argument_sources = model::bind_original_argument_plans_v1(
        input,
        super::super::super::super::borrowed_formal_uses::BorrowedCallSourceLoanV1::Static(
            &identity.source,
        ),
        identity.source.method_source(),
        &identity.candidates,
        old.map(|old| &old.argument_sources),
    )
    .map_err(|_| reject())?;
    Ok(())
}

/// Only explicit source Observe can invalidate a previous Home walk's current
/// qualification. Generic staging remains strict; executable phases never reset.
pub(in crate::mir::normal_callable_semantic_package) fn stage_observed_call_actuals_v1(
    staged: &mut PendingBorrowedFormalActualsV1,
    site: &OwnedExprSiteV1,
    result: Result<Option<PreparedBorrowedCallActualsV1>, String>,
) {
    let retain_static_qualification = match (staged.get(site), &result) {
        (Some(Ok(previous)), Ok(Some(current))) => match (&previous.phase, &current.phase) {
            (
                BorrowedCallActualEvidencePhaseV1::SourceStatic(old),
                BorrowedCallActualEvidencePhaseV1::SourceStatic(new),
            ) if old.same_original_source_v1(new)
                && previous.opaque_actuals == current.opaque_actuals
                && previous.ordered_arguments == current.ordered_arguments
                && old.argument_sources.len() == new.argument_sources.len()
                && new
                    .argument_sources
                    .values()
                    .all(|slot| matches!(slot, ArgumentExpressionV1::Source(_))) =>
            {
                old.argument_sources.iter().all(|(ordinal, old_slot)| {
                    match (old_slot, new.argument_sources.get(ordinal)) {
                        (
                            ArgumentExpressionV1::Source(old),
                            Some(ArgumentExpressionV1::Source(new)),
                        ) => Rc::ptr_eq(old, new),
                        (
                            ArgumentExpressionV1::Qualified(frame),
                            Some(ArgumentExpressionV1::Source(plan)),
                        ) => {
                            Rc::ptr_eq(frame.source(), plan)
                                && Rc::ptr_eq(frame.source_loan().plan(), plan)
                                && frame.evaluation().reference_coverage_is_exact()
                        }
                        _ => false,
                    }
                }) && old
                    .argument_sources
                    .values()
                    .any(|slot| matches!(slot, ArgumentExpressionV1::Qualified(_)))
            }
            _ => false,
        },
        _ => false,
    };
    let reset = match (staged.get(site), &result) {
        (Some(Ok(previous)), Ok(Some(current))) => match (&previous.phase, &current.phase) {
            (
                BorrowedCallActualEvidencePhaseV1::SourceStatic(old),
                BorrowedCallActualEvidencePhaseV1::SourceStatic(new),
            ) => {
                old.same_original_source_v1(new)
                    && previous.opaque_actuals == current.opaque_actuals
                    && previous.ordered_arguments == current.ordered_arguments
                    && new
                        .argument_sources
                        .values()
                        .all(|row| matches!(row, ArgumentExpressionV1::Source(_)))
            }
            (
                BorrowedCallActualEvidencePhaseV1::SourceInstance(old),
                BorrowedCallActualEvidencePhaseV1::SourceInstance(new),
            ) => {
                old.same_original_source_v1(new)
                    && (old.argument_sources.is_empty()
                        || old.argument_sources.len() == new.argument_sources.len()
                            && old.argument_sources.iter().all(|(ordinal, plan)| {
                                new.argument_sources
                                    .get(ordinal)
                                    .is_some_and(|other| Rc::ptr_eq(plan, other))
                            }))
                    && previous.opaque_actuals == current.opaque_actuals
                    && previous.ordered_arguments == current.ordered_arguments
                    && new
                        .argument_sources
                        .values()
                        .all(|row| matches!(row, ArgumentExpressionV1::Source(_)))
            }
            (
                BorrowedCallActualEvidencePhaseV1::SourceObject(old),
                BorrowedCallActualEvidencePhaseV1::SourceObject(new),
            ) => {
                old.same_original_source_v1(new)
                    && (old.argument_sources.is_empty()
                        || old.argument_sources.len() == new.argument_sources.len()
                            && old.argument_sources.iter().all(|(ordinal, plan)| {
                                new.argument_sources
                                    .get(ordinal)
                                    .is_some_and(|other| Rc::ptr_eq(plan, other))
                            }))
                    && previous.opaque_actuals == current.opaque_actuals
                    && previous.ordered_arguments == current.ordered_arguments
                    && new
                        .argument_sources
                        .values()
                        .all(|row| matches!(row, ArgumentExpressionV1::Source(_)))
            }
            _ => false,
        },
        _ => false,
    };
    if crate::config::env::builder_debug_enabled() && reset {
        eprintln!("[argument-expression] source re-observe reset site={site:?}");
    }
    if reset {
        let mut current = result
            .expect("checked observation")
            .expect("checked source tuple");
        if retain_static_qualification {
            let Some(Ok(previous)) = staged.get(site) else {
                unreachable!("retained source row")
            };
            let BorrowedCallActualEvidencePhaseV1::SourceStatic(old) = &previous.phase else {
                unreachable!("retained Static source row")
            };
            let BorrowedCallActualEvidencePhaseV1::SourceStatic(new) = &mut current.phase else {
                unreachable!("retained Static source row")
            };
            for (ordinal, old_slot) in &old.argument_sources {
                if let ArgumentExpressionV1::Qualified(frame) = old_slot {
                    let Some(ArgumentExpressionV1::Source(plan)) =
                        new.argument_sources.get(ordinal)
                    else {
                        unreachable!("exact source-plan correspondence checked")
                    };
                    debug_assert!(Rc::ptr_eq(frame.source(), plan));
                    new.argument_sources
                        .insert(*ordinal, ArgumentExpressionV1::Qualified(Rc::clone(frame)));
                }
            }
        }
        staged.insert(site.clone(), Ok(current));
    } else {
        super::super::super::stage_borrowed_call_actuals_v1(staged, site, result);
    }
}

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn validate_static_source_actuals_v1<
    'a,
>(
    prepared: &PreparedBorrowedFormalIngressV1,
    rows: &'a PreparedBorrowedCallActualsV1,
    site: &OwnedExprSiteV1,
    required_i64_arguments: &[u32],
) -> Result<&'a StaticSourceActualIdentityV1, String> {
    let rows = rows
        .source_static_snapshot_v1()
        .ok_or_else(|| freeze("borrowed-static/source-phase-required"))?;
    let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &rows.phase else {
        return Err(freeze("borrowed-static/source-phase-required"));
    };
    let retained = original_static_source_v1(prepared, site)?
        .ok_or_else(|| freeze("borrowed-static/source-observation-missing"))?;
    if !Rc::ptr_eq(retained, &identity.source)
        || identity.source.call_site() != site
        || required_i64_arguments != identity.source.required_i64_arguments()
        || rows.ordered_arguments.len() != identity.source.argument_sites().len()
        || identity.candidates.len() != rows.ordered_arguments.len()
        || identity.integer_evidence.len() != rows.ordered_arguments.len()
        || !rows.opaque_actuals.is_empty()
    {
        return Err(freeze("borrowed-static/source-projection-identity"));
    }
    for (index, ((formal, candidate), argument)) in identity
        .source
        .parameters()
        .iter()
        .zip(identity.candidates.iter())
        .zip(rows.ordered_arguments.iter())
        .enumerate()
    {
        if formal.ordinal as usize != index
            || candidate.ordinal != formal.ordinal
            || formal.binding.owner() != identity.source.callee_owner()
            || identity.source.argument_sites().get(index) != Some(&candidate.site)
        {
            return Err(freeze("borrowed-static/source-projection-identity"));
        }
        let plan = identity
            .argument_sources
            .get(&candidate.ordinal)
            .filter(|plan| {
                corroborates_argument_plan_v1(
                    plan,
                    &identity.source,
                    candidate.ordinal,
                    &candidate.site,
                )
            })
            .ok_or_else(|| freeze("borrowed-static/core-source-identity"))?;
        use crate::mir::resolved_semantics::{
            ResolvedLexicalRefV1 as Ref, ResolvedLiteralSourceV1 as Literal,
        };
        use crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1 as Node;
        match (&candidate.value, plan.nodes()[0].source()) {
            (BorrowedCallActualValueV1::Expression, Node::Binary(_))
                if identity.integer_evidence.get(index) == Some(&false) => {}
            (BorrowedCallActualValueV1::Expression, _) => {
                return Err(freeze("borrowed-static/argument-source-identity"))
            }
            (
                BorrowedCallActualValueV1::String(value),
                Node::Literal(Literal::String(original)),
            ) if value == original => {}
            (BorrowedCallActualValueV1::String(_), _) => {
                return Err(freeze("borrowed-static/argument-source-identity"))
            }
            (
                BorrowedCallActualValueV1::Binding(binding),
                Node::Reference(Ref::Local(original)),
            ) if binding == original => {}
            (BorrowedCallActualValueV1::Binding(_), _) => {
                return Err(freeze("borrowed-static/argument-source-identity"))
            }
            _ => {}
        }
        match (
            &candidate.value,
            identity.core_arguments.get(&candidate.ordinal),
        ) {
            (BorrowedCallActualValueV1::MethodCall(method), Some(receipt)) => {
                let site = OwnedExprSiteV1::new(site.owner(), candidate.site.clone());
                if !identity.argument_sources.get(&candidate.ordinal).is_some_and(|plan|
                    corroborates_argument_source_v1(plan, identity.source.as_ref(), candidate.ordinal, method))
                    || !prepared.ordinary_core_source_v1(&site).is_some_and(|original| Rc::ptr_eq(original, receipt))
                    || !std::sync::Arc::ptr_eq(method, receipt.source().source())
                    || method.owner() != site.owner() || method.site() != site.site()
                    || identity.integer_evidence.get(index) != Some(&false)
                    || !super::super::static_source::core_argument_source_v1(prepared, &site, method, &formal.kind)?
                        .is_some_and(|original| Rc::ptr_eq(&original, receipt))
                { return Err(freeze("borrowed-static/core-source-identity")); }
            }
            (BorrowedCallActualValueV1::MethodCall(method), None)
                if ordinary_argument_call_source_v1(
                    prepared,
                    &OwnedExprSiteV1::new(site.owner(), candidate.site.clone()),
                    method,
                )?
                .is_some() =>
            {
                if !matches!(formal.kind, CallableParameterContractKindV1::OpaqueHandle)
                    || identity.integer_evidence.get(index) != Some(&false)
                    || !corroborates_argument_source_v1(
                        plan,
                        &identity.source,
                        candidate.ordinal,
                        method,
                    )
                {
                    return Err(freeze("borrowed-static/core-source-identity"));
                }
            }
            (BorrowedCallActualValueV1::MethodCall(_), None) | (_, Some(_)) => {
                return Err(freeze("borrowed-static/core-source-identity"))
            }
            (_, None) => {}
        }
        let matches = match (&formal.kind, &candidate.value, argument) {
            (kind, _, LocalCallArgumentV1::BorrowedActual { ordinal, site })
                if kind.is_ordinary_borrowed_handle() =>
            {
                *ordinal == formal.ordinal && site == &candidate.site
            }
            (
                CallableParameterContractKindV1::ExactTrivial(abi),
                BorrowedCallActualValueV1::Integer(expected),
                LocalCallArgumentV1::Integer(actual),
            ) if abi.is_i64() => expected == actual,
            (
                CallableParameterContractKindV1::ExactTrivial(abi),
                BorrowedCallActualValueV1::Scalar(expected, SourceScalarKind::Integer),
                LocalCallArgumentV1::Scalar(actual),
            ) if abi.is_i64() => expected == actual && actual.owner() == site.owner(),
            (
                CallableParameterContractKindV1::OpaqueHandle,
                BorrowedCallActualValueV1::Expression,
                LocalCallArgumentV1::Expression { ordinal, site: original },
            ) if identity.integer_evidence.get(index) == Some(&false)
                && *ordinal == formal.ordinal
                && original == &candidate.site
                && prepared
                    .source_value_sum_v1(formal.binding)
                    .is_some_and(|sum| sum.signed_only()) => true,
            (
                CallableParameterContractKindV1::ExactTrivial(abi),
                BorrowedCallActualValueV1::Expression | BorrowedCallActualValueV1::MethodCall(_),
                LocalCallArgumentV1::Expression {
                    ordinal,
                    site: original,
                },
            ) if abi.is_i64() => *ordinal == formal.ordinal && original == &candidate.site,
            _ => false,
        };
        if !matches {
            return Err(freeze("borrowed-static/source-projection-identity"));
        }
    }
    if identity.argument_sources.len() != identity.candidates.len()
        || identity
            .argument_sources
            .keys()
            .any(|ordinal| *ordinal as usize >= identity.candidates.len())
        || identity
            .core_arguments
            .keys()
            .any(|ordinal| *ordinal as usize >= identity.candidates.len())
    {
        return Err(freeze("borrowed-static/core-source-identity"));
    }
    Ok(identity)
}

/// During Home scanning this lends only already-qualified source arguments.
/// Accepted Home membership and executable payload belong to later activation.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn validate_qualified_argument_sources_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    pending: &PendingBorrowedFormalActualsV1,
    site: &OwnedExprSiteV1,
    identity: &StaticSourceActualIdentityV1,
) -> Result<(), String> {
    use crate::mir::resolved_value_profile::{
        expression_class::ExpressionValueClassV1 as Class,
        expression_projection::ExpressionLeafClassV1 as Leaf,
    };
    for actual in &identity.candidates {
        let core = identity.core_arguments.contains_key(&actual.ordinal);
        let ordinary_call = match &actual.value {
            BorrowedCallActualValueV1::MethodCall(method) => ordinary_argument_call_source_v1(
                prepared,
                &OwnedExprSiteV1::new(site.owner(), actual.site.clone()),
                method,
            )?
            .is_some(),
            _ => false,
        };
        let finite_integer = matches!(
            actual.value,
            BorrowedCallActualValueV1::Scalar(_, SourceScalarKind::Integer)
        ) && identity
            .source
            .parameters()
            .get(actual.ordinal as usize)
            .is_some_and(|formal| {
                matches!(formal.kind, CallableParameterContractKindV1::OpaqueHandle)
                    && prepared
                        .source_value_sum_v1(formal.binding)
                        .is_some_and(|sum| sum.signed_only())
            });
        let expression = core
            || ordinary_call
            || finite_integer
            || matches!(
                actual.value,
                BorrowedCallActualValueV1::Expression
                    | BorrowedCallActualValueV1::String(_)
                    | BorrowedCallActualValueV1::Binding(_)
            )
            || matches!(actual.value, BorrowedCallActualValueV1::SelfRooted { .. })
                && !prepared
                    .static_arguments
                    .contains_key(&(site.clone(), actual.ordinal));
        if !expression {
            continue;
        }
        let slot = identity
            .argument_sources
            .get(&actual.ordinal)
            .ok_or_else(|| freeze("borrowed-static/core-source-identity"))?;
        let ArgumentExpressionV1::Qualified(frame) = slot else {
            if crate::config::env::builder_debug_enabled() {
                eprintln!("[static-argument] site={site:?} ordinal={} target={:?} actual={:?} pending={slot:?}",
                    actual.ordinal, identity.source.target(), actual.value);
            }
            return Err(freeze(if core {
                "borrowed-static/core-expression-execution-pending"
            } else {
                "borrowed-static/text-expression-execution-pending"
            }));
        };
        let original =
            lend_argument_source_inner_v1(prepared, pending, None, site, actual.ordinal)?
                .ok_or_else(|| freeze("borrowed-static/argument-frame-identity"))?;
        if !Rc::ptr_eq(frame.source(), slot)
            || frame.source_loan() != &original
            || !frame.evaluation().reference_coverage_is_exact()
            || frame.evaluation().root_class()
                != Leaf::Value(if original.requires_i64_v1() {
                    Class::I64
                } else if original.dynamic_array_carrier_append_root_v1()
                    && original
                        .plan()
                        .nodes()
                        .first()
                        .is_some_and(|root| original.dynamic_array_carrier_call_is_v1(root.site()))
                {
                    Class::Dynamic
                } else {
                    Class::String
                })
        {
            return Err(freeze("borrowed-static/argument-frame-identity"));
        }
    }
    Ok(())
}

pub(in crate::mir::normal_callable_semantic_package) fn complete_argument_expression_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    prepared: &PreparedBorrowedFormalIngressV1,
    pending: &mut PendingBorrowedFormalActualsV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    site: &OwnedExprSiteV1,
    dynamic_source_calls: &[crate::mir::source_call_target::VerifiedSourceBoundDynamicMemberCallV1],
    frame: &Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1>,
) -> Result<
    Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1>,
    String,
> {
    let reject = || freeze("borrowed-static/argument-frame-identity");
    let ExpressionPlanRootV1::CallArgument { call, ordinal, .. } = frame.source().root() else {
        return Err(reject());
    };
    if input.owner() != site.owner()
        || call.owner() != site.owner()
        || call.site() != site.site()
        || frame.source().owner() != site.owner()
        || !frame.evaluation().reference_coverage_is_exact()
    {
        return Err(reject());
    }
    let retained_carrier = frame.source_loan().dynamic_array_carrier_append_v1();
    let original = match retained_carrier {
        Some(holder) => observed_carrier_array_append_argument_source_v1(
            input,
            site,
            *ordinal,
            holder,
            dynamic_source_calls,
            Some(frame.source_loan()),
        )?,
        None => lend_argument_expression_source_v1(prepared, pending, contracts, site, *ordinal)?,
    };
    let original = original.ok_or_else(reject)?;
    if frame.source_loan() != &original || !Rc::ptr_eq(frame.source(), original.plan()) {
        return Err(reject());
    }
    // A Dynamic carrier append belongs to the original Array Home walk, not
    // to the borrowed-formal pending table for a callable argument.
    if retained_carrier.is_some() {
        return Ok(Rc::clone(frame));
    }
    let rows = pending
        .get_mut(site)
        .ok_or_else(reject)?
        .as_mut()
        .map_err(|error| error.clone())?;
    if let Some(dynamic) = rows.dynamic_invocation_arguments.as_mut() {
        let slot = dynamic.arguments.get_mut(ordinal).ok_or_else(reject)?;
        match slot {
            ArgumentExpressionV1::Qualified(previous) => {
                if previous.as_ref() != frame.as_ref() {
                    return Err(freeze("dynamic-argument/frame-current-drift"));
                }
                return Ok(Rc::clone(previous));
            }
            ArgumentExpressionV1::Source(plan) => {
                if !Rc::ptr_eq(plan, frame.source()) {
                    return Err(reject());
                }
                *slot = ArgumentExpressionV1::Qualified(Rc::clone(frame));
                return Ok(Rc::clone(frame));
            }
        }
    }
    let slots = match &mut rows.phase {
        BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) => &mut identity.argument_sources,
        BorrowedCallActualEvidencePhaseV1::SourceInstance(identity) => {
            &mut identity.argument_sources
        }
        BorrowedCallActualEvidencePhaseV1::SourceObject(identity) => &mut identity.argument_sources,
        _ => return Err(reject()),
    };
    let slot = slots.get_mut(ordinal).ok_or_else(reject)?;
    match slot {
        ArgumentExpressionV1::Qualified(previous) => {
            if previous.as_ref() != frame.as_ref() {
                return Err(freeze("borrowed-static/argument-frame-current-drift"));
            }
            Ok(Rc::clone(previous))
        }
        ArgumentExpressionV1::Source(plan) => {
            if !Rc::ptr_eq(plan, frame.source()) {
                return Err(reject());
            }
            *slot = ArgumentExpressionV1::Qualified(Rc::clone(frame));
            Ok(Rc::clone(frame))
        }
    }
}
