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
    issue_core_method_manifest_row_ref_v2, CORE_METHOD_MANIFEST_BRAND_V2,
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

use super::result_contract::VerifiedCallableResultContractBuilderV1;
use super::s6c_effects::{
    issue_s6c_physical_function_effects_v1, S6CPhysicalFunctionEffectsRejectV1,
    VerifiedS6CPhysicalFunctionEffectsV1,
};
use super::selected_mapping::{
    SelectedCallableBatchMapRowRefV1, VerifiedSelectedCallableBatchMapV1,
};

use crate::mir::builder::SelectedCallableConsumptionRoleV1;
use crate::parser::CallableDeclarationIdentityV1;
use std::rc::Rc;

#[derive(Debug)]
pub(in crate::mir) enum S6CSemanticChildIssueV1 {
    BatchLoan {
        _error: ResolvedCallableSemanticBatchLoanErrorV1,
    },
    TypedSource {
        _error: S6CTypedInputRelationRejectV1,
    },
    CallRelation {
        _error: S6CSourceBoundCallRelationRejectV1,
    },
    ExitTail {
        _error: S6CExitTailSourceCoSealRejectV1,
    },
    Facts {
        _error: S6CScanWithInitFactsRejectV1,
    },
    Recipe {
        _error: S6CScanWithInitRecipeProducerRejectV2,
    },
    Logical {
        _detail: &'static str,
    },
    Ingress {
        _error: S6CPrephysicalIngressRejectV2,
    },
    PhysicalEffects {
        _error: S6CPhysicalFunctionEffectsRejectV1,
    },
    MissingCompletionSeed,
    CompletionShared,
    UnexpectedTerminalRelation,
    DuplicateCandidate,
    ResultMismatch,
}

#[derive(Debug)]
pub(super) struct VerifiedS6CSemanticChildV1 {
    batch_slot: u32,
    owner: FunctionOwnerIdV1,
    _identity: CallableDeclarationIdentityV1,
    _role: SelectedCallableConsumptionRoleV1,
    result: crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1,
    ingress: crate::mir::loop_recipe_contract::VerifiedS6CPrephysicalIngressV2,
    physical_effects: VerifiedS6CPhysicalFunctionEffectsV1,
}

impl VerifiedS6CSemanticChildV1 {
    pub(super) const fn batch_slot(&self) -> u32 {
        self.batch_slot
    }

    pub(super) const fn result(
        &self,
    ) -> crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1 {
        self.result
    }

    pub(super) const fn physical_effects(&self) -> &VerifiedS6CPhysicalFunctionEffectsV1 {
        &self.physical_effects
    }
}

pub(super) struct S6CSemanticChildRefV1<'loan> {
    pub(super) child: &'loan VerifiedS6CSemanticChildV1,
}

