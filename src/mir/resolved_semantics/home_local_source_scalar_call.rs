//! Exact source scalar Return/Discard from the same package result loan.
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use super::*;
use crate::mir::normal_callable_semantic_package::BorrowedScalarResultSourceLoanV1;

/// Source-recorded result class of one local direct-call continuation.
/// `Map` marks a call into an unannotated callee whose sealed terminal
/// relation proves a Map return; `I64` marks the exact-i64 lane.
/// `Handle` marks a call into an unannotated callee whose sealed terminal
/// relation proves a `return new` construction — the caller receives the
/// transferred object as an owned Home and owes exactly one release.
/// `Nullable` marks a `me.m(..)` receiver call whose callee claim is
/// `NullableObject` — the caller receives an owned object or the `Void`
/// sentinel and owes a checked release, never an unconditional one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LocalCallResultClassV1 {
    Unit,
    I64,
    Bool,
    /// The ordinary call transfers one owned String pair only on Normal.
    Text,
    Map,
    Handle,
    Nullable,
    /// Source-proved nullable borrowed identity; owns no received Home.
    BorrowedNullable,
    /// Fresh or borrowed (or Null); non-null alone never proves ownership.
    MixedObject,
}

/// The original result source, shared by expression qualification and the
/// existing Call packet. Retention alone grants no executable result or Home.
#[derive(Debug, Clone)]
pub(crate) enum LocalCallResultSourceV1 {
    Scalar(crate::mir::normal_callable_semantic_package::BorrowedScalarResultSourceLoanV1),
    Static(crate::mir::normal_callable_semantic_package::StaticResultSourceLoanV1),
}
impl PartialEq for LocalCallResultSourceV1 {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Scalar(a), Self::Scalar(b)) => a == b,
            (Self::Static(a), Self::Static(b)) => a == b,
            _ => false,
        }
    }
}
impl Eq for LocalCallResultSourceV1 {}
impl LocalCallResultSourceV1 {
    pub(crate) fn call_site(&self) -> &OwnedExprSiteV1 {
        match self { Self::Scalar(loan) => loan.source().call_site(), Self::Static(loan) => loan.source().call_site() }
    }
    pub(crate) fn receiver_site(&self) -> &SourceExprSiteV1 {
        match self { Self::Scalar(loan) => loan.source().receiver_site(), Self::Static(loan) => loan.source().method_source().receiver_site() }
    }
    pub(crate) fn argument_sites(&self) -> &[SourceExprSiteV1] {
        match self { Self::Scalar(loan) => loan.source().argument_sites(), Self::Static(loan) => loan.source().argument_sites() }
    }
    pub(crate) fn result(&self) -> LocalCallResultClassV1 {
        match self {
            Self::Scalar(loan) => match loan.kind() { SourceScalarKind::Integer => LocalCallResultClassV1::I64, SourceScalarKind::Bool => LocalCallResultClassV1::Bool },
            Self::Static(loan) => loan.result(),
        }
    }
}

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
        BorrowedCallActualRequestV1::SourceResultArguments,
    )?
    else {
        return Ok(SourceScalarDemandV1::Unselected);
    };
    let BorrowedCallArgumentsV1::SourceResult { loan, arguments } = response else {
        return Ok(SourceScalarDemandV1::Refused);
    };
    if !result_matches_source_v1(input, site, &loan, &arguments) {
        return Ok(SourceScalarDemandV1::Refused);
    }
    let LocalCallResultSourceV1::Scalar(loan) = loan else { return Ok(SourceScalarDemandV1::Refused) };
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
    result_observation_v1(owner, statement, site, destination, prior_homes,
        LocalCallResultSourceV1::Scalar(loan), arguments)
}

