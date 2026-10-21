//! Plain checked rebind; RHS reads the old state before the destination transition.
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use super::*;
use std::rc::Rc;
use crate::mir::resolved_value_profile::expression_plan::assignment_expression_source_plan_v1;

pub(super) fn observe<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    locals: &mut PrefixLocalFlow<'_>, homes: &mut Vec<LiveHomeV1>, known: bool,
    calls: &mut Vec<LocalCallObservationV1>, path_calls: &mut BTreeSet<OwnedExprSiteV1>,
    fields: &mut impl FnMut(&[LocalFieldReadRequestV1], LocalFieldReadDemandV1)
        -> Result<Option<Vec<LocalFieldReadResultV1>>, E>,
    actuals: &mut impl FnMut(&OwnedExprSiteV1, BorrowedCallActualRequestV1<'_>)
        -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<bool, E> {
    if !known || !matches!(statement.node(), ASTNode::Assignment { .. }) { return Ok(false); }
    let Some(shape) = input.body_shape() else { return Ok(false) };
    let mut assignments = shape.assignment_sources().iter().filter(|row| row.statement_site() == statement.site());
    let Some(assignment) = assignments.next() else { return Ok(false) };
    if assignments.next().is_some() { return Ok(false); }
    let Some(super::super::ResolvedAssignmentTargetV1::BindingRebind(target)) =
        input.function().assignment_target(assignment.target_site()) else { return Ok(false) };
    let Some(destination) = locals.rebind_destination_v1(*target, homes) else { return Ok(false) };
    let Ok(ledger) = input.forest().callable_source_ledger(input.owner()) else { return Ok(false) };
    let Ok(source) = assignment_expression_source_plan_v1(&ledger, shape, assignment) else { return Ok(false) };
    let owned = OwnedExprSiteV1::new(input.owner(), assignment.value_site().clone());
    local_call_flow::stage_argument_expressions(input, statement, &owned, locals, known, homes,
        calls, fields, actuals)?;
    let Some(prepared) = expression_plan::prepare_expression_calls_v1(&source,
        expression_plan::ExpressionLeafProfileV1::BindingRebind, &BTreeMap::new(), homes,
        &mut |site| local_call_flow::issue_source_expression_call_v1(input, &source, site, homes,
            &mut |binding| locals.field_alias_call_path(binding), actuals))? else { return Ok(false) };
    let mut installation = None;
    let Some(plan) = qualify_binding_rebind_expression_v1(input, prepared, destination,
        &mut |site| locals.static_text_reference(site), &mut |site| locals.text_reference(site),
        &mut |site| locals.current_scalar_read_v1(site, calls),
        &mut |site| field_read::expression_field_read_request(input, &source, site, locals),
        &mut |plan| {
            let mut pending = locals.expression_installation();
            if !pending.approve(plan) { return false; }
            installation = Some(pending); true
        }, &mut |requests, demand| Ok(fields(requests, demand)?.is_some()))? else { return Ok(false) };
    let frame = plan.rebind_frame().expect("rebind purpose");
    installation.take().expect("approved rebind installation").commit();
    drop(installation);
    path_calls.extend(frame.evaluation().calls().iter().map(|call| call.site().clone()));
    calls.extend_from_slice(frame.evaluation().calls());
    if frame.is_text_residence() {
        homes.retain(|home| home.binding() != *target);
        homes.push(LiveHomeV1::Text(frame.text_acquisition_v1()));
        locals.install_text_rebind(Rc::clone(frame));
    } else {
        let kind = if frame.evaluation().root_class() == crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1::Value(
            crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::Bool) {
            SourceScalarKind::Bool
        } else { SourceScalarKind::Integer };
        locals.install_scalar_call_result(*target, kind);
    }
    Ok(true)
}
