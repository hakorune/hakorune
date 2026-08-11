//! Atomic Dynamic source/Recipe/JoinSig/After semantic-program boundary.
//!
//! The issuer consumes only the existing exact envelope. It never accepts a
//! caller-supplied owner, Recipe, JoinSig, After, Continuation, or Completion.

mod exit_transaction;
mod fault_cut_points;
mod invocation_carrier_lifecycle;
mod invocation_cleanup;
mod physical_input;

#[cfg(test)]
mod tests;

use crate::mir::loop_recipe_contract::{
    issue_sole_root_carrier_join_closure_v2, LoopBindingKeyV1, LoopJoinClosureRejectV2,
    LoopJoinLogicalTransferRejectV2, LoopJoinLogicalTransferViewV2, LoopNodeKeyV1,
    LoopValueClassV2, LoopValueKeyV1, VerifiedLoopJoinClosureV2,
};
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, RegionId, SourceStmtSiteV1};

use super::a_prime_source::{
    DynamicAPrimeI64SourceRelationRejectV1, DynamicAPrimeI64SourceRelationViewV1,
};
use super::{DynamicIterationLocalValueRefV2, VerifiedDynamicFullLoopSourceRecipeEnvelopeV2};
pub(in crate::mir) use exit_transaction::{
    issue_dynamic_exit_transaction_coseal_i0, DynamicExitTransactionCoSealRejectV1,
    VerifiedDynamicExitTransactionCoSealV1,
};
use fault_cut_points::{issue_fault_cut_points_v2, VerifiedDynamicFullLoopFaultCutPointCatalogV2};
pub(in crate::mir) use fault_cut_points::{
    DynamicFullLoopFaultCutPointCatalogRefV2, DynamicFullLoopFaultCutPointV2,
    DynamicFullLoopFaultFamilyV2,
};
use invocation_carrier_lifecycle::{
    issue_invocation_carrier_lifecycle_v1, DynamicInvocationCarrierLifecycleRejectV1,
    VerifiedDynamicInvocationCarrierLifecycleCatalogV1,
};
pub(in crate::mir) use invocation_carrier_lifecycle::{
    DynamicInvocationCarrierDestinationRefV1, DynamicInvocationCarrierLifecycleCatalogRefV1,
    DynamicInvocationCarrierLifecycleRowRefV1, DynamicInvocationCarrierPublicationV1,
};
pub(in crate::mir) use invocation_cleanup::{
    issue_dynamic_invocation_cleanup_projection_i0, DynamicInvocationCleanupCurrentDispositionV1,
    DynamicInvocationCleanupProjectionRejectV1, VerifiedDynamicInvocationCleanupProjectionV1,
};
pub(in crate::mir) use physical_input::{
    DynamicFullLoopPhysicalInputRejectV2, DynamicFullLoopPhysicalInputViewV2,
    DynamicLoopPhysicalControlViewV2,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum DynamicFullLoopSemanticProgramRejectV2 {
    FaultCutPoints,
    JoinClosure(LoopJoinClosureRejectV2),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum DynamicInvocationCarrierLifecycleProgramRejectV1 {
    Lifecycle(DynamicInvocationCarrierLifecycleRejectV1),
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
pub(in crate::mir) struct VerifiedDynamicFullLoopSemanticProgramV2 {
    envelope: VerifiedDynamicFullLoopSourceRecipeEnvelopeV2,
    fault_cut_points: VerifiedDynamicFullLoopFaultCutPointCatalogV2,
    control: VerifiedLoopJoinClosureV2,
}

/// The complete Dynamic semantic program plus the complete invocation-result
/// lifecycle family. It stays non-splittable and owns no Home/physical state.
#[derive(Debug)]
pub(in crate::mir) struct VerifiedDynamicInvocationCarrierLifecycleProgramV1 {
    program: VerifiedDynamicFullLoopSemanticProgramV2,
    invocation_lifecycle: VerifiedDynamicInvocationCarrierLifecycleCatalogV1,
}

impl VerifiedDynamicInvocationCarrierLifecycleProgramV1 {
    pub(in crate::mir) fn with_semantic_program<R>(
        &self,
        callback: impl for<'program> FnOnce(&'program VerifiedDynamicFullLoopSemanticProgramV2) -> R,
    ) -> R {
        callback(&self.program)
    }

    pub(in crate::mir) fn invocation_lifecycle(
        &self,
    ) -> DynamicInvocationCarrierLifecycleCatalogRefV1<'_> {
        self.invocation_lifecycle.borrow()
    }

    pub(in crate::mir) fn after(&self) -> DynamicFullLoopAfterRefV2<'_> {
        self.program.after()
    }

    pub(in crate::mir) fn a_prime_source_relation_view(
        &self,
    ) -> Result<DynamicAPrimeI64SourceRelationViewV1<'_>, DynamicAPrimeI64SourceRelationRejectV1>
    {
        self.program.a_prime_source_relation_view()
    }

    pub(in crate::mir) fn physical_input_view(
        &self,
    ) -> Result<DynamicFullLoopPhysicalInputViewV2<'_>, DynamicFullLoopPhysicalInputRejectV2> {
        self.program.physical_input_view()
    }

    pub(in crate::mir) fn fault_cut_points(&self) -> DynamicFullLoopFaultCutPointCatalogRefV2<'_> {
        self.program.fault_cut_points()
    }
}