fn result_matches_source_v1(input: ResolvedFunctionLoweringInputV1<'_>, site: &OwnedExprSiteV1,
    loan: &LocalCallResultSourceV1, arguments: &[LocalCallArgumentV1],
) -> bool {
    let arity = match loan {
        LocalCallResultSourceV1::Scalar(loan) => loan.source().target().arity(),
        LocalCallResultSourceV1::Static(loan) => {
            let source = loan.source();
            if !input.function().method_call_source_v1(site.site())
                .is_some_and(|method| std::sync::Arc::ptr_eq(method, source.method_source())) { return false; }
            source.target().arity()
        }
    };
    input.owner() == site.owner() && loan.call_site() == site
        && input.function().method_call(site.site()).is_some_and(|call|
            call.owner() == input.owner() && call.receiver_site() == loan.receiver_site()
                && call.arity() == arity && arguments.len() == arity as usize
                && call.arguments().iter().map(|argument| argument.site()).eq(loan.argument_sites().iter()))
        && lexical_local_call::corroborate_received_arguments_v1(input, site, arguments)
}

fn result_observation_v1(owner: FunctionOwnerIdV1, statement: &SourceStmtSiteV1,
    site: &OwnedExprSiteV1, destination: LocalCallDestinationV1, prior_homes: &[LiveHomeV1],
    loan: LocalCallResultSourceV1, arguments: Box<[LocalCallArgumentV1]>,
) -> LocalCallObservationV1 {
    LocalCallObservationV1 { owner, statement: statement.clone(), site: site.clone(), destination,
        prior_homes: prior_homes.into(), arguments, result: loan.result(), result_source: Some(loan),
        unit_source: None, borrowed_nullable: None, mixed: None, field_alias: None }
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
        arguments, result: LocalCallResultClassV1::I64, result_source: Some(LocalCallResultSourceV1::Scalar(loan)), unit_source: None, borrowed_nullable: None, mixed: None, field_alias,
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
    let Some(BorrowedCallArgumentsV1::SourceResult { loan, arguments }) =
        port(site, BorrowedCallActualRequestV1::SourceResultArguments)? else { return Ok(None) };
    if !result_matches_source_v1(input, site, &loan, &arguments) { return Ok(None); }
    let field_alias = match &loan {
        LocalCallResultSourceV1::Scalar(loan) if loan.kind() == SourceScalarKind::Integer =>
            loan.source().receiver_binding().ok().and_then(receiver_path),
        LocalCallResultSourceV1::Static(loan) if loan.result() == LocalCallResultClassV1::Text && matches!(source.root(),
            crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { .. }
                | crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::LocalInitializer { .. }) => None,
        _ => return Ok(None),
    };
    let mut observation = result_observation_v1(input.owner(), source.root().statement(), site,
        LocalCallDestinationV1::ExpressionValue, prior_homes, loan, arguments);
    observation.field_alias = field_alias;
    Ok(Some(Call::Ordinary(observation)))
}

/// Direct Static Bool Local uses the same result source and ordered Call proof.
/// It never enters the I64/String initializer expression carrier.
pub(crate) fn issue_static_bool_local_call_v1<E>(
    input: ResolvedFunctionLoweringInputV1<'_>, statement: &SourceStmtSiteV1,
    site: &OwnedExprSiteV1, declaration: SourceBindingSiteV1, destination: BindingRefV1,
    prior_homes: &[LiveHomeV1],
    port: &mut impl FnMut(&OwnedExprSiteV1, BorrowedCallActualRequestV1<'_>)
        -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<Option<LocalCallObservationV1>, E> {
    let Some(BorrowedCallArgumentsV1::SourceResult { loan, arguments }) =
        port(site, BorrowedCallActualRequestV1::SourceResultArguments)? else { return Ok(None); };
    if !matches!(&loan, LocalCallResultSourceV1::Static(source) if source.result() == LocalCallResultClassV1::Bool)
        || !result_matches_source_v1(input, site, &loan, &arguments)
        || !lexical_local_call::corroborate_local_initializer_v1(input, statement, site, &declaration, destination, prior_homes)
        || !lexical_local_call::corroborate_received_arguments_v1(input, site, &arguments) { return Ok(None); }
    Ok(Some(result_observation_v1(input.owner(), statement, site,
        LocalCallDestinationV1::LocalBinding { declaration, binding: destination }, prior_homes, loan, arguments)))
}
