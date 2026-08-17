//! Detached Generic G0 physical-entry canary.
//!
//! This is a compiler-side co-seal of existing source products.  It does not
//! create Builder state, a session, a BindingSSA row, or a new semantic fact.
//! The detached skeleton remains a caller-zero canary until the combined
//! emitter admission is consumed by the unpublished function transaction.

use crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1;
use crate::mir::resolved_semantics::{
    issue_resolved_block_expr_expectation_v1, FunctionOwnerIdV1,
    VerifiedResolvedBlockExpressionExpectationV1,
};

use super::generic_g0_physical_function_skeleton::
    PreparedGenericG0PhysicalFunctionSkeletonV1;
use super::generic_g0_source_parent::GenericG0SourceParentRefV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum GenericG0DetachedEntryCanaryRejectV1 {
    BlockExpr(String),
    OuterIf(String),
    OwnerMismatch,
    CompletionTargetMismatch,
    LaneCountOverflow,
}

/// Mechanical cohort identity carried until the canonical session consumer
/// validates the detached shell.  It is not a source or ABI authority.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GenericG0DetachedEntryCanaryStampV1 {
    owner: FunctionOwnerIdV1,
    lane_count: u32,
    function_name: Box<str>,
}

impl GenericG0DetachedEntryCanaryStampV1 {
    pub(crate) fn validate(
        &self,
        owner: FunctionOwnerIdV1,
        function_name: &str,
        lane_count: usize,
    ) -> Result<(), String> {
        let lane_count = u32::try_from(lane_count)
            .map_err(|_| "generic physical entry lane count overflow".to_owned())?;
        if self.owner != owner
            || self.lane_count != lane_count
            || self.function_name.as_ref() != function_name
        {
            return Err("generic physical entry cohort stamp drift".to_owned());
        }
        Ok(())
    }
}

/// Source-owned control facts shared by the old detached canary and the new
/// combined emitter admission.  This is an aggregate of existing resolver
/// products, not a second control authority.
pub(in crate::mir::compiler) struct PreparedGenericG0EntryControlFactsV1 {
    expectation: VerifiedResolvedBlockExpressionExpectationV1,
    outer_if: VerifiedResolvedFunctionIfControlV1,
}

impl PreparedGenericG0EntryControlFactsV1 {
    pub(in crate::mir::compiler) fn expectation(
        &self,
    ) -> &VerifiedResolvedBlockExpressionExpectationV1 {
        &self.expectation
    }

    pub(in crate::mir::compiler) fn outer_if(&self) -> &VerifiedResolvedFunctionIfControlV1 {
        &self.outer_if
    }
}

/// One-shot detached entry canary.  All fields are existing source/cohort
/// products; the wrapper only prevents re-pairing them in the legacy probe.
pub(crate) struct GenericG0DetachedEntryCanaryV1<'loan, 'source> {
    skeleton: PreparedGenericG0PhysicalFunctionSkeletonV1<'loan, 'source>,
    expectation: VerifiedResolvedBlockExpressionExpectationV1,
    outer_if: VerifiedResolvedFunctionIfControlV1,
    stamp: GenericG0DetachedEntryCanaryStampV1,
}

impl<'loan, 'source> GenericG0DetachedEntryCanaryV1<'loan, 'source> {
    pub(crate) fn into_parts(
        self,
    ) -> (
        PreparedGenericG0PhysicalFunctionSkeletonV1<'loan, 'source>,
        VerifiedResolvedBlockExpressionExpectationV1,
        VerifiedResolvedFunctionIfControlV1,
        GenericG0DetachedEntryCanaryStampV1,
    ) {
        (self.skeleton, self.expectation, self.outer_if, self.stamp)
    }
}

/// Issue the reusable resolver-control aggregate from one exact source parent.
/// Completion is validated in the same borrow but remains owned by the parent.
pub(in crate::mir::compiler) fn issue_generic_g0_entry_control_facts_v1(
    parent: &GenericG0SourceParentRefV1<'_, '_>,
) -> Result<PreparedGenericG0EntryControlFactsV1, GenericG0DetachedEntryCanaryRejectV1> {
    let parts = parent.physical_emitter_source_parts();
    let input = parts.input();
    let expectation = issue_resolved_block_expr_expectation_v1(
        input.function(),
        parts.body_shape(),
    )
    .map_err(|error| GenericG0DetachedEntryCanaryRejectV1::BlockExpr(format!("{error:?}")))?;
    // The source parent retains the exact selected loop.  A Generic fixture
    // may contain a nested loop, so a function-wide singleton is insufficient.
    let loop_site = parts.product().context().loop_site();
    let outer_if = VerifiedResolvedFunctionIfControlV1::empty_for_owned_loop_profile(
        *input,
        loop_site.node(),
    )
    .map_err(GenericG0DetachedEntryCanaryRejectV1::OuterIf)?;
    let completion = parts.completion();
    if expectation.owner() != parent.owner()
        || outer_if.owner() != parent.owner()
        || completion.owner() != parent.owner()
    {
        return Err(GenericG0DetachedEntryCanaryRejectV1::OwnerMismatch);
    }
    if completion.target_function() != input.function().function_region() {
        return Err(GenericG0DetachedEntryCanaryRejectV1::CompletionTargetMismatch);
    }
    Ok(PreparedGenericG0EntryControlFactsV1 {
        expectation,
        outer_if,
    })
}

/// Co-seal the Generic source views with one detached canary shell.  Every
/// failure occurs before a Builder/session can be opened.
pub(crate) fn issue_generic_g0_detached_entry_canary_v1<'loan, 'source>(
    skeleton: PreparedGenericG0PhysicalFunctionSkeletonV1<'loan, 'source>,
) -> Result<GenericG0DetachedEntryCanaryV1<'loan, 'source>, GenericG0DetachedEntryCanaryRejectV1>
{
    let parent = skeleton.parent();
    let PreparedGenericG0EntryControlFactsV1 {
        expectation,
        outer_if,
    } = issue_generic_g0_entry_control_facts_v1(parent)?;
    let lane_count = skeleton.descriptors().len();
    let lane_count = u32::try_from(lane_count)
        .map_err(|_| GenericG0DetachedEntryCanaryRejectV1::LaneCountOverflow)?;
    let stamp = GenericG0DetachedEntryCanaryStampV1 {
        owner: parent.owner(),
        lane_count,
        function_name: skeleton.function().signature.name.clone().into_boxed_str(),
    };
    Ok(GenericG0DetachedEntryCanaryV1 {
        skeleton,
        expectation,
        outer_if,
        stamp,
    })
}
