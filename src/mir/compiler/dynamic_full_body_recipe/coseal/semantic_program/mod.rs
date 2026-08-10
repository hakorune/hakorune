//! Atomic Dynamic source/Recipe/JoinSig/After semantic-program boundary.
//!
//! The issuer consumes only the existing exact envelope. It never accepts a
//! caller-supplied owner, Recipe, JoinSig, After, Continuation, or Completion.

mod carrier_rebind;
mod fault_cut_points;
mod ingress;
mod invocation_carrier_lifecycle;
mod operator_carrier_lifecycle;

#[cfg(test)]
mod carrier_rebind_tests;

#[cfg(test)]
mod tests;

#[cfg(test)]
use crate::mir::loop_recipe_contract::VerifiedLoopJoinSigV2;
use crate::mir::loop_recipe_contract::{
    issue_sole_root_carrier_join_closure_v2, LoopBindingKeyV1, LoopJoinClosureRejectV2,
    LoopNodeKeyV1, LoopValueClassV2, VerifiedLoopJoinClosureV2,
};

use super::{DynamicIterationLocalValueRefV2, VerifiedDynamicFullLoopSourceRecipeEnvelopeV2};
pub(in crate::mir) use carrier_rebind::{
    issue_dynamic_carrier_rebind_transaction_program_v1, DynamicCarrierCurrentDispositionV1,
    DynamicCarrierRebindTransactionRejectV1, VerifiedDynamicCarrierRebindTransactionProgramV1,
};
use fault_cut_points::{issue_fault_cut_points_v2, VerifiedDynamicFullLoopFaultCutPointCatalogV2};
pub(in crate::mir) use fault_cut_points::{
    DynamicFullLoopFaultCutPointCatalogRefV2, DynamicFullLoopFaultCutPointV2,
    DynamicFullLoopFaultFamilyV2,
};
pub(in crate::mir) use ingress::{
    issue_dynamic_carrier_ingress_lifecycle_program_v1,
    DynamicCarrierIngressLifecycleProgramRejectV1, VerifiedDynamicCarrierIngressLifecycleProgramV1,
};
use invocation_carrier_lifecycle::{
    issue_invocation_carrier_lifecycle_v1, DynamicInvocationCarrierLifecycleRejectV1,
    VerifiedDynamicInvocationCarrierLifecycleCatalogV1,
};
pub(in crate::mir) use invocation_carrier_lifecycle::{
    DynamicInvocationCarrierDestinationRefV1, DynamicInvocationCarrierLifecycleCatalogRefV1,
    DynamicInvocationCarrierLifecycleRowRefV1, DynamicInvocationCarrierPublicationV1,
};
use operator_carrier_lifecycle::{
    issue_operator_carrier_lifecycle_v1, VerifiedDynamicOperatorCarrierLifecycleCatalogV1,
};
pub(in crate::mir) use operator_carrier_lifecycle::{
    DynamicOperatorCarrierDestinationRefV1, DynamicOperatorCarrierLifecycleCatalogRefV1,
    DynamicOperatorCarrierLifecycleProgramRejectV1, DynamicOperatorCarrierLifecycleRowRefV1,
    DynamicOperatorCarrierPublicationV1,
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

/// The complete Dynamic semantic program plus invocation and operator-result
/// lifecycle relations. It remains non-splittable and performs no rebind or
/// physical cleanup.
#[derive(Debug)]
pub(in crate::mir) struct VerifiedDynamicOperatorCarrierLifecycleProgramV1 {
    invocation_program: VerifiedDynamicInvocationCarrierLifecycleProgramV1,
    operator_lifecycle: VerifiedDynamicOperatorCarrierLifecycleCatalogV1,
}

impl VerifiedDynamicOperatorCarrierLifecycleProgramV1 {
    pub(in crate::mir) fn operator_lifecycle(
        &self,
    ) -> DynamicOperatorCarrierLifecycleCatalogRefV1<'_> {
        self.operator_lifecycle.borrow()
    }

    pub(in crate::mir) fn invocation_lifecycle(
        &self,
    ) -> DynamicInvocationCarrierLifecycleCatalogRefV1<'_> {
        self.invocation_program.invocation_lifecycle()
    }

    pub(in crate::mir) fn after(&self) -> DynamicFullLoopAfterRefV2<'_> {
        self.invocation_program.after()
    }
}

impl VerifiedDynamicInvocationCarrierLifecycleProgramV1 {
    pub(in crate::mir) fn invocation_lifecycle(
        &self,
    ) -> DynamicInvocationCarrierLifecycleCatalogRefV1<'_> {
        self.invocation_lifecycle.borrow()
    }

    pub(in crate::mir) fn after(&self) -> DynamicFullLoopAfterRefV2<'_> {
        self.program.after()
    }

    pub(in crate::mir) fn fault_cut_points(&self) -> DynamicFullLoopFaultCutPointCatalogRefV2<'_> {
        self.program.fault_cut_points()
    }
}

impl VerifiedDynamicFullLoopSemanticProgramV2 {
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

pub(in crate::mir) fn issue_dynamic_operator_carrier_lifecycle_program_v1(
    invocation_program: VerifiedDynamicInvocationCarrierLifecycleProgramV1,
) -> Result<
    VerifiedDynamicOperatorCarrierLifecycleProgramV1,
    DynamicOperatorCarrierLifecycleProgramRejectV1,
> {
    let operator_lifecycle = issue_operator_carrier_lifecycle_v1(&invocation_program)?;
    Ok(VerifiedDynamicOperatorCarrierLifecycleProgramV1 {
        invocation_program,
        operator_lifecycle,
    })
}
