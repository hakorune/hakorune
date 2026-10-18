use super::phi_lifecycle::{PhiTxn, PhiTxnAbortErrorV1};
use crate::mir::builder::MirBuilder;
use crate::mir::{
    BasicBlock, BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirInstruction, MirType,
    ValueId,
};

fn block(id: u32) -> BasicBlockId {
    BasicBlockId::new(id)
}

fn builder_with_blocks() -> MirBuilder {
    let mut builder = MirBuilder::new();
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "phi_txn_test/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        block(0),
    );
    function.add_block(BasicBlock::new(block(1)));
    function.add_block(BasicBlock::new(block(2)));
    builder.function_state.current_function = Some(function);
    builder
}

fn define_two(builder: &mut MirBuilder) -> PhiTxn {
    let mut txn = PhiTxn::begin("phi_txn_test");
    txn.define_provisional_phi(builder, block(1), ValueId::new(10), "first")
        .unwrap();
    txn.define_provisional_phi(builder, block(2), ValueId::new(11), "second")
        .unwrap();
    txn
}

fn has_phi(builder: &MirBuilder, block: BasicBlockId, dst: ValueId) -> bool {
    builder.function_state.current_function
        .as_ref()
        .and_then(|function| function.get_block(block))
        .is_some_and(|block| {
            block.instructions.iter().any(
                |instruction| matches!(instruction, MirInstruction::Phi { dst: phi, .. } if *phi == dst),
            )
        })
}

fn abort(txn: PhiTxn, builder: &mut MirBuilder) -> PhiTxnAbortErrorV1 {
    txn.abort_on_err(builder, "primary failure".to_string())
}

#[test]
fn abort_rolls_back_every_pending_phi_when_cleanup_succeeds() {
    let mut builder = builder_with_blocks();
    let txn = define_two(&mut builder);

    let error = abort(txn, &mut builder);

    assert_eq!(error.primary(), "primary failure");
    assert_eq!(error.pending_count(), 2);
    assert!(error.cleanup_failures().is_empty());
    assert!(!has_phi(&builder, block(1), ValueId::new(10)));
    assert!(!has_phi(&builder, block(2), ValueId::new(11)));
}

#[test]
fn rollback_continues_after_one_pending_block_was_removed() {
    let mut builder = builder_with_blocks();
    let txn = define_two(&mut builder);
    builder
        .function_state
        .current_function
        .as_mut()
        .unwrap()
        .blocks
        .remove(&block(1));

    let error = abort(txn, &mut builder);

    assert_eq!(error.primary(), "primary failure");
    assert_eq!(error.cleanup_failures().len(), 1);
    assert_eq!(error.cleanup_failures()[0].block(), block(1));
    assert_eq!(error.cleanup_failures()[0].dst(), ValueId::new(10));
    assert!(error.cleanup_failures()[0]
        .error()
        .contains("rollback_missing_block"));
    assert!(!has_phi(&builder, block(2), ValueId::new(11)));
}

#[test]
fn abort_retains_every_cleanup_failure_with_the_primary_error() {
    let mut builder = builder_with_blocks();
    let txn = define_two(&mut builder);
    let discarded_draft = builder.function_state.current_function.take().unwrap();

    let error = abort(txn, &mut builder);

    assert_eq!(error.primary(), "primary failure");
    assert_eq!(error.pending_count(), 2);
    assert_eq!(error.cleanup_failures().len(), 2);
    assert!(error
        .cleanup_failures()
        .iter()
        .all(|failure| failure.error().contains("rollback_no_function")));
    assert!(error.to_string().contains("cleanup_failure_count=2"));
    assert!(builder.function_state.current_function.is_none());
    drop(discarded_draft);
}

