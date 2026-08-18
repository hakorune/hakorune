//! Return projection for the strict draft-seal seam.
//!
//! This child module owns the existing bounded exit vocabulary and its
//! detached projection.  Multi-site claims are supplied by the focused
//! `multi_site_exit` child; this module never creates a second
//! Completion/Return authority.

use crate::mir::{BasicBlockId, ConstValue, MirBuilder, MirInstruction, MirType};

use crate::mir::builder::resolved_lowering::canonical_cfg::CanonicalCfgSessionV1;
use crate::mir::pinned_text_residence_lifecycle::PinnedTextResidenceFinishCapabilityV1;

use super::{
    FunctionDraftSealProjectionErrorV1, FunctionDraftSealProjectionV1, PreparedFunctionExitSetV1,
    ReadyFunctionDraftSealV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder::resolved_lowering) enum PreparedFunctionExitV1 {
    ExplicitValue {
        block: BasicBlockId,
        value: crate::mir::ValueId,
    },
    ExplicitUnit {
        block: BasicBlockId,
    },
    ImplicitUnit {
        block: BasicBlockId,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder::resolved_lowering) enum FunctionDraftSealPreparationErrorV1 {
    ExplicitValueOperandMissing,
    MultipleExplicitReturnClaimsUnsupported,
    MultiSite(super::multi_site_exit::MultiSiteExitPreparationErrorV1),
}

impl ReadyFunctionDraftSealV1 {
    /// Borrow-only exit projection used by the owner-preserving Open seam.
    /// No completion witness or session is consumed until every later plan
    /// has passed its own borrow-only checks.
    pub(in crate::mir::builder::resolved_lowering) fn prepare_exit_borrowed(
        &self,
    ) -> Result<PreparedFunctionExitV1, FunctionDraftSealPreparationErrorV1> {
        if self.completion.is_implicit_void() {
            return Ok(PreparedFunctionExitV1::ImplicitUnit {
                block: self.current_block,
            });
        }
        if self.completion.explicit_claims().len() > 1 {
            return Err(
                FunctionDraftSealPreparationErrorV1::MultipleExplicitReturnClaimsUnsupported,
            );
        }
        if self.completion.returns_value() {
            let Some(witness) = self.completion.explicit_operand() else {
                return Err(FunctionDraftSealPreparationErrorV1::ExplicitValueOperandMissing);
            };
            return Ok(PreparedFunctionExitV1::ExplicitValue {
                block: witness.block(),
                value: witness.value(),
            });
        }
        Ok(PreparedFunctionExitV1::ExplicitUnit {
            block: self.current_block,
        })
    }
}

impl FunctionDraftSealProjectionV1 {
    /// Construct a detached projection from a borrowed Builder and one
    /// already-resolved exit. The caller keeps its outer owner intact when a
    /// projection-stage error occurs.
    pub(super) fn project_from_builder(
        builder: &MirBuilder,
        exit: PreparedFunctionExitV1,
    ) -> Result<Self, FunctionDraftSealProjectionErrorV1> {
        Self::project_from_builder_exit_set(builder, PreparedFunctionExitSetV1::single(exit))
            .map_err(|(_, error)| error)
    }

    pub(in crate::mir::builder::resolved_lowering) fn project_from_builder_exit_set(
        builder: &MirBuilder,
        exit: PreparedFunctionExitSetV1,
    ) -> Result<
        Self,
        (
            PreparedFunctionExitSetV1,
            FunctionDraftSealProjectionErrorV1,
        ),
    > {
        Self::project_from_builder_exit_set_with_finish(builder, exit, None)
    }

    #[cfg(test)]
    pub(super) fn project_from_builder_pinned_text(
        builder: &MirBuilder,
        consumer: super::text_residence_ingress::PreparedPinnedTextResidenceDraftSealConsumerV1,
        plans: &crate::mir::pinned_text_access_plan::PinnedTextAccessPlanTableV1,
        frame: crate::mir::compiler::pinned_text_backend_frame::PinnedTextBackendFrameBorrowV1<'_>,
    ) -> Result<Self, String> {
        let (exit, finish) = consumer.into_projection_parts(plans, frame)?;
        Self::project_from_builder_exit_set_with_finish(builder, exit, Some(finish))
            .map_err(|(_, error)| format!("{error:?}"))
    }

    fn project_from_builder_exit_set_with_finish(
        builder: &MirBuilder,
        exit: PreparedFunctionExitSetV1,
        finish: Option<PinnedTextResidenceFinishCapabilityV1>,
    ) -> Result<
        Self,
        (
            PreparedFunctionExitSetV1,
            FunctionDraftSealProjectionErrorV1,
        ),
    > {
        let mut function = match builder.function_state.current_function.as_ref() {
            Some(function) => function.clone(),
            None => {
                return Err((
                    exit,
                    FunctionDraftSealProjectionErrorV1::CurrentFunctionMissing,
                ))
            }
        };
        let mut type_ctx = super::clone_type_context(&builder.function_state.type_ctx);
        let origin_caller_rows = builder.value_origin_caller_rows();
        let mut seen_blocks = std::collections::BTreeSet::new();
        let result = exit.try_for_each_exit(|exit| {
            let block = match exit {
                PreparedFunctionExitV1::ExplicitValue { block, .. }
                | PreparedFunctionExitV1::ExplicitUnit { block }
                | PreparedFunctionExitV1::ImplicitUnit { block } => block,
            };
            let block_data = function
                .blocks
                .get(&block)
                .ok_or(FunctionDraftSealProjectionErrorV1::ExitBlockMissing { block })?;
            if block_data.terminator.is_some() {
                return Err(
                    FunctionDraftSealProjectionErrorV1::ExitBlockAlreadyTerminated { block },
                );
            }
            if !seen_blocks.insert(block) {
                return Err(
                    FunctionDraftSealProjectionErrorV1::ExitBlockAlreadyTerminated { block },
                );
            }

            match exit {
                PreparedFunctionExitV1::ExplicitValue { value, .. } => {
                    if let Some(finish) = finish.as_ref() {
                        CanonicalCfgSessionV1::emit_pinned_text_residence_finish_detached(
                            &mut function,
                            block,
                            finish.residence(),
                        )
                        .map_err(|error| {
                            FunctionDraftSealProjectionErrorV1::PinnedTextResidence(
                                error.to_string(),
                            )
                        })?;
                    }
                    function
                        .blocks
                        .get_mut(&block)
                        .expect("validated exit block")
                        .add_instruction(MirInstruction::Return { value: Some(value) });
                }
                PreparedFunctionExitV1::ExplicitUnit { .. }
                | PreparedFunctionExitV1::ImplicitUnit { .. } => {
                    if finish.is_some() {
                        return Err(FunctionDraftSealProjectionErrorV1::PinnedTextResidence(
                            "pinned-Text Finish requires an explicit value exit".to_owned(),
                        ));
                    }
                    let value = super::allocate_projected_void(
                        &mut function,
                        &builder.function_state.compilation.reserved_value_ids,
                    )?;
                    type_ctx.value_types.insert(value, MirType::Void);
                    super::set_projected_return_type(&mut function, MirType::Void)?;
                    let block_data = function
                        .blocks
                        .get_mut(&block)
                        .expect("validated exit block");
                    block_data.add_instruction(MirInstruction::Const {
                        dst: value,
                        value: ConstValue::Void,
                    });
                    block_data.add_instruction(MirInstruction::Return { value: Some(value) });
                }
            }
            Ok(())
        });
        if let Err(error) = result {
            return Err((exit, error));
        }

        Ok(Self {
            function,
            type_ctx,
            exit,
            origin_caller_rows,
        })
    }
}
