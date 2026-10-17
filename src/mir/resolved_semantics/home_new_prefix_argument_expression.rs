//! Connect the retained source loan at the same real pre-evaluation Prefix point.
use super::*;
use crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1;

pub(super) fn text_reference(input: ResolvedFunctionLoweringInputV1<'_>, locals: &PrefixLocalFlow<'_>,
    site: &SourceExprSiteV1) -> Option<(BindingRefV1, BindingRefV1)> {
    let ResolvedLexicalRefV1::Local(binding) = input.function().variable_ref(site)? else { return None };
    let OrdinaryObservation::Handle(root) = locals.observe(site)? else { return None };
    let super::super::BindingKindV1::Parameter { index } = input.function().binding(root)?.kind() else { return None };
    (locals.is_self_rooted_handle(root)
        && input.function().declaration_binding(&SourceBindingSiteV1::Parameter { index }) == Some(root))
        .then_some((binding, root))
}

pub(super) fn observe<E>(
    input: ResolvedFunctionLoweringInputV1<'_>, statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    call_site: &OwnedExprSiteV1, actuals: &[BorrowedCallActualCandidateV1],
    locals: &PrefixLocalFlow<'_>, prefix_known: bool, homes: &[BindingRefV1],
    fields: &mut impl FnMut(&[LocalFieldReadRequestV1], LocalFieldReadDemandV1) -> Result<Option<Vec<LocalFieldReadResultV1>>, E>,
    port: &mut impl FnMut(&OwnedExprSiteV1, BorrowedCallActualRequestV1<'_>) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<(), E> {
    if !prefix_known { return Ok(()); }
    for actual in actuals {
        if !matches!(actual.value, BorrowedCallActualValueV1::MethodCall(_)) { continue; }
        let Some(BorrowedCallArgumentsV1::ArgumentExpressionSource(loan)) =
            port(call_site, BorrowedCallActualRequestV1::ArgumentExpressionSource(input, actual.ordinal))? else { continue };
        if loan.plan().root().statement() != statement.site() || loan.plan().nodes().first().is_none_or(|node| node.site() != &actual.site) { continue; }
        let mut reads = Vec::new();
        let mut ready = true;
        for node in loan.plan().nodes() {
            if !loan.text_operand_is(node.site()) || !matches!(node.source(), ExpressionNodeSourceV1::Reference(_)) { continue; }
            let Some((binding, root)) = text_reference(input, locals, node.site()) else { ready = false; break };
            let Some(BorrowedCallArgumentsV1::TextEntryRead(read)) = port(call_site,
                BorrowedCallActualRequestV1::ArgumentTextEntryRead { source: &loan, read: node.site(), binding, root })?
                else { ready = false; break };
            reads.push(read);
        }
        if !ready { continue; }
        let source = std::rc::Rc::clone(loan.plan());
        let mut installation = locals.expression_installation();
        if qualify_argument_expression_v1(loan, statement.site(), homes, &reads,
            &mut |site| text_reference(input, locals, site),
            &mut |site| match locals.observe(site) {
                Some(OrdinaryObservation::TrivialLocal(binding, Some(SourceScalarKind::Integer))) => Some((binding, SourceScalarKind::Integer)),
                _ => None,
            },
            &mut |site| field_read::expression_field_read_request(input, &source, site, locals),
            &mut |site| local_call_flow::issue_source_scalar_expression_call(input, statement.site(), site, homes,
                &mut |binding| locals.field_alias_call_path(binding), port),
            &mut |plan| installation.approve(plan),
            &mut |requests, demand| Ok(fields(requests, demand)?.is_some()))?.is_some() {
            installation.commit();
        }
    }
    Ok(())
}

#[cfg(test)]
pub(crate) fn observe_for_test<E>(
    input: ResolvedFunctionLoweringInputV1<'_>, statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    parameters: Vec<(u32, BindingRefV1, crate::mir::callable_parameter_contract::CallableParameterContractKindV1)>,
    prefix_known: bool, unavailable: Option<BindingRefV1>,
    fields: &mut impl FnMut(&[LocalFieldReadRequestV1], LocalFieldReadDemandV1) -> Result<Option<Vec<LocalFieldReadResultV1>>, E>,
    port: &mut impl FnMut(&OwnedExprSiteV1, BorrowedCallActualRequestV1<'_>) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<Vec<HomeExpressionPlanV1>, E> {
    let mut locals = PrefixLocalFlow::new(input);
    assert!(locals.install_parameters(parameters));
    if let Some(binding) = unavailable { locals.install_uninitialized(binding); }
    local_call_flow::stage_statement_actuals(input, statement, &locals, prefix_known, &[], fields, port)?;
    Ok(locals.expression_plans())
}