impl S6CSemanticChildRefV1<'_> {
    pub(crate) const fn result(
        &self,
    ) -> crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1 {
        self.child.result()
    }

    pub(crate) fn physical_effects(&self) -> &VerifiedS6CPhysicalFunctionEffectsV1 {
        self.child.physical_effects()
    }

    pub(crate) fn with_completion_parity<R>(
        &self,
        callback: impl FnOnce(
            crate::mir::loop_recipe_contract::S6CPrephysicalCompletionParityRefV2,
        ) -> R,
    ) -> R {
        self.child.ingress.with_completion_parity(callback)
    }
}
pub(super) fn issue_s6c_semantic_child_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    seeds: &mut VerifiedCallableResultContractBuilderV1,
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
    seeds: &mut VerifiedCallableResultContractBuilderV1,
    result: crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1,
) -> Result<Option<VerifiedS6CSemanticChildV1>, S6CSemanticChildIssueV1> {
    let result = batch
        .with_declaration_semantics(|view| {
            let Some(row) = view.declarations().iter().find(|row| {
                row.batch_slot() == map_row.batch_slot()
                    && row.identity().same_as(map_row.identity())
            }) else {
                return Err(S6CSemanticChildIssueV1::BatchLoan {
                    _error: ResolvedCallableSemanticBatchLoanErrorV1::SourceCoverage,
                });
            };
            let Ok(loop_site) = row.function().only_loop_site() else {
                return Ok(None);
            };
            let typed = match issue_s6c_typed_input_relation_v1(row, &loop_site) {
                Ok(typed) => typed,
                Err(S6CTypedInputRelationRejectV1::SourceLedger(issue)) => {
                    return Err(S6CSemanticChildIssueV1::TypedSource {
                        _error: S6CTypedInputRelationRejectV1::SourceLedger(issue),
                    });
                }
                Err(_) => return Ok(None),
            };
            let mut targets =
                CoreMethodInstanceTargetIssuerV1::string_box_text(CORE_METHOD_MANIFEST_BRAND_V2)
                    .map_err(|_| S6CSemanticChildIssueV1::CallRelation {
                        _error: S6CSourceBoundCallRelationRejectV1::MixedManifestBrand,
                    })?;
            let length_row = issue_core_method_manifest_row_ref_v2(CoreMethodOp::StringLen, 0)
                .ok_or(S6CSemanticChildIssueV1::CallRelation {
                    _error: S6CSourceBoundCallRelationRejectV1::WrongTargetRole {
                        role: crate::mir::source_call_target::S6CSourceBoundCallRoleV1::Length,
                        op: CoreMethodOp::StringLen,
                        arity: 0,
                    },
                })?;
            let length =
                targets
                    .issue(length_row)
                    .map_err(|_| S6CSemanticChildIssueV1::CallRelation {
                        _error: S6CSourceBoundCallRelationRejectV1::WrongTargetRole {
                            role: crate::mir::source_call_target::S6CSourceBoundCallRoleV1::Length,
                            op: CoreMethodOp::StringLen,
                            arity: 0,
                        },
                    })?;
            let substring_row = issue_core_method_manifest_row_ref_v2(
                CoreMethodOp::StringSubstring,
                2,
            )
            .ok_or(S6CSemanticChildIssueV1::CallRelation {
                _error: S6CSourceBoundCallRelationRejectV1::WrongTargetRole {
                    role: crate::mir::source_call_target::S6CSourceBoundCallRoleV1::Substring,
                    op: CoreMethodOp::StringSubstring,
                    arity: 2,
                },
            })?;
            let substring = targets.issue(substring_row).map_err(|_| {
                S6CSemanticChildIssueV1::CallRelation {
                    _error: S6CSourceBoundCallRelationRejectV1::WrongTargetRole {
                        role: crate::mir::source_call_target::S6CSourceBoundCallRoleV1::Substring,
                        op: CoreMethodOp::StringSubstring,
                        arity: 2,
                    },
                }
            })?;
            let contract_row = seeds
                .take_main_child_row(map_row)
                .ok_or(S6CSemanticChildIssueV1::MissingCompletionSeed)?;
            let (
                batch_slot,
                owner,
                identity,
                role,
                _,
                completion,
                terminal_relations,
            ) = contract_row.into_parts();
            if !terminal_relations.is_empty() {
                return Err(S6CSemanticChildIssueV1::UnexpectedTerminalRelation);
            }
            let coseal = row
                .with_source_ledger(|ledger| {
                    let calls =
                        issue_source_bound_s6c_call_relation_v1(&ledger, typed, length, substring)
                            .map_err(|error| S6CSemanticChildIssueV1::CallRelation {
                                _error: error,
                            })?;
                    let completion = Rc::try_unwrap(completion)
                        .map_err(|_| S6CSemanticChildIssueV1::CompletionShared)?;
                    issue_s6c_exit_tail_source_coseal_v1(&ledger, calls, completion)
                        .map_err(|error| S6CSemanticChildIssueV1::ExitTail { _error: error })
                })
                .map_err(|issue| S6CSemanticChildIssueV1::TypedSource {
                    _error: S6CTypedInputRelationRejectV1::SourceLedger(issue),
                })??;
            let facts = issue_s6c_scan_with_init_facts_v1(coseal)
                .map_err(|error| S6CSemanticChildIssueV1::Facts { _error: error })?;
            let recipe = produce_s6c_scan_with_init_recipe_v2(facts)
                .map_err(|error| S6CSemanticChildIssueV1::Recipe { _error: error })?;
            let output = issue_s6c_scan_with_init_logical_output_v1(recipe).map_err(|_| {
                S6CSemanticChildIssueV1::Logical {
                    _detail: "logical output",
                }
            })?;
            let ingress = issue_s6c_prephysical_ingress_v2(output)
                .map_err(|error| S6CSemanticChildIssueV1::Ingress { _error: error })?;
            let physical_effects = issue_s6c_physical_function_effects_v1(&ingress, owner)
                .map_err(|error| S6CSemanticChildIssueV1::PhysicalEffects { _error: error })?;
            Ok(Some(VerifiedS6CSemanticChildV1 {
                batch_slot,
                owner,
                _identity: identity,
                _role: role,
                result,
                ingress,
                physical_effects,
            }))
        })
        .map_err(|error| S6CSemanticChildIssueV1::BatchLoan { _error: error })??;
    Ok(result)
}
