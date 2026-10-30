//! First closed canonical function-family lowering.
//!
//! The compiler preflight produces the only input accepted here. This module
//! owns exact source traversal and BindingRef-based value publication; legacy
//! statement/expression dispatch is intentionally not reachable from it.

mod branch_transaction;
mod callable_module_transaction;
pub(in crate::mir::builder) mod canonical_cfg;
mod canonical_compare_writer;
pub(in crate::mir) mod canonical_ssa;
mod common_v2_after_block_allocation;
mod common_v2_if_continuation_target;
#[cfg(test)]
mod common_v2_s6c_observation_fixture;
mod common_v2_s6c_substring_callout_admission;
#[allow(dead_code)]
mod common_v2_s6c_text_content_root_admission;
#[allow(dead_code)]
mod common_v2_s6c_text_cursor_preheader;
#[allow(dead_code)]
mod common_v2_s6c_textref_entry_bridge;
mod common_v2_segment_block_allocation;
mod common_v2_session;
mod completion_consumption;
mod direct_accum_adapter;
mod direct_accum_lowerer;
mod draft_seal;
mod draft_seal_owner;
mod function_draft_entry;
pub(in crate::mir::builder) use draft_seal::ReadyFunctionDraftSealV1;
pub(in crate::mir::builder) use draft_seal_owner::{
    CompletedCatalogedBoxCallableDraftV1, DiscardedFunctionDraftSealErrorV1,
    SelectedDynamicCandidateMetadataV1,
};
pub(in crate::mir::builder) mod dynamic_loop_phi;
mod dynamic_loop_phi_close;
mod flow_consumption;
#[cfg(test)]
mod generic_g0_physical_emitter_session;
mod identity;
pub(in crate::mir::builder) mod if_cfg_ready_bridge;
mod if_materialization;
mod if_recipe_adapter;
mod located_if;
mod lowerer;
mod nested_predicate_adapter;
mod nested_predicate_lowerer;
mod nested_predicate_physicalizer;
mod ownership;
mod physical_entry_lane_adoption;
mod physical_entry_session;
mod selected_dynamic_physical_abi;
mod selected_dynamic_physical_capability;
mod selected_dynamic_physical_emitter;
mod semantic_stack;
mod static_loop_body;
mod static_loop_body_exit;
mod static_loop_draft_finish;
mod static_loop_entry_session;
mod static_loop_header;
mod static_loop_publication;
mod static_loop_tail;
mod trivial_ssa;

pub(in crate::mir) use callable_module_transaction::{
    CallableModuleTransactionErrorV1, VerifiedUnpublishedCallableDraftSetV1,
};
pub(in crate::mir::builder) use common_v2_s6c_text_content_root_admission::{
    issue_common_v2_s6c_text_content_root_admission_v1,
    CommonV2S6CTextContentRootAdmissionRejectV1, CommonV2S6CTextContentRootAdmissionV1,
    CommonV2S6CTextContentRootRoleV1, CommonV2S6CTextContentRootRowV1,
};
pub(in crate::mir::builder) use common_v2_s6c_text_cursor_preheader::issue_common_v2_s6c_text_cursor_preheader_v1;
#[cfg(test)]
pub(in crate::mir::builder) use common_v2_s6c_text_cursor_preheader::CommonV2S6CTextCursorPreheaderRejectV1;
#[cfg(test)]
pub(in crate::mir) use common_v2_session::with_common_v2_canonical_session;
#[cfg(test)]
pub(in crate::mir::builder) use common_v2_session::{
    issue_common_v2_s6c_text_scalar_equality_leaf_v1, CommonV2S6CTextScalarEqualityLeafShapeV1,
};
pub(in crate::mir::builder) use loop_recipe_physicalizer::{
    lower_callable_single_loop_function_draft_v1, lower_main0_continue_function_draft_v1,
    lower_main0_derived_predicate_function_draft_v1, lower_main0_in_body_step_function_draft_v1,
};
#[cfg(test)]
pub(in crate::mir::builder) use physical_entry_draftseal::with_common_v2_s6c_physical_entry_draft_seal;
#[cfg(test)]
pub(in crate::mir::builder) use physical_entry_draftseal::with_common_v2_s6c_pinned_text_physical_entry_draft_seal;
#[cfg(test)]
pub(in crate::mir::builder) use physical_entry_session::with_common_v2_physical_entry_session;
#[cfg(test)]
pub(in crate::mir::builder) use physical_entry_session::with_common_v2_physical_entry_session_expected_brand;
#[cfg(test)]
pub(in crate::mir::builder) use physical_entry_session::with_common_v2_physical_entry_session_with_s6c_effects;
#[cfg(test)]
pub(in crate::mir::builder) use physical_entry_session::with_common_v2_physical_entry_session_with_s6c_loan;
pub(in crate::mir) use selected_dynamic_physical_abi::{
    issue_selected_dynamic_v2_emission_plan, DynamicV2PhysicalScheduleSegmentV1,
};
#[cfg(test)]
pub(in crate::mir) use selected_dynamic_physical_capability::issue_selected_dynamic_v2_physical_capability_admission;
pub(in crate::mir::builder) use selected_dynamic_physical_capability::issue_selected_dynamic_v2_physical_capability_admission_from_brand;
#[cfg(test)]
pub(in crate::mir) use selected_dynamic_physical_capability::{
    DynamicV2PhysicalCapabilityDispositionV1, DynamicV2PhysicalRepresentationV1,
    DynamicV2ProducerFamilyV1, SelectedDynamicV2PhysicalCapabilityRejectV1,
};
pub(in crate::mir::builder) use selected_dynamic_physical_emitter::assemble_unpublished_selected_dynamic_w6_from_parts;
pub(in crate::mir::builder) use static_loop_entry_session::{
    prepare_selected_static_loop_entry_v1, PreparedSelectedStaticLoopEntryV1,
};
pub(in crate::mir) use static_loop_publication::SelectedStaticLoopPublicationBatchV1;

