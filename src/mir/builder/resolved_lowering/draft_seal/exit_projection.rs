//! Single-site Return projection for the strict draft-seal seam.
//!
//! This child module owns only the existing bounded exit vocabulary and its
//! borrow-only projection.  It deliberately does not add multi-site
//! physicalization or a second Completion/Return authority.

use crate::mir::{BasicBlockId, ConstValue, MirBuilder, MirInstruction, MirType};

use super::{
    FunctionDraftSealProjectionErrorV1, FunctionDraftSealProjectionV1, ReadyFunctionDraftSealV1,
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
        let mut function = builder
            .function_state
            .current_function
            .as_ref()
            .ok_or(FunctionDraftSealProjectionErrorV1::CurrentFunctionMissing)?
            .clone();
        let mut type_ctx = super::clone_type_context(&builder.function_state.type_ctx);
        let origin_caller_rows = builder.value_origin_caller_rows();
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
            return Err(FunctionDraftSealProjectionErrorV1::ExitBlockAlreadyTerminated { block });
        }

        match exit {
            PreparedFunctionExitV1::ExplicitValue { value, .. } => {
                // The exact operand type is resolved after the private type
                // propagation plan. Projection only records the physical
                // operand and never makes an early signature decision.
                function
                    .blocks
                    .get_mut(&block)
                    .expect("validated exit block")
                    .add_instruction(MirInstruction::Return { value: Some(value) });
            }
            PreparedFunctionExitV1::ExplicitUnit { .. }
            | PreparedFunctionExitV1::ImplicitUnit { .. } => {
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

        Ok(Self {
            function,
            type_ctx,
            exit,
            origin_caller_rows,
        })
    }
}
