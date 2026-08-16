//! Caller-zero S6C child composition inside one installed semantic cohort.
//!
//! This module is the package issuer's only S6C candidate path.  It consumes
//! the selected-map role, the same batch-owned resolver row, and one completion
//! seed; it never accepts a caller-supplied slot, Recipe, Facts, or Completion.

use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;
use crate::mir::callable_semantic_batch::{
    issue_s6c_typed_input_relation_v1, ResolvedCallableSemanticBatchLoanErrorV1,
    S6CTypedInputRelationRejectV1,
};
use crate::mir::core_method_op::CoreMethodOp;
use crate::mir::core_method_result_kind::{
    issue_core_method_manifest_row_ref_v1, CORE_METHOD_MANIFEST_BRAND_V1,
};
use crate::mir::loop_recipe_contract::{
    issue_s6c_prephysical_ingress_v2, issue_s6c_scan_with_init_logical_output_v1,
    produce_s6c_scan_with_init_recipe_v2, S6CPrephysicalIngressRejectV2,
    S6CScanWithInitRecipeProducerRejectV2,
};
use crate::mir::loop_structural_facts::{
    issue_s6c_exit_tail_source_coseal_v1, issue_s6c_scan_with_init_facts_v1,
    S6CExitTailSourceCoSealRejectV1, S6CScanWithInitFactsRejectV1,
};
use crate::mir::resolved_semantics::{CoreMethodInstanceTargetIssuerV1, FunctionOwnerIdV1};
use crate::mir::source_call_target::{
    issue_source_bound_s6c_call_relation_v1, S6CSourceBoundCallRelationRejectV1,
};

use super::completion_seed::VerifiedCallableCompletionSeedCohortV1;
use super::selected_mapping::{
    SelectedCallableBatchMapRowRefV1, VerifiedSelectedCallableBatchMapV1,
};

use crate::mir::builder::SelectedCallableConsumptionRoleV1;
use crate::parser::CallableDeclarationIdentityV1;

#[derive(Debug)]
pub(super) enum S6CSemanticChildIssueV1 {
    BatchLoan(ResolvedCallableSemanticBatchLoanErrorV1),
    TypedSource(S6CTypedInputRelationRejectV1),
    CallRelation(S6CSourceBoundCallRelationRejectV1),
    ExitTail(S6CExitTailSourceCoSealRejectV1),
    Facts(S6CScanWithInitFactsRejectV1),
    Recipe(S6CScanWithInitRecipeProducerRejectV2),
    Logical(&'static str),
    Ingress(S6CPrephysicalIngressRejectV2),
    MissingCompletionSeed,
    DuplicateCandidate,
    ResultMismatch,
}

#[derive(Debug)]
pub(super) struct VerifiedS6CSemanticChildV1 {
    batch_slot: u32,
    owner: FunctionOwnerIdV1,
    identity: CallableDeclarationIdentityV1,
    role: SelectedCallableConsumptionRoleV1,
    result: crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1,
    ingress: crate::mir::loop_recipe_contract::VerifiedS6CPrephysicalIngressV2,
}

impl VerifiedS6CSemanticChildV1 {
    pub(super) const fn batch_slot(&self) -> u32 {
        self.batch_slot
    }

    pub(super) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) fn identity(&self) -> &CallableDeclarationIdentityV1 {
        &self.identity
    }

    pub(super) const fn role(&self) -> SelectedCallableConsumptionRoleV1 {
        self.role
    }

    pub(super) const fn result(
        &self,
    ) -> crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1 {
        self.result
    }
}

pub(super) struct S6CSemanticChildRefV1<'loan> {
    pub(super) child: &'loan VerifiedS6CSemanticChildV1,
}

impl S6CSemanticChildRefV1<'_> {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.child.owner()
    }

    pub(crate) const fn result(
        &self,
    ) -> crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1 {
        self.child.result()
    }

    pub(crate) fn with_completion<R>(
        &self,
        callback: impl for<'facts> FnOnce(
            crate::mir::loop_recipe_contract::S6CPrephysicalCompletionRefV2<'facts>,
        ) -> R,
    ) -> R {
        self.child.ingress.with_completion(callback)
    }

    pub(crate) fn with_completion_parity<R>(
        &self,
        callback: impl FnOnce(
            crate::mir::loop_recipe_contract::S6CPrephysicalCompletionParityRefV2,
        ) -> R,
    ) -> R {
        self.child.ingress.with_completion_parity(callback)
    }

    /// Issue the caller-zero common V2 sibling products while the installed
    /// child still owns the retained source cohort.  The nested `Result`
    /// keeps ingress rejection and common projection rejection distinct; no
    /// source row is copied out of this HRTB loan.
    pub(crate) fn with_common_v2_pre_session<R>(
        &self,
        callback: impl for<'source, 'join> FnOnce(
            crate::mir::loop_recipe_contract::PreparedLoopV2PreSessionEnvelopeV1<'source, 'join>,
        ) -> R,
    ) -> Result<R, crate::mir::loop_recipe_contract::CommonV2IssuerRejectV1> {
        let nested = self.child.ingress.with_ingress(|view| {
            Ok(
                crate::mir::loop_recipe_contract::issue_s6c_common_v2_pre_session_v1(
                    view,
                    self.child.owner,
                )
                .map(callback),
            )
        });
        match nested {
            Err(error) => {
                Err(crate::mir::loop_recipe_contract::CommonV2IssuerRejectV1::Ingress(error))
            }
            Ok(Err(error)) => Err(error),
            Ok(Ok(result)) => Ok(result),
        }
    }
}
pub(super) fn issue_s6c_semantic_child_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    seeds: &mut VerifiedCallableCompletionSeedCohortV1,
) -> Result<Option<VerifiedS6CSemanticChildV1>, S6CSemanticChildIssueV1> {
    let mut candidate = None;
    for row in selected.main_static_child_rows() {
        let Some(result) = seeds.peek_main_child_result(row) else {
            return Err(S6CSemanticChildIssueV1::MissingCompletionSeed);
        };
        let Some(result) = result else {
            continue;
        };
        let Some(child) = issue_s6c_child_for_row(batch, row, seeds, result)? else {
            continue;
        };
        if candidate.replace(child).is_some() {
            return Err(S6CSemanticChildIssueV1::DuplicateCandidate);
        }
    }
    Ok(candidate)
}

