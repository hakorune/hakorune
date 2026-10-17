//! Real Local initializer qualification and Normal-only Text installation.
use super::*;
use crate::mir::resolved_value_profile::expression_plan::{local_expression_source_plan_v1, ExpressionNodeSourceV1};

pub(super) fn observe<E>(
    input: ResolvedFunctionLoweringInputV1<'_>, statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    initializer: &super::super::ResolvedInitializerRelationV1,
    locals: &mut PrefixLocalFlow<'_>, homes: &mut Vec<BindingRefV1>, prefix_known: bool,
    local_calls: &mut Vec<LocalCallObservationV1>, path_calls: &mut BTreeSet<OwnedExprSiteV1>,
    fields: &mut impl FnMut(&[LocalFieldReadRequestV1], LocalFieldReadDemandV1) -> Result<Option<Vec<LocalFieldReadResultV1>>, E>,
    port: &mut impl FnMut(&OwnedExprSiteV1, BorrowedCallActualRequestV1<'_>) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<bool, E> {
    if !prefix_known { return Ok(false); }
    let Some(site) = initializer.initializer_site() else { return Ok(false) };
    if initializer.binding().owner() != input.owner()
        || !matches!(initializer.declaration_site(), SourceBindingSiteV1::Local { statement: original, .. } if original == statement.site()) { return Ok(false); }
    if let Some((_, root, acquisition)) = locals.text_reference(site) {
        locals.install_text_alias(initializer.binding(), root, acquisition);
        return Ok(true);
    }
    let Ok(ledger) = input.forest().callable_source_ledger(input.owner()) else { return Ok(false) };
    let Ok(source) = local_expression_source_plan_v1(&ledger, input.body_shape(), initializer) else { return Ok(false) };
    if !source.nodes().first().and_then(|node| node.binary()).is_some_and(|binary|
        binary.operator() == super::super::ResolvedBinaryOperatorV1::Add) { return Ok(false); }
    let owned = OwnedExprSiteV1::new(input.owner(), site.clone());
    let mut reads = Vec::new();
    for (_, node) in source.nodes_outside_calls() {
        if !matches!(node.source(), ExpressionNodeSourceV1::Reference(_)) { continue; }
        let Some((binding, root)) = super::argument_expression::text_reference(input, locals, node.site()) else { continue };
        if let Some(BorrowedCallArgumentsV1::InitializerTextEntryRead(read)) = port(&owned,
            BorrowedCallActualRequestV1::InitializerTextEntryRead { input, plan: &source, read: node.site(), binding, root })? {
            reads.push(read);
        }
    }
    let has_parent_text = source.nodes_outside_calls().any(|(index, node)| {
        matches!(node.source(), ExpressionNodeSourceV1::Literal(ResolvedLiteralSourceV1::String(_)))
            || reads.iter().any(|read| read.requirement().node() == index)
            || locals.text_reference(node.site()).is_some()
    });
    if !has_parent_text { return Ok(false); }
    let mut installation = locals.expression_installation();
    let plan = qualify_text_initializer_v1(std::rc::Rc::clone(&source), homes, &reads,
        &mut |site| super::argument_expression::text_reference(input, locals, site),
        &mut |site, binding, root| locals.entry_null_exclusion(site, binding, root),
        &mut |site| locals.text_reference(site),
        &mut |site| locals.signed_entry_reference(site),
        &mut |site| field_read::expression_field_read_request(input, &source, site, locals),
        &mut |site| local_call_flow::issue_source_scalar_expression_call(input, statement.site(), site, homes,
            &mut |binding| locals.field_alias_call_path(binding), port),
        &mut |plan| installation.approve(plan),
        &mut |requests, demand| Ok(fields(requests, demand)?.is_some()))?;
    let Some(plan) = plan else { return Ok(false) };
    let frame = std::rc::Rc::clone(plan.text_initializer().expect("Text initializer purpose"));
    let calls = frame.evaluation().calls();
    installation.commit();
    path_calls.extend(calls.iter().map(|call| call.site().clone()));
    local_calls.extend_from_slice(calls);
    if frame.owns_root() { homes.push(frame.binding()); }
    locals.install_text_initializer(frame);
    Ok(true)
}

#[cfg(test)]
pub(crate) fn observe_for_test<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    parameters: Vec<(u32, BindingRefV1, crate::mir::callable_parameter_contract::CallableParameterContractKindV1)>,
    prefix_known: bool, invalid: Option<BindingRefV1>,
    fields: &mut impl FnMut(&[LocalFieldReadRequestV1], LocalFieldReadDemandV1) -> Result<Option<Vec<LocalFieldReadResultV1>>, E>,
    port: &mut impl FnMut(&OwnedExprSiteV1, BorrowedCallActualRequestV1<'_>) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<(Vec<HomeExpressionPlanV1>, Vec<BindingRefV1>), E> {
    let mut locals = PrefixLocalFlow::new(input);
    assert!(locals.install_parameters(parameters));
    if let Some(binding) = invalid { locals.install_uninitialized(binding); }
    let mut homes = Vec::new();
    let mut calls = Vec::new();
    let mut path = BTreeSet::new();
    let mut known = prefix_known;
    let body = input.source().root_body().unwrap();
    for index in 0..body.statements().len() {
        let statement = input.source().body_stmt(&body, index).unwrap();
        if !matches!(statement.node(), ASTNode::Local { .. }) { continue; }
        let initializer = input.function().expression_source().initializers()
            .find(|row| matches!(row.declaration_site(), SourceBindingSiteV1::Local { statement: site, ordinal: 0 } if site == statement.site())).unwrap();
        let root = OwnedExprSiteV1::new(input.owner(), initializer.initializer_site().unwrap().clone());
        local_call_flow::stage_argument_expressions(input, &statement, &root, &locals, known, &homes, fields, port)?;
        known &= observe(input, &statement, initializer, &mut locals, &mut homes, known, &mut calls, &mut path, fields, port)?;
    }
    Ok((locals.expression_plans(), homes))
}
