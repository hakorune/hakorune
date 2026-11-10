//! Observation-preserving sibling of `issue_new_home_prefixes_v1`.
//!
//! Same single source walk, but the caller keeps the selected-New argument
//! observations the walk already computes and installs the caller's declared
//! parameter contracts. Declared contracts are the only installed entry
//! bindings — nothing is invented.

use super::{
    scan_new_home_flow, BorrowedCallActualRequestV1, BorrowedCallArgumentsV1, LocalCallRouteV1,
    BorrowedViewUseRequestV1, CallerNewHomePrefixV1, HomeFieldSourceConsultV1,
    HomePrefixUnavailableV1, LocalFieldReadDemandV1, LocalFieldReadRequestV1,
    LocalFieldReadResultV1, ObjectReturnCallQualificationV1, ResultNewHomePrefixV1,
    SelectedNewArgumentObservationV1,
};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, OwnedExprSiteV1, SourceExprSiteV1, VerifiedInstanceEntryHomeLoanV1,
};
use std::collections::{BTreeMap, BTreeSet};

pub(crate) fn issue_new_home_prefixes_with_arguments_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    selected: &BTreeMap<OwnedExprSiteV1, BindingRefV1>,
    result_sites: &BTreeSet<OwnedExprSiteV1>,
    parameters: impl IntoIterator<
        Item = (
            u32,
            BindingRefV1,
            crate::mir::callable_parameter_contract::CallableParameterContractKindV1,
            crate::mir::resolved_semantics::HomeDemandV1,
        ),
    >,
    entry_home: Option<&super::VerifiedInstanceEntryHomeLoanV1>,
    source_value_class_facts: Option<std::rc::Rc<crate::mir::normal_callable_semantic_package::SourceValueClassFactsV1>>,
) -> (
    BTreeMap<OwnedExprSiteV1, Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>>,
    BTreeMap<OwnedExprSiteV1, SelectedNewArgumentObservationV1>,
    BTreeMap<OwnedExprSiteV1, Result<ResultNewHomePrefixV1, HomePrefixUnavailableV1>>,
) {
    let (prefixes, _, _, observations, result_prefixes) = scan_new_home_flow(
        input,
        selected,
        parameters,
        entry_home,
        source_value_class_facts,
        &[],
        false,
        result_sites,
        &mut |_, _| Ok::<_, std::convert::Infallible>(false),
        &mut |_| Ok(false),
        // The bounded sibling mints no call routes: lexical, receiver and
        // static membership live on the verified-completion lane alone.
        &mut |_| Ok(None),
        &mut crate::mir::resolved_semantics::home_new_prefix::NoScalarFieldSourceV1,
        &mut |_, _| Ok(None),
        // The dominated-view consult stays unavailable on this lane for
        // the same reason — a `Handle` leaf is truthfully uncovered here.
        &mut |_, _| Ok(false),
        &mut |_| Ok(None),
        &mut |_| Ok(None),
    )
    .unwrap_or_else(|never| match never {});
    (prefixes, observations, result_prefixes)
}

/// Readiness probe for the caller-side `new` gate. The verified-completion
/// lane runs the issuer's field predicates; this probe must predict that
/// lane faithfully, so it shares the caller's predicate authority instead of
/// the always-false stubs — an argument `me.f` read the issuer can prove must
/// not look uncovered here. Nothing is staged: the proof is re-derived by the
/// verified walk, which owns the ledger rows.
pub(crate) fn issue_new_home_prefixes_probing_fields_v1<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    selected: &BTreeMap<OwnedExprSiteV1, BindingRefV1>,
    entry_home: Option<&VerifiedInstanceEntryHomeLoanV1>,
    explicit_sites: &[crate::mir::resolved_semantics::SourceStmtSiteV1],
    // The probe installs the caller's declared parameter contracts —
    // identical to the verified lane — so a scalar-binding argument to a
    // qualified static call is observed exactly once by one authority.
    parameters: impl IntoIterator<
        Item = (
            u32,
            BindingRefV1,
            crate::mir::callable_parameter_contract::CallableParameterContractKindV1,
            crate::mir::resolved_semantics::HomeDemandV1,
        ),
    >,
    // The probe asks the same route question as the verified lane, narrowed
    // by the package to the static and lexical routes: an admitted
    // `local x = Alias.m(..)` or `local x = recv.m(..)` keeps this walk
    // covered, so the readiness gate never under- or over-predicts.
    local_call_route: &mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<LocalCallRouteV1>, E>,
    scalar_field: &mut impl crate::mir::resolved_semantics::home_new_prefix::HomeFieldSourceConsultV1<E>,
    borrowed_actuals: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
    // The probe must see the same dominated-view use membership the
    // verified lane sees: an admitted `ArrayElementValue`/`AddOperand`/
    // `NewArgument` leaf keeps this walk covered exactly as the verified
    // lane admits it — the draft stays the sole admission authority.
    view_use: &mut impl FnMut(&OwnedExprSiteV1, BorrowedViewUseRequestV1<'_>) -> Result<bool, E>,

    object_return: &mut impl FnMut(
        &OwnedExprSiteV1,
    ) -> Result<Option<ObjectReturnCallQualificationV1>, E>,
) -> Result<BTreeMap<OwnedExprSiteV1, Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>>, E> {
    scan_new_home_flow(
        input,
        selected,
        parameters,
        entry_home,
        None,
        explicit_sites,
        false,
        &BTreeSet::new(),

        &mut |_, _| Ok(false),
        &mut |_| Ok(false),
        local_call_route,
        scalar_field,
        borrowed_actuals,
        view_use,
        &mut |_| Ok(None),
        object_return,
    )
    .map(|outcome| outcome.0)
}
