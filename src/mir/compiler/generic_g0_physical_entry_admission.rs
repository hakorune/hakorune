//! Generic G0 physical-entry admission.
//!
//! This is a compiler-side co-seal of existing source products.  It does not
//! create Builder state, a session, a BindingSSA row, or a new semantic fact.
//! The detached skeleton remains the rollback boundary until a later
//! consumer opens the unpublished function transaction.

use crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1;
use crate::mir::resolved_semantics::{
    issue_resolved_block_expr_expectation_v1, FunctionOwnerIdV1,
    VerifiedResolvedBlockExpressionExpectationV1,
};

use super::generic_g0_physical_function_skeleton::
    PreparedGenericG0PhysicalFunctionSkeletonV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum GenericG0PhysicalEntryAdmissionRejectV1 {
    BlockExpr(String),
    OuterIf(String),
    OwnerMismatch,
    CompletionTargetMismatch,
    LaneCountOverflow,
}

/// Mechanical cohort identity carried until the canonical session consumer
/// validates the detached shell.  It is not a source or ABI authority.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GenericG0PhysicalEntryCohortStampV1 {
    owner: FunctionOwnerIdV1,
    lane_count: u32,
    function_name: Box<str>,
}

impl GenericG0PhysicalEntryCohortStampV1 {
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

/// One-shot, callback-scoped Generic admission.  All fields are existing
/// source/cohort products; the wrapper only prevents re-pairing them.
pub(crate) struct GenericG0PhysicalEntryAdmissionV1<'loan, 'source> {
    skeleton: PreparedGenericG0PhysicalFunctionSkeletonV1<'loan, 'source>,
    expectation: VerifiedResolvedBlockExpressionExpectationV1,
    outer_if: VerifiedResolvedFunctionIfControlV1,
    stamp: GenericG0PhysicalEntryCohortStampV1,
}

impl<'loan, 'source> GenericG0PhysicalEntryAdmissionV1<'loan, 'source> {
    pub(crate) fn into_parts(
        self,
    ) -> (
        PreparedGenericG0PhysicalFunctionSkeletonV1<'loan, 'source>,
        VerifiedResolvedBlockExpressionExpectationV1,
        VerifiedResolvedFunctionIfControlV1,
        GenericG0PhysicalEntryCohortStampV1,
    ) {
        (self.skeleton, self.expectation, self.outer_if, self.stamp)
    }
}

/// Co-seal the Generic source views with one detached shell.  Every failure
/// occurs before a Builder/session can be opened.
pub(crate) fn issue_generic_g0_physical_entry_admission_v1<'loan, 'source>(
    skeleton: PreparedGenericG0PhysicalFunctionSkeletonV1<'loan, 'source>,
) -> Result<GenericG0PhysicalEntryAdmissionV1<'loan, 'source>, GenericG0PhysicalEntryAdmissionRejectV1>
{
    let parent = skeleton.parent();
    let parts = parent.physical_emitter_source_parts();
    let input = parts.input();
    let expectation = issue_resolved_block_expr_expectation_v1(
        input.function(),
        parts.body_shape(),
    )
    .map_err(|error| GenericG0PhysicalEntryAdmissionRejectV1::BlockExpr(format!("{error:?}")))?;
    // The Generic source parent retains the exact loop selected by the
    // source cohort.  Do not collapse this to a function-wide singleton:
    // Generic fixtures may legitimately contain a nested loop.
    let loop_site = parts.product().context().loop_site();
    let outer_if = VerifiedResolvedFunctionIfControlV1::empty_for_owned_loop_profile(
        *input,
        loop_site.node(),
    )
    .map_err(GenericG0PhysicalEntryAdmissionRejectV1::OuterIf)?;
    let completion = parts.completion();
    if expectation.owner() != parent.owner()
        || outer_if.owner() != parent.owner()
        || completion.owner() != parent.owner()
    {
        return Err(GenericG0PhysicalEntryAdmissionRejectV1::OwnerMismatch);
    }
    if completion.target_function() != input.function().function_region() {
        return Err(GenericG0PhysicalEntryAdmissionRejectV1::CompletionTargetMismatch);
    }
    let lane_count = skeleton.descriptors().len();
    let lane_count = u32::try_from(lane_count)
        .map_err(|_| GenericG0PhysicalEntryAdmissionRejectV1::LaneCountOverflow)?;
    let stamp = GenericG0PhysicalEntryCohortStampV1 {
        owner: parent.owner(),
        lane_count,
        function_name: skeleton.function().signature.name.clone().into_boxed_str(),
    };
    Ok(GenericG0PhysicalEntryAdmissionV1 {
        skeleton,
        expectation,
        outer_if,
        stamp,
    })
}
