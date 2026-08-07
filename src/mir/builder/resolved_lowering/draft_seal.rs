//! Strict F1 draft-seal vocabulary.
//!
//! This module is deliberately small in DRAFT-SEAL0-S0.  It does not touch
//! the legacy `MirBuilder::finalize_function_draft` terminal.  The only
//! responsibility here is to turn the already verified completion witness
//! into one move-only exit plan before a future builder/session prepare is
//! added.  Keeping this transition separate prevents the lowerers from
//! becoming a second Return/signature authority.

use std::collections::HashSet;

use crate::mir::builder::calls::{
    CanonicalFunctionLoweringSessionV1, PreparedFunctionSessionCommitInputV1,
};
use crate::mir::builder::emission::value_lifecycle_definition::{
    prepare_transient_stale_value_facts_v1, PreparedTransientStaleValueFactsV1,
};
use crate::mir::builder::function_signature_lookup::FunctionSignatureLookupV1;
use crate::mir::builder::type_context::TypeContext;
use crate::mir::function::FunctionMetadata;
use crate::mir::{
    BasicBlockId, ConstValue, MirBuilder, MirFunction, MirInstruction, MirType, ValueId,
};

use super::completion_consumption::ReadyFunctionCompletionV1;
use super::draft_seal_owner::OpenFunctionDraftSealV1;

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

/// Detached exit-only plan used by the PREPARE0 fixtures.  It owns no live
/// Builder session and therefore cannot be the canonical draft-seal commit.
#[derive(Debug)]
pub(super) struct PreparedFunctionExitPlanV1 {
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
    origin_caller_rows: Vec<(ValueId, String)>,
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
    owner: PreparedFunctionExitPlanV1,
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
pub(super) struct PreparedFunctionDraftSealPlanV1 {
    metadata: PreparedFunctionMetadataV1,
    projection: FunctionDraftSealProjectionV1,
    stale: PreparedTransientStaleValueFactsV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum FunctionDraftSealPreparationErrorV1 {
    ExplicitValueOperandMissing,
}

impl ReadyFunctionDraftSealV1 {
    pub(in crate::mir::builder::resolved_lowering) fn from_v2_finish(
        completion: ReadyFunctionCompletionV1,
        current_block: BasicBlockId,
    ) -> Self {
        Self {
            completion,
            current_block,
        }
    }

    pub(super) fn new(completion: ReadyFunctionCompletionV1, current_block: BasicBlockId) -> Self {
        Self {
            completion,
            current_block,
        }
    }

    pub(super) fn prepare(
        self,
    ) -> Result<PreparedFunctionExitPlanV1, FunctionDraftSealPreparationErrorV1> {
        let exit = match self.prepare_exit_borrowed() {
            Ok(exit) => exit,
            Err(error) => return Err(error),
        };

        Ok(PreparedFunctionExitPlanV1 {
            completion: self.completion,
            exit,
        })
    }

    /// Borrow-only exit projection used by the owner-preserving Open seam.
    /// No completion witness or session is consumed until every later plan
    /// has passed its own borrow-only checks.
    pub(super) fn prepare_exit_borrowed(
        &self,
    ) -> Result<PreparedFunctionExitV1, FunctionDraftSealPreparationErrorV1> {
        if self.completion.is_implicit_void() {
            return Ok(PreparedFunctionExitV1::ImplicitUnit {
                block: self.current_block,
            });
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

    pub(super) fn open<'builder>(
        self,
        session: CanonicalFunctionLoweringSessionV1<'builder>,
    ) -> OpenFunctionDraftSealV1<'builder> {
        super::draft_seal_owner::OpenFunctionDraftSealV1::new(session, self)
    }

    pub(super) fn into_completion(self) -> ReadyFunctionCompletionV1 {
        self.completion
    }
}

impl PreparedFunctionExitPlanV1 {
    /// Build the projected completed-draft image without mutating `builder`.
    ///
    /// This is the first real PREPARE0 seam.  The copied maps are a planning
    /// image, not rollback state; the future commit terminal will move the
    /// approved image into the exclusive function owner exactly once.
    pub(super) fn project(
        self,
        builder: &MirBuilder,
    ) -> Result<FunctionDraftSealProjectionV1, RejectedFunctionDraftProjectionV1> {
        let exit = self.exit;
        FunctionDraftSealProjectionV1::project_from_builder(builder, exit)
            .map_err(|error| RejectedFunctionDraftProjectionV1 { owner: self, error })
    }

