//! Exact physical `main/0` thunk for one sealed canonical source Main.
//!
//! This box performs no source lookup and owns no process-exit policy.  It
//! materializes only the call/return relation already fixed by the thunk plan.

use crate::mir::builder::emission::value_lifecycle_definition::{
    verify_completed_draft_typed_value_definitions_v1, CompletedDraftTypedValueDefinitionErrorV1,
};
use crate::mir::canonical_direct_call::materialize_direct_call_effect_v1;
use crate::mir::canonical_direct_call_contract::VerifiedDirectCallEffectV1;
use crate::mir::compiler::capability::VerifiedResolvedOwnerHeaderV1;
use crate::mir::compiler::normal_source_plan::{
    VerifiedNormalMainEntryRelationV1, VerifiedNormalMainThunkResultV1,
};
use crate::mir::verification::MirVerifier;
use crate::mir::verification_types::VerificationError;
use crate::mir::{BasicBlockId, Callee, FunctionSignature, MirFunction, MirInstruction, ValueId};

use super::result_type::normal_main_result_mir_type;

#[derive(Debug, PartialEq)]
pub(in crate::mir::builder) enum NormalMainPhysicalThunkErrorV1 {
    SourceArityMismatch { actual: usize },
    PhysicalArityMismatch { actual: usize },
    TypedDefinition(CompletedDraftTypedValueDefinitionErrorV1),
    Verification(Box<[VerificationError]>),
}

#[derive(Debug)]
pub(in crate::mir::builder) struct VerifiedNormalMainPhysicalThunkDraftV1 {
    draft: MirFunction,
    result: VerifiedNormalMainThunkResultV1,
    _seal: VerifiedNormalMainPhysicalThunkDraftSealV1,
}

#[derive(Debug)]
struct VerifiedNormalMainPhysicalThunkDraftSealV1;

impl VerifiedNormalMainPhysicalThunkDraftV1 {
    pub(in crate::mir::builder) fn prepare(
        source: &VerifiedResolvedOwnerHeaderV1,
        result: VerifiedNormalMainThunkResultV1,
        entry: &VerifiedNormalMainEntryRelationV1,
    ) -> Result<Self, NormalMainPhysicalThunkErrorV1> {
        if source.arity() != 0 {
            return Err(NormalMainPhysicalThunkErrorV1::SourceArityMismatch {
                actual: source.arity(),
            });
        }
        if entry.physical_arity() != 0 {
            return Err(NormalMainPhysicalThunkErrorV1::PhysicalArityMismatch {
                actual: entry.physical_arity(),
            });
        }

        let return_type = normal_main_result_mir_type(result);
        let effects =
            materialize_direct_call_effect_v1(VerifiedDirectCallEffectV1::ConservativeBarrier);
        let entry_block = BasicBlockId::new(0);
        let mut draft = MirFunction::new(
            FunctionSignature {
                name: entry.physical_symbol().to_owned(),
                params: Vec::new(),
                return_type: return_type.clone(),
                effects,
            },
            entry_block,
        );
        let returned = if result == VerifiedNormalMainThunkResultV1::Unit {
            None
        } else {
            let value = draft.next_value_id();
            draft.metadata.value_types.insert(value, return_type);
            Some(value)
        };
        let block = draft
            .get_block_mut(entry_block)
            .expect("MirFunction::new installs its entry block");
        block.add_instruction(MirInstruction::Call {
            dst: returned,
            func: ValueId::INVALID,
            callee: Some(Callee::Global(source.symbol().as_mir_name().to_owned())),
            args: Vec::new(),
            effects,
        });
        block.set_terminator(MirInstruction::Return { value: returned });

        verify_completed_draft_typed_value_definitions_v1(&draft, &draft.metadata.value_types)
            .map_err(NormalMainPhysicalThunkErrorV1::TypedDefinition)?;
        MirVerifier::new()
            .verify_function(&draft)
            .map_err(|errors| {
                NormalMainPhysicalThunkErrorV1::Verification(errors.into_boxed_slice())
            })?;
        Ok(Self {
            draft,
            result,
            _seal: VerifiedNormalMainPhysicalThunkDraftSealV1,
        })
    }

    pub(in crate::mir::builder) fn draft(&self) -> &MirFunction {
        &self.draft
    }

    pub(in crate::mir::builder) fn into_draft(self) -> MirFunction {
        self.draft
    }

    pub(in crate::mir::builder) const fn result(&self) -> VerifiedNormalMainThunkResultV1 {
        self.result
    }
}
