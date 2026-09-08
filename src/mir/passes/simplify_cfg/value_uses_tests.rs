use super::rewrite_value_uses_in_instruction;
use crate::mir::definitions::call_unified::{Callee, CalleeBoxKind, TypeCertainty};
use crate::mir::{EffectMask, MirInstruction, ValueId};

#[test]
fn simplify_cfg_call_use_rewrite_preserves_typed_targets_and_args() {
    let from = ValueId::new(1);
    let to = ValueId::new(2);
    let mut instruction = MirInstruction::LegacyCallV0 {
        dst: Some(ValueId::new(30)),
        func: ValueId::new(99),
        callee: Some(Callee::Closure {
            params: vec!["x".to_string()],
            captures: vec![("a".to_string(), from), ("b".to_string(), from)],
            me_capture: Some(from),
        }),
        args: vec![from, ValueId::new(4)],
        effects: EffectMask::PURE,
    };

    rewrite_value_uses_in_instruction(&mut instruction, from, to);

    let MirInstruction::LegacyCallV0 {
        dst,
        func,
        callee:
            Some(Callee::Closure {
                captures,
                me_capture,
                ..
            }),
        args,
        ..
    } = instruction
    else {
        panic!("typed Call shape changed");
    };
    assert_eq!(dst, Some(ValueId::new(30)));
    assert_eq!(func, ValueId::new(99));
    assert_eq!(
        captures
            .into_iter()
            .map(|(_, value)| value)
            .collect::<Vec<_>>(),
        vec![to, to]
    );
    assert_eq!(me_capture, Some(to));
    assert_eq!(args, vec![to, ValueId::new(4)]);
}

#[test]
fn simplify_cfg_call_use_rewrite_keeps_targetless_callees_empty() {
    let from = ValueId::new(1);
    let to = ValueId::new(2);
    let mut shapes = vec![
        Callee::Global(crate::mir::test_global_target("f".to_string())),
        Callee::Extern("env.f".to_string()),
        Callee::Constructor {
            box_type: "Box".to_string(),
        },
        Callee::Method {
            box_name: "Box".to_string(),
            method: "f".to_string(),
            receiver: None,
            certainty: TypeCertainty::Known,
            box_kind: CalleeBoxKind::UserDefined,
        },
    ];
    for callee in &mut shapes {
        let mut instruction = MirInstruction::LegacyCallV0 {
            dst: None,
            func: ValueId::new(99),
            callee: Some(callee.clone()),
            args: vec![from],
            effects: EffectMask::PURE,
        };
        rewrite_value_uses_in_instruction(&mut instruction, from, to);
        let MirInstruction::LegacyCallV0 {
            func,
            callee: Some(actual),
            args,
            ..
        } = instruction
        else {
            panic!("targetless Call shape changed");
        };
        assert_eq!(func, ValueId::new(99));
        assert_eq!(actual, *callee);
        assert_eq!(args, vec![to]);
    }
}

#[test]
fn simplify_cfg_call_use_rewrite_preserves_legacy_func_parity() {
    let from = ValueId::new(1);
    let to = ValueId::new(2);
    let mut instruction = MirInstruction::LegacyCallV0 {
        dst: None,
        func: from,
        callee: None,
        args: vec![from],
        effects: EffectMask::PURE,
    };

    rewrite_value_uses_in_instruction(&mut instruction, from, to);

    let MirInstruction::LegacyCallV0 {
        func, callee, args, ..
    } = instruction
    else {
        panic!("legacy Call shape changed");
    };
    assert_eq!(func, to);
    assert!(callee.is_none());
    assert_eq!(args, vec![to]);
}
