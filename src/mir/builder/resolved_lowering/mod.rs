//! First closed canonical function-family lowering.
//!
//! The compiler preflight produces the only input accepted here. This module
//! owns exact source traversal and BindingRef-based value publication; legacy
//! statement/expression dispatch is intentionally not reachable from it.

mod branch_transaction;
mod callable_module_transaction;
pub(in crate::mir::builder) mod canonical_cfg;
mod canonical_ssa;
mod completion_consumption;
mod direct_accum_adapter;
mod direct_accum_lowerer;
mod draft_seal;
mod draft_seal_owner;
mod flow_consumption;
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
mod semantic_stack;
mod trivial_ssa;

pub(in crate::mir) use callable_module_transaction::{
    CallableModuleTransactionErrorV1, VerifiedUnpublishedCallableDraftSetV1,
};

#[cfg(test)]
mod block_expr_tests;
#[cfg(test)]
mod callable_module_transaction_tests;
#[cfg(test)]
mod completion_tests;
#[cfg(test)]
mod flow_consumption_tests;
#[cfg(test)]
mod identity_separation_tests;
#[cfg(test)]
mod if_materialization_tests;
#[cfg(test)]
mod if_tests;
#[cfg(test)]
mod loop_recipe_physicalizer;
#[cfg(test)]
mod nested_predicate_effect_adapter_tests;
#[cfg(test)]
mod normal_function_draft_lowering_tests;
#[cfg(test)]
mod null_tests;
#[cfg(test)]
mod parameter_tests;
#[cfg(test)]
mod return_tests;
#[cfg(test)]
mod semantic_stack_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod void_tests;

use crate::mir::compiler::capability::{
    CanonicalCurrentAPlusPlanV1, CanonicalTrivialBindingSsaPlanV1,
};
use crate::mir::compiler::direct_accum_profile::CanonicalDirectAccumPlanV1;
use crate::mir::function::MirParamDecl;
use crate::mir::{MirFunction, MirModule};

