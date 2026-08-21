use crate::mir::builder::{
    CatalogedBoxMethodPhysicalHeaderProjectionV1, VerifiedSourceBackedDynamicCallableV1,
    VerifiedSourceBackedSameModuleCallableCatalogV1,
};
use crate::mir::callable_parameter_contract::{
    CallableParameterContractKindV1, CallableParameterDeclarationModeV1,
};
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;
use crate::mir::compiler::dynamic_full_body_recipe::VerifiedDynamicExitTransactionCoSealV1;
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1};
use crate::parser::{ParserNormalProgramSourceLoanRejectV1, ParserNormalProgramSourceLoanV1};
use std::rc::Rc;

#[derive(Debug)]
pub(super) struct OwnedCallableParameterContractV1 {
    pub(super) ordinal: u32,
    pub(super) binding: BindingRefV1,
    pub(super) kind: CallableParameterContractKindV1,
}

#[derive(Debug)]
pub(super) struct OwnedCallableParameterContractDeclarationV1 {
    pub(super) batch_slot: u32,
    pub(super) owner: FunctionOwnerIdV1,
    pub(super) mode: CallableParameterDeclarationModeV1,
    pub(super) parameters: Box<[OwnedCallableParameterContractV1]>,
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
    pub(super) instance_constructors:
        super::instance_constructor_semantic::VerifiedInstanceConstructorSemanticBatchV1,
    pub(super) selected: super::selected_mapping::VerifiedSelectedCallableBatchMapV1,
    pub(super) parameter_contracts: Box<[OwnedCallableParameterContractDeclarationV1]>,
    pub(super) physical_signature:
        super::physical_signature::VerifiedCallablePhysicalSignatureCohortV1,
    pub(super) s6c_child: Option<super::s6c_child::VerifiedS6CSemanticChildV1>,
    pub(super) s6c_storage_header:
        Option<super::s6c_storage_header::VerifiedS6CStorageHeaderProjectionV1>,
    pub(super) physical_header: super::physical_header::VerifiedCallablePhysicalHeaderCohortV1,
    pub(super) dynamic: NormalCallableDynamicProjectionV1,
    pub(super) dynamic_physical_header: Option<CatalogedBoxMethodPhysicalHeaderProjectionV1>,
}

#[derive(Debug)]
pub(super) enum NormalCallableDynamicProjectionV1 {
    ValidUnselected,
    Selected {
        batch_slot: u32,
        owner: FunctionOwnerIdV1,
        source: Rc<VerifiedSourceBackedDynamicCallableV1>,
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

    pub(crate) fn with_normal_program_source_loan<R>(
        &self,
        callback: impl for<'source> FnOnce(ParserNormalProgramSourceLoanV1<'source>) -> R,
    ) -> Result<R, ParserNormalProgramSourceLoanRejectV1> {
        self.batch.with_normal_program_source_loan(callback)
    }

    pub(crate) fn instance_constructors(
        &self,
    ) -> &super::instance_constructor_semantic::VerifiedInstanceConstructorSemanticBatchV1 {
        &self.instance_constructors
    }

    #[cfg(test)]
    pub(crate) fn batch(&self) -> &VerifiedResolvedCallableSemanticBatchV1 {
        &self.batch
    }

    pub(crate) fn selected_callable_sources(
        &self,
    ) -> &crate::mir::builder::VerifiedSelectedNormalCallableSourceInventoryV1 {
        self.catalog.catalog().selected_source_inventory()
    }

    #[cfg(test)]
    pub(crate) fn parameter_declaration_count(&self) -> usize {
        self.parameter_contracts.len()
    }

    #[cfg(test)]
    pub(crate) fn parameter_count(&self) -> usize {
        self.parameter_contracts
            .iter()
            .map(|row| row.parameters.len())
            .sum()
    }

    #[cfg(test)]
    pub(crate) fn physical_signature(
        &self,
    ) -> &super::physical_signature::VerifiedCallablePhysicalSignatureCohortV1 {
        &self.physical_signature
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
