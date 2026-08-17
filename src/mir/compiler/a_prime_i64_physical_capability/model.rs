//! Builder-free A-prime physical-demand product.

use crate::ast::DeclarationAttrs;
use crate::mir::builder::SelectedNormalCallableKeyV1;
use crate::mir::builder::{
    NormalCatalogedBoxMethodAdmissionErrorV1, NormalCatalogedBoxMethodDraftAdmissionV1,
};
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSourceIdentityV1;
use crate::mir::compiler::dynamic_full_body_recipe::{
    DynamicAPrimeI64SourceRelationViewV1, DynamicCanonicalSessionAuthorityRefV1,
    DynamicFullLoopPhysicalDemandRejectV2, DynamicFullLoopPhysicalInputRejectV2,
    DynamicInvocationCleanupRowViewV1, PreparedDynamicLoopOperationProgramV2,
    VerifiedDynamicExitTransactionCoSealV1,
};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::function::MirParamDecl;
use crate::mir::EffectMask;

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
    PhysicalFunctionEffect,
    PhysicalFunctionHeader,
    PackagePhysicalHeader,
    CallEdgeCoverage,
    PhysicalHeader(NormalCatalogedBoxMethodAdmissionErrorV1),
}

/// Builder-free physical projection of the already admitted callable header.
/// It owns storage-facing declaration data only; callable meaning remains in
/// the catalog, Completion, and verified Dynamic program owners.
#[derive(Debug)]
pub(in crate::mir) struct APrimePhysicalFunctionHeaderV1 {
    catalog: NormalCatalogedBoxMethodDraftAdmissionV1,
    params: Box<[MirParamDecl]>,
    return_type_name: Option<Box<str>>,
    attrs: DeclarationAttrs,
    uses: Box<[String]>,
    effects: EffectMask,
}

impl APrimePhysicalFunctionHeaderV1 {
    pub(super) fn new(
        catalog: NormalCatalogedBoxMethodDraftAdmissionV1,
        params: Box<[MirParamDecl]>,
        return_type_name: Option<Box<str>>,
        attrs: DeclarationAttrs,
        uses: Box<[String]>,
        effects: EffectMask,
    ) -> Self {
        Self {
            catalog,
            params,
            return_type_name,
            attrs,
            uses,
            effects,
        }
    }

    pub(in crate::mir) fn catalog(&self) -> &NormalCatalogedBoxMethodDraftAdmissionV1 {
        &self.catalog
    }

    pub(in crate::mir) fn params(&self) -> &[MirParamDecl] {
        &self.params
    }

    pub(in crate::mir) fn return_type_name(&self) -> Option<&str> {
        self.return_type_name.as_deref()
    }

    pub(in crate::mir) fn attrs(&self) -> &DeclarationAttrs {
        &self.attrs
    }

    pub(in crate::mir) fn uses(&self) -> &[String] {
        &self.uses
    }

    pub(in crate::mir) const fn effects(&self) -> EffectMask {
        self.effects
    }
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
    input: ResolvedFunctionLoweringInputV1<'program>,
    selected_key: SelectedNormalCallableKeyV1,
    identity: VerifiedResolvedCallableSourceIdentityV1,
    program: &'program VerifiedDynamicExitTransactionCoSealV1,
    source_relation: DynamicAPrimeI64SourceRelationViewV1<'program>,
    operation_program: PreparedDynamicLoopOperationProgramV2<'program>,
    physical_function_header: APrimePhysicalFunctionHeaderV1,
    requirement: APrimeI64PhysicalRequirementV1,
}

impl<'program> VerifiedAPrimeI64PhysicalDemandV1<'program> {
    pub(in crate::mir) fn input(&self) -> ResolvedFunctionLoweringInputV1<'program> {
        self.input
    }

    pub(in crate::mir) fn selected_key(&self) -> &SelectedNormalCallableKeyV1 {
        &self.selected_key
    }

    pub(in crate::mir) fn physical_header(&self) -> &NormalCatalogedBoxMethodDraftAdmissionV1 {
        self.physical_function_header.catalog()
    }

    pub(in crate::mir) fn physical_function_header(&self) -> &APrimePhysicalFunctionHeaderV1 {
        &self.physical_function_header
    }

    pub(in crate::mir) const fn function_effects(&self) -> EffectMask {
        self.physical_function_header.effects()
    }

    pub(in crate::mir) fn with_canonical_session_authority<R>(
        &self,
        callback: impl for<'authority> FnOnce(DynamicCanonicalSessionAuthorityRefV1<'authority>) -> R,
    ) -> R {
        self.program.with_canonical_session_authority(callback)
    }

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
    input: ResolvedFunctionLoweringInputV1<'program>,
    selected_key: SelectedNormalCallableKeyV1,
    identity: VerifiedResolvedCallableSourceIdentityV1,
    program: &'program VerifiedDynamicExitTransactionCoSealV1,
    source_relation: DynamicAPrimeI64SourceRelationViewV1<'program>,
    operation_program: PreparedDynamicLoopOperationProgramV2<'program>,
    physical_function_header: APrimePhysicalFunctionHeaderV1,
) -> VerifiedAPrimeI64PhysicalDemandV1<'program> {
    VerifiedAPrimeI64PhysicalDemandV1 {
        input,
        selected_key,
        identity,
        program,
        source_relation,
        operation_program,
        physical_function_header,
        requirement: APrimeI64PhysicalRequirementV1::DirectExactI64,
    }
}
