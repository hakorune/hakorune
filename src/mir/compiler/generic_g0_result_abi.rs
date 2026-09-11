//! Same-cohort Generic G0 result ABI transport.
//!
//! The selected Generic observation already owns the source result ABI.  This
//! module only verifies its identity and declaration parity before retaining
//! one private row in the Generic source parent.  It does not classify a new
//! ABI, rescan syntax, or issue physical function metadata.

use crate::mir::exact_trivial_return_abi::ExactTrivialReturnAbiV1;
use crate::mir::loop_route_policy::{
    CanonicalLoopFamilyCandidateV1, CanonicalLoopFamilySelectionV1,
};
use crate::mir::resolved_semantics::{
    FunctionOriginV1, FunctionOwnerIdV1, SemanticOwnerSourceKindV1,
};
use crate::mir::loop_structural_facts::generic_g0::{
    VerifiedGenericG0PolicyHandoffV1, VerifiedGenericStructuralFactsG0,
};

use super::function_input::ResolvedFunctionLoweringInputV1;
use super::generic_g0_top_level_declaration_header::
    VerifiedGenericG0TopLevelDeclarationHeaderV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericG0ResultAbiRejectV1 {
    SelectionFamilyMismatch,
    CandidateOwnerMismatch,
    CandidateOriginMismatch,
    CandidateSourceKindMismatch,
    CandidateSiteMismatch,
    CandidateFrameMismatch,
    HeaderOwnerMismatch,
    HeaderOriginMismatch,
    HeaderSourceKindMismatch,
    ReturnAnnotationMissing,
    ReturnAbiMismatch,
}

/// Opaque source result ABI retained by the Generic source parent.
///
/// This is not a physical function signature and carries no `ValueId`, MIR
/// type, Completion, or Builder/session state.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedGenericG0ResultAbiV1 {
    owner: FunctionOwnerIdV1,
    origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    abi: ExactTrivialReturnAbiV1,
}

impl VerifiedGenericG0ResultAbiV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn origin(&self) -> FunctionOriginV1 {
        self.origin
    }

    pub(crate) const fn source_kind(&self) -> SemanticOwnerSourceKindV1 {
        self.source_kind
    }

    pub(crate) const fn abi(&self) -> ExactTrivialReturnAbiV1 {
        self.abi
    }
}

pub(crate) fn issue_generic_g0_result_abi_transport_v1(
    input: &ResolvedFunctionLoweringInputV1<'_>,
    selection: &CanonicalLoopFamilySelectionV1,
    header: &VerifiedGenericG0TopLevelDeclarationHeaderV1,
) -> Result<VerifiedGenericG0ResultAbiV1, GenericG0ResultAbiRejectV1> {
    let candidate = match selection.candidate() {
        CanonicalLoopFamilyCandidateV1::GenericG0(candidate) => candidate,
        _ => return Err(GenericG0ResultAbiRejectV1::SelectionFamilyMismatch),
    };
    let structural = candidate.observation().bundle().source().structural();
    issue_generic_g0_result_abi_transport_from_parts(
        input,
        structural,
        selection.lease().site(),
        selection.lease().frame(),
        candidate.observation().bundle().return_abi(),
        header,
    )
}

pub(crate) fn issue_generic_g0_result_abi_transport_from_handoff_v1(
    input: &ResolvedFunctionLoweringInputV1<'_>,
    handoff: &VerifiedGenericG0PolicyHandoffV1,
    header: &VerifiedGenericG0TopLevelDeclarationHeaderV1,
) -> Result<VerifiedGenericG0ResultAbiV1, GenericG0ResultAbiRejectV1> {
    let structural = handoff.bundle().source().structural();
    issue_generic_g0_result_abi_transport_from_parts(
        input,
        structural,
        structural.root_loop(),
        structural.root_frame().clone(),
        handoff.bundle().return_abi(),
        header,
    )
}

fn issue_generic_g0_result_abi_transport_from_parts(
    input: &ResolvedFunctionLoweringInputV1<'_>,
    structural: &VerifiedGenericStructuralFactsG0,
    expected_site: &crate::mir::resolved_semantics::SourceStmtSiteV1,
    expected_frame: crate::mir::resolved_semantics::LoopExecutionFrameKeyV1,
    candidate_abi: ExactTrivialReturnAbiV1,
    header: &VerifiedGenericG0TopLevelDeclarationHeaderV1,
) -> Result<VerifiedGenericG0ResultAbiV1, GenericG0ResultAbiRejectV1> {
    if structural.owner() != input.owner() {
        return Err(GenericG0ResultAbiRejectV1::CandidateOwnerMismatch);
    }
    if structural.origin() != input.function().function_origin() {
        return Err(GenericG0ResultAbiRejectV1::CandidateOriginMismatch);
    }
    if structural.source_kind() != input.function().source_kind() {
        return Err(GenericG0ResultAbiRejectV1::CandidateSourceKindMismatch);
    }
    if expected_site != structural.root_loop() {
        return Err(GenericG0ResultAbiRejectV1::CandidateSiteMismatch);
    }
    if !expected_frame.matches(&structural.root_frame()) {
        return Err(GenericG0ResultAbiRejectV1::CandidateFrameMismatch);
    }
    if header.owner() != input.owner() {
        return Err(GenericG0ResultAbiRejectV1::HeaderOwnerMismatch);
    }
    if header.origin() != input.function().function_origin() {
        return Err(GenericG0ResultAbiRejectV1::HeaderOriginMismatch);
    }
    if header.source_kind() != input.function().source_kind() {
        return Err(GenericG0ResultAbiRejectV1::HeaderSourceKindMismatch);
    }
    let Some(return_type_name) = header.return_type_name() else {
        return Err(GenericG0ResultAbiRejectV1::ReturnAnnotationMissing);
    };
    let Some(header_abi) = ExactTrivialReturnAbiV1::classify(return_type_name) else {
        return Err(GenericG0ResultAbiRejectV1::ReturnAbiMismatch);
    };
    if candidate_abi != header_abi {
        return Err(GenericG0ResultAbiRejectV1::ReturnAbiMismatch);
    }
    Ok(VerifiedGenericG0ResultAbiV1 {
        owner: input.owner(),
        origin: input.function().function_origin(),
        source_kind: input.function().source_kind(),
        abi: candidate_abi,
    })
}
