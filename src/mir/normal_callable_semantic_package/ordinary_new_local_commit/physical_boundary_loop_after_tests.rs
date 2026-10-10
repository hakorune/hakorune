use super::*;

#[test]
fn verified_loop_after_phi_requires_exact_block_and_predecessor() {
    let (mut function, bindings) = join_fixture(1, true, false);
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: ValueId(2),
            value: ConstValue::Integer(2),
        });
    let MirInstruction::Phi { inputs, .. } = &mut function
        .blocks
        .get_mut(&BasicBlockId(3))
        .unwrap()
        .instructions[0]
    else {
        panic!("fixture PHI missing")
    };
    inputs[0].1 = ValueId(1);
    let sealed_phi = function.blocks[&BasicBlockId(3)].instructions[0].clone();
    let after =
        std::collections::BTreeMap::from([(BasicBlockId(3), vec![sealed_phi].into_boxed_slice())]);
    let boundary = PhysicalBoundary::capture_with_loop_after_roles(
        &function,
        &bindings,
        &[],
        &[],
        &after,
        None,
    )
    .expect("the verified after role admits its exact leading PHI");
    assert!(validate(&boundary, &function, &bindings));

    let wrong_block =
        std::collections::BTreeMap::from([(BasicBlockId(0), after[&BasicBlockId(3)].clone())]);
    let error = PhysicalBoundary::capture_with_loop_after_roles(
        &function,
        &bindings,
        &[],
        &[],
        &wrong_block,
        None,
    )
    .unwrap_err();
    assert!(error.contains("phi-in-recorded-block"), "{error}");

    let mut wrong_input = function.clone();
    let MirInstruction::Phi { inputs, .. } = &mut wrong_input
        .blocks
        .get_mut(&BasicBlockId(3))
        .unwrap()
        .instructions[0]
    else {
        panic!("fixture PHI missing")
    };
    inputs[0].0 = BasicBlockId(1);
    let error = PhysicalBoundary::capture_with_loop_after_roles(
        &wrong_input,
        &bindings,
        &[],
        &[],
        &after,
        None,
    )
    .unwrap_err();
    assert!(error.contains("phi-in-recorded-block"), "{error}");

    let mut wrong_value = function.clone();
    let MirInstruction::Phi { inputs, .. } = &mut wrong_value
        .blocks
        .get_mut(&BasicBlockId(3))
        .unwrap()
        .instructions[0]
    else {
        panic!("fixture PHI missing")
    };
    // Value 2 is already defined in the same predecessor, so CFG and generic
    // SSA checks alone cannot distinguish it from the sealed PHI input.
    inputs[0].1 = ValueId(2);
    let error = PhysicalBoundary::capture_with_loop_after_roles(
        &wrong_value,
        &bindings,
        &[],
        &[],
        &after,
        None,
    )
    .unwrap_err();
    assert!(error.contains("loop-after-phi-prefix-drift"), "{error}");

    let mut wrong_destination = function.clone();
    let MirInstruction::Phi { dst, .. } = &mut wrong_destination
        .blocks
        .get_mut(&BasicBlockId(3))
        .unwrap()
        .instructions[0]
    else {
        panic!("fixture PHI missing")
    };
    *dst = ValueId(7);
    let error = PhysicalBoundary::capture_with_loop_after_roles(
        &wrong_destination,
        &bindings,
        &[],
        &[],
        &after,
        None,
    )
    .unwrap_err();
    assert!(error.contains("loop-after-phi-prefix-drift"), "{error}");
}