use super::calls::CanonicalFunctionSessionErrorV1;
use super::MirBuilder;
use direct_accum_lowerer::CanonicalDirectAccumSsaLowererV1;
use draft_seal::ReadyFunctionDraftSealV1;
use draft_seal_owner::{FunctionDraftSealStageV1, RejectedFunctionDraftSealV1};
use if_recipe_adapter::{
    admit_trivial_if_recipe_v1, produce_trivial_if_physical_input_v1,
    CanonicalIfRecipeAdmissionDispositionV1,
};
use lowerer::CanonicalFunctionLowererV1;
use trivial_ssa::{install_trivial_callable_abi_v1, CanonicalTrivialSsaLowererV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum CanonicalResolvedBuildErrorV1 {
    BuilderContract(String),
    DuplicateFunctionPublication { function_name: String },
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

impl MirBuilder {
    pub(in crate::mir) fn build_resolved_function_module(
        &mut self,
        plan: CanonicalCurrentAPlusPlanV1<'_>,
    ) -> Result<MirModule, CanonicalResolvedBuildErrorV1> {
        self.prepare_module()?;
        let draft = self.lower_resolved_function_draft(plan)?;
        self.current_module
            .as_mut()
            .expect("prepare_module installs the candidate module")
            .try_add_function(draft)
            .map_err(
                |error| CanonicalResolvedBuildErrorV1::DuplicateFunctionPublication {
                    function_name: error.function_name,
                },
            )?;
        let entry_result = crate::mir::builder::emission::constant::emit_void(self)?;
        Ok(self.finalize_module(entry_result)?)
    }

    /// LOWER0 draft-only A+ consumer.  The plan is moved into the canonical
    /// lowerer and no module entry/finalization/publication is performed.
    pub(in crate::mir) fn lower_resolved_function_draft(
        &mut self,
        plan: CanonicalCurrentAPlusPlanV1<'_>,
    ) -> Result<MirFunction, CanonicalResolvedBuildErrorV1> {
        let (input, flow, completion, block_expr_count) = plan.into_parts();
        let crate::ast::ASTNode::FunctionDeclaration {
            name,
            params,
            body,
            return_type_name,
            attrs,
            uses,
            ..
        } = input.source().root()
        else {
            unreachable!("preflight seals one FunctionDeclaration root")
        };
        let function_name = format!("{}/{}", name, params.len());
        let session_name = function_name.clone();
        let mut session = self.open_resolved_function_draft_seal_session_v1(&session_name);
        let lowering = {
            let builder = session.builder_view_mut_for_lowering();
            (|| {
                builder
                    .function_state
                    .resolved_binding_state
                    .install(input.function())?;
                builder.create_function_skeleton(function_name, params, body)?;
                builder.set_current_function_declared_signature(
                    params
                        .iter()
                        .map(|name| MirParamDecl {
                            name: name.clone(),
                            declared_type_name: None,
                            implicit_receiver: false,
                        })
                        .collect(),
                    return_type_name.clone(),
                );
                builder.set_current_function_runes(attrs);
                builder.set_current_function_declared_capability_uses(uses);

                let ready = CanonicalFunctionLowererV1::new(
                    builder,
                    input,
                    flow,
                    completion,
                    block_expr_count,
                )?
                .lower()?;
                let current_block = builder.function_state.current_block.ok_or_else(|| {
                    "[freeze:contract][f1_draft_seal/current_block_missing]".to_string()
                })?;
                Ok::<_, String>((ready, current_block))
            })()
        };
        let (ready, current_block) = match lowering {
            Ok(result) => result,
            Err(error) => {
                session.discard_unpublished();
                return Err(error.into());
            }
        };
        let open = ReadyFunctionDraftSealV1::new(ready, current_block).open(session);
        let prepared = open.prepare().map_err(reject_draft_seal)?;
        Ok(prepared.commit().into_draft())
    }

    /// Caller-zero DirectAccum draft consumer. The plan is lowered only on a
    /// private function session; the outer candidate remains the sole abort
    /// boundary and no route scheduler is reachable from this method.
    pub(in crate::mir) fn lower_resolved_direct_accum_function_draft(
        &mut self,
        plan: CanonicalDirectAccumPlanV1<'_>,
    ) -> Result<MirFunction, CanonicalResolvedBuildErrorV1> {
        self.lower_resolved_direct_accum_function_draft_inner(plan, false)
    }

    pub(in crate::mir) fn lower_resolved_nested_predicate_function_draft(
        &mut self,
        plan: crate::mir::compiler::nested_predicate_profile::CanonicalNestedPredicatePlanV1<'_>,
    ) -> Result<MirFunction, CanonicalResolvedBuildErrorV1> {
        nested_predicate_lowerer::lower_nested_predicate_function_draft(self, plan)
    }

    #[cfg(test)]
    pub(in crate::mir) fn lower_resolved_direct_accum_function_draft_with_seal_failure_for_test(
        &mut self,
        plan: CanonicalDirectAccumPlanV1<'_>,
    ) -> Result<MirFunction, CanonicalResolvedBuildErrorV1> {
        self.lower_resolved_direct_accum_function_draft_inner(plan, true)
    }

    fn lower_resolved_direct_accum_function_draft_inner(
        &mut self,
        plan: CanonicalDirectAccumPlanV1<'_>,
        _inject_seal_failure: bool,
    ) -> Result<MirFunction, CanonicalResolvedBuildErrorV1> {
        let input = plan.input();
        let crate::ast::ASTNode::FunctionDeclaration {
            name,
            params,
            body,
            return_type_name,
            attrs,
            uses,
            ..
        } = input.source().root()
        else {
            return Err(CanonicalResolvedBuildErrorV1::BuilderContract(
                "[freeze:contract][direct_accum/root_not_function]".into(),
            ));
        };
        let function_name = format!("{}/{}", name, params.len());
        let mut session = self.open_resolved_function_draft_seal_session_v1(&function_name);
        let lowering = {
            let builder = session.builder_view_mut_for_lowering();
            (|| {
                builder
                    .function_state
                    .resolved_binding_state
                    .install(input.function())?;
                builder.create_function_skeleton(function_name, params, body)?;
                builder.set_current_function_declared_signature(
                    params
                        .iter()
                        .map(|name| MirParamDecl {
                            name: name.clone(),
                            declared_type_name: None,
                            implicit_receiver: false,
                        })
                        .collect(),
                    return_type_name.clone(),
                );
                builder.set_current_function_runes(attrs);
                builder.set_current_function_declared_capability_uses(uses);
                let (ready, final_receipt) =
                    CanonicalDirectAccumSsaLowererV1::new(builder, plan)?.lower()?;
                final_receipt.consume_for_candidate()?;
                #[cfg(test)]
                if _inject_seal_failure {
                    let current_block = builder.function_state.current_block.ok_or_else(|| {
                        "[freeze:contract][direct_accum/current_block_missing]".to_string()
                    })?;
                    builder
                        .function_state
                        .current_function
                        .as_mut()
                        .and_then(|function| function.get_block_mut(current_block))
                        .ok_or_else(|| {
                            "[freeze:contract][direct_accum/test_failure_block_missing]".to_string()
                        })?
                        .set_terminator(crate::mir::MirInstruction::Return { value: None });
                }
                Ok::<_, String>(ready)
            })()
        };
        let ready = match lowering {
            Ok(ready) => ready,
            Err(error) => {
                session.discard_unpublished();
                return Err(error.into());
            }
        };
        let open = ready.open(session);
        let prepared = open.prepare().map_err(reject_draft_seal)?;
        Ok(prepared.commit().into_draft())
    }

    pub(in crate::mir) fn build_resolved_trivial_function_module(
        &mut self,
        plan: CanonicalTrivialBindingSsaPlanV1<'_>,
    ) -> Result<MirModule, CanonicalResolvedBuildErrorV1> {
        self.prepare_module()?;
        let draft = self.lower_resolved_trivial_function_draft(plan)?;
        self.current_module
            .as_mut()
            .expect("prepare_module installs the candidate module")
            .try_add_function(draft)
            .map_err(
                |error| CanonicalResolvedBuildErrorV1::DuplicateFunctionPublication {
                    function_name: error.function_name,
                },
            )?;

        let entry_result = crate::mir::builder::emission::constant::emit_void(self)?;
        Ok(self.finalize_module(entry_result)?)
    }

    pub(in crate::mir) fn lower_resolved_trivial_function_draft(
        &mut self,
        plan: CanonicalTrivialBindingSsaPlanV1<'_>,
    ) -> Result<MirFunction, CanonicalResolvedBuildErrorV1> {
        self.lower_resolved_trivial_function_draft_inner(plan, false)
            .map_err(RejectedNormalFunctionDraftLoweringV1::into_compatibility_error)
    }

    #[cfg(test)]
    pub(in crate::mir) fn lower_resolved_trivial_function_draft_with_seal_failure_for_test(
        &mut self,
        plan: CanonicalTrivialBindingSsaPlanV1<'_>,
    ) -> Result<MirFunction, CanonicalResolvedBuildErrorV1> {
        self.lower_resolved_trivial_function_draft_inner(plan, true)
            .map_err(RejectedNormalFunctionDraftLoweringV1::into_compatibility_error)
    }

    /// TX0-private typed terminal. It consumes the preflight plan once and
    /// either yields the exact draft or a typed failure after the child
    /// session's discard terminal has restored the caller context.
    pub(in crate::mir) fn lower_resolved_trivial_function_draft_retaining_failure_v1(
        &mut self,
        plan: CanonicalTrivialBindingSsaPlanV1<'_>,
    ) -> Result<MirFunction, RejectedNormalFunctionDraftLoweringV1> {
        self.lower_resolved_trivial_function_draft_inner(plan, false)
    }

    fn lower_resolved_trivial_function_draft_inner(
        &mut self,
        plan: CanonicalTrivialBindingSsaPlanV1<'_>,
        _inject_seal_failure: bool,
    ) -> Result<MirFunction, RejectedNormalFunctionDraftLoweringV1> {
        let (input, if_control, completion, profile, block_expr_count) = plan.into_parts();
        let crate::ast::ASTNode::FunctionDeclaration {
            name,
            params,
            body,
            attrs,
            uses,
            ..
        } = input.source().root()
        else {
            unreachable!("preflight seals one FunctionDeclaration root")
        };
        let function_name = format!("{}/{}", name, params.len());
        let session_name = function_name.clone();
        let mut session = self.open_resolved_function_draft_seal_session_v1(&session_name);
        let lowering = {
            let builder = session.builder_view_mut_for_lowering();
            (|| -> Result<_, (NormalFunctionDraftLoweringStageV1, String)> {
                let recipe_preflight =
                    produce_trivial_if_physical_input_v1(&profile, input.function()).map_err(
                        |error| {
                            (
                                NormalFunctionDraftLoweringStageV1::BodyLowering,
                                format!("[freeze:contract][if_recipe/producer] {error:?}"),
                            )
                        },
                    )?;
                let recipe_admission =
                    admit_trivial_if_recipe_v1(recipe_preflight, input.function(), &if_control)
                        .map_err(|error| {
                            (
                                NormalFunctionDraftLoweringStageV1::BodyLowering,
                                format!("[freeze:contract][if_recipe/admission] {error:?}"),
                            )
                        })?;
                builder
                    .function_state
                    .resolved_binding_state
                    .install(input.function())
                    .map_err(|error| {
                        (
                            NormalFunctionDraftLoweringStageV1::BindingInstall,
                            error.to_string(),
                        )
                    })?;
                builder
                    .create_function_skeleton(function_name, params, body)
                    .map_err(|error| (NormalFunctionDraftLoweringStageV1::Skeleton, error))?;
                install_trivial_callable_abi_v1(builder, &profile);
                builder.set_current_function_runes(attrs);
                builder.set_current_function_declared_capability_uses(uses);

                let ready = CanonicalTrivialSsaLowererV1::new(
                    builder,
                    input,
                    if_control,
                    completion,
                    profile,
                    block_expr_count,
                    recipe_admission,
                )
                .map_err(|error| (NormalFunctionDraftLoweringStageV1::BodyLowering, error))?
                .lower()
                .map_err(|error| (NormalFunctionDraftLoweringStageV1::BodyLowering, error))?;
                #[cfg(test)]
                if _inject_seal_failure {
                    let current_block = builder.function_state.current_block.ok_or_else(|| {
                        (
                            NormalFunctionDraftLoweringStageV1::BodyLowering,
                            "[freeze:contract][if_recipe/test_failure_block_missing]"
                                .to_string(),
                        )
                    })?;
                    builder
                        .function_state
                        .current_function
                        .as_mut()
                        .and_then(|function| function.get_block_mut(current_block))
                        .ok_or_else(|| {
                            (
                                NormalFunctionDraftLoweringStageV1::BodyLowering,
                                "[freeze:contract][if_recipe/test_failure_block_missing]"
                                    .to_string(),
                            )
                        })?
                        .set_terminator(crate::mir::MirInstruction::Return { value: None });
                }
                Ok(ready)
            })()
        };
        let ready = match lowering {
            Ok(ready) => ready,
            Err((stage, error)) => return Err(reject_after_session_discard(session, stage, error)),
        };
        let open = ready.open(session);
        let prepared = match open.prepare() {
            Ok(prepared) => prepared,
            Err(rejected) => return Err(reject_draft_seal_typed(rejected)),
        };
        Ok(prepared.commit().into_draft())
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
