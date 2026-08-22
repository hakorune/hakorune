use super::{CanonicalCfgErrorV1, CanonicalCfgSessionV1, CanonicalOpenInstructionTargetErrorV1};
use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;
use crate::mir::{
    BasicBlock, BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirInstruction, MirType,
    ValueId,
};

fn block(id: u32) -> BasicBlockId {
    BasicBlockId::new(id)
}

fn function(block_count: u32) -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "canonical_cfg_test/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        block(0),
    );
    for id in 1..block_count {
        function.add_block(BasicBlock::new(block(id)));
    }
    function
}

fn owner() -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
    FunctionOwnerIssuerV1::new_for_compilation()
        .expect("owner issuer")
        .issue()
        .expect("owner")
}

fn seal_all(
    session: &mut CanonicalCfgSessionV1,
    function: &mut MirFunction,
) -> Result<(), CanonicalCfgErrorV1> {
    for id in function.block_ids() {
        session.seal_block(function, id)?;
    }
    Ok(())
}

#[test]
fn jump_updates_both_caches_and_seals_from_terminator_truth() {
    let mut function = function(2);
    let mut session = CanonicalCfgSessionV1::new();

    session
        .emit_jump(&mut function, block(0), block(1))
        .unwrap();
    let target = session.seal_block(&mut function, block(1)).unwrap();
    assert_eq!(target.block(), block(1));
    assert_eq!(target.predecessors(), &[block(0)]);
    session.seal_block(&mut function, block(0)).unwrap();

    let verified = session.finish(&function).unwrap();
    assert_eq!(verified.blocks().len(), 2);
}

#[test]
fn branch_updates_exact_then_and_else_predecessors() {
    let mut function = function(3);
    let mut session = CanonicalCfgSessionV1::new();

    session
        .emit_branch(&mut function, block(0), ValueId::new(7), block(1), block(2))
        .unwrap();
    let then_witness = session.seal_block(&mut function, block(1)).unwrap();
    let else_witness = session.seal_block(&mut function, block(2)).unwrap();
    assert_eq!(then_witness.predecessors(), &[block(0)]);
    assert_eq!(else_witness.predecessors(), &[block(0)]);
    session.seal_block(&mut function, block(0)).unwrap();
    session.finish(&function).unwrap();
}

#[test]
fn prepared_branch_has_no_effect_until_commit() {
    let mut function = function(3);
    let session = CanonicalCfgSessionV1::new();

    let prepared = session
        .prepare_branch(&function, block(0), ValueId::new(7), block(1), block(2))
        .expect("prepare branch");
    assert!(function.get_block(block(0)).unwrap().terminator.is_none());
    assert!(function
        .get_block(block(1))
        .unwrap()
        .predecessors
        .is_empty());
    assert!(function
        .get_block(block(2))
        .unwrap()
        .predecessors
        .is_empty());

    prepared.commit(&mut function);
    assert!(matches!(
        function.get_block(block(0)).unwrap().terminator,
        Some(MirInstruction::Branch {
            condition,
            then_bb,
            else_bb,
            ..
        }) if condition == ValueId::new(7) && then_bb == block(1) && else_bb == block(2)
    ));
    assert_eq!(
        function.get_block(block(1)).unwrap().predecessors,
        [block(0)].into_iter().collect()
    );
    assert_eq!(
        function.get_block(block(2)).unwrap().predecessors,
        [block(0)].into_iter().collect()
    );
}

#[test]
fn duplicate_branch_edge_is_rejected_before_mutation() {
    let mut function = function(2);
    let session = CanonicalCfgSessionV1::new();

    let error = session
        .emit_branch(&mut function, block(0), ValueId::new(1), block(1), block(1))
        .unwrap_err();
    assert_eq!(
        error,
        CanonicalCfgErrorV1::DuplicateEdge {
            source: block(0),
            target: block(1),
        }
    );
    assert!(function.get_block(block(0)).unwrap().terminator.is_none());
    assert!(function
        .get_block(block(1))
        .unwrap()
        .predecessors
        .is_empty());
}

#[test]
fn missing_source_is_a_typed_error() {
    let mut function = function(1);
    let session = CanonicalCfgSessionV1::new();

    assert!(matches!(
        session.emit_jump(&mut function, block(99), block(0)),
        Err(CanonicalCfgErrorV1::MissingBlock { block: id, .. }) if id == block(99)
    ));
    assert!(function
        .get_block(block(0))
        .unwrap()
        .predecessors
        .is_empty());
}

#[test]
fn missing_target_is_a_typed_error_before_source_mutation() {
    let mut function = function(1);
    let session = CanonicalCfgSessionV1::new();

    assert!(matches!(
        session.emit_jump(&mut function, block(0), block(99)),
        Err(CanonicalCfgErrorV1::MissingBlock { block: id, .. }) if id == block(99)
    ));
    assert!(function.get_block(block(0)).unwrap().terminator.is_none());
}

