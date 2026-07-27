use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::callable_result_representation::{
    project_static_exact_i64_requirement_v1, CallableBodyProofIssueErrorV1,
    StaticExactI64RequirementErrorV1, VerifiedSameModuleCallableResultCatalogV1,
};
use crate::mir::resolved_semantics::{ExprChildRoleV1, SourceExprSiteV1, SourcePathSegmentV1};
use crate::mir::source_call_target::{
    RawSourceCursorErrorV1, VerifiedRawCallableSourceViewV1,
    VerifiedWholeSourceStaticCallTargetInventoryV1,
};
use crate::mir::source_instance_result_contract::{
    prepare_preloop_located_argument_v1, prepare_preloop_nested_result_association_v1,
    seal_nested_instance_result_contract, NestedInstanceResultContractErrorV1,
    VerifiedCurrentOwnerInstanceResultTargetV1,
};

use super::activation::{
    prepare_preloop_stageb_carrier_rows_v1, PreparedPreloopStageBCarrierRowsV1,
};
use super::outer_result::seal_preloop_outer_carrier_result_v1;
use super::source_inventory_error::{
    PreloopStageBSourceInventoryCauseV1, PreloopStageBSourceInventoryErrorV1,
    PreloopStageBSourceInventoryStageV1,
};

const SELECTED_ARGUMENT_INDEX: usize = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PreloopStageBCandidateIdentityV1 {
    caller: CanonicalSameModuleCallableKeyV1,
    outer_call_site: SourceExprSiteV1,
    selected_argument_index: u32,
    inner_call_site: SourceExprSiteV1,
    outer_target: CanonicalSameModuleCallableKeyV1,
}

impl PreloopStageBCandidateIdentityV1 {
    pub(crate) const fn caller(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.caller
    }

    pub(crate) const fn outer_call_site(&self) -> &SourceExprSiteV1 {
        &self.outer_call_site
    }

    pub(crate) const fn selected_argument_index(&self) -> u32 {
        self.selected_argument_index
    }

    pub(crate) const fn inner_call_site(&self) -> &SourceExprSiteV1 {
        &self.inner_call_site
    }

    pub(crate) const fn outer_target(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.outer_target
    }
}

#[derive(Debug)]
struct VerifiedPreloopStageBCandidateV1 {
    identity: PreloopStageBCandidateIdentityV1,
    rows: PreparedPreloopStageBCarrierRowsV1,
}

#[derive(Debug)]
pub(crate) struct VerifiedPreloopStageBCandidateInventoryV1 {
    catalog_identity: usize,
    observed_declaration_count: usize,
    observed_method_call_count: usize,
    candidates: Box<[VerifiedPreloopStageBCandidateV1]>,
}

#[derive(Debug)]
pub(super) enum PreloopStageBCandidateCardinalityV1 {
    Zero(VerifiedPreloopStageBCandidateInventoryV1),
    One {
        identity: PreloopStageBCandidateIdentityV1,
        rows: PreparedPreloopStageBCarrierRowsV1,
    },
    Many(VerifiedPreloopStageBCandidateInventoryV1),
}

impl VerifiedPreloopStageBCandidateInventoryV1 {
    pub(crate) fn is_branded_by(
        &self,
        declarations: &crate::mir::builder::VerifiedSameModuleCallableDeclarationCatalogV1,
    ) -> bool {
        self.catalog_identity == declarations as *const _ as usize
    }

    pub(crate) const fn observed_declaration_count(&self) -> usize {
        self.observed_declaration_count
    }

    pub(crate) const fn observed_method_call_count(&self) -> usize {
        self.observed_method_call_count
    }

    pub(crate) const fn candidate_count(&self) -> usize {
        self.candidates.len()
    }

    pub(crate) fn candidate_identities(
        &self,
    ) -> impl Iterator<Item = &PreloopStageBCandidateIdentityV1> {
        self.candidates.iter().map(|candidate| &candidate.identity)
    }

    pub(super) fn classify(self) -> PreloopStageBCandidateCardinalityV1 {
        match self.candidates.len() {
            0 => PreloopStageBCandidateCardinalityV1::Zero(self),
            1 => {
                let Self { candidates, .. } = self;
                let candidate = candidates
                    .into_vec()
                    .pop()
                    .expect("one-candidate cardinality");
                PreloopStageBCandidateCardinalityV1::One {
                    identity: candidate.identity,
                    rows: candidate.rows,
                }
            }
            _ => PreloopStageBCandidateCardinalityV1::Many(self),
        }
    }

    pub(crate) fn discard(self) {}
}

