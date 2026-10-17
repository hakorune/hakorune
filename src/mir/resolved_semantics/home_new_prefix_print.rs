//! Install one whole Print from the original pre-statement Home/actual basis.
use super::*;

pub(super) fn observe<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    locals: &PrefixLocalFlow<'_>,
    homes: &[BindingRefV1],
    prefix_known: bool,
    local_calls: &mut Vec<LocalCallObservationV1>,
    path_calls: &mut BTreeSet<OwnedExprSiteV1>,
    fields: &mut impl FnMut(&[LocalFieldReadRequestV1], LocalFieldReadDemandV1)
        -> Result<Option<Vec<LocalFieldReadResultV1>>, E>,
    actuals: &mut impl FnMut(&OwnedExprSiteV1, BorrowedCallActualRequestV1<'_>)
        -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<bool, E> {
    // Facts are retained before taking the scoped expression-store borrow.
    super::branch::stage_unobserved_statement_actuals(input, statement, locals, prefix_known, homes, actuals)?;
    if !prefix_known { return Ok(false); }
    let Some(shape) = input.body_shape() else { return Ok(false) };
    let Ok(source_ledger) = input.forest().callable_source_ledger(input.owner()) else { return Ok(false) };
    let Ok(Some(source)) = crate::mir::resolved_value_profile::expression_plan::statement_expression_source_plan_v1(
        &source_ledger, shape, statement.site()) else { return Ok(false) };
    let mut installation = locals.expression_installation();
    let plan = qualify_print_expression_v1(std::rc::Rc::clone(&source), homes,
        &mut |site| super::field_read::expression_field_read_request(input, &source, site, locals),
        &mut |site| local_call_flow::issue_source_scalar_expression_call(input, statement.site(), site, homes, &mut |binding| locals.field_alias_call_path(binding), actuals),
        &mut |plan| installation.approve(plan),
        &mut |requests, demand| Ok(fields(requests, demand)?.is_some()))?;
    let Some(plan) = plan else { return Ok(false) };
    // The last atomic field batch succeeded. Publication of the exact approved
    // immutable frame and its original call observations cannot reject now.
    let calls = plan.print_frame().expect("Print qualifier retains its frame").calls();
    installation.commit();
    path_calls.extend(calls.iter().map(|call| call.site().clone()));
    local_calls.extend_from_slice(calls);
    Ok(true)
}
