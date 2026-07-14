use super::*;
use crate::box_trait::IntegerBox;
use std::sync::Arc;

fn box_value(value: i64) -> VMValue {
    VMValue::BoxRef(Arc::new(IntegerBox::new(value)))
}

#[test]
fn copy_owned_creates_an_independent_exact_register_owner() {
    let mut interpreter = MirInterpreter::new();
    let src = ValueId::new(1);
    let dst = ValueId::new(2);
    interpreter.write_reg(src, box_value(7));

    interpreter
        .execute_instruction(&MirInstruction::CopyOwned { dst, src })
        .expect("BoxRef owner copy");
    let src_box = match interpreter.reg_peek_raw(src).unwrap() {
        VMValue::BoxRef(value) => value,
        _ => unreachable!(),
    };
    let dst_box = match interpreter.reg_peek_raw(dst).unwrap() {
        VMValue::BoxRef(value) => value,
        _ => unreachable!(),
    };
    assert!(Arc::ptr_eq(src_box, dst_box));

    interpreter
        .execute_instruction(&MirInstruction::DestroyOwned { value: dst })
        .expect("destroy exact copied owner");
    assert!(interpreter.reg_peek_raw(dst).is_none());
    assert!(matches!(
        interpreter.reg_peek_raw(src),
        Some(VMValue::BoxRef(_))
    ));
}

#[test]
fn destroy_owned_removes_only_the_named_register() {
    let mut interpreter = MirInterpreter::new();
    let first = ValueId::new(1);
    let second = ValueId::new(2);
    let VMValue::BoxRef(shared) = box_value(9) else {
        unreachable!()
    };
    interpreter.write_reg(first, VMValue::BoxRef(shared.clone()));
    interpreter.write_reg(second, VMValue::BoxRef(shared));

    interpreter
        .execute_instruction(&MirInstruction::DestroyOwned { value: first })
        .expect("destroy first owner");
    assert!(interpreter.reg_peek_raw(first).is_none());
    assert!(matches!(
        interpreter.reg_peek_raw(second),
        Some(VMValue::BoxRef(_))
    ));
}

#[test]
fn copy_owned_rejects_non_boxref_without_writing_destination() {
    let mut interpreter = MirInterpreter::new();
    let src = ValueId::new(1);
    let dst = ValueId::new(2);
    interpreter.write_reg(src, VMValue::Integer(7));

    let error = interpreter
        .execute_instruction(&MirInstruction::CopyOwned { dst, src })
        .expect_err("non-BoxRef must reject");
    assert!(matches!(error, VMError::TypeError(_)));
    assert!(interpreter.reg_peek_raw(dst).is_none());
}

#[test]
fn copy_owned_rejects_an_already_defined_destination() {
    let mut interpreter = MirInterpreter::new();
    let src = ValueId::new(1);
    let dst = ValueId::new(2);
    interpreter.write_reg(src, box_value(7));
    interpreter.write_reg(dst, VMValue::Integer(99));

    let error = interpreter
        .execute_instruction(&MirInstruction::CopyOwned { dst, src })
        .expect_err("defined destination must reject");
    assert!(matches!(error, VMError::InvalidInstruction(_)));
    assert!(matches!(
        interpreter.reg_peek_raw(dst),
        Some(VMValue::Integer(99))
    ));
}

#[test]
fn destroy_owned_rejects_non_boxref_without_consuming_it() {
    let mut interpreter = MirInterpreter::new();
    let value = ValueId::new(1);
    interpreter.write_reg(value, VMValue::Integer(7));

    let error = interpreter
        .execute_instruction(&MirInstruction::DestroyOwned { value })
        .expect_err("non-BoxRef destroy must reject");
    assert!(matches!(error, VMError::TypeError(_)));
    assert!(matches!(
        interpreter.reg_peek_raw(value),
        Some(VMValue::Integer(7))
    ));
}
