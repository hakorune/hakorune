//! P0 proof cases for the disconnected, strict PHI-edge verifier.

use super::edge_verifier::{verify_phi_edges_v1, PhiEdgeVerificationErrorV1};
use super::test_support::test_signature;
use crate::mir::{BasicBlock, BasicBlockId, ConstValue, MirFunction, MirInstruction, ValueId};

fn function(name: &str) -> MirFunction {
    MirFunction::new(test_signature(name), BasicBlockId::new(0))
}

fn add_branch(function: &mut MirFunction, condition: ValueId) {
    function
        .get_block_mut(BasicBlockId::new(0))
        .unwrap()
        .set_terminator(MirInstruction::Branch {
            condition,
            then_bb: BasicBlockId::new(1),
            else_bb: BasicBlockId::new(2),
            then_edge_args: None,
            else_edge_args: None,
        });
}

fn branch_merge(name: &str) -> (MirFunction, ValueId, ValueId) {
    let mut function = function(name);
    for block in [1, 2, 3] {
        function.add_block(BasicBlock::new(BasicBlockId::new(block)));
    }
    let condition = function.next_value_id();
    let seed = function.next_value_id();
    function
        .get_block_mut(BasicBlockId::new(0))
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: condition,
            value: ConstValue::Bool(true),
        });
    function
        .get_block_mut(BasicBlockId::new(0))
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: seed,
            value: ConstValue::Integer(1),
        });
    add_branch(&mut function, condition);
    for block in [1, 2] {
        function
            .get_block_mut(BasicBlockId::new(block))
            .unwrap()
            .set_terminator(MirInstruction::Jump {
                target: BasicBlockId::new(3),
                edge_args: None,
            });
    }
    (function, seed, condition)
}

fn errors(function: &MirFunction) -> Vec<PhiEdgeVerificationErrorV1> {
    verify_phi_edges_v1(function).expect_err("fixture must be rejected")
}

#[test]
fn strict_verifier_rejects_missing_phantom_and_undefined_phi_rows() {
    let (mut missing, seed, _) = branch_merge("phi-edge-missing");
    let phi = missing.next_value_id();
    missing
        .get_block_mut(BasicBlockId::new(3))
        .unwrap()
        .add_instruction(MirInstruction::Phi {
            dst: phi,
            inputs: vec![(BasicBlockId::new(1), seed)],
            type_hint: None,
        });
    assert!(
        errors(&missing).contains(&PhiEdgeVerificationErrorV1::MissingPredecessor {
            block: BasicBlockId::new(3),
            phi_ordinal: 0,
            predecessor: BasicBlockId::new(2),
        })
    );

    let (mut phantom, seed, _) = branch_merge("phi-edge-phantom");
    let phi = phantom.next_value_id();
    phantom
        .get_block_mut(BasicBlockId::new(3))
        .unwrap()
        .add_instruction(MirInstruction::Phi {
            dst: phi,
            inputs: vec![
                (BasicBlockId::new(1), seed),
                (BasicBlockId::new(2), seed),
                (BasicBlockId::new(0), seed),
            ],
            type_hint: None,
        });
    assert!(
        errors(&phantom).contains(&PhiEdgeVerificationErrorV1::PhantomPredecessor {
            block: BasicBlockId::new(3),
            phi_ordinal: 0,
            predecessor: BasicBlockId::new(0),
        })
    );

    let (mut undefined, seed, _) = branch_merge("phi-edge-undefined");
    let phi = undefined.next_value_id();
    undefined
        .get_block_mut(BasicBlockId::new(3))
        .unwrap()
        .add_instruction(MirInstruction::Phi {
            dst: phi,
            inputs: vec![
                (BasicBlockId::new(1), seed),
                (BasicBlockId::new(2), ValueId::new(99)),
            ],
            type_hint: None,
        });
    assert!(
        errors(&undefined).contains(&PhiEdgeVerificationErrorV1::UndefinedIncoming {
            block: BasicBlockId::new(3),
            phi_ordinal: 0,
            predecessor: BasicBlockId::new(2),
            value: ValueId::new(99),
        })
    );
}

