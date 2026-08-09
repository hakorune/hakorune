//! Atomic Dynamic source/Recipe/JoinSig/After semantic-program boundary.
//!
//! The issuer consumes only the existing exact envelope. It never accepts a
//! caller-supplied owner, Recipe, JoinSig, After, Continuation, or Completion.

mod fault_cut_points;

#[cfg(test)]
mod tests;

#[cfg(test)]
use crate::mir::loop_recipe_contract::VerifiedLoopJoinSigV2;
use crate::mir::loop_recipe_contract::{
    issue_sole_root_carrier_join_closure_v2, LoopBindingKeyV1, LoopJoinClosureRejectV2,
    LoopNodeKeyV1, LoopValueClassV2, VerifiedLoopJoinClosureV2,
};

use super::{DynamicIterationLocalValueRefV2, VerifiedDynamicFullLoopSourceRecipeEnvelopeV2};
use fault_cut_points::{issue_fault_cut_points_v2, VerifiedDynamicFullLoopFaultCutPointCatalogV2};
pub(in crate::mir) use fault_cut_points::{
    DynamicFullLoopFaultCutPointCatalogRefV2, DynamicFullLoopFaultCutPointV2,
    DynamicFullLoopFaultFamilyV2,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum DynamicFullLoopSemanticProgramRejectV2 {
    FaultCutPoints,
    JoinClosure(LoopJoinClosureRejectV2),
}

/// Borrow-only view of the exact After retained by the semantic program.
#[derive(Debug)]
pub(in crate::mir) struct DynamicFullLoopAfterRefV2<'program> {
    control: &'program VerifiedLoopJoinClosureV2,
}

impl DynamicFullLoopAfterRefV2<'_> {
    pub(in crate::mir) fn loop_key(&self) -> LoopNodeKeyV1 {
        self.control.after_loop_key()
    }

    pub(in crate::mir) fn binding(&self) -> LoopBindingKeyV1 {
        self.control.after_binding()
    }

    pub(in crate::mir) fn class(&self) -> LoopValueClassV2 {
        self.control.after_class()
    }
}

/// Complete caller-zero semantic truth for the unchanged Dynamic Loop.
///
/// This product is deliberately non-`Clone` and non-splittable. Later owners
/// must consume it whole rather than pairing an equal-looking Recipe, JoinSig,
/// After, or Completion from another source session.
#[derive(Debug)]
pub(in crate::mir) struct VerifiedDynamicFullLoopSemanticProgramV2<'env, 'decl> {
    envelope: VerifiedDynamicFullLoopSourceRecipeEnvelopeV2<'env, 'decl>,
    fault_cut_points: VerifiedDynamicFullLoopFaultCutPointCatalogV2,
    control: VerifiedLoopJoinClosureV2,
}

impl VerifiedDynamicFullLoopSemanticProgramV2<'_, '_> {
    pub(in crate::mir) fn after(&self) -> DynamicFullLoopAfterRefV2<'_> {
        DynamicFullLoopAfterRefV2 {
            control: &self.control,
        }
    }

    pub(in crate::mir) fn iteration_local(&self) -> DynamicIterationLocalValueRefV2<'_> {
        self.envelope.iteration_local()
    }

    pub(in crate::mir) fn fault_cut_points(&self) -> DynamicFullLoopFaultCutPointCatalogRefV2<'_> {
        self.fault_cut_points.borrow()
    }

    #[cfg(test)]
    fn join_sig(&self) -> &VerifiedLoopJoinSigV2 {
        self.control.join_sig()
    }
}

pub(in crate::mir) fn issue_dynamic_full_loop_semantic_program_v2<'env, 'decl>(
    envelope: VerifiedDynamicFullLoopSourceRecipeEnvelopeV2<'env, 'decl>,
) -> Result<
    VerifiedDynamicFullLoopSemanticProgramV2<'env, 'decl>,
    DynamicFullLoopSemanticProgramRejectV2,
> {
    let fault_cut_points = issue_fault_cut_points_v2(&envelope)
        .map_err(|_| DynamicFullLoopSemanticProgramRejectV2::FaultCutPoints)?;
    let control = issue_sole_root_carrier_join_closure_v2(envelope.artifact.recipe())
        .map_err(DynamicFullLoopSemanticProgramRejectV2::JoinClosure)?;
    Ok(VerifiedDynamicFullLoopSemanticProgramV2 {
        envelope,
        fault_cut_points,
        control,
    })
}
