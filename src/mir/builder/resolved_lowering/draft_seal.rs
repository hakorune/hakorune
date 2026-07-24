//! Strict F1 draft-seal vocabulary.
//!
//! This module is deliberately small in DRAFT-SEAL0-S0.  It does not touch
//! the legacy `MirBuilder::finalize_function_draft` terminal.  The only
//! responsibility here is to turn the already verified completion witness
//! into one move-only exit plan before a future builder/session prepare is
//! added.  Keeping this transition separate prevents the lowerers from
//! becoming a second Return/signature authority.

use std::collections::HashSet;

use crate::mir::builder::emission::value_lifecycle_definition::{
    prepare_transient_stale_value_facts_v1, PreparedTransientStaleValueFactsV1,
};
use crate::mir::builder::type_context::TypeContext;
use crate::mir::function::FunctionMetadata;
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
    PhiClosureFailed(String),
    TypeAnalysisFailed(String),
    MetadataContractFailed(String),
    StaleFacts(String),
    TypedValueVerificationFailed(String),
    ProjectedVerificationFailed(String),
    ValueIdOverflow,
}

#[derive(Debug)]
pub(super) struct RejectedFunctionDraftProjectionV1 {
    owner: PreparedFunctionDraftSealV1,
    error: FunctionDraftSealProjectionErrorV1,
}

#[derive(Debug)]
pub(super) struct PreparedFunctionStaleFactsV1 {
    metadata: PreparedFunctionMetadataV1,
    stale: PreparedTransientStaleValueFactsV1,
}

/// Metadata and executable boundary contracts prepared on the projected
/// function.  This is a private plan: the live function metadata remains
/// untouched until the eventual draft-seal commit.
#[derive(Debug)]
pub(super) struct PreparedFunctionMetadataV1 {
    projection: FunctionDraftSealProjectionV1,
    metadata: FunctionMetadata,
    signature: PreparedFunctionSignatureV1,
    phi: PreparedFunctionPhiClosureReceiptV1,
}

/// Proof-only receipt for the strict PHI/CFG closure check. The draft seal
/// never repairs PHI edges; repair-required functions reject before commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct PreparedFunctionPhiClosureReceiptV1;

#[derive(Debug)]
pub(super) struct PreparedFunctionPhiSealV1 {
    projection: FunctionDraftSealProjectionV1,
    receipt: PreparedFunctionPhiClosureReceiptV1,
}