fn issue_s6c_child_for_row(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    map_row: SelectedCallableBatchMapRowRefV1<'_>,
    seeds: &mut VerifiedCallableCompletionSeedCohortV1,
    result: crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1,
) -> Result<Option<VerifiedS6CSemanticChildV1>, S6CSemanticChildIssueV1> {
    let result = batch
        .with_declaration_semantics(|view| {
            let Some(row) = view.declarations().iter().find(|row| {
                row.batch_slot() == map_row.batch_slot()
                    && row.identity().same_as(map_row.identity())
            }) else {
                return Err(S6CSemanticChildIssueV1::BatchLoan(
                    ResolvedCallableSemanticBatchLoanErrorV1::SourceCoverage,
                ));
            };
            let Ok(loop_site) = row.function().only_loop_site() else {
                return Ok(None);
            };
            let typed = match issue_s6c_typed_input_relation_v1(row, &loop_site) {
                Ok(typed) => typed,
                Err(S6CTypedInputRelationRejectV1::SourceLedger(issue)) => {
                    return Err(S6CSemanticChildIssueV1::TypedSource(
                        S6CTypedInputRelationRejectV1::SourceLedger(issue),
                    ));
                }
                Err(_) => return Ok(None),
            };
            let mut targets =
                CoreMethodInstanceTargetIssuerV1::string_box_text(CORE_METHOD_MANIFEST_BRAND_V1)
                    .map_err(|_| {
                        S6CSemanticChildIssueV1::CallRelation(
                            S6CSourceBoundCallRelationRejectV1::MixedManifestBrand,
                        )
                    })?;
            let length_row = issue_core_method_manifest_row_ref_v1(CoreMethodOp::StringLen, 0)
                .ok_or(S6CSemanticChildIssueV1::CallRelation(
                    S6CSourceBoundCallRelationRejectV1::WrongTargetRole {
                        role: crate::mir::source_call_target::S6CSourceBoundCallRoleV1::Length,
                        op: CoreMethodOp::StringLen,
                        arity: 0,
                    },
                ))?;
            let length = targets.issue(length_row).map_err(|_| {
                S6CSemanticChildIssueV1::CallRelation(
                    S6CSourceBoundCallRelationRejectV1::WrongTargetRole {
                        role: crate::mir::source_call_target::S6CSourceBoundCallRoleV1::Length,
                        op: CoreMethodOp::StringLen,
                        arity: 0,
                    },
                )
            })?;
            let substring_row = issue_core_method_manifest_row_ref_v1(
                CoreMethodOp::StringSubstring,
                2,
            )
            .ok_or(S6CSemanticChildIssueV1::CallRelation(
                S6CSourceBoundCallRelationRejectV1::WrongTargetRole {
                    role: crate::mir::source_call_target::S6CSourceBoundCallRoleV1::Substring,
                    op: CoreMethodOp::StringSubstring,
                    arity: 2,
                },
            ))?;
            let substring = targets.issue(substring_row).map_err(|_| {
                S6CSemanticChildIssueV1::CallRelation(
                    S6CSourceBoundCallRelationRejectV1::WrongTargetRole {
                        role: crate::mir::source_call_target::S6CSourceBoundCallRoleV1::Substring,
                        op: CoreMethodOp::StringSubstring,
                        arity: 2,
                    },
                )
            })?;
            let seed = seeds
                .take_main_child_seed(map_row)
                .ok_or(S6CSemanticChildIssueV1::MissingCompletionSeed)?;
            let owner = seed.owner();
            let identity = seed.identity().clone();
            let role = seed.role();
            let batch_slot = seed.batch_slot();
            let coseal = row
                .with_source_ledger(|ledger| {
                    let calls =
                        issue_source_bound_s6c_call_relation_v1(&ledger, typed, length, substring)
                            .map_err(S6CSemanticChildIssueV1::CallRelation)?;
                    issue_s6c_exit_tail_source_coseal_v1(&ledger, calls, seed.take_completion())
                        .map_err(S6CSemanticChildIssueV1::ExitTail)
                })
                .map_err(|issue| {
                    S6CSemanticChildIssueV1::TypedSource(
                        S6CTypedInputRelationRejectV1::SourceLedger(issue),
                    )
                })??;
            let facts = issue_s6c_scan_with_init_facts_v1(coseal)
                .map_err(S6CSemanticChildIssueV1::Facts)?;
            let recipe = produce_s6c_scan_with_init_recipe_v2(facts)
                .map_err(S6CSemanticChildIssueV1::Recipe)?;
            let output = issue_s6c_scan_with_init_logical_output_v1(recipe)
                .map_err(|_| S6CSemanticChildIssueV1::Logical("logical output"))?;
            let ingress = issue_s6c_prephysical_ingress_v2(output)
                .map_err(S6CSemanticChildIssueV1::Ingress)?;
            Ok(Some(VerifiedS6CSemanticChildV1 {
                batch_slot,
                owner,
                identity,
                role,
                result,
                ingress,
            }))
        })
        .map_err(S6CSemanticChildIssueV1::BatchLoan)??;
    Ok(result)
}
