//! Exact source scalar Return/Discard from the same package result loan.
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use super::*;
use crate::mir::normal_callable_semantic_package::BorrowedScalarResultSourceLoanV1;

pub(super) enum SourceScalarDemandV1 {
    Unselected,
    Refused,
    Ready {
        loan: BorrowedScalarResultSourceLoanV1,
        arguments: Box<[LocalCallArgumentV1]>,
    },
}

pub(super) fn demand_v1<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &OwnedExprSiteV1,
    borrowed_arguments: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<SourceScalarDemandV1, E> {
    let Some(response) = borrowed_arguments(
        site,
        BorrowedCallActualRequestV1::SourceScalarResultArguments,
    )?
    else {
        return Ok(SourceScalarDemandV1::Unselected);
    };
    let BorrowedCallArgumentsV1::SourceScalarResult { loan, arguments } = response else {
        return Ok(SourceScalarDemandV1::Refused);
    };
    let target = loan.source();
    if input.owner() != site.owner()
        || target.call_site() != site
        || !input
            .function()
            .method_call(site.site())
            .is_some_and(|call| {
                call.owner() == input.owner()
                    && call.receiver_site() == target.receiver_site()
                    && call.arity() == target.target().arity()
                    && call
                        .arguments()
                        .iter()
                        .map(|argument| argument.site())
                        .eq(target.argument_sites().iter())
            })
        || !lexical_local_call::corroborate_received_arguments_v1(input, site, &arguments)
    {
        return Ok(SourceScalarDemandV1::Refused);
    }
    Ok(SourceScalarDemandV1::Ready { loan, arguments })
}

pub(super) fn observation_v1(
    owner: FunctionOwnerIdV1,
    statement: &SourceStmtSiteV1,
    site: &OwnedExprSiteV1,
    destination: LocalCallDestinationV1,
    prior_homes: &[LiveHomeV1],
    loan: BorrowedScalarResultSourceLoanV1,
    arguments: Box<[LocalCallArgumentV1]>,
) -> LocalCallObservationV1 {
    let result = match loan.kind() {
        SourceScalarKind::Integer => LocalCallResultClassV1::I64,
        SourceScalarKind::Bool => LocalCallResultClassV1::Bool,
    };
    LocalCallObservationV1 {
        owner,
        statement: statement.clone(),
        site: site.clone(),
        destination,
        prior_homes: prior_homes.into(),
        arguments,
        result,
        borrowed_scalar: Some(loan), unit_source: None,
        borrowed_nullable: None, mixed: None, field_alias: None,
    }
}

