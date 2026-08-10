use crate::mir::builder::VerifiedSourceBackedSameModuleCallableCatalogV1;
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;
use crate::mir::compiler::dynamic_full_body_recipe::VerifiedDynamicExitTransactionCoSealV1;
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

/// The sole owned pre-Builder semantic package for one complete callable batch.
///
/// This product is deliberately non-`Clone` and has no consuming parts API.
/// Later stages must transform it whole instead of pairing a foreign batch,
/// parameter catalog, Dynamic candidate, or private batch slot.
#[derive(Debug)]
pub(crate) struct VerifiedNormalCallableSemanticPackageV1 {
    pub(super) catalog: VerifiedSourceBackedSameModuleCallableCatalogV1,
    pub(super) batch: VerifiedResolvedCallableSemanticBatchV1,
    pub(super) selected: super::selected_mapping::VerifiedSelectedCallableBatchMapV1,
    pub(super) parameter_demands: Box<[OwnedCallableParameterDemandDeclarationV1]>,
    pub(super) dynamic: NormalCallableDynamicProjectionV1,
}

#[derive(Debug)]
pub(super) enum NormalCallableDynamicProjectionV1 {
    ValidUnselected,
    Selected {
        batch_slot: u32,
        owner: FunctionOwnerIdV1,
        program: VerifiedDynamicExitTransactionCoSealV1,
    },
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum NormalCallableDynamicProjectionRefV1<'package> {
    ValidUnselected,
    Selected {
        program: &'package VerifiedDynamicExitTransactionCoSealV1,
    },
}

impl VerifiedNormalCallableSemanticPackageV1 {
    pub(crate) fn source_ast(&self) -> &crate::ast::ASTNode {
        self.batch.source_ast()
    }

    #[cfg(test)]
    pub(crate) fn batch(&self) -> &VerifiedResolvedCallableSemanticBatchV1 {
        &self.batch
    }

    #[cfg(test)]
    pub(crate) fn parameter_declaration_count(&self) -> usize {
        self.parameter_demands.len()
    }

    #[cfg(test)]
    pub(crate) fn parameter_count(&self) -> usize {
        self.parameter_demands
            .iter()
            .map(|row| row.parameters.len())
            .sum()
    }

    #[cfg(test)]
    pub(crate) fn dynamic_projection(&self) -> NormalCallableDynamicProjectionRefV1<'_> {
        match &self.dynamic {
            NormalCallableDynamicProjectionV1::ValidUnselected => {
                NormalCallableDynamicProjectionRefV1::ValidUnselected
            }
            NormalCallableDynamicProjectionV1::Selected { program, .. } => {
                NormalCallableDynamicProjectionRefV1::Selected { program }
            }
        }
    }
}
