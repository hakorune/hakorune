use super::*;
use crate::box_trait::IntegerBox;

#[test]
fn await_errors_on_non_future_operand() {
    let mut interp = MirInterpreter::new();
    let future = ValueId::new(1);
    let dst = ValueId::new(2);

    interp.regs.insert(future, VMValue::Integer(7));

    let err = interp
        .execute_instruction(&MirInstruction::Await { dst, future })
        .expect_err("await on non-future must fail fast");

    match err {
        VMError::TypeError(msg) => {
            assert_eq!(msg, "Await expects Future in `future` operand");
        }
        other => panic!("unexpected error kind: {}", other),
    }
}

#[test]
fn await_returns_stored_value_for_resolved_future() {
    let mut interp = MirInterpreter::new();
    let future_reg = ValueId::new(1);
    let dst = ValueId::new(2);
    let fut = crate::boxes::future::FutureBox::new();
    fut.set_result(Box::new(IntegerBox::new(42)));

    interp.regs.insert(future_reg, VMValue::Future(fut));

    interp
        .execute_instruction(&MirInstruction::Await {
            dst,
            future: future_reg,
        })
        .expect("resolved future await must succeed");

    let out = interp.reg_load(dst).expect("await result register");
    assert_eq!(out, VMValue::Integer(42));
}