#[derive(Debug)]
pub(super) struct PreparedFunctionTypeFactsV1 {
    projection: FunctionDraftSealProjectionV1,
    phi: PreparedFunctionPhiClosureReceiptV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum PreparedFunctionResultV1 {
    Unit,
    ExactOperand {
        value: ValueId,
        return_type: MirType,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PreparedFunctionSignatureV1 {
    result: PreparedFunctionResultV1,
}

#[derive(Debug)]
pub(super) struct VerifiedFunctionDraftProjectionV1 {
    metadata: PreparedFunctionMetadataV1,
    projection: FunctionDraftSealProjectionV1,
    stale: PreparedTransientStaleValueFactsV1,
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
    /// Verify terminator-derived PHI/CFG closure on the private projection.
    /// This is the strict verifier only; legacy whole-function PHI repair is
    /// not a draft-seal responsibility.
    pub(super) fn prepare_phi_closure(
        self,
    ) -> Result<PreparedFunctionPhiSealV1, FunctionDraftSealProjectionErrorV1> {
        crate::mir::builder::ssa::phi_input_materializer::edge_verifier::verify_phi_edges_v1(
            &self.function,
        )
        .map_err(|errors| {
            FunctionDraftSealProjectionErrorV1::PhiClosureFailed(format!("{errors:?}"))
        })?;
        Ok(PreparedFunctionPhiSealV1 {
            projection: self,
            receipt: PreparedFunctionPhiClosureReceiptV1,
        })
    }

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

    /// Prepare the existing parameter/return contract carriers on the private
    /// projection.  These helpers are the metadata SSOT; the draft seal only
    /// snapshots their planned result and never invents a second contract.
    pub(super) fn prepare_metadata(
        mut self,
    ) -> Result<PreparedFunctionMetadataV1, FunctionDraftSealProjectionErrorV1> {
        let signature = self.prepare_signature()?;
        let return_type = match &signature.result {
            PreparedFunctionResultV1::Unit => MirType::Void,
            PreparedFunctionResultV1::ExactOperand { return_type, .. } => return_type.clone(),
        };
        set_projected_return_type(&mut self.function, return_type)?;
        crate::mir::type_contracts::parameter_entry::refresh_function_parameter_entry_contracts(
            &mut self.function,
        );
        crate::mir::type_contracts::return_exit::refresh_function_return_exit_contract(
            &mut self.function,
        );
        crate::mir::type_contracts::parameter_entry::validate_parameter_entry_contracts(
            &self.function,
        )
        .map_err(FunctionDraftSealProjectionErrorV1::MetadataContractFailed)?;
        crate::mir::type_contracts::return_exit::validate_return_exit_contract(&self.function)
            .map_err(FunctionDraftSealProjectionErrorV1::MetadataContractFailed)?;
        self.function.metadata.value_types = self.type_ctx.value_types.clone();
        let metadata = self.function.metadata.clone();
        Ok(PreparedFunctionMetadataV1 {
            projection: self,
            metadata,
            signature,
            phi: PreparedFunctionPhiClosureReceiptV1,
        })
    }

    /// Resolve the result/signature relation from the already projected exit
    /// plan.  This deliberately does not scan Return instructions or infer
    /// from the last produced ValueId.
    pub(super) fn prepare_signature(
        &self,
    ) -> Result<PreparedFunctionSignatureV1, FunctionDraftSealProjectionErrorV1> {
        let result = match self.exit {
            PreparedFunctionExitV1::ExplicitValue { value, .. } => {
                let Some(return_type) = self.type_ctx.value_types.get(&value).cloned() else {
                    return Err(FunctionDraftSealProjectionErrorV1::ReturnValueTypeMissing {
                        value,
                    });
                };
                if return_type == MirType::Unknown {
                    return Err(FunctionDraftSealProjectionErrorV1::UnknownReturnValueType {
                        value,
                    });
                }
                if !matches!(
                    return_type,
                    MirType::Integer | MirType::Bool | MirType::Float | MirType::Void
                ) {
                    return Err(
                        FunctionDraftSealProjectionErrorV1::UnsupportedReturnValueType {
                            value,
                            actual: return_type,
                        },
                    );
                }
                PreparedFunctionResultV1::ExactOperand { value, return_type }
            }
            PreparedFunctionExitV1::ExplicitUnit { .. }
            | PreparedFunctionExitV1::ImplicitUnit { .. } => PreparedFunctionResultV1::Unit,
        };
        Ok(PreparedFunctionSignatureV1 { result })
    }
}

impl PreparedFunctionPhiSealV1 {
    /// Continue PREPARE0 only after the pure PHI/CFG receipt exists.
    pub(super) fn prepare_type_facts(
        self,
    ) -> Result<PreparedFunctionTypeFactsV1, FunctionDraftSealProjectionErrorV1> {
        let projection = self.projection.prepare_type_facts()?;
        Ok(PreparedFunctionTypeFactsV1 {
            projection,
            phi: self.receipt,
        })
    }
}

impl PreparedFunctionTypeFactsV1 {
    /// Prepare metadata from the type-planned private image while retaining
    /// the PHI closure receipt for the future outer draft-seal owner.
    pub(super) fn prepare_metadata(
        self,
    ) -> Result<PreparedFunctionMetadataV1, FunctionDraftSealProjectionErrorV1> {
        let mut metadata = self.projection.prepare_metadata()?;
        metadata.phi = self.phi;
        Ok(metadata)
    }

    #[cfg(test)]
    pub(super) fn projection(&self) -> &FunctionDraftSealProjectionV1 {
        &self.projection
    }
}

impl PreparedFunctionMetadataV1 {
    /// Prepare stale-fact removal after metadata/contract planning.  The
    /// planned metadata remains installed on the private projection.
    pub(super) fn prepare_stale_facts(
        self,
        builder: &MirBuilder,
    ) -> Result<PreparedFunctionStaleFactsV1, (Self, FunctionDraftSealProjectionErrorV1)> {
        let pending_phi_destinations = builder
            .function_state
            .pending_phis
            .iter()
            .map(|(_, value, _)| *value)
            .collect();
        let pinned_values = builder
            .function_state
            .pin_slot_names
            .keys()
            .copied()
            .collect();
        let stale = match prepare_transient_stale_value_facts_v1(
            &self.projection.function,
            &self.projection.type_ctx.value_types,
            &pending_phi_destinations,
            &pinned_values,
        ) {
            Ok(stale) => stale,
            Err(error) => {
                return Err((
                    self,
                    FunctionDraftSealProjectionErrorV1::StaleFacts(error.to_string()),
                ))
            }
        };
        Ok(PreparedFunctionStaleFactsV1 {
            metadata: self,
            stale,
        })
    }

    #[cfg(test)]
    pub(super) fn metadata(&self) -> &FunctionMetadata {
        &self.metadata
    }

    #[cfg(test)]
    pub(super) fn projection(&self) -> &FunctionDraftSealProjectionV1 {
        &self.projection
    }

    #[cfg(test)]
    pub(super) fn signature(&self) -> PreparedFunctionSignatureV1 {
        self.signature.clone()
    }
}

impl PreparedFunctionSignatureV1 {
    #[cfg(test)]
    pub(super) fn result(&self) -> PreparedFunctionResultV1 {
        self.result.clone()
    }
}

impl FunctionDraftSealProjectionV1 {
    #[cfg(test)]
    pub(super) fn function(&self) -> &MirFunction {
        &self.function
    }

    #[cfg(test)]
    pub(super) fn type_ctx(&self) -> &TypeContext {
        &self.type_ctx
    }
}

impl PreparedFunctionStaleFactsV1 {
    /// Verify the projected completed-draft image after applying stale-fact
    /// removals to a second private facts map. The original plan remains
    /// available for the eventual commit terminal.
    pub(super) fn verify(
        self,
    ) -> Result<VerifiedFunctionDraftProjectionV1, FunctionDraftSealProjectionErrorV1> {
        let mut verified_type_ctx = clone_type_context(&self.metadata.projection.type_ctx);
        self.stale.apply_to_type_context(&mut verified_type_ctx);
        crate::mir::builder::emission::value_lifecycle_definition::verify_completed_draft_typed_value_definitions_v1(
            &self.metadata.projection.function,
            &verified_type_ctx.value_types,
        )
        .map_err(|error| {
            FunctionDraftSealProjectionErrorV1::TypedValueVerificationFailed(error.to_string())
        })?;
        crate::mir::verification::MirVerifier::new()
            .verify_function(&self.metadata.projection.function)
            .map_err(|errors| {
                FunctionDraftSealProjectionErrorV1::ProjectedVerificationFailed(format!(
                    "{errors:?}"
                ))
            })?;
        Ok(VerifiedFunctionDraftProjectionV1 {
            projection: FunctionDraftSealProjectionV1 {
                function: self.metadata.projection.function.clone(),
                type_ctx: verified_type_ctx,
                exit: self.metadata.projection.exit,
            },
            metadata: self.metadata,
            stale: self.stale,
        })
    }

    #[cfg(test)]
    pub(super) fn stale_count(&self) -> usize {
        self.stale.len()
    }

    #[cfg(test)]
    pub(super) fn projection(&self) -> &FunctionDraftSealProjectionV1 {
        &self.metadata.projection
    }
}

impl VerifiedFunctionDraftProjectionV1 {
    #[cfg(test)]
    pub(super) fn metadata(&self) -> &FunctionMetadata {
        &self.metadata.metadata
    }

    #[cfg(test)]
    pub(super) fn projection(&self) -> &FunctionDraftSealProjectionV1 {
        &self.projection
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
