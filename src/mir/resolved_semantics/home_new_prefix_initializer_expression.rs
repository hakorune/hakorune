//! Real Local initializer qualification and Normal-only Text installation.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use crate::mir::resolved_value_profile::expression_plan::{
    local_expression_source_plan_v1, ExpressionNodeSourceV1,
};

pub(super) fn observe<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    initializer: &super::super::ResolvedInitializerRelationV1,
    locals: &mut PrefixLocalFlow<'_>,
    homes: &mut Vec<LiveHomeV1>,
    prefix_known: bool,
    local_calls: &mut Vec<LocalCallObservationV1>,
    path_calls: &mut BTreeSet<OwnedExprSiteV1>,
    scalar_source: &mut impl HomeFieldSourceConsultV1<E>,
    fields: &mut impl FnMut(
        &[LocalFieldReadRequestV1],
        LocalFieldReadDemandV1,
    ) -> Result<Option<Vec<LocalFieldReadResultV1>>, E>,
    port: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<bool, E> {
    let debug = crate::config::env::builder_debug_enabled();
    if !prefix_known {
        if debug {
            eprintln!(
                "[initializer-expression] statement={:?} prefix unavailable",
                statement.site()
            );
        }
        return declined(statement.site(), 1);
    }
    let Some(site) = initializer.initializer_site() else {
        return declined(statement.site(), 2);
    };
    if initializer.binding().owner() != input.owner()
        || !matches!(initializer.declaration_site(), SourceBindingSiteV1::Local { statement: original, .. } if original == statement.site())
    {
        return declined(statement.site(), 3);
    }
    if matches!(
        input.function().expression_source().literal(site),
        Some(ResolvedLiteralSourceV1::String(_))
    ) {
        locals.install_static_text(initializer.binding(), site.clone());
        return Ok(true);
    }
    if let Some(read) = locals.static_text_reference(site) {
        locals.install_static_text(initializer.binding(), read.definition().clone());
        return Ok(true);
    }
    if let Some((_, root, acquisition)) = locals.text_reference(site) {
        locals.install_text_alias(initializer.binding(), root, acquisition);
        return Ok(true);
    }
    let Ok(ledger) = input.forest().callable_source_ledger(input.owner()) else {
        return declined(statement.site(), 4);
    };
    let Ok(source) = local_expression_source_plan_v1(&ledger, input.body_shape(), initializer)
    else {
        return declined(statement.site(), 5);
    };
    let root_add = source
        .nodes()
        .first()
        .and_then(|node| node.binary())
        .is_some_and(|binary| binary.operator() == super::super::ResolvedBinaryOperatorV1::Add);
    let root_reference = matches!(
        source.nodes()[0].source(),
        ExpressionNodeSourceV1::Reference(_)
    );
    let owned = OwnedExprSiteV1::new(input.owner(), site.clone());
    let arithmetic_requested = source.nodes()[0].binary().is_some_and(|binary| matches!(binary.operator(),
        super::super::ResolvedBinaryOperatorV1::Add | super::super::ResolvedBinaryOperatorV1::Multiply
            | super::super::ResolvedBinaryOperatorV1::Subtract | super::super::ResolvedBinaryOperatorV1::Divide
            | super::super::ResolvedBinaryOperatorV1::Modulo))
        // A selected source Loop consumes a complete evaluation frame, too.
        // This purpose does not classify leaves or grant executable input.
        && (locals.in_source_loop_v1() || scalar_source.requires_initializer_use_frame(&owned)?);
    let root_compare = source.nodes()[0].binary().is_some_and(|binary| matches!(binary.operator(),
        super::super::ResolvedBinaryOperatorV1::Equal | super::super::ResolvedBinaryOperatorV1::NotEqual));
    if !arithmetic_requested
        && !root_add
        && !root_reference
        && !root_compare
        && !matches!(
            source.nodes()[0].source(),
            ExpressionNodeSourceV1::MethodCall
        )
    {
        return declined(statement.site(), 6);
    }
    if root_compare {
        // A comparison is not itself a call root: stage every call under it in
        // source order from this real Prefix, as the whole-expression consumers do.
        local_call_flow::stage_expression_actuals_v1(
            input, statement, &source, locals, prefix_known, homes, local_calls, path_calls, fields, port,
        )?;
    }
    let Some(prepared) = super::expression_plan::prepare_expression_calls_v1(
        &source,
        super::expression_plan::ExpressionLeafProfileV1::LocalInitializer,
        &BTreeMap::new(),
        homes,
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
    )?
    else {
        if debug {
            eprintln!("[initializer-expression] statement={:?} site={site:?} call preparation unavailable", statement.site());
        }
        return declined(statement.site(), 7);
    };
    let mut reads = Vec::new();
    let mut dynamic_reads = Vec::new();
    for (index, node) in source.nodes().iter().enumerate() {
        if !prepared.is_active_reference(index) {
            continue;
        }
        if !matches!(node.source(), ExpressionNodeSourceV1::Reference(_)) {
            continue;
        }
        let Some((binding, root)) =
            super::argument_expression::entry_formal_reference(input, locals, node.site())
        else {
            continue;
        };
        if let Some(BorrowedCallArgumentsV1::FiniteEntryRead(read)) = port(
            &owned,
            BorrowedCallActualRequestV1::FiniteEntryRead {
                input,
                plan: &source,
                read: node.site(),
                binding,
                root,
            },
        )? {
            reads.push(read);
        } else if let Some(BorrowedCallArgumentsV1::DynamicEntryRead(read)) = port(
            &owned,
            BorrowedCallActualRequestV1::DynamicEntryRead {
                input,
                plan: &source,
                read: node.site(),
                binding,
                root,
            },
        )? {
            dynamic_reads.push(read);
        }
    }
    let has_parent_text = source.nodes().iter().enumerate().any(|(index, node)| {
        matches!(
            node.source(),
            ExpressionNodeSourceV1::Literal(ResolvedLiteralSourceV1::String(_))
        ) || reads.iter().any(|read| read.requirement().node() == index)
            || dynamic_reads
                .iter()
                .any(|read| read.requirement().node() == index)
            || locals.text_reference(node.site()).is_some()
    });
    let signed_entry = root_reference
        && reads
            .iter()
            .any(|read| read.requirement().node() == 0 && read.signed_only());
    let signed_root_sum = if root_reference
        && !signed_entry
        && scalar_source.requires_initializer_use_frame(&owned)?
    {
        locals
            .current_scalar_read_v1(site, local_calls)
            .and_then(|read| scalar_source.scalar_read_sum(&read))
            .filter(|sum| sum.signed_only())
    } else {
        None
    };
    if !prepared.has_core()
        && !prepared.has_owned_text_call()
        && !prepared.has_dynamic_invocation_call()
        && !(root_reference && locals.dynamic_invocation_result_reference_v1(site).is_some())
        && !(root_compare && (has_parent_text || prepared.has_dynamic_invocation_call()))
        && !(root_add && has_parent_text)
        && !signed_entry
        && signed_root_sum.is_none()
        && !arithmetic_requested
    {
        if debug {
            eprintln!("[initializer-expression] statement={:?} site={site:?} no admissible call or root", statement.site());
        }
        return declined(statement.site(), 8);
    }
    let mut installation = None;
    let plan = qualify_initializer_expression_v1(
        prepared,
        &reads,
        &dynamic_reads,
        &mut |site| locals.dynamic_invocation_result_reference_v1(site),
        signed_root_sum,
        &mut |site| super::argument_expression::entry_formal_reference(input, locals, site),
        &mut |site, binding, root| locals.entry_null_exclusion(site, binding, root),
        &mut |site| locals.text_reference(site),
        &mut |site| locals.static_text_reference(site),
        &mut |site| locals.current_scalar_read_v1(site, local_calls),
        &mut |site| field_read::expression_field_read_request(input, &source, site, locals),
        &mut |plan| {
            let mut candidate = locals.expression_installation();
            if !candidate.approve(plan) {
                return false;
            }
            installation = Some(candidate);
            true
        },
        &mut |requests, demand| Ok(fields(requests, demand)?.is_some()),
    )?;
    if plan.is_none() && debug {
        eprintln!("[initializer-expression] statement={:?} site={site:?} qualification unavailable reads={}", statement.site(), reads.len());
    }
    let Some(plan) = plan else { return declined(statement.site(), 10) };
    let frame = std::rc::Rc::clone(plan.initializer_frame().expect("Local initializer purpose"));
    let calls = frame.evaluation().calls();
    installation
        .expect("approved initializer installation")
        .commit();
    path_calls.extend(calls.iter().map(|call| call.site().clone()));
    local_calls.extend_from_slice(calls);
    if frame.is_text_residence() {
        homes.push(LiveHomeV1::Text(frame.text_acquisition_v1()));
    }
    if debug {
        eprintln!("[initializer-expression] statement={:?} installed i64={} dynamic_result={} dynamic_alias={} text={}",
            statement.site(), frame.is_i64(), frame.is_dynamic_invocation_result(), frame.is_dynamic_alias(), frame.is_text_residence());
    }
    if frame.is_bool() {
        locals.install_scalar_call_result(frame.binding(), SourceScalarKind::Bool);
    } else if frame.is_i64() {
        locals.install_i64_call_result(frame.binding());
    } else if frame.is_dynamic_invocation_result() {
        locals.install_dynamic_invocation_result(frame.binding(), std::rc::Rc::clone(&frame));
        homes.push(LiveHomeV1::Stable(frame.binding()));
    } else if frame.is_dynamic_alias() {
        locals.install_dynamic_invocation_result(frame.binding(), std::rc::Rc::clone(&frame));
    } else if frame.is_text_residence() {
        locals.install_text_initializer(frame);
    } else {
        return declined(statement.site(), 9);
    }
    Ok(true)
}