/// Consume one already-admitted trivial Binding-SSA plan on an existing
/// function skeleton.  The caller owns the package recipe port; this helper
/// only wires it into the existing canonical session and does not create a
/// second function or fallback route.
pub(in crate::mir::builder) fn lower_resolved_trivial_body_with_qualified_method_port_v1(
    builder: &mut MirBuilder,
    plan: CanonicalTrivialBindingSsaPlanV1<'_>,
    port: &mut dyn QualifiedMethodRecipePortV1,
) -> Result<crate::mir::ValueId, String> {
    let (input, if_control, completion, profile, block_expr_count) = plan.into_parts();
    let recipe_preflight = produce_trivial_if_physical_input_v1(&profile, input.function())
        .map_err(|error| format!("[freeze:contract][if_recipe/producer] {error:?}"))?;
    let recipe_admission =
        admit_trivial_if_recipe_v1(recipe_preflight, input.function(), &if_control)
            .map_err(|error| format!("[freeze:contract][if_recipe/admission] {error:?}"))?;
    builder
        .function_state
        .resolved_binding_state
        .install(input.function())
        .map_err(|error| error.to_string())?;
    install_trivial_callable_abi_v1(builder, &profile);
    let ready = CanonicalTrivialSsaLowererV1::new_with_qualified_method_port(
        builder,
        input,
        if_control,
        completion,
        profile,
        block_expr_count,
        recipe_admission,
        port,
    )?
    .lower()?;
    let (block, value) = ready.explicit_operand().ok_or_else(|| {
        "[freeze:contract][canonical_qualified_method/value_return_required]".to_owned()
    })?;
    let function = builder
        .function_state
        .current_function
        .as_mut()
        .ok_or_else(|| {
            "[freeze:contract][canonical_qualified_method/function_missing]".to_owned()
        })?;
    function
        .get_block_mut(block)
        .ok_or_else(|| {
            "[freeze:contract][canonical_qualified_method/return_block_missing]".to_owned()
        })?
        .set_terminator(crate::mir::MirInstruction::Return { value: Some(value) });
    builder.function_state.current_block = Some(block);
    Ok(value)
}

#[cfg(test)]
mod block_expr_tests;
#[cfg(test)]
mod common_v2_condition_bool_tests;
#[cfg(test)]
mod common_v2_if_continuation_target_tests;
#[cfg(test)]
mod common_v2_initial_index_seed_tests;
#[cfg(test)]
mod common_v2_return_read_tests;
#[cfg(test)]
mod common_v2_s6c_cursor_cfg_tests;
#[cfg(test)]
mod common_v2_s6c_operand_issuer_tests;
#[cfg(test)]
mod completion_consumption_tests;
#[cfg(test)]
mod completion_draft_seal_tests;
#[cfg(test)]
mod completion_test_support;
#[cfg(test)]
mod dynamic_loop_discard_tests;
#[cfg(test)]
mod dynamic_loop_phi_tests;
#[cfg(test)]
mod dynamic_loop_rebind_tests;
#[cfg(test)]
mod flow_consumption_tests;
#[cfg(test)]
mod identity_separation_tests;
#[cfg(test)]
mod if_materialization_tests;
#[cfg(test)]
mod if_tests;
pub(in crate::mir::builder) mod loop_recipe_physicalizer;

