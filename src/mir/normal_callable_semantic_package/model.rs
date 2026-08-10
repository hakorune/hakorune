use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;
use crate::mir::compiler::dynamic_full_body_recipe::VerifiedDynamicOperatorCarrierLifecycleProgramV1;
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1, HomeDemandV1};

#[derive(Debug)]
pub(super) struct OwnedCallableParameterDemandV1 {
    pub(super) ordinal: u32,
    pub(super) binding: BindingRefV1,
    pub(super) demand: HomeDemandV1,
}

#[derive(Debug)]
pub(super) struct OwnedCallableParameterDemandDeclarationV1 {
    pub(super) batch_slot: u32,
    pub(super) owner: FunctionOwnerIdV1,
    pub(super) parameters: Box<[OwnedCallableParameterDemandV1]>,
}

/// The sole owned pre-Builder semantic package for the bounded Dynamic lane.
///
/// This product is deliberately non-`Clone` and has no consuming parts API.
/// Later stages must transform it whole instead of pairing a foreign batch,
/// parameter catalog, or Dynamic Recipe candidate.
#[derive(Debug)]
pub(crate) struct VerifiedNormalCallableSemanticDynamicPackageV1 {
    pub(super) batch: VerifiedResolvedCallableSemanticBatchV1,
    pub(super) parameter_demands: Box<[OwnedCallableParameterDemandDeclarationV1]>,
    pub(super) dynamic_batch_slot: u32,
    pub(super) dynamic_owner: FunctionOwnerIdV1,
    pub(super) dynamic_program: VerifiedDynamicOperatorCarrierLifecycleProgramV1,
}

impl VerifiedNormalCallableSemanticDynamicPackageV1 {
    pub(crate) fn batch(&self) -> &VerifiedResolvedCallableSemanticBatchV1 {
        &self.batch
    }

    pub(crate) const fn dynamic_batch_slot(&self) -> u32 {
        self.dynamic_batch_slot
    }

    pub(crate) const fn dynamic_owner(&self) -> FunctionOwnerIdV1 {
        self.dynamic_owner
    }

    pub(crate) fn parameter_declaration_count(&self) -> usize {
        self.parameter_demands.len()
    }

    pub(crate) fn parameter_count(&self) -> usize {
        self.parameter_demands
            .iter()
            .map(|row| row.parameters.len())
            .sum()
    }

    pub(crate) fn dynamic_program(&self) -> &VerifiedDynamicOperatorCarrierLifecycleProgramV1 {
        &self.dynamic_program
    }
}
