//! Observation-preserving sibling of `issue_new_home_prefixes_v1`.
//!
//! Same single source walk, but the caller keeps the selected-New argument
//! observations the walk already computes and installs the caller's declared
//! parameter contracts. Declared contracts are the only installed entry
//! bindings — nothing is invented.

use super::{
    scan_new_home_flow, CallerNewHomePrefixV1, HomePrefixUnavailableV1, LocalFieldReadRequestV1,
    LocalFieldReadResultV1, ResultNewHomePrefixV1, SelectedNewArgumentObservationV1,
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
        // This bounded sibling mints no lexical instance-call rows: the
        // i64 lane membership lives on the verified-completion lane alone.
        &mut |_| Ok(false),
        // The nullable-handle lexical lane likewise lives only on the
        // verified-completion lane — the bounded sibling mints none.
        &mut |_| Ok(false),
        // This bounded sibling mints no receiver-call rows: nullable flow
        // membership lives on the verified-completion lane alone.
        &mut |_| Ok(false),
        // Qualified static-call claims are likewise issued only by the
        // verified-completion lane; the bounded sibling admits none.
        &mut |_| Ok(None),
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
        // The `ArrayBox` field census stays unavailable on this lane —
        // a `me.<field>.get(..)` result claims no i64 here.
        &mut |_, _, _, _, _| Ok(false),
        // The `formal.<field>` index proof stays unavailable on this lane.
        &mut |_, _, _, _, _| Ok(false),
        // Local-initializer field reads stay unavailable on this lane —
        // the verified-completion lane owns the issuer predicate.
        &mut |_, _| Ok(None),
        &mut |_, _| Ok(None),
        // The dominated-view consult stays unavailable on this lane for
        // the same reason — a `Handle` leaf is truthfully uncovered here.
        &mut |_| Ok(false),
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
        ),
    >,
    // The probe must see the same lexical instance-call membership the
    // verified lane sees: an admitted `local x = recv.m(..)` keeps this
    // walk covered, so the readiness gate never under- or over-predicts.
    local_lexical_i64_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
    // The nullable lexical membership must be visible to the probe for
    // the same reason — a `NullableObject` callee's `local x = recv.m(..)`
    // keeps this walk covered exactly as the verified lane admits it.
    local_lexical_nullable_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
    // The probe must see the same qualified static-call membership the
    // verified lane sees: an admitted `local x = Alias.m(..)` keeps this
    // walk covered, so the readiness gate never under- or over-predicts.
    local_static_call: &mut impl FnMut(
        &OwnedExprSiteV1,
    ) -> Result<
        Option<crate::mir::resolved_semantics::home_new_prefix::QualifiedStaticCallClaimV1>,
        E,
    >,
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
    // The probe must see the same `ArrayBox` field-census membership
    // the verified lane sees: an admitted `me.<field>.get(..)` i64 result
    // keeps this walk covered exactly as the verified lane admits it.
    array_i64_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    // The probe must see the same `formal.<field>` index proof the
    // verified lane sees: an admitted `formal.<unique-i64 field>` index
    // keeps this walk covered exactly as the verified lane admits it.
    formal_i64_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    // The probe must see the same local-initializer field-read membership
    // the verified lane sees: an admitted `local x = recv.field` keeps
    // this walk covered, so the readiness gate never under- or
    // over-predicts.
    local_field_read: &mut impl FnMut(
        &[LocalFieldReadRequestV1],
        bool,
    ) -> Result<Option<Vec<LocalFieldReadResultV1>>, E>,
    borrowed_actuals: &mut impl FnMut(
        &crate::mir::resolved_semantics::OwnedExprSiteV1,
        crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualRequestV1<'_>,
    ) -> Result<
        Option<crate::mir::resolved_semantics::home_new_prefix::BorrowedCallArgumentsV1>,
        E,
    >,
    // The probe must see the same dominated-view use membership the
    // verified lane sees: an admitted `ArrayElementValue`/`AddOperand`/
    // `NewArgument` leaf keeps this walk covered exactly as the verified
    // lane admits it — the draft stays the sole admission authority.
    view_use: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
) -> Result<BTreeMap<OwnedExprSiteV1, Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>>, E> {
    scan_new_home_flow(
        input,
        selected,
        parameters,
        entry_home,
        explicit_sites,
        false,
        &BTreeSet::new(),
        field_is_integer,
        &mut |_, _| Ok(false),
        &mut |_| Ok(false),
        &mut |_| Ok(false),
        &mut |_| Ok(false),
        local_lexical_i64_call,
        local_lexical_nullable_call,
        &mut |_| Ok(false),
        local_static_call,
        argument_i64_field,
        scalar_field,
        container_field,
        array_i64_field,
        formal_i64_field,
        local_field_read,
        borrowed_actuals,
        view_use,
    )
    .map(|outcome| outcome.0)
}
