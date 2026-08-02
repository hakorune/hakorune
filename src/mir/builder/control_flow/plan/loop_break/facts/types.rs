//! Phase 29ai P11: loop_break facts type definitions.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::stmt_view::LoopSourceBodySiteV1;
use crate::mir::builder::control_flow::plan::LoopBreakStepPlacement;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopBreakFacts {
    pub loop_var: String,
    pub carrier_var: String,
    pub loop_condition: ASTNode,
    pub break_condition: ASTNode,
    pub carrier_update_in_break: Option<ASTNode>,
    pub carrier_update_in_body: ASTNode,
    pub loop_increment: ASTNode,
    pub step_placement: LoopBreakStepPlacement,
    /// Provenance exists only for the generic direct three-statement branch.
    pub source_topology: Option<LoopBreakSourceTopologyV1>,
}

/// Opaque source coordinates retained by the generic LoopBreak extractor.
///
/// These coordinates describe whole statements only. They never resolve or
/// rebuild AST nodes, and specialized LoopBreak subsets intentionally do not
/// populate them yet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct LoopBreakSourceTopologyV1 {
    break_if: LoopSourceBodySiteV1,
    carrier_update: LoopSourceBodySiteV1,
    step: LoopSourceBodySiteV1,
}

impl LoopBreakSourceTopologyV1 {
    pub(in crate::mir::builder) fn break_if(&self) -> &LoopSourceBodySiteV1 {
        &self.break_if
    }

    pub(in crate::mir::builder) fn carrier_update(&self) -> &LoopSourceBodySiteV1 {
        &self.carrier_update
    }

    pub(in crate::mir::builder) fn step(&self) -> &LoopSourceBodySiteV1 {
        &self.step
    }

    pub(super) fn generic_direct_three(
        break_if: LoopSourceBodySiteV1,
        carrier_update: LoopSourceBodySiteV1,
        step: LoopSourceBodySiteV1,
    ) -> Self {
        Self {
            break_if,
            carrier_update,
            step,
        }
    }
}