pub(in crate::mir::builder::resolved_lowering) use canonical_compare_writer::{
    CanonicalLoopCompareI64WriterV1, PreparedCanonicalLoopCompareI64V1,
};
#[cfg(test)]
mod nested_predicate_effect_adapter_tests;
#[cfg(test)]
mod normal_function_draft_lowering_tests;
#[cfg(test)]
mod null_tests;
#[cfg(test)]
mod parameter_tests;
#[cfg(test)]
mod physical_entry_draftseal;
#[cfg(test)]
mod physical_entry_lane_adoption_tests;
#[cfg(test)]
mod return_tests;
#[cfg(test)]
mod semantic_stack_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod void_tests;

use crate::mir::compiler::capability::{
    CanonicalCurrentAPlusPlanV1, CanonicalGenericG0PlanV1, CanonicalTrivialBindingSsaPlanV1,
};
use crate::mir::compiler::direct_accum_profile::CanonicalDirectAccumPlanV1;
use crate::mir::function::MirParamDecl;
use crate::mir::{MirFunction, MirModule};

use super::calls::{
    CanonicalFunctionLoweringSessionV1, CanonicalFunctionSessionErrorV1,
    PendingFunctionSessionCloseV1,
};
use super::MirBuilder;
use direct_accum_lowerer::CanonicalDirectAccumSsaLowererV1;
use draft_seal_owner::{FunctionDraftSealStageV1, RejectedFunctionDraftSealV1};
use if_recipe_adapter::{admit_trivial_if_recipe_v1, produce_trivial_if_physical_input_v1};
use lowerer::CanonicalFunctionLowererV1;
pub(in crate::mir::builder) use trivial_ssa::QualifiedMethodRecipePortV1;
use trivial_ssa::{install_trivial_callable_abi_v1, CanonicalTrivialSsaLowererV1};

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) enum CanonicalResolvedBuildErrorV1 {
    BuilderContract(String),
    DuplicateFunctionPublication { function_name: String },
    GenericG0Admission(
        crate::mir::compiler::generic_g0_physical_operation_cohort::
            GenericG0PhysicalEmitterAdmissionRejectV1,
    ),
    GenericG0Lowerer(String),
}

/// Complete one CallableSingleLoop draft seal without restoring the captured
/// parent until collector admission has consumed the pending draft.  The
/// selected cataloged entry owns the function session; this helper only joins
/// the ready physical result to the existing pending restoration terminal.
pub(in crate::mir::builder) fn commit_callable_single_loop_ready_to_pending_v1(
    session: CanonicalFunctionLoweringSessionV1<'_>,
    ready: ReadyFunctionDraftSealV1,
) -> Result<PendingFunctionSessionCloseV1<'_>, CanonicalFunctionSessionErrorV1> {
    let open = ready.open(session);
    let prepared = match open.prepare() {
        Ok(prepared) => prepared,
        Err(rejected) => {
            return Err(CanonicalFunctionSessionErrorV1::DraftSeal(
                rejected.into_discarded_error(),
            ));
        }
    };
    Ok(prepared.commit_pending())
}

impl From<String> for CanonicalResolvedBuildErrorV1 {
    fn from(detail: String) -> Self {
        Self::BuilderContract(detail)
    }
}

impl From<CanonicalFunctionSessionErrorV1> for CanonicalResolvedBuildErrorV1 {
    fn from(error: CanonicalFunctionSessionErrorV1) -> Self {
        match error.duplicate_function_name() {
            Some(function_name) => Self::DuplicateFunctionPublication {
                function_name: function_name.to_string(),
            },
            None => Self::BuilderContract(error.to_string()),
        }
    }
}

