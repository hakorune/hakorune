use super::*;
use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
use crate::mir::{Callee, ConstValue, EffectMask, MirInstruction, ValueId};

fn method_set_callee(receiver: Option<ValueId>, method: &str) -> Callee {
    Callee::Method {
        box_name: "ArrayBox".to_string(),
        method: method.to_string(),
        receiver,
        certainty: TypeCertainty::Union,
        box_kind: CalleeBoxKind::UserDefined,
    }
}

#[test]
fn rewrite_method_set_value_preserves_typed_call_fields_and_replaces_only_value() {
    let callee = method_set_callee(Some(ValueId(1)), "set");
    let inst = MirInstruction::Call {
        dst: Some(ValueId(9)),
        func: ValueId(99),
        callee: Some(callee.clone()),
        args: vec![ValueId(2), ValueId(3)],
        effects: EffectMask::READ,
    };

    let rewritten = rewrite_method_set_value(&inst, ValueId(4)).expect("set call rewrite");
    match rewritten {
        MirInstruction::Call {
            dst,
            func,
            callee: Some(actual_callee),
            args,
            effects,
        } => {
            assert_eq!(dst, Some(ValueId(9)));
            assert_eq!(func, ValueId::INVALID, "legacy decoration is retired");
            assert_eq!(actual_callee, callee);
            assert_eq!(args, vec![ValueId(2), ValueId(4)]);
            assert_eq!(effects, EffectMask::READ);
        }
        other => panic!("expected canonical method-set Call, got {other:?}"),
    }
}

#[test]
fn rewrite_method_set_value_rejects_finite_non_method_set_shapes() {
    let base_args = vec![ValueId(2), ValueId(3)];
    let cases = vec![
        (
            "method-none",
            MirInstruction::Call {
                dst: Some(ValueId(9)),
                func: ValueId(99),
                callee: Some(method_set_callee(None, "set")),
                args: base_args.clone(),
                effects: EffectMask::READ,
            },
        ),
        (
            "global",
            MirInstruction::Call {
                dst: Some(ValueId(9)),
                func: ValueId(99),
                callee: Some(Callee::Global(crate::mir::test_global_target(
                    "array.set/2".to_string(),
                ))),
                args: base_args.clone(),
                effects: EffectMask::READ,
            },
        ),
        (
            "missing-callee",
            MirInstruction::Call {
                dst: Some(ValueId(9)),
                func: ValueId(99),
                callee: None,
                args: base_args.clone(),
                effects: EffectMask::READ,
            },
        ),
        (
            "wrong-method",
            MirInstruction::Call {
                dst: Some(ValueId(9)),
                func: ValueId(99),
                callee: Some(method_set_callee(Some(ValueId(1)), "get")),
                args: base_args.clone(),
                effects: EffectMask::READ,
            },
        ),
        (
            "wrong-arity",
            MirInstruction::Call {
                dst: Some(ValueId(9)),
                func: ValueId(99),
                callee: Some(method_set_callee(Some(ValueId(1)), "set")),
                args: vec![ValueId(2)],
                effects: EffectMask::READ,
            },
        ),
        (
            "non-call",
            MirInstruction::Const {
                dst: ValueId(9),
                value: ConstValue::Integer(0),
            },
        ),
    ];

    for (label, inst) in cases {
        assert!(
            rewrite_method_set_value(&inst, ValueId(4)).is_none(),
            "{label} must not publish a replacement"
        );
    }
}
