//! Builder-free A-prime physical-demand product.

use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSourceIdentityV1;
use crate::mir::compiler::dynamic_full_body_recipe::{
    DynamicAPrimeI64SourceRelationViewV1, DynamicFullLoopPhysicalDemandRejectV2,
    DynamicFullLoopPhysicalInputRejectV2, DynamicInvocationCleanupRowViewV1,
    PreparedDynamicLoopOperationProgramV2, VerifiedDynamicExitTransactionCoSealV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum APrimeI64PhysicalDemandRejectV1 {
    NotSelectedDynamic,
    CallableIdentity,
    ParameterContract,
    SourceRelation(
        crate::mir::compiler::dynamic_full_body_recipe::DynamicAPrimeI64SourceRelationRejectV1,
    ),
    PhysicalInput(DynamicFullLoopPhysicalInputRejectV2),
    PhysicalDemand(DynamicFullLoopPhysicalDemandRejectV2),
    CallEdgeCoverage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum APrimeI64PhysicalRequirementV1 {
    DirectExactI64,
}

/// Complete pre-session A-prime demand for the selected Dynamic callable.
///
/// The product carries only already verified semantic/source views.  Physical
/// values, blocks, MIR instructions, helper calls, and backend receipts begin
/// in the later session-local realization stage.
#[derive(Debug)]
pub(in crate::mir) struct VerifiedAPrimeI64PhysicalDemandV1<'program> {
    identity: VerifiedResolvedCallableSourceIdentityV1,
    program: &'program VerifiedDynamicExitTransactionCoSealV1,
    source_relation: DynamicAPrimeI64SourceRelationViewV1<'program>,
    operation_program: PreparedDynamicLoopOperationProgramV2<'program>,
    requirement: APrimeI64PhysicalRequirementV1,
}

impl<'program> VerifiedAPrimeI64PhysicalDemandV1<'program> {
    pub(in crate::mir) fn identity(&self) -> &VerifiedResolvedCallableSourceIdentityV1 {
        &self.identity
    }

    pub(in crate::mir) const fn requirement(&self) -> APrimeI64PhysicalRequirementV1 {
        self.requirement
    }

    pub(in crate::mir) fn source_relation(&self) -> &DynamicAPrimeI64SourceRelationViewV1<'_> {
        &self.source_relation
    }

    pub(in crate::mir) fn with_cleanup_physical_rows<R>(
        &self,
        callback: impl FnOnce([DynamicInvocationCleanupRowViewV1; 4]) -> R,
    ) -> R {
        self.program.with_cleanup_physical_rows(callback)
    }

    pub(in crate::mir) fn completion_sites(
        &self,
    ) -> Option<[crate::mir::resolved_semantics::SourceStmtSiteV1; 2]> {
        self.program.completion_sites()
    }

    pub(in crate::mir) fn with_operation_program<R>(
        &self,
        callback: impl FnOnce(&PreparedDynamicLoopOperationProgramV2<'_>) -> R,
    ) -> R {
        callback(&self.operation_program)
    }
}

pub(super) fn from_parts<'program>(
    identity: VerifiedResolvedCallableSourceIdentityV1,
    program: &'program VerifiedDynamicExitTransactionCoSealV1,
    source_relation: DynamicAPrimeI64SourceRelationViewV1<'program>,
    operation_program: PreparedDynamicLoopOperationProgramV2<'program>,
) -> VerifiedAPrimeI64PhysicalDemandV1<'program> {
    VerifiedAPrimeI64PhysicalDemandV1 {
        identity,
        program,
        source_relation,
        operation_program,
        requirement: APrimeI64PhysicalRequirementV1::DirectExactI64,
    }
}