/// Exact failure boundary for the TX0 retaining draft-lowering terminal.
///
/// The outer stage is issued at the operation that failed; callers never
/// classify a formatted legacy diagnostic to recover it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum NormalFunctionDraftLoweringStageV1 {
    SessionOpen,
    BindingInstall,
    Skeleton,
    BodyLowering,
    DraftSeal(FunctionDraftSealStageV1),
    SessionRestore,
}

#[derive(Debug)]
pub(in crate::mir) enum NormalFunctionDraftLoweringCauseV1 {
    BuilderContract(Box<str>),
    DraftSeal(Box<str>),
}

/// Sealed evidence that the unpublished child session was discarded and its
/// caller context restored before the typed rejection was issued.
#[derive(Debug)]
pub(in crate::mir) struct NormalFunctionDraftBuilderRestorationReceiptV1 {
    session: super::calls::CanonicalFunctionSessionRestorationReceiptV1,
    _seal: NormalFunctionDraftBuilderRestorationReceiptSealV1,
}

#[derive(Debug)]
struct NormalFunctionDraftBuilderRestorationReceiptSealV1;

#[derive(Debug)]
pub(in crate::mir) struct RejectedNormalFunctionDraftLoweringV1 {
    stage: NormalFunctionDraftLoweringStageV1,
    cause: NormalFunctionDraftLoweringCauseV1,
    restoration: NormalFunctionDraftBuilderRestorationReceiptV1,
}

impl RejectedNormalFunctionDraftLoweringV1 {
    pub(in crate::mir) const fn stage(&self) -> NormalFunctionDraftLoweringStageV1 {
        self.stage
    }

    pub(in crate::mir) const fn cause(&self) -> &NormalFunctionDraftLoweringCauseV1 {
        &self.cause
    }

    pub(in crate::mir) const fn has_restoration_receipt(&self) -> bool {
        let _ = &self.restoration.session;
        true
    }

    pub(in crate::mir) fn into_compatibility_error(self) -> CanonicalResolvedBuildErrorV1 {
        match self.cause {
            NormalFunctionDraftLoweringCauseV1::BuilderContract(detail) => {
                CanonicalResolvedBuildErrorV1::BuilderContract(detail.into())
            }
            NormalFunctionDraftLoweringCauseV1::DraftSeal(detail) => {
                let NormalFunctionDraftLoweringStageV1::DraftSeal(stage) = self.stage else {
                    unreachable!("draft-seal cause keeps one draft-seal stage")
                };
                CanonicalResolvedBuildErrorV1::BuilderContract(format!(
                    "[freeze:contract][f1_draft_seal/{:?}] {detail}",
                    stage
                ))
            }
        }
    }
}

fn reject_after_session_discard(
    session: super::calls::CanonicalFunctionLoweringSessionV1<'_>,
    stage: NormalFunctionDraftLoweringStageV1,
    detail: String,
) -> RejectedNormalFunctionDraftLoweringV1 {
    let session = session.discard_unpublished();
    RejectedNormalFunctionDraftLoweringV1 {
        stage,
        cause: NormalFunctionDraftLoweringCauseV1::BuilderContract(detail.into_boxed_str()),
        restoration: NormalFunctionDraftBuilderRestorationReceiptV1 {
            session,
            _seal: NormalFunctionDraftBuilderRestorationReceiptSealV1,
        },
    }
}

fn reject_draft_seal_typed(
    rejected: RejectedFunctionDraftSealV1<'_>,
) -> RejectedNormalFunctionDraftLoweringV1 {
    let stage = rejected.stage();
    let detail = format!("{:?}", rejected.error()).into_boxed_str();
    let session = rejected.discard_with_restoration_receipt();
    RejectedNormalFunctionDraftLoweringV1 {
        stage: NormalFunctionDraftLoweringStageV1::DraftSeal(stage),
        cause: NormalFunctionDraftLoweringCauseV1::DraftSeal(detail),
        restoration: NormalFunctionDraftBuilderRestorationReceiptV1 {
            session,
            _seal: NormalFunctionDraftBuilderRestorationReceiptSealV1,
        },
    }
}

fn reject_draft_seal<'builder>(
    rejected: RejectedFunctionDraftSealV1<'builder>,
) -> CanonicalResolvedBuildErrorV1 {
    let stage = rejected.stage();
    let error = format!("{:?}", rejected.error());
    rejected.discard();
    CanonicalResolvedBuildErrorV1::BuilderContract(format!(
        "[freeze:contract][f1_draft_seal/{stage:?}] {error}"
    ))
}
