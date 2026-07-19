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
    txn.patch_phi_inputs(
        &mut builder,
        token,
        vec![(block(0), ValueId::new(1))],
        "patch",
    )
    .unwrap();

    txn.commit(&mut builder).unwrap();
    assert!(has_phi(&builder, block(1), ValueId::new(10)));
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
