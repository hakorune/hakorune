use nyash_rust::mir::{
    BinaryOp, ConstValue, Effect, EffectMask, MirInstruction, ValueId, WeakRefOp,
};
use nyash_rust::mir::definitions::call_unified::TypeCertainty;

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
fn test_boxcall_getfield_instruction() {
    let dst = ValueId::new(0);
    let reference = ValueId::new(1);
    let field_name = ValueId::new(2);
    let inst = nyash_rust::mir::ssot::method_call::runtime_method_call(
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
    assert!(inst.effects().contains(Effect::ReadHeap));
}

#[test]
fn test_boxcall_setfield_instruction() {
    let reference = ValueId::new(0);
    let field_name = ValueId::new(1);
    let value = ValueId::new(2);
    let inst = nyash_rust::mir::ssot::method_call::runtime_method_call(
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
    assert!(inst.effects().contains(Effect::WriteHeap));
}

#[test]
fn test_weakref_new_instruction() {
    let dst = ValueId::new(0);
    let box_val = ValueId::new(1);
    let inst = MirInstruction::WeakRef {
        dst,
        op: WeakRefOp::New,
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
        op: WeakRefOp::Load,
        value: weak_ref,
    };
    assert_eq!(inst.dst_value(), Some(dst));
    assert_eq!(inst.used_values(), vec![weak_ref]);
    assert!(!inst.effects().is_pure());
    assert!(inst.effects().contains(Effect::ReadHeap));
}

#[test]
fn test_barrier_instructions() {
    let ptr = ValueId::new(0);
    let read_barrier = MirInstruction::Barrier {
        op: nyash_rust::mir::BarrierOp::Read,
        ptr,
    };
    assert_eq!(read_barrier.dst_value(), None);
    assert_eq!(read_barrier.used_values(), vec![ptr]);
    assert!(read_barrier.effects().contains(Effect::Barrier));
    assert!(read_barrier.effects().contains(Effect::ReadHeap));

    let write_barrier = MirInstruction::Barrier {
        op: nyash_rust::mir::BarrierOp::Write,
        ptr,
    };
    assert_eq!(write_barrier.dst_value(), None);
    assert_eq!(write_barrier.used_values(), vec![ptr]);
    assert!(write_barrier.effects().contains(Effect::Barrier));
    assert!(write_barrier.effects().contains(Effect::WriteHeap));
}

#[test]
fn test_extern_call_instruction() {
    let dst = ValueId::new(0);
    let arg1 = ValueId::new(1);
    let arg2 = ValueId::new(2);
    let inst = nyash_rust::mir::ssot::extern_call::extern_call(
        Some(dst),
        "env.console",
        "log",
        vec![arg1, arg2],
        EffectMask::IO,
    );
    assert_eq!(inst.dst_value(), Some(dst));
    assert_eq!(inst.used_values(), vec![arg1, arg2]);
    assert_eq!(inst.effects(), EffectMask::IO);

    let void_inst = nyash_rust::mir::ssot::extern_call::extern_call(
        None,
        "env.canvas",
        "fillRect",
        vec![arg1],
        EffectMask::IO,
    );
    assert_eq!(void_inst.dst_value(), None);
    assert_eq!(void_inst.used_values(), vec![arg1]);
}
