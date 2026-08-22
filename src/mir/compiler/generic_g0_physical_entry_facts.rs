//! Resolver-owned Generic G0 entry-control facts.
//!
//! This module contains only the source-backed validator shared by the
//! combined emitter admission.  The former detached entry canary is retired;
//! these facts do not allocate a function, Builder state, or session.

use crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1;
use crate::mir::resolved_semantics::{
    issue_resolved_block_expr_expectation_v1, VerifiedResolvedBlockExpressionExpectationV1,
};

use super::generic_g0_source_parent::GenericG0SourceParentRefV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum GenericG0PhysicalEntryControlRejectV1 {
    BlockExpr(String),
    OuterIf(String),
    OwnerMismatch,
    CompletionTargetMismatch,
}

/// Existing resolver products co-sealed for the Generic entry consumer.
/// Neither field is a new semantic authority.
pub(in crate::mir::compiler) struct PreparedGenericG0EntryControlFactsV1 {
    expectation: Option<VerifiedResolvedBlockExpressionExpectationV1>,
    outer_if: Option<VerifiedResolvedFunctionIfControlV1>,
}

impl PreparedGenericG0EntryControlFactsV1 {
    pub(in crate::mir::compiler) fn expectation(
        &self,
    ) -> &VerifiedResolvedBlockExpressionExpectationV1 {
        self.expectation
            .as_ref()
            .expect("Generic entry control expectation already consumed")
    }

    pub(in crate::mir::compiler) fn outer_if(&self) -> &VerifiedResolvedFunctionIfControlV1 {
        self.outer_if
            .as_ref()
            .expect("Generic entry control outer-if already consumed")
    }

    pub(in crate::mir::compiler) fn take_expectation(
        &mut self,
    ) -> Result<VerifiedResolvedBlockExpressionExpectationV1, String> {
        self.expectation
            .take()
            .ok_or_else(|| "Generic entry control expectation already consumed".to_owned())
    }

    pub(in crate::mir::compiler) fn take_outer_if(
        &mut self,
    ) -> Result<VerifiedResolvedFunctionIfControlV1, String> {
        self.outer_if
            .take()
            .ok_or_else(|| "Generic entry control outer-if already consumed".to_owned())
    }
}

/// Issue the reusable resolver-control aggregate from one exact source
/// parent. Completion remains owned by that parent and is only checked here.
pub(in crate::mir::compiler) fn issue_generic_g0_entry_control_facts_v1(
    parent: &GenericG0SourceParentRefV1<'_, '_>,
) -> Result<PreparedGenericG0EntryControlFactsV1, GenericG0PhysicalEntryControlRejectV1> {
    let parts = parent.physical_emitter_source_parts();
    let input = parts.input();
    let expectation = issue_resolved_block_expr_expectation_v1(
        input.function(),
        parts.body_shape(),
    )
    .map_err(|error| GenericG0PhysicalEntryControlRejectV1::BlockExpr(format!("{error:?}")))?;
    let loop_site = parts.product().context().loop_site();
    let outer_if = VerifiedResolvedFunctionIfControlV1::empty_for_owned_loop_profile(
        *input,
        loop_site.node(),
    )
    .map_err(GenericG0PhysicalEntryControlRejectV1::OuterIf)?;
    let completion = parts.completion();
    if expectation.owner() != parent.owner()
        || outer_if.owner() != parent.owner()
        || completion.owner() != parent.owner()
    {
        return Err(GenericG0PhysicalEntryControlRejectV1::OwnerMismatch);
    }
    if completion.target_function() != input.function().function_region() {
        return Err(GenericG0PhysicalEntryControlRejectV1::CompletionTargetMismatch);
    }
    Ok(PreparedGenericG0EntryControlFactsV1 {
        expectation: Some(expectation),
        outer_if: Some(outer_if),
    })
}