pub(crate) fn inventory_preloop_stageb_candidates_v1(
    inventory: &VerifiedWholeSourceStaticCallTargetInventoryV1<'_>,
) -> Result<VerifiedPreloopStageBCandidateInventoryV1, PreloopStageBSourceInventoryErrorV1> {
    let declarations = inventory.declarations();
    let results =
        VerifiedSameModuleCallableResultCatalogV1::verify(declarations, inventory.targets())
            .map_err(|cause| {
                reject(
                    PreloopStageBSourceInventoryStageV1::CallableResults,
                    None,
                    None,
                    PreloopStageBSourceInventoryCauseV1::CallableResults(cause),
                )
            })?;
    let mut candidates = Vec::new();

    for ((caller, outer_site), _) in inventory.targets().rows() {
        let requirement = match project_static_exact_i64_requirement_v1(
            declarations,
            caller,
            outer_site,
            inventory.targets(),
            &results,
        ) {
            Ok(requirement) => requirement,
            Err(
                StaticExactI64RequirementErrorV1::TargetResultUnavailable
                | StaticExactI64RequirementErrorV1::GeneralCallResultAlreadyAvailable,
            ) => continue,
            Err(cause) => {
                return Err(reject(
                    PreloopStageBSourceInventoryStageV1::OuterRequirement,
                    Some(caller.clone()),
                    Some(outer_site.clone()),
                    PreloopStageBSourceInventoryCauseV1::OuterRequirement(cause),
                ))
            }
        };
        if requirement.required_i64_arguments() != [SELECTED_ARGUMENT_INDEX as u32] {
            continue;
        }
        let [SourcePathSegmentV1::Body(statement_index), SourcePathSegmentV1::Value] =
            outer_site.node().segments()
        else {
            continue;
        };
        let statement_index = usize::try_from(*statement_index).map_err(|_| {
            reject(
                PreloopStageBSourceInventoryStageV1::RawSourceProjection,
                Some(caller.clone()),
                Some(outer_site.clone()),
                PreloopStageBSourceInventoryCauseV1::RawSourceProjection(
                    RawSourceCursorErrorV1::SourceIndexOverflow {
                        caller: caller.clone(),
                        value: usize::MAX,
                        role: "preloop_stageb_statement_index",
                    },
                ),
            )
        })?;
        let view =
            VerifiedRawCallableSourceViewV1::verify(declarations, caller).map_err(|cause| {
                reject(
                    PreloopStageBSourceInventoryStageV1::RawSourceProjection,
                    Some(caller.clone()),
                    Some(outer_site.clone()),
                    PreloopStageBSourceInventoryCauseV1::RawSourceProjection(cause),
                )
            })?;
        let body = view.root_body();
        let statement = view
            .body_stmt(&body, statement_index)
            .map_err(|cause| raw_rejection(caller, outer_site, cause))?;
        let outer = match view.child_expr_from_stmt(&statement, ExprChildRoleV1::AssignmentValue) {
            Ok(outer) => outer,
            Err(RawSourceCursorErrorV1::ExpressionRoleParentMismatch { .. }) => continue,
            Err(cause) => return Err(raw_rejection(caller, outer_site, cause)),
        };
        if outer.site() != outer_site {
            return Err(reject(
                PreloopStageBSourceInventoryStageV1::RawSourceProjection,
                Some(caller.clone()),
                Some(outer_site.clone()),
                PreloopStageBSourceInventoryCauseV1::OuterSiteProjectionMismatch,
            ));
        }
        let outer_input = view
            .method_call_input(&outer)
            .map_err(|cause| raw_rejection(caller, outer_site, cause))?;
        let selected = view
            .method_call_argument(outer_input, SELECTED_ARGUMENT_INDEX)
            .map_err(|rejected| {
                let cause = rejected.cause().clone();
                rejected.discard();
                raw_rejection(caller, outer_site, cause)
            })?;
        let inner_input = match view.method_call_input(selected.child()) {
            Ok(input) => input,
            Err(RawSourceCursorErrorV1::MethodCallRequired { .. }) => {
                selected.discard();
                continue;
            }
            Err(cause) => {
                selected.discard();
                return Err(raw_rejection(caller, outer_site, cause));
            }
        };
        let Some(inner_call) = inventory.call(caller, inner_input.site()) else {
            selected.discard();
            return Err(reject(
                PreloopStageBSourceInventoryStageV1::CompleteObservation,
                Some(caller.clone()),
                Some(outer_site.clone()),
                PreloopStageBSourceInventoryCauseV1::InnerCallMissingFromCompleteInventory,
            ));
        };
        let inner_target = match VerifiedCurrentOwnerInstanceResultTargetV1::seal(inner_call) {
            Ok(target) => target,
            Err(_) => {
                selected.discard();
                continue;
            }
        };
        let inner_proof = match results.issue_unannotated_body_proof(inner_target.target()) {
            Ok(proof) => proof,
            Err(CallableBodyProofIssueErrorV1::DeclaredResultAuthorityForbidden { .. }) => {
                selected.discard();
                continue;
            }
            Err(cause) => {
                selected.discard();
                return Err(reject(
                    PreloopStageBSourceInventoryStageV1::InnerBodyProof,
                    Some(caller.clone()),
                    Some(outer_site.clone()),
                    PreloopStageBSourceInventoryCauseV1::InnerBodyProof(cause),
                ));
            }
        };
        let inner_contract = match seal_nested_instance_result_contract(inner_target, inner_proof) {
            Ok(contract) => contract,
            Err(rejected)
                if !matches!(
                    rejected.cause(),
                    NestedInstanceResultContractErrorV1::TargetProofMismatch
                ) =>
            {
                rejected.discard();
                selected.discard();
                continue;
            }
            Err(rejected) => {
                let cause = rejected.cause().clone();
                rejected.discard();
                selected.discard();
                return Err(reject(
                    PreloopStageBSourceInventoryStageV1::InnerContract,
                    Some(caller.clone()),
                    Some(outer_site.clone()),
                    PreloopStageBSourceInventoryCauseV1::InnerContract(cause),
                ));
            }
        };
        let association = prepare_preloop_nested_result_association_v1(inner_contract, inner_input)
            .map_err(|rejected| {
                let cause = rejected.cause();
                rejected.discard();
                reject(
                    PreloopStageBSourceInventoryStageV1::SourceAssociation,
                    Some(caller.clone()),
                    Some(outer_site.clone()),
                    PreloopStageBSourceInventoryCauseV1::SourceAssociation(cause),
                )
            })?;
        let prepared =
            prepare_preloop_located_argument_v1(selected, association).map_err(|rejected| {
                let cause = rejected.cause();
                rejected.discard();
                reject(
                    PreloopStageBSourceInventoryStageV1::LocatedArgument,
                    Some(caller.clone()),
                    Some(outer_site.clone()),
                    PreloopStageBSourceInventoryCauseV1::LocatedArgument(cause),
                )
            })?;
        let contract =
            seal_preloop_outer_carrier_result_v1(requirement, prepared).map_err(|rejected| {
                let stage = rejected.stage();
                let cause = rejected.cause().clone();
                rejected.discard();
                reject(
                    PreloopStageBSourceInventoryStageV1::OuterContract,
                    Some(caller.clone()),
                    Some(outer_site.clone()),
                    PreloopStageBSourceInventoryCauseV1::OuterContract { stage, cause },
                )
            })?;
        let rows = prepare_preloop_stageb_carrier_rows_v1(contract).map_err(|rejected| {
            let stage = rejected.stage();
            let cause = rejected.cause().clone();
            rejected.discard();
            reject(
                PreloopStageBSourceInventoryStageV1::OwnedRow,
                Some(caller.clone()),
                Some(outer_site.clone()),
                PreloopStageBSourceInventoryCauseV1::OwnedRow { stage, cause },
            )
        })?;
        let identity = PreloopStageBCandidateIdentityV1 {
            caller: rows.caller().clone(),
            outer_call_site: rows.outer_call_site().clone(),
            selected_argument_index: rows.selected_argument_index(),
            inner_call_site: rows.inner_call_site().clone(),
            outer_target: rows.outer_target().clone(),
        };
        candidates.push(VerifiedPreloopStageBCandidateV1 { identity, rows });
    }

    Ok(VerifiedPreloopStageBCandidateInventoryV1 {
        catalog_identity: declarations as *const _ as usize,
        observed_declaration_count: inventory.observed_declaration_count(),
        observed_method_call_count: inventory.len(),
        candidates: candidates.into_boxed_slice(),
    })
}

fn raw_rejection(
    caller: &CanonicalSameModuleCallableKeyV1,
    outer_site: &SourceExprSiteV1,
    cause: RawSourceCursorErrorV1,
) -> PreloopStageBSourceInventoryErrorV1 {
    reject(
        PreloopStageBSourceInventoryStageV1::RawSourceProjection,
        Some(caller.clone()),
        Some(outer_site.clone()),
        PreloopStageBSourceInventoryCauseV1::RawSourceProjection(cause),
    )
}

fn reject(
    stage: PreloopStageBSourceInventoryStageV1,
    caller: Option<CanonicalSameModuleCallableKeyV1>,
    outer_site: Option<SourceExprSiteV1>,
    cause: PreloopStageBSourceInventoryCauseV1,
) -> PreloopStageBSourceInventoryErrorV1 {
    PreloopStageBSourceInventoryErrorV1::new(stage, caller, outer_site, cause)
}
