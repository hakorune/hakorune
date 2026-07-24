//! Strict F1 draft-seal vocabulary.
//!
//! This module is deliberately small in DRAFT-SEAL0-S0.  It does not touch
//! the legacy `MirBuilder::finalize_function_draft` terminal.  The only
//! responsibility here is to turn the already verified completion witness
//! into one move-only exit plan before a future builder/session prepare is
//! added.  Keeping this transition separate prevents the lowerers from
//! becoming a second Return/signature authority.

use std::collections::HashSet;

use crate::mir::builder::type_context::TypeContext;
use crate::mir::{
    BasicBlockId, ConstValue, MirBuilder, MirFunction, MirInstruction, MirType, ValueId,
};

use super::completion_consumption::ReadyFunctionCompletionV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PreparedFunctionExitV1 {
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

#[derive(Debug)]
pub(super) struct ReadyFunctionDraftSealV1 {
    completion: ReadyFunctionCompletionV1,
    current_block: BasicBlockId,
}

#[derive(Debug)]
pub(super) struct PreparedFunctionDraftSealV1 {
    completion: ReadyFunctionCompletionV1,
    exit: PreparedFunctionExitV1,
}

/// Private non-authority image used by the prepare phase.  It is deliberately
/// not a Builder/module owner: all propagation and exit materialization happen
/// on this image before the live function state can be touched.
#[derive(Debug)]
pub(super) struct FunctionDraftSealProjectionV1 {
    function: MirFunction,
    type_ctx: TypeContext,
    exit: PreparedFunctionExitV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum FunctionDraftSealProjectionErrorV1 {
    CurrentFunctionMissing,
    ExitBlockMissing { block: BasicBlockId },
    ExitBlockAlreadyTerminated { block: BasicBlockId },
    ReturnValueTypeMissing { value: ValueId },
    UnknownReturnValueType { value: ValueId },
    UnsupportedReturnValueType { value: ValueId, actual: MirType },
    ReturnSignatureMismatch { expected: MirType, actual: MirType },
    TypeAnalysisFailed(String),
    ValueIdOverflow,
}

#[derive(Debug)]
pub(super) struct RejectedFunctionDraftProjectionV1 {
    owner: PreparedFunctionDraftSealV1,
    error: FunctionDraftSealProjectionErrorV1,
}

#[derive(Debug)]
pub(super) struct CompletedFunctionDraftV1 {
    completion: ReadyFunctionCompletionV1,
    exit: PreparedFunctionExitV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum FunctionDraftSealPreparationErrorV1 {
    ExplicitValueOperandMissing,
}

#[derive(Debug)]
pub(super) struct RejectedFunctionDraftSealV1 {
    owner: ReadyFunctionDraftSealV1,
    error: FunctionDraftSealPreparationErrorV1,
}

impl ReadyFunctionDraftSealV1 {
    pub(super) fn new(completion: ReadyFunctionCompletionV1, current_block: BasicBlockId) -> Self {
        Self {
            completion,
            current_block,
        }
    }

    pub(super) fn prepare(
        self,
    ) -> Result<PreparedFunctionDraftSealV1, RejectedFunctionDraftSealV1> {
        let exit = if self.completion.is_implicit_void() {
            PreparedFunctionExitV1::ImplicitUnit {
                block: self.current_block,
            }
        } else if self.completion.returns_value() {
            let Some(witness) = self.completion.explicit_operand() else {
                return Err(RejectedFunctionDraftSealV1 {
                    owner: self,
                    error: FunctionDraftSealPreparationErrorV1::ExplicitValueOperandMissing,
                });
            };
            let block = witness.block();
            let value = witness.value();
            PreparedFunctionExitV1::ExplicitValue { block, value }
        } else {
            PreparedFunctionExitV1::ExplicitUnit {
                block: self.current_block,
            }
        };

        Ok(PreparedFunctionDraftSealV1 {
            completion: self.completion,
            exit,
        })
    }
}

impl PreparedFunctionDraftSealV1 {
    /// Build the projected completed-draft image without mutating `builder`.
    ///
    /// This is the first real PREPARE0 seam.  The copied maps are a planning
    /// image, not rollback state; the future commit terminal will move the
    /// approved image into the exclusive function owner exactly once.
    pub(super) fn project(
        self,
        builder: &MirBuilder,
    ) -> Result<FunctionDraftSealProjectionV1, RejectedFunctionDraftProjectionV1> {
        let mut function = match builder.function_state.current_function.as_ref() {
            Some(function) => function.clone(),
            None => {
                return Err(RejectedFunctionDraftProjectionV1 {
                    owner: self,
                    error: FunctionDraftSealProjectionErrorV1::CurrentFunctionMissing,
                })
            }
        };
        let mut type_ctx = clone_type_context(&builder.function_state.type_ctx);
        let exit = self.exit;
        let block = match exit {
            PreparedFunctionExitV1::ExplicitValue { block, .. }
            | PreparedFunctionExitV1::ExplicitUnit { block }
            | PreparedFunctionExitV1::ImplicitUnit { block } => block,
        };
        let Some(block_data) = function.blocks.get(&block) else {
            return Err(RejectedFunctionDraftProjectionV1 {
                owner: self,
                error: FunctionDraftSealProjectionErrorV1::ExitBlockMissing { block },
            });
        };
        if block_data.terminator.is_some() {
            return Err(RejectedFunctionDraftProjectionV1 {
                owner: self,
                error: FunctionDraftSealProjectionErrorV1::ExitBlockAlreadyTerminated { block },
            });
        }

        match exit {
            PreparedFunctionExitV1::ExplicitValue { value, .. } => {
                let Some(value_type) = type_ctx.value_types.get(&value).cloned() else {
                    return Err(RejectedFunctionDraftProjectionV1 {
                        owner: self,
                        error: FunctionDraftSealProjectionErrorV1::ReturnValueTypeMissing { value },
                    });
                };
                if value_type == MirType::Unknown {
                    return Err(RejectedFunctionDraftProjectionV1 {
                        owner: self,
                        error: FunctionDraftSealProjectionErrorV1::UnknownReturnValueType { value },
                    });
                }
                if !matches!(
                    value_type,
                    MirType::Integer | MirType::Bool | MirType::Float | MirType::Void
                ) {
                    return Err(RejectedFunctionDraftProjectionV1 {
                        owner: self,
                        error: FunctionDraftSealProjectionErrorV1::UnsupportedReturnValueType {
                            value,
                            actual: value_type,
                        },
                    });
                }
                if let Err(error) = set_projected_return_type(&mut function, value_type) {
                    return Err(RejectedFunctionDraftProjectionV1 { owner: self, error });
                }
                function
                    .blocks
                    .get_mut(&block)
                    .expect("validated exit block")
                    .add_instruction(MirInstruction::Return { value: Some(value) });
            }
            PreparedFunctionExitV1::ExplicitUnit { .. }
            | PreparedFunctionExitV1::ImplicitUnit { .. } => {
                let value = match allocate_projected_void(
                    &mut function,
                    &builder.function_state.compilation.reserved_value_ids,
                ) {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(RejectedFunctionDraftProjectionV1 { owner: self, error })
                    }
                };
                type_ctx.value_types.insert(value, MirType::Void);
                if let Err(error) = set_projected_return_type(&mut function, MirType::Void) {
                    return Err(RejectedFunctionDraftProjectionV1 { owner: self, error });
                }
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

        Ok(FunctionDraftSealProjectionV1 {
            function,
            type_ctx,
            exit,
        })
    }

    /// The DRAFT-SEAL0 commit is intentionally an ownership-only transition.
    /// Physical Return/signature writes will be added here once the projected
    /// type/session plans land; no fallible edge is allowed after this point.
    pub(super) fn commit(self) -> CompletedFunctionDraftV1 {
        CompletedFunctionDraftV1 {
            completion: self.completion,
            exit: self.exit,
        }
    }
}

impl RejectedFunctionDraftSealV1 {
    pub(super) fn error(&self) -> FunctionDraftSealPreparationErrorV1 {
        self.error
    }

    pub(super) fn discard(self) {
        let _ = self.owner;
    }
}

impl RejectedFunctionDraftProjectionV1 {
    pub(super) fn error(&self) -> &FunctionDraftSealProjectionErrorV1 {
        &self.error
    }

    pub(super) fn discard(self) {
        let _ = self.owner;
    }
}

impl FunctionDraftSealProjectionV1 {
    /// Run the shared type propagation order on the private projection only.
    /// No live `TypeContext` or `MirFunction` is passed to this entry.
    pub(super) fn prepare_type_facts(mut self) -> Result<Self, FunctionDraftSealProjectionErrorV1> {
        crate::mir::type_propagation::TypePropagationPipeline::run(
            &mut self.function,
            &mut self.type_ctx.value_types,
        )
        .map_err(FunctionDraftSealProjectionErrorV1::TypeAnalysisFailed)?;
        Ok(self)
    }

    #[cfg(test)]
    pub(super) fn function(&self) -> &MirFunction {
        &self.function
    }

    #[cfg(test)]
    pub(super) fn type_ctx(&self) -> &TypeContext {
        &self.type_ctx
    }
}

impl CompletedFunctionDraftV1 {
    pub(super) fn exit(&self) -> PreparedFunctionExitV1 {
        self.exit
    }

    pub(super) fn completion(&self) -> &ReadyFunctionCompletionV1 {
        &self.completion
    }
}

fn clone_type_context(source: &TypeContext) -> TypeContext {
    let mut target = TypeContext::new();
    target.value_types = source.value_types.clone();
    target.value_kinds = source.value_kinds.clone();
    target.value_origin_newbox = source.value_origin_newbox.clone();
    target.string_literals = source.string_literals.clone();
    target.map_value_types = source.map_value_types.clone();
    target.map_literal_value_types = source.map_literal_value_types.clone();
    target
}

fn allocate_projected_void(
    function: &mut MirFunction,
    reserved_value_ids: &HashSet<ValueId>,
) -> Result<ValueId, FunctionDraftSealProjectionErrorV1> {
    let mut next = function.next_value_id;
    loop {
        let candidate = ValueId::new(next);
        if !reserved_value_ids.contains(&candidate) {
            function.next_value_id = next
                .checked_add(1)
                .ok_or(FunctionDraftSealProjectionErrorV1::ValueIdOverflow)?;
            return Ok(candidate);
        }
        next = next
            .checked_add(1)
            .ok_or(FunctionDraftSealProjectionErrorV1::ValueIdOverflow)?;
    }
}

fn set_projected_return_type(
    function: &mut MirFunction,
    return_type: MirType,
) -> Result<(), FunctionDraftSealProjectionErrorV1> {
    if !matches!(
        function.signature.return_type,
        MirType::Void | MirType::Unknown
    ) && function.signature.return_type != return_type
    {
        return Err(
            FunctionDraftSealProjectionErrorV1::ReturnSignatureMismatch {
                expected: function.signature.return_type.clone(),
                actual: return_type,
            },
        );
    }
    function.signature.return_type = return_type;
    Ok(())
}
