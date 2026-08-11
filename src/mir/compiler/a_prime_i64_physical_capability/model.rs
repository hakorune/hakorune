//! Builder-free A-prime physical-demand product.

use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSourceIdentityV1;
use crate::mir::compiler::dynamic_full_body_recipe::{
    DynamicAPrimeI64SourceRelationViewV1, DynamicFullLoopPhysicalInputRejectV2,
    DynamicFullLoopPhysicalInputViewV2,
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
    source_relation: DynamicAPrimeI64SourceRelationViewV1<'program>,
    physical_input: DynamicFullLoopPhysicalInputViewV2<'program>,
    requirement: APrimeI64PhysicalRequirementV1,
}

impl VerifiedAPrimeI64PhysicalDemandV1<'_> {
    pub(in crate::mir) fn identity(&self) -> &VerifiedResolvedCallableSourceIdentityV1 {
        &self.identity
    }

    pub(in crate::mir) const fn requirement(&self) -> APrimeI64PhysicalRequirementV1 {
        self.requirement
    }

    pub(in crate::mir) fn source_relation(&self) -> &DynamicAPrimeI64SourceRelationViewV1<'_> {
        &self.source_relation
    }

    pub(in crate::mir) fn physical_input(&self) -> &DynamicFullLoopPhysicalInputViewV2<'_> {
        &self.physical_input
    }
}

pub(super) fn from_parts<'program>(
    identity: VerifiedResolvedCallableSourceIdentityV1,
    source_relation: DynamicAPrimeI64SourceRelationViewV1<'program>,
    physical_input: DynamicFullLoopPhysicalInputViewV2<'program>,
) -> VerifiedAPrimeI64PhysicalDemandV1<'program> {
    VerifiedAPrimeI64PhysicalDemandV1 {
        identity,
        source_relation,
        physical_input,
        requirement: APrimeI64PhysicalRequirementV1::DirectExactI64,
    }
}
