//! Complete caller-zero V2 Recipe producer for one resolver-backed Dynamic Loop.
//!
//! The producer consumes the source inventory once.  Its non-`Clone` Loop
//! source token moves into the verified artifact; the remaining source facts
//! stay beside that artifact for the later atomic source/envelope co-seal.

mod claims;
mod coseal;
mod mapping;
mod physical_demand;

#[allow(unused_imports)]
pub(in crate::mir) use coseal::{
    issue_dynamic_exit_transaction_coseal_i0, issue_dynamic_full_loop_semantic_program_v2,
    issue_dynamic_full_loop_source_recipe_envelope_v2,
    issue_dynamic_invocation_carrier_lifecycle_program_v1,
    issue_dynamic_invocation_cleanup_projection_i0, DynamicAPrimeI64SourceRelationRejectV1,
    DynamicAPrimeFormalRelationRowV1, DynamicAPrimeI64SourceRelationViewV1,
    DynamicExitTransactionCoSealRejectV1,
    DynamicFullLoopAfterRefV2, DynamicFullLoopFaultCutPointCatalogRefV2,
    DynamicFullLoopFaultCutPointV2, DynamicFullLoopFaultFamilyV2, DynamicFullLoopOperationEffectV2,
    DynamicFullLoopOperationPhysicalRefV2, DYNAMIC_FULL_LOOP_PHYSICAL_OPERATION_COUNT_V2,
    DynamicFullLoopPhysicalInputRejectV2,
    DynamicFullLoopPhysicalInputViewV2, DynamicFullLoopPhysicalItemKindV2,
    DynamicFullLoopPhysicalItemPlacementV2, DynamicFullLoopSemanticProgramRejectV2,
    DynamicCanonicalSessionAuthorityRefV1,
    DynamicFullLoopSourceRecipeEnvelopeRejectV2, DynamicInvocationCarrierDestinationRefV1,
    DynamicInvocationCarrierLifecycleCatalogRefV1,
    DynamicInvocationCarrierLifecycleProgramRejectV1, DynamicInvocationCarrierLifecycleRowRefV1,
    DynamicInvocationCarrierPublicationV1, DynamicInvocationCleanupActionViewV1,
    DynamicInvocationCleanupCurrentDispositionV1, DynamicInvocationCleanupProjectionRejectV1,
    DynamicInvocationCleanupRowKindV1, DynamicInvocationCleanupRowViewV1,
    DynamicIterationLocalValueRefV2, DynamicLoopPhysicalArmV2, DynamicLoopPhysicalControlViewV2,
    VerifiedDynamicExitTransactionCoSealV1, VerifiedDynamicFullLoopSemanticProgramV2,
    VerifiedDynamicFullLoopSourceRecipeEnvelopeV2,
    VerifiedDynamicInvocationCarrierLifecycleProgramV1,
    VerifiedDynamicInvocationCleanupProjectionV1,
};
#[allow(unused_imports)]
pub(in crate::mir) use physical_demand::{
    issue_dynamic_full_loop_operation_physical_demand_v2, DynamicFullLoopPhysicalDemandCoverageV2,
    DynamicFullLoopPhysicalDemandRejectV2, PreparedDynamicLoopOperationProgramV2,
    VerifiedDynamicLoopOperationPhysicalDemandV2,
};

#[cfg(test)]
mod tests;

use crate::mir::loop_recipe_contract::{
    LoopRecipeProducerIdV1, LoopRecipeProvenanceV1, LoopRecipeV2RejectReason, LoopRecipeVerifierV2,
    VerifiedLoopRecipeArtifactV2,
};
use crate::mir::loop_structural_facts::{
    bind_resolved_loop_root_v1, LoopRootSourceBindingRejectV1,
};
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1, ResolvedScopeRegionPairV1,
};

use super::dynamic_full_body_source::{
    DynamicFullBodyBindingRowV1, DynamicFullBodySourceRowV1,
    VerifiedDynamicLoopFullBodySourceInventoryV1,
};
use claims::DynamicFullLoopRecipeClaimsV2;