impl VerifiedDynamicFullLoopSemanticProgramV2 {
    pub(in crate::mir) fn with_a_prime_source_relation<R>(
        &self,
        callback: impl for<'program> FnOnce(DynamicAPrimeI64SourceRelationViewV1<'program>) -> R,
    ) -> Result<R, DynamicAPrimeI64SourceRelationRejectV1> {
        self.envelope.with_a_prime_source_relation(callback)
    }

    pub(in crate::mir) fn a_prime_source_relation_view(
        &self,
    ) -> Result<DynamicAPrimeI64SourceRelationViewV1<'_>, DynamicAPrimeI64SourceRelationRejectV1>
    {
        super::a_prime_source::issue_view(&self.envelope)
    }

    pub(in crate::mir) fn physical_input_view(
        &self,
    ) -> Result<DynamicFullLoopPhysicalInputViewV2<'_>, DynamicFullLoopPhysicalInputRejectV2> {
        physical_input::issue_view(self)
    }

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

    pub(in crate::mir) fn logical_transfer_view(
        &self,
    ) -> Result<LoopJoinLogicalTransferViewV2<'_>, LoopJoinLogicalTransferRejectV2> {
        self.control.logical_transfer_view()
    }

    pub(in crate::mir) fn completion_sites(&self) -> Option<[SourceStmtSiteV1; 2]> {
        let sites = self.envelope.source.completion.explicit_sites();
        if sites.len() != 2 {
            return None;
        }
        Some([sites[0].clone(), sites[1].clone()])
    }

    pub(in crate::mir) fn completion_summary(&self) -> Option<(FunctionOwnerIdV1, RegionId, bool)> {
        let completion = &self.envelope.source.completion;
        (completion.explicit_sites().len() == 2).then_some((
            completion.owner(),
            completion.target_function(),
            completion.returns_value(),
        ))
    }

    pub(in crate::mir) fn recipe_value_class(
        &self,
        key: LoopValueKeyV1,
    ) -> Option<LoopValueClassV2> {
        self.envelope
            .artifact
            .recipe()
            .as_recipe()
            .values
            .iter()
            .find(|row| row.key == key)
            .map(|row| row.class)
    }
}

pub(in crate::mir) fn issue_dynamic_full_loop_semantic_program_v2(
    envelope: VerifiedDynamicFullLoopSourceRecipeEnvelopeV2,
) -> Result<VerifiedDynamicFullLoopSemanticProgramV2, DynamicFullLoopSemanticProgramRejectV2> {
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

pub(in crate::mir) fn issue_dynamic_invocation_carrier_lifecycle_program_v1(
    program: VerifiedDynamicFullLoopSemanticProgramV2,
) -> Result<
    VerifiedDynamicInvocationCarrierLifecycleProgramV1,
    DynamicInvocationCarrierLifecycleProgramRejectV1,
> {
    let invocation_lifecycle = issue_invocation_carrier_lifecycle_v1(&program)
        .map_err(DynamicInvocationCarrierLifecycleProgramRejectV1::Lifecycle)?;
    Ok(VerifiedDynamicInvocationCarrierLifecycleProgramV1 {
        program,
        invocation_lifecycle,
    })
}
