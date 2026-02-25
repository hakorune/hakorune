// TODO: This test uses internal VM methods that are no longer exposed.
// Need to rewrite this test using the public API (execute_module).
#[test]
#[ignore]
fn vm_compare_integerbox_boxref_lt() {
    use crate::backend::vm::VMValue;
    use crate::backend::VM;
    use crate::box_trait::IntegerBox;
    use std::sync::Arc;

    let _vm = VM::new();
    let _left = VMValue::BoxRef(Arc::new(IntegerBox::new(0)));
    let _right = VMValue::BoxRef(Arc::new(IntegerBox::new(3)));
    // FIXME: execute_compare_op is no longer a public method
    // let out = vm
    //     .execute_compare_op(&crate::mir::CompareOp::Lt, &left, &right)
    //     .unwrap();
    // assert!(out, "0 < 3 should be true");
}