/// Private parameter facts admitted by the callable package and consumed by
/// the sole bounded Recipe producer.  This is deliberately not a public
/// semantic product: it only preserves the exact A2 contract-to-source
/// relation while the candidate is being co-sealed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum DynamicFullLoopParameterClassV2 {
    Dynamic,
    I64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) struct DynamicFullLoopParameterContractRowV2 {
    pub(in crate::mir) ordinal: u32,
    pub(in crate::mir) binding: BindingRefV1,
    pub(in crate::mir) class: DynamicFullLoopParameterClassV2,
}

#[derive(Debug)]
pub(in crate::mir) struct DynamicFullLoopParameterContractV2 {
    rows: Box<[DynamicFullLoopParameterContractRowV2]>,
}

impl DynamicFullLoopParameterContractV2 {
    pub(in crate::mir) fn new(rows: Box<[DynamicFullLoopParameterContractRowV2]>) -> Self {
        Self { rows }
    }

    fn rows(&self) -> &[DynamicFullLoopParameterContractRowV2] {
        &self.rows
    }
}

#[derive(Debug)]
pub(in crate::mir) struct DynamicFullLoopRetainedSourceV1 {
    owner: FunctionOwnerIdV1,
    frame: LoopExecutionFrameKeyV1,
    scope_region: ResolvedScopeRegionPairV1,
    bindings: Box<[DynamicFullBodyBindingRowV1]>,
    rows: Box<[DynamicFullBodySourceRowV1]>,
    completion: VerifiedFunctionCompletionV1,
    parameter_contract: DynamicFullLoopParameterContractV2,
}

impl DynamicFullLoopRetainedSourceV1 {
    #[cfg(test)]
    fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    #[cfg(test)]
    fn bindings(&self) -> &[DynamicFullBodyBindingRowV1] {
        &self.bindings
    }

    #[cfg(test)]
    fn rows(&self) -> &[DynamicFullBodySourceRowV1] {
        &self.rows
    }

    #[cfg(test)]
    fn completion(&self) -> &VerifiedFunctionCompletionV1 {
        &self.completion
    }

    #[cfg(test)]
    fn frame(&self) -> &LoopExecutionFrameKeyV1 {
        &self.frame
    }

    #[cfg(test)]
    fn scope_region(&self) -> ResolvedScopeRegionPairV1 {
        self.scope_region
    }
}

#[derive(Debug)]
pub(in crate::mir) struct DynamicFullLoopRecipeCandidateV2 {
    source: DynamicFullLoopRetainedSourceV1,
    artifact: VerifiedLoopRecipeArtifactV2,
    claims: DynamicFullLoopRecipeClaimsV2,
}

impl DynamicFullLoopRecipeCandidateV2 {
    #[cfg(test)]
    fn source(&self) -> &DynamicFullLoopRetainedSourceV1 {
        &self.source
    }

    #[cfg(test)]
    fn artifact(&self) -> &VerifiedLoopRecipeArtifactV2 {
        &self.artifact
    }

    fn into_parts(
        self,
    ) -> (
        DynamicFullLoopRetainedSourceV1,
        VerifiedLoopRecipeArtifactV2,
        DynamicFullLoopRecipeClaimsV2,
    ) {
        (self.source, self.artifact, self.claims)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) enum DynamicFullLoopRecipeProducerRejectV2 {
    ParameterContractMismatch,
    SourceRoot(LoopRootSourceBindingRejectV1),
    Recipe(LoopRecipeV2RejectReason),
}

pub(in crate::mir) fn produce_dynamic_full_loop_recipe_v2_with_contract(
    source: VerifiedDynamicLoopFullBodySourceInventoryV1,
    parameter_contract: DynamicFullLoopParameterContractV2,
) -> Result<DynamicFullLoopRecipeCandidateV2, DynamicFullLoopRecipeProducerRejectV2> {
    let owner = source.owner();
    let (membership, bindings, rows, completion) = source.into_parts();
    verify_parameter_contract(&bindings, &parameter_contract)?;
    let (resolved_loop_source, frame, scope_region) = membership.into_parts();
    let source_root = bind_resolved_loop_root_v1(resolved_loop_source)
        .map_err(DynamicFullLoopRecipeProducerRejectV2::SourceRoot)?;

    let verified_recipe = LoopRecipeVerifierV2::verify(mapping::complete_dynamic_loop_recipe_v2())
        .map_err(DynamicFullLoopRecipeProducerRejectV2::Recipe)?;
    let source_binding = source_root.into_root_claim_v2(&verified_recipe);
    let artifact = LoopRecipeVerifierV2::bind_verified_artifact(
        LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::CallableSingleLoopV1),
        source_binding,
        verified_recipe,
    )
    .map_err(DynamicFullLoopRecipeProducerRejectV2::Recipe)?;

