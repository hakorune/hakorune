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
    local_calls: &[LocalCallObservationV1],
    fields: &mut impl FnMut(&[LocalFieldReadRequestV1], LocalFieldReadDemandV1) -> Result<Option<Vec<LocalFieldReadResultV1>>, E>,
    port: &mut impl FnMut(&OwnedExprSiteV1, BorrowedCallActualRequestV1<'_>) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<(), E> {
    if !prefix_known { return Ok(()); }
    for actual in actuals {
        match &actual.value {
            BorrowedCallActualValueV1::Expression | BorrowedCallActualValueV1::MethodCall(_) | BorrowedCallActualValueV1::String(_)
            | BorrowedCallActualValueV1::SelfRooted { .. } => {},
            BorrowedCallActualValueV1::Binding(_) if locals.static_text_reference(&actual.site).is_some()
                || locals.text_reference(&actual.site).is_some() => {},
            _ => continue,
        }
        let Some(BorrowedCallArgumentsV1::ArgumentExpressionSource(loan)) =
            port(call_site, BorrowedCallActualRequestV1::ArgumentExpressionSource(input, actual.ordinal))? else { continue };
        if loan.plan().root().statement() != statement.site() || loan.plan().nodes().first().is_none_or(|node| node.site() != &actual.site) { continue; }
        let mut reads = Vec::new();
        let mut ready = true;
        for node in loan.plan().nodes() {
            if !loan.text_operand_is(node.site()) || !matches!(node.source(), ExpressionNodeSourceV1::Reference(_)) { continue; }
            if locals.static_text_reference(node.site()).is_some() || locals.text_reference(node.site()).is_some() { continue; }
            let Some((binding, root)) = text_reference(input, locals, node.site()) else { ready = false; break };
            let Some(BorrowedCallArgumentsV1::TextEntryRead(read)) = port(call_site,
                BorrowedCallActualRequestV1::ArgumentTextEntryRead { source: &loan, read: node.site(), binding, root })?
                else { ready = false; break };
            reads.push(read);
        }
        if !ready { continue; }
        let source = std::rc::Rc::clone(loan.plan());
        let mut installation = None;
        if let Some(candidate) = qualify_argument_expression_v1(loan, statement.site(), homes, &reads,
            &mut |site| text_reference(input, locals, site),
            &mut |site| locals.static_text_reference(site),
            &mut |site| locals.text_reference(site),
            &mut |site| locals.current_scalar_read_v1(site, local_calls),
            &mut |site| field_read::expression_field_read_request(input, &source, site, locals),
            &mut |site| local_call_flow::issue_source_expression_call_v1(input, &source, site, homes,
                &mut |binding| locals.field_alias_call_path(binding), port),
            &mut |plan| {
                let mut candidate = locals.expression_installation();
                if !candidate.approve(plan) { return false; }
                installation = Some(candidate);
                true
            },
            &mut |requests, demand| Ok(fields(requests, demand)?.is_some()))? {
            let frame = candidate.argument_frame().expect("argument qualifier retains its frame");
            let Some(BorrowedCallArgumentsV1::ArgumentExpressionFrame(frame)) = port(call_site,
                BorrowedCallActualRequestV1::CompleteArgumentExpression(frame))? else {
                unreachable!("successful argument completion must return its canonical frame");
            };
            // The approved candidate must use the same canonical frame that
            // the pending Call now owns for this current Home walk.
            let mut installation = installation.expect("approved argument installation");
            assert!(installation.retain_approved_argument(frame), "completed argument must match the approved Home frame");
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
    local_call_flow::stage_statement_actuals(input, statement, &locals, prefix_known, &[], &[], fields, port)?;
    Ok(locals.expression_plans())
}
