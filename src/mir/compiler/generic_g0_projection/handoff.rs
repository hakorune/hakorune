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
use crate::mir::resolved_semantics::{ResolvedExitOriginV1, ResolvedExitSiteV1};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum GenericG0PolicyHandoffIssueV1 {
    Source(GenericG0SourceTypeProjectionRejectV1),
    Numeric(GenericG0NumericProjectionRejectV1),
    Window,
    ReturnMissing,
    ReturnOriginMismatch,
    Seal(GenericG0PolicyHandoffSealRejectV1),
}

/// Issue the single co-sealed Generic G0 handoff from one exact source input.
pub(crate) fn issue_generic_g0_policy_handoff_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    target: NumericTarget,
) -> Result<VerifiedGenericG0PolicyHandoffV1, GenericG0PolicyHandoffIssueV1> {
    let source_bundle = issue_generic_g0_source_type_bundle_v1(input)
        .map_err(GenericG0PolicyHandoffIssueV1::Source)?;
    let bundle = issue_generic_g0_typed_source_bundle_v1(source_bundle, target)
        .map_err(GenericG0PolicyHandoffIssueV1::Numeric)?;
    let structural = bundle.source().structural();
    let root_site = structural.root_loop().clone();
    let window_lease = input
        .function()
        .issue_loop_family_window_lease_v1(&root_site)
        .map_err(|_| GenericG0PolicyHandoffIssueV1::Window)?;
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