pub(crate) fn issue_source_scalar_expression_call<E>(
    input: ResolvedFunctionLoweringInputV1<'_>, statement: &SourceStmtSiteV1,
    site: &OwnedExprSiteV1, prior_homes: &[LiveHomeV1],
    receiver_path: &mut impl FnMut(BindingRefV1) -> Option<super::super::FieldAliasPathV1>,
    borrowed_arguments: &mut impl FnMut(&OwnedExprSiteV1, BorrowedCallActualRequestV1<'_>)
        -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<Option<LocalCallObservationV1>, E> {
    let SourceScalarDemandV1::Ready { loan, arguments } =
        demand_v1(input, site, borrowed_arguments)? else { return Ok(None) };
    if loan.kind() != SourceScalarKind::Integer { return Ok(None); }
    let field_alias = loan.source().receiver_binding().ok()
        .and_then(receiver_path);
    Ok(Some(LocalCallObservationV1 {
        owner: input.owner(), statement: statement.clone(), site: site.clone(),
        destination: LocalCallDestinationV1::ExpressionValue, prior_homes: prior_homes.into(),
        arguments, result: LocalCallResultClassV1::I64, borrowed_scalar: Some(loan), unit_source: None, borrowed_nullable: None, mixed: None, field_alias,
    }))
}


/// Selection keeps a refused original scalar demand distinct from absence.
pub(crate) enum SourceScalarConditionCallV1 {
    Unselected,
    Refused,
    Ready(LocalCallObservationV1),
}

pub(crate) fn issue_source_scalar_condition_call_v1<E>(
    input: ResolvedFunctionLoweringInputV1<'_>, statement: &SourceStmtSiteV1,
    site: &OwnedExprSiteV1, prior_homes: &[LiveHomeV1],
    receiver_path: &mut impl FnMut(BindingRefV1) -> Option<super::super::FieldAliasPathV1>,
    borrowed_arguments: &mut impl FnMut(&OwnedExprSiteV1, BorrowedCallActualRequestV1<'_>)
        -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<SourceScalarConditionCallV1, E> {
    if input.function().method_call(site.site()).is_none()
        || input.function().with_if_region_for_condition(site.site(), |row| row.site().clone())
            .ok().as_ref() != Some(statement)
    { return Ok(SourceScalarConditionCallV1::Unselected); }
    let (loan, arguments) = match demand_v1(input, site, borrowed_arguments)? {
        SourceScalarDemandV1::Unselected => return Ok(SourceScalarConditionCallV1::Unselected),
        SourceScalarDemandV1::Refused => return Ok(SourceScalarConditionCallV1::Refused),
        SourceScalarDemandV1::Ready { loan, arguments } if loan.kind() == SourceScalarKind::Bool => (loan, arguments),
        SourceScalarDemandV1::Ready { .. } => return Ok(SourceScalarConditionCallV1::Refused),
    };
    let field_alias = loan.source().receiver_binding().ok().and_then(receiver_path);
    let mut observation = observation_v1(input.owner(), statement, site, LocalCallDestinationV1::ExpressionValue, prior_homes, loan, arguments);
    observation.field_alias = field_alias;
    Ok(SourceScalarConditionCallV1::Ready(observation))
}

/// A whole original If frame borrows both primitive scalar result kinds from
/// the same canonical demand. Print and direct-Bool entry policies stay exact.
pub(crate) fn issue_source_scalar_frame_call_v1<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    source: &std::rc::Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
    site: &OwnedExprSiteV1, prior_homes: &[LiveHomeV1],
    receiver_path: &mut impl FnMut(BindingRefV1) -> Option<super::super::FieldAliasPathV1>,
    actuals: &mut impl FnMut(&OwnedExprSiteV1, BorrowedCallActualRequestV1<'_>)
        -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<Option<LocalCallObservationV1>, E> {
    use crate::mir::resolved_value_profile::expression_plan::{ExpressionPlanRootV1, ExpressionNodeSourceV1};
    let ExpressionPlanRootV1::IfCondition { statement, condition, .. } = source.root()
        else { return Ok(None) };
    if source.owner() != input.owner() || site.owner() != source.owner()
        || input.function().with_if_region_for_condition(condition, |row| row.site().clone()).ok().as_ref() != Some(statement)
        || !source.nodes().iter().any(|node| node.site() == site.site()
            && matches!(node.source(), ExpressionNodeSourceV1::MethodCall))
    { return Ok(None); }
    let SourceScalarDemandV1::Ready { loan, arguments } = demand_v1(input, site, actuals)?
        else { return Ok(None) };
    let field_alias = loan.source().receiver_binding().ok().and_then(receiver_path);
    let mut observation = observation_v1(input.owner(), statement, site, LocalCallDestinationV1::ExpressionValue, prior_homes, loan, arguments);
    observation.field_alias = field_alias;
    Ok(Some(observation))
}


/// One mutable callback lends either the original Core receipt or the existing
/// ordinary Call proof. A refused selected request never enters another writer.
pub(crate) fn issue_source_expression_call_v1<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    source: &std::rc::Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
    site: &OwnedExprSiteV1, prior_homes: &[LiveHomeV1],
    receiver_path: &mut impl FnMut(BindingRefV1) -> Option<super::super::FieldAliasPathV1>,
    port: &mut impl FnMut(&OwnedExprSiteV1, BorrowedCallActualRequestV1<'_>)
        -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<Option<crate::mir::resolved_semantics::home_new_prefix::ExpressionCallSourceV1>, E> {
    use crate::mir::resolved_semantics::home_new_prefix::ExpressionCallSourceV1 as Call;
    match port(site, BorrowedCallActualRequestV1::ExpressionCoreSource { input, plan: source })? {
        Some(BorrowedCallArgumentsV1::ExpressionCoreSource(core)) => return Ok(Some(Call::Core(core))),
        Some(_) => return Ok(None),
        None => {}
    }
    issue_source_scalar_expression_call(input, source.root().statement(), site, prior_homes, receiver_path, port)
        .map(|call| call.map(Call::Ordinary))
}
