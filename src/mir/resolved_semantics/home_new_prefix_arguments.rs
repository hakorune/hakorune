//! Observation-preserving sibling of `issue_new_home_prefixes_v1`.
//!
//! Same single source walk, but the caller keeps the selected-New argument
//! observations the walk already computes and installs the caller's declared
//! parameter contracts. Declared contracts are the only installed entry
//! bindings — nothing is invented.

use super::{
    scan_new_home_flow, CallerNewHomePrefixV1, HomePrefixUnavailableV1, ResultNewHomePrefixV1,
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
        ),
    >,
    entry_home: Option<&super::VerifiedInstanceEntryHomeLoanV1>,
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
        &[],
        false,
        result_sites,
        &mut |_, _, _, _, _| Ok::<_, std::convert::Infallible>(false),
        &mut |_, _| Ok(false),
        &mut |_| Ok(false),
        &mut |_| Ok(false),
        &mut |_| Ok(false),
        // This bounded sibling mints no receiver-call rows: nullable flow
        // membership lives on the verified-completion lane alone.
        &mut |_| Ok(false),
        // This bounded sibling issues no field-read evidence: argument
        // `me.f` reads stay truthfully unavailable here. Only the
        // verified-completion lane carries the issuer predicate.
        &mut |_, _, _, _, _| Ok(false),
        // Field-write scalar proof stays unavailable on this lane for the
        // same reason — no `me.<field> = ..` statement is admitted here.
        &mut |_, _, _, _, _| Ok(false),
        // Container-field proof stays unavailable on this lane as well —
        // no `me.<field>.m(..)` statement is admitted here.
        &mut |_, _, _, _, _| Ok(false),
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
    field_is_integer: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    argument_i64_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    scalar_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    container_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
) -> Result<BTreeMap<OwnedExprSiteV1, Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>>, E> {
    scan_new_home_flow(
        input,
        selected,
        std::iter::empty(),
        entry_home,
        &[],
        false,
        &BTreeSet::new(),
        field_is_integer,
        &mut |_, _| Ok(false),
        &mut |_| Ok(false),
        &mut |_| Ok(false),
        &mut |_| Ok(false),
        &mut |_| Ok(false),
        argument_i64_field,
        scalar_field,
        container_field,
    )
    .map(|outcome| outcome.0)
}