#[test]
fn second_terminator_is_rejected_without_rewriting_the_first() {
    let mut function = function(3);
    let session = CanonicalCfgSessionV1::new();
    session
        .emit_jump(&mut function, block(0), block(1))
        .unwrap();

    let error = session
        .emit_jump(&mut function, block(0), block(2))
        .unwrap_err();
    assert_eq!(
        error,
        CanonicalCfgErrorV1::SourceAlreadyTerminated { source: block(0) }
    );
    assert_eq!(
        function.get_block(block(0)).unwrap().successors,
        [block(1)].into_iter().collect()
    );
    assert!(function
        .get_block(block(2))
        .unwrap()
        .predecessors
        .is_empty());
}

#[test]
fn edge_after_seal_is_rejected_before_source_mutation() {
    let mut function = function(2);
    let mut session = CanonicalCfgSessionV1::new();
    session.seal_block(&mut function, block(1)).unwrap();

    let error = session
        .emit_jump(&mut function, block(0), block(1))
        .unwrap_err();
    assert_eq!(
        error,
        CanonicalCfgErrorV1::EdgeAfterSeal {
            source: block(0),
            target: block(1),
        }
    );
    assert!(function.get_block(block(0)).unwrap().terminator.is_none());
}

#[test]
fn sealing_twice_is_a_typed_error() {
    let mut function = function(1);
    let mut session = CanonicalCfgSessionV1::new();
    session.seal_block(&mut function, block(0)).unwrap();

    assert_eq!(
        session.seal_block(&mut function, block(0)).unwrap_err(),
        CanonicalCfgErrorV1::SealTwice { block: block(0) }
    );
}

#[test]
fn stale_successor_cache_is_not_repaired() {
    let mut function = function(2);
    function
        .get_block_mut(block(0))
        .unwrap()
        .successors
        .insert(block(1));
    let mut session = CanonicalCfgSessionV1::new();

    assert!(matches!(
        session.seal_block(&mut function, block(0)),
        Err(CanonicalCfgErrorV1::CachedSuccessorsMismatch { block: id, .. }) if id == block(0)
    ));
    assert_eq!(
        function.get_block(block(0)).unwrap().successors,
        [block(1)].into_iter().collect()
    );
}

#[test]
fn stale_predecessor_cache_is_not_repaired() {
    let mut function = function(2);
    function
        .get_block_mut(block(1))
        .unwrap()
        .predecessors
        .insert(block(0));
    let mut session = CanonicalCfgSessionV1::new();

    assert!(matches!(
        session.seal_block(&mut function, block(1)),
        Err(CanonicalCfgErrorV1::CachedPredecessorsMismatch { block: id, .. }) if id == block(1)
    ));
    assert!(function
        .get_block(block(1))
        .unwrap()
        .predecessors
        .contains(&block(0)));
}

#[test]
fn dangling_terminator_target_is_rejected() {
    let mut function = function(1);
    function
        .get_block_mut(block(0))
        .unwrap()
        .set_terminator(MirInstruction::Jump {
            target: block(99),
            edge_args: None,
        });
    let mut session = CanonicalCfgSessionV1::new();

    assert_eq!(
        session.seal_block(&mut function, block(0)).unwrap_err(),
        CanonicalCfgErrorV1::DanglingTerminatorTarget {
            source: block(0),
            target: block(99),
        }
    );
}

#[test]
fn finish_rejects_any_unsealed_function_block() {
    let function = function(2);
    let session = CanonicalCfgSessionV1::new();

    assert_eq!(
        session.finish(&function).unwrap_err(),
        CanonicalCfgErrorV1::UnsealedBlockAtFinish { block: block(0) }
    );
}

#[test]
fn finish_detects_raw_late_edge_after_a_seal_witness() {
    let mut function = function(2);
    let mut session = CanonicalCfgSessionV1::new();
    session.seal_block(&mut function, block(0)).unwrap();
    session.seal_block(&mut function, block(1)).unwrap();

    function
        .get_block_mut(block(0))
        .unwrap()
        .set_terminator(MirInstruction::Jump {
            target: block(1),
            edge_args: None,
        });
    function
        .get_block_mut(block(1))
        .unwrap()
        .add_predecessor(block(0));
    assert!(matches!(
        session.finish(&function),
        Err(CanonicalCfgErrorV1::SealedPredecessorsChanged { block: id, .. })
            if id == block(1)
    ));
}

#[test]
fn self_loop_is_valid_until_the_header_is_sealed() {
    let mut function = function(1);
    let mut session = CanonicalCfgSessionV1::new();
    session
        .emit_jump(&mut function, block(0), block(0))
        .unwrap();
    let witness = session.seal_block(&mut function, block(0)).unwrap();

    assert_eq!(witness.predecessors(), &[block(0)]);
    session.finish(&function).unwrap();
}

#[test]
fn all_blocks_can_finish_without_outgoing_edges() {
    let mut function = function(3);
    let mut session = CanonicalCfgSessionV1::new();
    seal_all(&mut session, &mut function).unwrap();
    assert_eq!(session.finish(&function).unwrap().blocks().len(), 3);
}

