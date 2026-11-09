//! ROOTBATCH0-S0b: the one typed Raw compatibility condition draft.
//!
//! This factory is intentionally independent of source AST and caller policy.
//! A required Raw root batch always receives the same `condition_fn/1` draft:
//! one `Integer` parameter, an `Integer` result, and a pure constant-one
//! return.  The product is non-Clone so a later batch terminal can consume it
//! exactly once.

use crate::mir::{
    BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirFunction, MirInstruction, MirType,
};

#[derive(Debug)]
pub(in crate::mir) struct RawRequiredConditionDraftV1 {
    draft: MirFunction,
    _seal: RawRequiredConditionDraftSealV1,
}

#[derive(Debug)]
struct RawRequiredConditionDraftSealV1;

impl RawRequiredConditionDraftV1 {
    /// Construct the exact Raw compatibility condition function.
    pub(in crate::mir::builder) fn build() -> Self {
        let mut draft = MirFunction::new(
            FunctionSignature {
                name: "condition_fn".to_owned(),
                params: vec![MirType::Integer],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let value = draft.next_value_id();
        let entry = draft
            .get_block_mut(BasicBlockId::new(0))
            .expect("MirFunction::new always creates its entry block");
        entry.add_instruction(MirInstruction::Const {
            dst: value,
            value: ConstValue::Integer(1),
        });
        entry.add_instruction(MirInstruction::Return { value: Some(value) });
        Self {
            draft,
            _seal: RawRequiredConditionDraftSealV1,
        }
    }

    pub(in crate::mir::builder) fn draft(&self) -> &MirFunction {
        &self.draft
    }

    /// Consume the typed draft at the future private ROOTBATCH commit seam.
    pub(in crate::mir::builder) fn into_draft(self) -> MirFunction {
        self.draft
    }
}