#[test]
fn strict_verifier_rejects_nondominating_rows_and_cache_drift() {
    let (mut nondominating, seed, _) = branch_merge("phi-edge-nondominating");
    let foreign = nondominating.next_value_id();
    nondominating
        .get_block_mut(BasicBlockId::new(1))
        .unwrap()
        .add_instruction(MirInstruction::Copy {
            dst: foreign,
            src: seed,
        });
    let phi = nondominating.next_value_id();
    nondominating
        .get_block_mut(BasicBlockId::new(3))
        .unwrap()
        .add_instruction(MirInstruction::Phi {
            dst: phi,
            inputs: vec![
                (BasicBlockId::new(1), foreign),
                (BasicBlockId::new(2), foreign),
            ],
            type_hint: None,
        });
    assert!(
        errors(&nondominating).contains(&PhiEdgeVerificationErrorV1::NonDominatingIncoming {
            block: BasicBlockId::new(3),
            phi_ordinal: 0,
            predecessor: BasicBlockId::new(2),
            value: foreign,
            definition: BasicBlockId::new(1),
        })
    );

    let (mut stale, seed, _) = branch_merge("phi-edge-stale-cache");
    let phi = stale.next_value_id();
    stale
        .get_block_mut(BasicBlockId::new(3))
        .unwrap()
        .add_instruction(MirInstruction::Phi {
            dst: phi,
            inputs: vec![(BasicBlockId::new(1), seed), (BasicBlockId::new(2), seed)],
            type_hint: None,
        });
    stale
        .get_block_mut(BasicBlockId::new(0))
        .unwrap()
        .successors
        .clear();
    assert!(
        errors(&stale).contains(&PhiEdgeVerificationErrorV1::SuccessorCacheMismatch {
            block: BasicBlockId::new(0),
        })
    );
}

#[test]
fn strict_verifier_ignores_unreachable_phi_rows_but_keeps_reachable_order_stable() {
    let mut unreachable = function("phi-edge-unreachable");
    unreachable.add_block(BasicBlock::new(BasicBlockId::new(1)));
    let phi = unreachable.next_value_id();
    unreachable
        .get_block_mut(BasicBlockId::new(1))
        .unwrap()
        .add_instruction(MirInstruction::Phi {
            dst: phi,
            inputs: vec![(BasicBlockId::new(99), ValueId::new(100))],
            type_hint: None,
        });
    assert_eq!(verify_phi_edges_v1(&unreachable), Ok(()));

    let (mut ordered, seed, _) = branch_merge("phi-edge-diagnostic-order");
    for block in [3, 2, 1] {
        ordered
            .get_block_mut(BasicBlockId::new(block))
            .unwrap()
            .predecessors
            .clear();
    }
    let first_phi = ordered.next_value_id();
    let second_phi = ordered.next_value_id();
    let merge = ordered.get_block_mut(BasicBlockId::new(3)).unwrap();
    merge.add_instruction(MirInstruction::Phi {
        dst: first_phi,
        inputs: vec![(BasicBlockId::new(2), ValueId::new(700))],
        type_hint: None,
    });
    merge.add_instruction(MirInstruction::Phi {
        dst: second_phi,
        inputs: vec![(BasicBlockId::new(1), seed)],
        type_hint: None,
    });
    let observed = errors(&ordered);
    assert_eq!(
        observed,
        vec![
            PhiEdgeVerificationErrorV1::MissingPredecessor {
                block: BasicBlockId::new(3),
                phi_ordinal: 0,
                predecessor: BasicBlockId::new(1),
            },
            PhiEdgeVerificationErrorV1::MissingPredecessor {
                block: BasicBlockId::new(3),
                phi_ordinal: 1,
                predecessor: BasicBlockId::new(2),
            },
            PhiEdgeVerificationErrorV1::UndefinedIncoming {
                block: BasicBlockId::new(3),
                phi_ordinal: 0,
                predecessor: BasicBlockId::new(2),
                value: ValueId::new(700),
            },
        ]
    );
}
