//! Tests for MIR Instructions
//!
//! Comprehensive test suite for all MIR instruction types and their methods.

use super::super::{EffectMask, ValueId};
use super::MirInstruction;
use crate::mir::types::{BinaryOp, ConstValue};

#[test]
fn test_const_instruction() {
    let dst = ValueId::new(0);
    let inst = MirInstruction::Const {
        dst,
        value: ConstValue::Integer(42),
    };

    assert_eq!(inst.dst_value(), Some(dst));
    assert!(inst.used_values().is_empty());
    assert!(inst.effects().is_pure());
}

#[test]
fn test_binop_instruction() {
    let dst = ValueId::new(0);
    let lhs = ValueId::new(1);
    let rhs = ValueId::new(2);

    let inst = MirInstruction::BinOp {
        dst,
        op: BinaryOp::Add,
        lhs,
        rhs,
    };

    assert_eq!(inst.dst_value(), Some(dst));
    assert_eq!(inst.used_values(), vec![lhs, rhs]);
    assert!(inst.effects().is_pure());
}

#[test]
fn test_call_instruction() {
    let dst = ValueId::new(0);
    let func = ValueId::new(1);
    let arg1 = ValueId::new(2);
    let arg2 = ValueId::new(3);

    let inst = MirInstruction::Call {
        dst: Some(dst),
        func,
        callee: None, // Legacy mode for test
        args: vec![arg1, arg2],
        effects: EffectMask::IO,
    };

    assert_eq!(inst.dst_value(), Some(dst));
    assert_eq!(inst.used_values(), vec![func, arg1, arg2]);
    assert_eq!(inst.effects(), EffectMask::IO);
}

/*
#[test]
fn test_const_value_conversion() {
    let const_val = ConstValue::Integer(42);
    let nyash_val = const_val.to_nyash_value();

    assert_eq!(nyash_val, NyashValue::new_integer(42));

    let back = ConstValue::from_nyash_value(&nyash_val).unwrap();
    assert_eq!(back, const_val);
}
*/

#[test]
fn test_ref_new_instruction() {
    let dst = ValueId::new(0);
    let box_val = ValueId::new(1);
    let inst = MirInstruction::RefNew { dst, box_val };

    assert_eq!(inst.dst_value(), Some(dst));
    assert_eq!(inst.used_values(), vec![box_val]);
    assert!(inst.effects().is_pure());
}

#[test]
fn test_field_get_instruction() {
    let dst = ValueId::new(0);
    let base = ValueId::new(1);
    let inst = MirInstruction::FieldGet {
        dst,
        base,
        field: "x".to_string(),
        declared_type: Some(crate::mir::MirType::Box("IntegerBox".to_string())),
    };

    assert_eq!(inst.dst_value(), Some(dst));
    assert_eq!(inst.used_values(), vec![base]);
    assert!(!inst.effects().is_pure());
}

#[test]
fn test_field_set_instruction() {
    let base = ValueId::new(0);
    let value = ValueId::new(1);
    let inst = MirInstruction::FieldSet {
        base,
        field: "x".to_string(),
        value,
        declared_type: Some(crate::mir::MirType::Box("IntegerBox".to_string())),
    };

    assert_eq!(inst.dst_value(), None);
    assert_eq!(inst.used_values(), vec![base, value]);
    assert!(!inst.effects().is_pure());
}

#[test]
fn test_method_call_instruction() {
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::definitions::Callee;

    let dst = ValueId::new(0);
    let receiver = ValueId::new(1);
    let arg = ValueId::new(2);
    let inst = crate::mir::ssot::method_call::runtime_method_call(
        Some(dst),
        receiver,
        "ArrayBox".to_string(),
        "push".to_string(),
        vec![arg],
        EffectMask::MUT,
        TypeCertainty::Known,
    );

    assert!(matches!(
        &inst,
        MirInstruction::Call {
            func,
            callee: Some(Callee::Method {
                box_name,
                method,
                receiver: Some(recv),
                certainty,
                box_kind,
            }),
            ..
        } if *func == ValueId::INVALID
            && box_name == "ArrayBox"
            && method == "push"
            && *recv == receiver
            && *certainty == TypeCertainty::Known
            && *box_kind == CalleeBoxKind::RuntimeData
    ));

    assert_eq!(inst.dst_value(), Some(dst));
    assert_eq!(inst.used_values(), vec![receiver, arg]);
    assert_eq!(inst.effects(), EffectMask::MUT);
}