#[test]
fn named_block_owner_rejects_duplicate_creation() {
    let mut function = function(1);
    let mut session = CanonicalCfgSessionV1::new();
    assert_eq!(
        session.create_block(&mut function, block(0)).unwrap_err(),
        CanonicalCfgErrorV1::BlockAlreadyExists { block: block(0) }
    );
    session.create_block(&mut function, block(1)).unwrap();
    assert!(function.get_block(block(1)).is_some());
}

#[test]
fn open_instruction_target_requires_session_created_open_block() {
    let owner = owner();
    let mut function = function(1);
    let mut session = CanonicalCfgSessionV1::new_for_owner(owner);
    let target = block(1);
    session.create_block(&mut function, target).unwrap();

    let before_blocks = function.blocks.len();
    let before_instructions = function.get_block(target).unwrap().instructions.len();
    let witness = session
        .prepare_created_open_instruction_target(&function, owner, target)
        .unwrap();

    assert_eq!(witness.owner(), owner);
    assert_eq!(witness.block(), target);
    assert_eq!(function.blocks.len(), before_blocks);
    assert_eq!(
        function.get_block(target).unwrap().instructions.len(),
        before_instructions
    );
}

#[test]
fn open_instruction_target_rejects_unbound_or_foreign_session_owner() {
    let owner_a = owner();
    let foreign_owner = owner();
    let target = block(1);
    let mut function = function(1);

    let mut unbound = CanonicalCfgSessionV1::new();
    unbound.create_block(&mut function, target).unwrap();
    assert_eq!(
        unbound.prepare_created_open_instruction_target(&function, owner_a, target),
        Err(CanonicalOpenInstructionTargetErrorV1::SessionOwnerUnavailable)
    );

    let mut bound = CanonicalCfgSessionV1::new_for_owner(owner_a);
    bound.create_block(&mut function, block(2)).unwrap();
    assert_eq!(
        bound.prepare_created_open_instruction_target(&function, foreign_owner, block(2)),
        Err(CanonicalOpenInstructionTargetErrorV1::SessionOwnerMismatch)
    );
}

#[test]
fn open_instruction_target_rejects_foreign_session_and_missing_block() {
    let owner = owner();
    let target = block(1);
    let mut function = function(1);
    let mut creator = CanonicalCfgSessionV1::new_for_owner(owner);
    creator.create_block(&mut function, target).unwrap();

    let foreign_session = CanonicalCfgSessionV1::new_for_owner(owner);
    assert_eq!(
        foreign_session.prepare_created_open_instruction_target(&function, owner, target),
        Err(CanonicalOpenInstructionTargetErrorV1::SessionDidNotCreate(
            target
        ))
    );

    function.blocks.remove(&target);
    assert_eq!(
        creator.prepare_created_open_instruction_target(&function, owner, target),
        Err(CanonicalOpenInstructionTargetErrorV1::TargetBlockMissing(
            target
        ))
    );
}

#[test]
fn open_instruction_target_rejects_sealed_and_terminated_blocks() {
    let owner = owner();

    let sealed_target = block(1);
    let mut sealed_function = function(1);
    let mut sealed_session = CanonicalCfgSessionV1::new_for_owner(owner);
    sealed_session
        .create_block(&mut sealed_function, sealed_target)
        .unwrap();
    sealed_function.get_block_mut(sealed_target).unwrap().seal();
    assert_eq!(
        sealed_session.prepare_created_open_instruction_target(
            &sealed_function,
            owner,
            sealed_target
        ),
        Err(CanonicalOpenInstructionTargetErrorV1::TargetBlockSealed(
            sealed_target
        ))
    );

    let terminated_target = block(1);
    let mut terminated_function = function(1);
    let mut terminated_session = CanonicalCfgSessionV1::new_for_owner(owner);
    terminated_session
        .create_block(&mut terminated_function, terminated_target)
        .unwrap();
    terminated_function
        .get_block_mut(terminated_target)
        .unwrap()
        .set_terminator(MirInstruction::Return { value: None });
    assert_eq!(
        terminated_session.prepare_created_open_instruction_target(
            &terminated_function,
            owner,
            terminated_target
        ),
        Err(CanonicalOpenInstructionTargetErrorV1::TargetBlockTerminated(terminated_target))
    );
}

#[test]
fn named_return_owner_is_checked_and_updates_terminator_truth() {
    let mut function = function(1);
    let mut session = CanonicalCfgSessionV1::new();
    session.emit_return(&mut function, block(0), None).unwrap();
    assert!(function.get_block(block(0)).unwrap().is_terminated());
    assert_eq!(
        session
            .emit_return(&mut function, block(0), None)
            .unwrap_err(),
        CanonicalCfgErrorV1::SourceAlreadyTerminated { source: block(0) }
    );
}

#[test]
fn named_selection_never_creates_a_missing_block() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("canonical_cfg/select/0".to_owned());
    let session = CanonicalCfgSessionV1::new();
    assert!(matches!(
        session.select_block(&mut builder, block(99)),
        Err(CanonicalCfgErrorV1::MissingBlock { block: id, .. }) if id == block(99)
    ));
    assert!(builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .get_block(block(99))
        .is_none());
}