    Ok(DynamicFullLoopRecipeCandidateV2 {
        source: DynamicFullLoopRetainedSourceV1 {
            owner,
            frame,
            scope_region,
            bindings,
            rows,
            completion,
            parameter_contract,
        },
        artifact,
        claims: DynamicFullLoopRecipeClaimsV2::exact(),
    })
}

#[cfg(test)]
pub(in crate::mir) fn produce_dynamic_full_loop_recipe_v2(
    source: VerifiedDynamicLoopFullBodySourceInventoryV1,
) -> Result<DynamicFullLoopRecipeCandidateV2, DynamicFullLoopRecipeProducerRejectV2> {
    let contract = exact_test_parameter_contract(&source);
    produce_dynamic_full_loop_recipe_v2_with_contract(source, contract)
}

#[cfg(test)]
pub(in crate::mir) fn exact_test_parameter_contract(
    source: &VerifiedDynamicLoopFullBodySourceInventoryV1,
) -> DynamicFullLoopParameterContractV2 {
    use super::dynamic_full_body_source::DynamicFullBodyBindingRoleV1 as Role;
    use DynamicFullLoopParameterClassV2 as Class;
    let rows = [
        (0, Role::Src, Class::Dynamic),
        (1, Role::Pos, Class::I64),
        (2, Role::End, Class::I64),
        (3, Role::PredChars, Class::Dynamic),
    ]
    .into_iter()
    .map(|(ordinal, role, class)| {
        let binding = source
            .bindings()
            .iter()
            .find(|row| row.role() == role)
            .expect("test source binding")
            .binding();
        DynamicFullLoopParameterContractRowV2 {
            ordinal,
            binding,
            class,
        }
    })
    .collect::<Vec<_>>()
    .into_boxed_slice();
    DynamicFullLoopParameterContractV2::new(rows)
}

fn verify_parameter_contract(
    bindings: &[DynamicFullBodyBindingRowV1],
    parameter_contract: &DynamicFullLoopParameterContractV2,
) -> Result<(), DynamicFullLoopRecipeProducerRejectV2> {
    use super::dynamic_full_body_source::DynamicFullBodyBindingRoleV1 as Role;
    use DynamicFullLoopParameterClassV2 as Class;

    if parameter_contract.rows().len() != 4 {
        return Err(DynamicFullLoopRecipeProducerRejectV2::ParameterContractMismatch);
    }
    let expected = [
        (0, Role::Src, Class::Dynamic),
        (1, Role::Pos, Class::I64),
        (2, Role::End, Class::I64),
        (3, Role::PredChars, Class::Dynamic),
    ];
    for (ordinal, role, class) in expected {
        let Some(row) = parameter_contract
            .rows()
            .iter()
            .find(|row| row.ordinal == ordinal)
        else {
            return Err(DynamicFullLoopRecipeProducerRejectV2::ParameterContractMismatch);
        };
        if row.class != class
            || bindings
                .iter()
                .find(|binding| binding.role() == role)
                .map(|binding| binding.binding())
                != Some(row.binding)
        {
            return Err(DynamicFullLoopRecipeProducerRejectV2::ParameterContractMismatch);
        }
    }
    Ok(())
}