#[test]
fn test_method_call_getfield_instruction() {
    use crate::mir::definitions::call_unified::TypeCertainty;

    let dst = ValueId::new(0);
    let reference = ValueId::new(1);
    let field_name = ValueId::new(2);
    let inst = crate::mir::ssot::method_call::runtime_method_call(
        Some(dst),
        reference,
        "InstanceBox",
        "getField",
        vec![field_name],
        EffectMask::READ,
        TypeCertainty::Known,
    );

    assert_eq!(inst.dst_value(), Some(dst));
    assert_eq!(inst.used_values(), vec![reference, field_name]);
    assert!(!inst.effects().is_pure());
    assert!(inst
        .effects()
        .contains(super::super::effect::Effect::ReadHeap));
}

#[test]
fn test_method_call_setfield_instruction() {
    use crate::mir::definitions::call_unified::TypeCertainty;

    let reference = ValueId::new(0);
    let field_name = ValueId::new(1);
    let value = ValueId::new(2);
    let inst = crate::mir::ssot::method_call::runtime_method_call(
        None,
        reference,
        "InstanceBox",
        "setField",
        vec![field_name, value],
        EffectMask::WRITE,
        TypeCertainty::Known,
    );

    assert_eq!(inst.dst_value(), None);
    assert_eq!(inst.used_values(), vec![reference, field_name, value]);
    assert!(!inst.effects().is_pure());
    assert!(inst
        .effects()
        .contains(super::super::effect::Effect::WriteHeap));
}

#[test]
fn test_weakref_new_instruction() {
    let dst = ValueId::new(0);
    let box_val = ValueId::new(1);
    let inst = MirInstruction::WeakRef {
        dst,
        op: crate::mir::WeakRefOp::New,
        value: box_val,
    };

    assert_eq!(inst.dst_value(), Some(dst));
    assert_eq!(inst.used_values(), vec![box_val]);
    assert!(inst.effects().is_pure());
}

#[test]
fn test_weakref_load_instruction() {
    let dst = ValueId::new(0);
    let weak_ref = ValueId::new(1);
    let inst = MirInstruction::WeakRef {
        dst,
        op: crate::mir::WeakRefOp::Load,
        value: weak_ref,
    };

    assert_eq!(inst.dst_value(), Some(dst));
    assert_eq!(inst.used_values(), vec![weak_ref]);
    assert!(!inst.effects().is_pure());
    assert!(inst
        .effects()
        .contains(super::super::effect::Effect::ReadHeap));
}

#[test]
fn test_barrier_instructions() {
    let ptr = ValueId::new(0);

    let read_barrier = MirInstruction::Barrier {
        op: crate::mir::BarrierOp::Read,
        ptr,
    };
    assert_eq!(read_barrier.dst_value(), None);
    assert_eq!(read_barrier.used_values(), vec![ptr]);
    assert!(read_barrier
        .effects()
        .contains(super::super::effect::Effect::Barrier));
    assert!(read_barrier
        .effects()
        .contains(super::super::effect::Effect::ReadHeap));

    let write_barrier = MirInstruction::Barrier {
        op: crate::mir::BarrierOp::Write,
        ptr,
    };
    assert_eq!(write_barrier.dst_value(), None);
    assert_eq!(write_barrier.used_values(), vec![ptr]);
    assert!(write_barrier
        .effects()
        .contains(super::super::effect::Effect::Barrier));
    assert!(write_barrier
        .effects()
        .contains(super::super::effect::Effect::WriteHeap));
}

#[test]
fn test_extern_call_instruction() {
    use crate::mir::definitions::Callee;

    let dst = ValueId::new(0);
    let arg1 = ValueId::new(1);
    let arg2 = ValueId::new(2);
    let inst = crate::mir::ssot::extern_call::extern_call(
        Some(dst),
        "env.console".to_string(),
        "log".to_string(),
        vec![arg1, arg2],
        super::super::effect::EffectMask::IO,
    );

    assert!(matches!(
        &inst,
        MirInstruction::Call {
            func,
            callee: Some(Callee::Extern(name)),
            ..
        } if *func == ValueId::INVALID && name == "env.console.log"
    ));

    assert_eq!(inst.dst_value(), Some(dst));
    assert_eq!(inst.used_values(), vec![arg1, arg2]);
    assert_eq!(inst.effects(), super::super::effect::EffectMask::IO);

    // Test void extern call
    let void_inst = crate::mir::ssot::extern_call::extern_call(
        None,
        "env.canvas".to_string(),
        "fillRect".to_string(),
        vec![arg1],
        super::super::effect::EffectMask::IO,
    );

    assert_eq!(void_inst.dst_value(), None);
    assert_eq!(void_inst.used_values(), vec![arg1]);
}
