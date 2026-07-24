//! ROOTBATCH0-S0b focused contract tests for the typed condition factory.

use super::raw_required_condition_draft::RawRequiredConditionDraftV1;
use crate::mir::{ConstValue, EffectMask, MirInstruction, MirType};

#[test]
fn raw_required_condition_factory_has_exact_signature_and_body() {
    let condition = RawRequiredConditionDraftV1::build();
    let draft = condition.draft();

    assert_eq!(draft.signature.name, "condition_fn");
    assert_eq!(draft.signature.params, vec![MirType::Integer]);
    assert_eq!(draft.signature.return_type, MirType::Integer);
    assert_eq!(draft.signature.effects, EffectMask::PURE);
    assert_eq!(draft.blocks.len(), 1);

    let entry = draft
        .blocks
        .get(&draft.entry_block)
        .expect("factory entry block");
    assert_eq!(entry.instructions.len(), 1);
    let const_dst = match &entry.instructions[0] {
        MirInstruction::Const {
            dst,
            value: ConstValue::Integer(value),
        } => {
            assert_eq!(*value, 1);
            *dst
        }
        other => panic!("expected one integer-one const, got {other:?}"),
    };
    assert_eq!(
        entry.terminator,
        Some(MirInstruction::Return {
            value: Some(const_dst),
        })
    );
    assert!(entry.effects == EffectMask::PURE);
}