#[test]
fn a_fully_patched_transaction_commits_once_without_rollback() {
    let mut builder = builder_with_blocks();
    let mut txn = PhiTxn::begin("phi_txn_commit_test");
    let token = txn
        .define_provisional_phi(&mut builder, block(1), ValueId::new(10), "define")
        .unwrap();
    let mut foreign = PhiTxn::begin("foreign");
    foreign.define_provisional_phi(&mut builder, block(2), ValueId::new(11), "foreign-define").unwrap();
    let function = builder.function_state.current_function.as_ref().unwrap();
    let relation = builder.function_state.local_ssa_correspondence.loan(function, ValueId::new(10), ValueId::new(10)).unwrap();
    let aborted_relation = builder.function_state.local_ssa_correspondence.loan(function, ValueId::new(11), ValueId::new(11)).unwrap();
    assert!(matches!(&relation.identity_phi_binding_v1().unwrap().1, MirInstruction::Phi { dst, inputs, .. }
        if *dst == ValueId::new(10) && inputs.is_empty()));
    assert!(relation.require_completed_v1().is_err(), "a genuine pending definition is not completed");
    let before = builder.function_state.current_function.as_ref().unwrap().blocks[&block(1)].instructions.clone();
    assert!(foreign.patch_phi_inputs(&mut builder, token, vec![(block(0), ValueId::new(99))], "foreign-patch")
        .unwrap_err().contains("patch_not_pending"));
    assert_eq!(builder.function_state.current_function.as_ref().unwrap().blocks[&block(1)].instructions, before,
        "foreign transaction cannot mutate another pending Phi");
    txn.patch_phi_inputs(
        &mut builder,
        token,
        vec![(block(0), ValueId::new(1))],
        "patch",
    )
    .unwrap();
    let completed = builder.function_state.current_function.as_ref().unwrap().blocks[&block(1)].instructions.clone();
    assert!(txn.patch_phi_inputs(&mut builder, token, vec![(block(0), ValueId::new(99))], "repeat-patch")
        .unwrap_err().contains("patch_not_pending"));
    assert_eq!(builder.function_state.current_function.as_ref().unwrap().blocks[&block(1)].instructions, completed,
        "completed token cannot rewrite a finalized Phi");
    assert!(foreign.abort_on_err(&mut builder, "test cleanup".into()).cleanup_failures().is_empty());
    assert!(!has_phi(&builder, block(2), ValueId::new(11)));
    txn.commit(&mut builder).unwrap();
    relation.require_completed_v1().unwrap();
    relation.verify(builder.function_state.current_function.as_ref().unwrap()).unwrap();
    assert!(matches!(&relation.identity_phi_binding_v1().unwrap().1, MirInstruction::Phi { dst, inputs, .. }
        if *dst == ValueId::new(10) && inputs == &vec![(block(0), ValueId::new(1))]));
    assert!(aborted_relation.identity_phi_binding_v1().is_err(), "rolled-back observations stay unusable");
    assert!(has_phi(&builder, block(1), ValueId::new(10)));
    // Exact closure observes this same committed definition and real CFG edge.
    let function = builder.function_state.current_function.as_mut().unwrap();
    function.blocks.get_mut(&block(0)).unwrap().set_terminator(MirInstruction::Jump { target: block(1), edge_args: None });
    let rows = vec![(block(1), ValueId::new(10), vec![(block(0), ValueId::new(1))])];
    use crate::mir::builder::control_flow::plan::lowerer::capture_exact_loop_phis_v2 as capture;
    let observed = capture(function, &rows).unwrap();
    assert_eq!(observed[&block(1)].as_ref(), function.blocks[&block(1)].instructions.as_slice());
    for mutation in 0..8 {
        let mut changed = function.clone();
        let mut expected = rows.clone();
        let reason = match mutation {
            0 => {
                let MirInstruction::Phi { inputs, .. } = &mut changed.blocks.get_mut(&block(1)).unwrap().instructions[0] else { panic!("Phi"); };
                inputs[0].1 = ValueId::new(99);
                "plan-phi-drift"
            }
            1 => { changed.blocks.get_mut(&block(0)).unwrap().terminator = None; "predecessor-coverage" }
            2 => { changed.blocks.get_mut(&block(1)).unwrap().instructions.push(MirInstruction::Phi {
                dst: ValueId::new(12), inputs: rows[0].2.clone(), type_hint: None }); "phi-prefix-drift" }
            3 => { let duplicate = changed.blocks[&block(1)].instructions[0].clone();
                changed.blocks.get_mut(&block(2)).unwrap().instructions.push(duplicate); "phi-definition-cardinality" }
            4 => { changed.params.push(ValueId::new(10)); "phi-definition-cardinality" }
            5 => { expected[0].0 = block(2); "phi-definition-cardinality" }
            6 => { expected[0].2.clear(); "plan-input-missing" }
            7 => {
                let duplicate = expected[0].2[0];
                expected[0].2.push(duplicate);
                let MirInstruction::Phi { inputs, .. } = &mut changed.blocks.get_mut(&block(1)).unwrap().instructions[0] else { panic!("Phi"); };
                *inputs = expected[0].2.clone();
                "duplicate-predecessor"
            }
            _ => unreachable!(),
        };
        let error = capture(&changed, &expected).unwrap_err();
        assert!(error.contains(reason), "mutation {mutation}: {error}");
    }
}

#[test]
fn commit_with_pending_phis_rolls_them_all_back() {
    let mut builder = builder_with_blocks();
    let txn = define_two(&mut builder);

    let error = txn.commit(&mut builder).unwrap_err();

    assert!(error.primary().contains("provisional_left_unpatched"));
    assert_eq!(error.pending_count(), 2);
    assert!(error.cleanup_failures().is_empty());
    assert!(!has_phi(&builder, block(1), ValueId::new(10)));
    assert!(!has_phi(&builder, block(2), ValueId::new(11)));
}

#[test]
fn missing_provisional_phi_is_retained_as_a_cleanup_failure() {
    let mut builder = builder_with_blocks();
    let mut txn = PhiTxn::begin("phi_txn_missing_test");
    txn.define_provisional_phi(&mut builder, block(1), ValueId::new(10), "define")
        .unwrap();
    let target = builder
        .function_state
        .current_function
        .as_mut()
        .unwrap()
        .get_block_mut(block(1))
        .unwrap();
    target.instructions.clear();
    target.instruction_spans.clear();

    let error = abort(txn, &mut builder);

    assert_eq!(error.cleanup_failures().len(), 1);
    assert!(error.cleanup_failures()[0]
        .error()
        .contains("provisional PHI was not found"));
}