    pub(super) fn project_exit(
        builder: &MirBuilder,
        exit: PreparedFunctionExitV1,
    ) -> Result<FunctionDraftSealProjectionV1, FunctionDraftSealProjectionErrorV1> {
        FunctionDraftSealProjectionV1::project_from_builder(builder, exit)
    }

    #[cfg(test)]
    pub(super) fn exit(&self) -> PreparedFunctionExitV1 {
        self.exit
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
        let mut type_ctx = clone_type_context(&builder.function_state.type_ctx);
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
                let value = allocate_projected_void(
                    &mut function,
                    &builder.function_state.compilation.reserved_value_ids,
                )?;
                type_ctx.value_types.insert(value, MirType::Void);
                set_projected_return_type(&mut function, MirType::Void)?;
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
        self.prepare_type_facts_with_lookup(None)
    }

    pub(super) fn prepare_type_facts_with_lookup(
        mut self,
        lookup: Option<&dyn FunctionSignatureLookupV1>,
    ) -> Result<Self, FunctionDraftSealProjectionErrorV1> {
        crate::mir::type_propagation::TypePropagationPipeline::run(
            &mut self.function,
            &mut self.type_ctx.value_types,
        )
        .map_err(FunctionDraftSealProjectionErrorV1::TypeAnalysisFailed)?;
        if let Some(lookup) = lookup {
            crate::mir::builder::type_hint_providers::annotate_missing_result_types_from_calls_and_await_with_lookup(
                &mut self.type_ctx,
                &self.function,
                lookup,
            );
        }
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
        for (value, origin) in &self.origin_caller_rows {
            self.function
                .metadata
                .value_origin_callers
                .insert(*value, origin.clone());
        }
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
        self.prepare_type_facts_with_lookup(None)
    }

    pub(super) fn prepare_type_facts_with_lookup(
        self,
        lookup: Option<&dyn FunctionSignatureLookupV1>,
    ) -> Result<PreparedFunctionTypeFactsV1, FunctionDraftSealProjectionErrorV1> {
        let projection = self.projection.prepare_type_facts_with_lookup(lookup)?;
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
    ) -> Result<PreparedFunctionDraftSealPlanV1, FunctionDraftSealProjectionErrorV1> {
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
        Ok(PreparedFunctionDraftSealPlanV1 {
            projection: FunctionDraftSealProjectionV1 {
                function: self.metadata.projection.function.clone(),
                type_ctx: verified_type_ctx,
                exit: self.metadata.projection.exit,
                origin_caller_rows: self.metadata.projection.origin_caller_rows.clone(),
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

impl PreparedFunctionDraftSealPlanV1 {
    /// Move the final projected function/type facts into the neutral session
    /// payload. Metadata/stale/verification receipts remain owned by the
    /// outer draft-seal plan; this payload is only the physical apply input.
    pub(super) fn into_session_commit_input(self) -> PreparedFunctionSessionCommitInputV1 {
        let mut function = self.projection.function;
        let type_ctx = self.projection.type_ctx;
        function.metadata.value_types = type_ctx.value_types.clone();
        PreparedFunctionSessionCommitInputV1::new(function, type_ctx)
    }

    pub(super) fn into_commit_parts(
        self,
    ) -> (
        PreparedFunctionSessionCommitInputV1,
        super::draft_seal_owner::FunctionDraftSealReceiptV1,
    ) {
        let receipt = super::draft_seal_owner::FunctionDraftSealReceiptV1 {
            signature: self.metadata.signature.clone(),
            phi: self.metadata.phi,
            stale_fact_count: self.stale.len(),
        };
        (self.into_session_commit_input(), receipt)
    }

    #[cfg(test)]
    pub(super) fn metadata(&self) -> &FunctionMetadata {
        &self.metadata.metadata
    }

    #[cfg(test)]
    pub(super) fn projection(&self) -> &FunctionDraftSealProjectionV1 {
        &self.projection
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
