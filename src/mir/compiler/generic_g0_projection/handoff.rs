//! Caller-zero Generic G0 source-to-policy handoff.
//!
//! This is the sole test-only issuer that co-seals the typed G0 source bundle
//! with the resolver window brand and function-tail completion relation. It
//! consumes one exact `ResolvedFunctionLoweringInputV1`; it retains no AST or
//! source-view object after the handoff is issued.

use super::{
    issue_generic_g0_source_type_bundle_v1, issue_generic_g0_typed_source_bundle_v1,
    GenericG0NumericProjectionRejectV1, GenericG0SourceTypeProjectionRejectV1,
};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::loop_structural_facts::generic_g0::{
    GenericG0PolicyHandoffSealRejectV1, VerifiedGenericG0PolicyHandoffV1,
    VerifiedGenericG0PostLoopReadV1,
};
use crate::mir::numeric_substrate::NumericTarget;
use crate::mir::resolved_semantics::{
    ResolvedExitOriginV1, ResolvedExitSiteV1, SourceStmtSiteV1, VerifiedLoopFamilyWindowLeaseV1,
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum GenericG0PolicyHandoffIssueV1 {
    Source(GenericG0SourceTypeProjectionRejectV1),
    Numeric(GenericG0NumericProjectionRejectV1),
    Window,
    ReturnMissing,
    ReturnOriginMismatch,
    Seal(GenericG0PolicyHandoffSealRejectV1),
}

/// Keep the legacy test wrapper's resolver lookup inside the handoff owner.
///
/// The source-attempt adapter may request a temporary lease for old focused
/// fixtures, but it must not become a second resolver-window owner. The
/// canonical path passes an already-issued lease to the adapter instead.
pub(crate) fn issue_generic_g0_window_for_test(
    input: ResolvedFunctionLoweringInputV1<'_>,
    root_site: &SourceStmtSiteV1,
) -> Option<VerifiedLoopFamilyWindowLeaseV1> {
    input
        .function()
        .issue_loop_family_window_lease_v1(root_site)
        .ok()
}

/// Issue the single co-sealed Generic G0 handoff from one exact source input.
pub(crate) fn issue_generic_g0_policy_handoff_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    target: NumericTarget,
) -> Result<VerifiedGenericG0PolicyHandoffV1, GenericG0PolicyHandoffIssueV1> {
    let source_bundle = issue_generic_g0_source_type_bundle_v1(input)
        .map_err(GenericG0PolicyHandoffIssueV1::Source)?;
    let root_site = source_bundle.structural().root_loop().clone();
    let window_lease = issue_generic_g0_window_for_test(input, &root_site)
        .ok_or(GenericG0PolicyHandoffIssueV1::Window)?;
    finish_generic_g0_policy_handoff_v1(input, source_bundle, &window_lease, target)
}

pub(crate) fn issue_generic_g0_policy_handoff_with_window_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    window_lease: &VerifiedLoopFamilyWindowLeaseV1,
    target: NumericTarget,
) -> Result<VerifiedGenericG0PolicyHandoffV1, GenericG0PolicyHandoffIssueV1> {
    let source_bundle = issue_generic_g0_source_type_bundle_v1(input)
        .map_err(GenericG0PolicyHandoffIssueV1::Source)?;
    finish_generic_g0_policy_handoff_v1(input, source_bundle, window_lease, target)
}

fn finish_generic_g0_policy_handoff_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    source_bundle: super::VerifiedGenericSourceBundleG0,
    window_lease: &VerifiedLoopFamilyWindowLeaseV1,
    target: NumericTarget,
) -> Result<VerifiedGenericG0PolicyHandoffV1, GenericG0PolicyHandoffIssueV1> {
    let bundle = issue_generic_g0_typed_source_bundle_v1(source_bundle, target)
        .map_err(GenericG0PolicyHandoffIssueV1::Numeric)?;
    let structural = bundle.source().structural();
    let return_statement = structural.tail().statement.clone();
    let return_site = ResolvedExitSiteV1::Statement(return_statement.clone());
    let Some(exit) = input.function().resolved_exit(&return_site) else {
        return Err(GenericG0PolicyHandoffIssueV1::ReturnMissing);
    };
    if exit.origin() != ResolvedExitOriginV1::ExplicitReturn {
        return Err(GenericG0PolicyHandoffIssueV1::ReturnOriginMismatch);
    }
    let post_loop_read = VerifiedGenericG0PostLoopReadV1::new(
        return_statement,
        structural.tail().value.clone(),
        structural.tail().binding,
    );
    VerifiedGenericG0PolicyHandoffV1::seal(window_lease, bundle, post_loop_read, target)
        .map_err(GenericG0PolicyHandoffIssueV1::Seal)
}