/// Default-off: which gate left this Local initializer to the later lanes.
fn declined<E>(statement: &SourceStmtSiteV1, gate: u32) -> Result<bool, E> {
    if crate::config::env::builder_debug_enabled() {
        eprintln!("[initializer-expression] statement={statement:?} declined gate={gate}");
    }
    Ok(false)
}

#[cfg(test)]
pub(crate) fn observe_for_test<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    parameters: Vec<(
        u32,
        BindingRefV1,
        crate::mir::callable_parameter_contract::CallableParameterContractKindV1,
    )>,
    prefix_known: bool,
    invalid: Option<BindingRefV1>,
    fields: &mut impl FnMut(
        &[LocalFieldReadRequestV1],
        LocalFieldReadDemandV1,
    ) -> Result<Option<Vec<LocalFieldReadResultV1>>, E>,
    port: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<(Vec<HomeExpressionPlanV1>, Vec<LiveHomeV1>), E> {
    let mut locals = PrefixLocalFlow::new(input);
    assert!(locals.install_parameters(parameters));
    if let Some(binding) = invalid {
        locals.install_uninitialized(binding);
    }
    let mut homes = Vec::new();
    let mut calls = Vec::new();
    let mut path = BTreeSet::new();
    let mut known = prefix_known;
    let body = input.source().root_body().unwrap();
    for index in 0..body.statements().len() {
        let statement = input.source().body_stmt(&body, index).unwrap();
        if !matches!(statement.node(), ASTNode::Local { .. }) {
            continue;
        }
        let initializer = input.function().expression_source().initializers()
            .find(|row| matches!(row.declaration_site(), SourceBindingSiteV1::Local { statement: site, ordinal: 0 } if site == statement.site())).unwrap();
        let root = OwnedExprSiteV1::new(
            input.owner(),
            initializer.initializer_site().unwrap().clone(),
        );
        local_call_flow::stage_argument_expressions(
            input, &statement, &root, &locals, known, &homes, &mut calls, &mut path, fields, port,
        )?;
        if !observe(
            input,
            &statement,
            initializer,
            &mut locals,
            &mut homes,
            known,
            &mut calls,
            &mut path,
            &mut NoScalarFieldSourceV1,
            fields,
            port,
        )? {
            // Same generic alias/scalar installation used by the production
            // Local walker after the Text purpose declines this initializer.
            if known {
                if let Some(value) = locals.observe(initializer.initializer_site().unwrap()) {
                    locals.install_observed(initializer.binding(), value);
                } else {
                    known = false;
                }
            }
        }
    }
    Ok((locals.expression_plans(), homes))
}
