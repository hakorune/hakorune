use super::legacy_candidate::{prepare_legacy_phi_repair_candidate_v1, PhiRepairPreflightErrorV1};
use super::test_support::test_signature;
use crate::mir::{
    BasicBlock, BasicBlockId, BinaryOp, Callee, CompareOp, ConstValue, EffectMask, MirFunction,
    MirInstruction, UnaryOp, ValueId,
};
use hakorune_mir_defs::{CalleeBoxKind, TypeCertainty};

#[derive(Debug, Eq, PartialEq)]
enum FixtureArtifactClosureError {
    SpanAlignment,
    PositionalReferenceWithoutRewrite { instruction_index: usize },
    ValueReferenceWithoutRewrite { value: ValueId },
}

struct FixtureArtifactClosure {
    positional_references: Vec<usize>,
    value_references: Vec<ValueId>,
}

fn remove_unused_phi_with_fixture_closure(
    function: &MirFunction,
    block_id: BasicBlockId,
    instruction_index: usize,
    destination: ValueId,
    closure: &FixtureArtifactClosure,
) -> Result<MirFunction, FixtureArtifactClosureError> {
    let mut candidate = function.clone();
    let block = candidate.blocks.get_mut(&block_id).unwrap();
    if block.instructions.len() != block.instruction_spans.len() {
        return Err(FixtureArtifactClosureError::SpanAlignment);
    }
    if let Some(index) = closure
        .positional_references
        .iter()
        .copied()
        .find(|index| *index >= instruction_index)
    {
        return Err(
            FixtureArtifactClosureError::PositionalReferenceWithoutRewrite {
                instruction_index: index,
            },
        );
    }
    if closure.value_references.contains(&destination) {
        return Err(FixtureArtifactClosureError::ValueReferenceWithoutRewrite {
            value: destination,
        });
    }
    block.instructions.remove(instruction_index);
    block.instruction_spans.remove(instruction_index);
    Ok(candidate)
}

fn function(name: &str) -> MirFunction {
    MirFunction::new(test_signature(name), BasicBlockId::new(0))
}

fn snapshot(function: &MirFunction) -> String {
    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|block| block.0);
    let blocks = block_ids
        .into_iter()
        .map(|block_id| {
            let block = &function.blocks[&block_id];
            format!(
                "{block_id:?}:{:?}:{:?}:{:?}:{:?}:{:?}:{:?}",
                block.instructions,
                block.instruction_spans,
                block.terminator,
                block.predecessors,
                block.successors,
                block.reachable,
            )
        })
        .collect::<Vec<_>>();
    format!(
        "{:?}|{:?}|{}|{:?}|{}",
        function.signature,
        function.params,
        function.next_value_id,
        function.metadata,
        blocks.join("|")
    )
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

fn foreign_const_phi(reverse_block_insertion: bool) -> MirFunction {
    let mut function = function("candidate-foreign-const");
    let insertion = if reverse_block_insertion {
        [3, 2, 1]
    } else {
        [1, 2, 3]
    };
    for block in insertion {
        function.add_block(BasicBlock::new(BasicBlockId::new(block)));
    }
    let condition = function.next_value_id();
    let foreign = function.next_value_id();
    let phi = function.next_value_id();
    function
        .get_block_mut(BasicBlockId::new(0))
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: condition,
            value: ConstValue::Bool(true),
        });
    add_branch(&mut function, condition);
    function
        .get_block_mut(BasicBlockId::new(1))
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: foreign,
            value: ConstValue::Integer(7),
        });
    for block in [1, 2] {
        function
            .get_block_mut(BasicBlockId::new(block))
            .unwrap()
            .set_terminator(MirInstruction::Jump {
                target: BasicBlockId::new(3),
                edge_args: None,
            });
    }
    let phi_use = function.next_value_id();
    let merge = function.get_block_mut(BasicBlockId::new(3)).unwrap();
    merge.add_instruction(MirInstruction::Phi {
        dst: phi,
        inputs: vec![
            (BasicBlockId::new(1), foreign),
            (BasicBlockId::new(2), foreign),
        ],
        type_hint: None,
    });
    merge.add_instruction(MirInstruction::Copy {
        dst: phi_use,
        src: phi,
    });
    function
}

#[derive(Clone, Copy)]
enum RematerializationFamily {
    Copy,
    BinOp,
    Compare,
    Unary,
    Select,
    SubstringCall,
}

fn foreign_family_phi(family: RematerializationFamily) -> (MirFunction, usize) {
    let mut function = foreign_const_phi(false);
    let foreign = match &function.blocks[&BasicBlockId::new(3)].instructions[0] {
        MirInstruction::Phi { inputs, .. } => inputs[0].1,
        _ => panic!("expected Phi"),
    };
    let constant_count = match family {
        RematerializationFamily::Copy | RematerializationFamily::Unary => 1,
        RematerializationFamily::BinOp | RematerializationFamily::Compare => 2,
        RematerializationFamily::Select | RematerializationFamily::SubstringCall => 3,
    };
    let constants = (0..constant_count)
        .map(|_| function.next_value_id())
        .collect::<Vec<_>>();
    let block = function.get_block_mut(BasicBlockId::new(1)).unwrap();
    block.instructions.clear();
    block.instruction_spans.clear();
    let expected_nodes = match family {
        RematerializationFamily::Copy => {
            block.add_instruction(MirInstruction::Const {
                dst: constants[0],
                value: ConstValue::Integer(1),
            });
            block.add_instruction(MirInstruction::Copy {
                dst: foreign,
                src: constants[0],
            });
            2
        }
        RematerializationFamily::BinOp => {
            for (dst, value) in constants.iter().copied().zip([1, 2]) {
                block.add_instruction(MirInstruction::Const {
                    dst,
                    value: ConstValue::Integer(value),
                });
            }
            block.add_instruction(MirInstruction::BinOp {
                dst: foreign,
                op: BinaryOp::Add,
                lhs: constants[0],
                rhs: constants[1],
            });
            3
        }
        RematerializationFamily::Compare => {
            for (dst, value) in constants.iter().copied().zip([1, 2]) {
                block.add_instruction(MirInstruction::Const {
                    dst,
                    value: ConstValue::Integer(value),
                });
            }
            block.add_instruction(MirInstruction::Compare {
                dst: foreign,
                op: CompareOp::Lt,
                lhs: constants[0],
                rhs: constants[1],
            });
            3
        }
        RematerializationFamily::Unary => {
            block.add_instruction(MirInstruction::Const {
                dst: constants[0],
                value: ConstValue::Integer(1),
            });
            block.add_instruction(MirInstruction::UnaryOp {
                dst: foreign,
                op: UnaryOp::Neg,
                operand: constants[0],
            });
            2
        }
        RematerializationFamily::Select => {
            block.add_instruction(MirInstruction::Const {
                dst: constants[0],
                value: ConstValue::Bool(true),
            });
            for (dst, value) in constants[1..].iter().copied().zip([1, 2]) {
                block.add_instruction(MirInstruction::Const {
                    dst,
                    value: ConstValue::Integer(value),
                });
            }
            block.add_instruction(MirInstruction::Select {
                dst: foreign,
                cond: constants[0],
                then_val: constants[1],
                else_val: constants[2],
            });
            4
        }
        RematerializationFamily::SubstringCall => {
            block.add_instruction(MirInstruction::Const {
                dst: constants[0],
                value: ConstValue::String("abcdef".to_string()),
            });
            for (dst, value) in constants[1..].iter().copied().zip([1, 3]) {
                block.add_instruction(MirInstruction::Const {
                    dst,
                    value: ConstValue::Integer(value),
                });
            }
            block.add_instruction(MirInstruction::Call {
                dst: Some(foreign),
                func: ValueId::new(u32::MAX),
                callee: Some(Callee::Method {
                    box_name: "RuntimeDataBox".to_string(),
                    method: "substring".to_string(),
                    receiver: Some(constants[0]),
                    certainty: TypeCertainty::Union,
                    box_kind: CalleeBoxKind::RuntimeData,
                }),
                args: constants.clone(),
                effects: EffectMask::PURE,
            });
            4
        }
    };
    (function, expected_nodes)
}

#[test]
fn candidate_completes_self_carried_row_without_touching_live_function() {
    let mut function = function("candidate-self-carried");
    function.add_block(BasicBlock::new(BasicBlockId::new(1)));
    function.add_block(BasicBlock::new(BasicBlockId::new(2)));
    let seed = function.next_value_id();
    let phi = function.next_value_id();
    function
        .get_block_mut(BasicBlockId::new(0))
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: seed,
            value: ConstValue::Integer(0),
        });
    function
        .get_block_mut(BasicBlockId::new(0))
        .unwrap()
        .set_terminator(MirInstruction::Jump {
            target: BasicBlockId::new(1),
            edge_args: None,
        });
    function
        .get_block_mut(BasicBlockId::new(1))
        .unwrap()
        .add_instruction(MirInstruction::Phi {
            dst: phi,
            inputs: vec![(BasicBlockId::new(0), seed)],
            type_hint: None,
        });
    function
        .get_block_mut(BasicBlockId::new(1))
        .unwrap()
        .set_terminator(MirInstruction::Jump {
            target: BasicBlockId::new(2),
            edge_args: None,
        });
    let body_use = function.next_value_id();
    function
        .get_block_mut(BasicBlockId::new(2))
        .unwrap()
        .add_instruction(MirInstruction::Copy {
            dst: body_use,
            src: phi,
        });
    function
        .get_block_mut(BasicBlockId::new(2))
        .unwrap()
        .set_terminator(MirInstruction::Jump {
            target: BasicBlockId::new(1),
            edge_args: None,
        });
    let before = snapshot(&function);

    let candidate = prepare_legacy_phi_repair_candidate_v1(&function).unwrap();
    assert_eq!(candidate.schedule_counts(), (1, 0));
    let repaired = candidate.execute().unwrap();
    assert_eq!(snapshot(&function), before);
    let MirInstruction::Phi { inputs, .. } =
        &repaired.function().blocks[&BasicBlockId::new(1)].instructions[0]
    else {
        panic!("expected Phi");
    };
    assert!(inputs.contains(&(BasicBlockId::new(2), phi)));
}

#[test]
fn candidate_rematerializes_const_only_in_its_owned_clone() {
    let function = foreign_const_phi(false);
    let before = snapshot(&function);

    let candidate = prepare_legacy_phi_repair_candidate_v1(&function).unwrap();
    assert_eq!(candidate.schedule_counts(), (0, 1));
    let repaired = candidate.execute().unwrap();
    assert_eq!(snapshot(&function), before);
    assert!(matches!(
        repaired.function().blocks[&BasicBlockId::new(2)]
            .instructions
            .last(),
        Some(MirInstruction::Const {
            value: ConstValue::Integer(7),
            ..
        })
    ));
}

#[test]
fn candidate_accepts_each_bounded_rematerialization_family() {
    for family in [
        RematerializationFamily::Copy,
        RematerializationFamily::BinOp,
        RematerializationFamily::Compare,
        RematerializationFamily::Unary,
        RematerializationFamily::Select,
        RematerializationFamily::SubstringCall,
    ] {
        let (function, expected_nodes) = foreign_family_phi(family);
        let before = snapshot(&function);
        let candidate = prepare_legacy_phi_repair_candidate_v1(&function).unwrap();
        assert_eq!(candidate.schedule_counts(), (0, expected_nodes));
        let repaired = candidate.execute().unwrap();
        assert_eq!(snapshot(&function), before);
        assert_eq!(
            repaired.function().blocks[&BasicBlockId::new(2)]
                .instructions
                .len(),
            expected_nodes
        );
    }
}

#[test]
fn candidate_rejects_missing_rows_without_a_single_dominating_input() {
    let mut function = foreign_const_phi(false);
    let merge = function.get_block_mut(BasicBlockId::new(3)).unwrap();
    let MirInstruction::Phi { inputs, .. } = &mut merge.instructions[0] else {
        panic!("expected Phi");
    };
    inputs.retain(|(predecessor, _)| *predecessor == BasicBlockId::new(1));
    let before = snapshot(&function);

    assert!(matches!(
        prepare_legacy_phi_repair_candidate_v1(&function),
        Err(PhiRepairPreflightErrorV1::UnrepairableMissingPredecessor {
            block,
            phi_index: 0,
            predecessor,
        }) if block == BasicBlockId::new(3) && predecessor == BasicBlockId::new(2)
    ));
    assert_eq!(snapshot(&function), before);
}

#[test]
fn candidate_rejects_cycles_and_non_rematerializable_definitions_before_cloning() {
    let mut cyclic = foreign_const_phi(false);
    let foreign = match &cyclic.blocks[&BasicBlockId::new(3)].instructions[0] {
        MirInstruction::Phi { inputs, .. } => inputs[0].1,
        _ => panic!("expected Phi"),
    };
    let other = cyclic.next_value_id();
    let block = cyclic.get_block_mut(BasicBlockId::new(1)).unwrap();
    block.instructions.clear();
    block.instruction_spans.clear();
    block.add_instruction(MirInstruction::Copy {
        dst: foreign,
        src: other,
    });
    block.add_instruction(MirInstruction::Copy {
        dst: other,
        src: foreign,
    });
    let before_cycle = snapshot(&cyclic);
    assert!(matches!(
        prepare_legacy_phi_repair_candidate_v1(&cyclic),
        Err(PhiRepairPreflightErrorV1::RematerializationCycle { predecessor, value })
            if predecessor == BasicBlockId::new(2) && value == foreign
    ));
    assert_eq!(snapshot(&cyclic), before_cycle);

    let mut non_rematerializable = foreign_const_phi(false);
    let foreign = match &non_rematerializable.blocks[&BasicBlockId::new(3)].instructions[0] {
        MirInstruction::Phi { inputs, .. } => inputs[0].1,
        _ => panic!("expected Phi"),
    };
    non_rematerializable
        .get_block_mut(BasicBlockId::new(1))
        .unwrap()
        .instructions[0] = MirInstruction::FieldGet {
        dst: foreign,
        base: ValueId::new(0),
        field: "not-a-repair-value".to_string(),
        declared_type: None,
    };
    let before_non_rematerializable = snapshot(&non_rematerializable);
    assert!(matches!(
        prepare_legacy_phi_repair_candidate_v1(&non_rematerializable),
        Err(PhiRepairPreflightErrorV1::NonRematerializable { predecessor, value })
            if predecessor == BasicBlockId::new(2) && value == foreign
    ));
    assert_eq!(snapshot(&non_rematerializable), before_non_rematerializable);
}

#[test]
fn candidate_rejects_impure_calls_cursor_faults_and_exception_regions_preflight() {
    let (mut impure_call, _) = foreign_family_phi(RematerializationFamily::SubstringCall);
    let block = impure_call.get_block_mut(BasicBlockId::new(1)).unwrap();
    let Some(MirInstruction::Call { effects, .. }) = block.instructions.last_mut() else {
        panic!("expected substring call");
    };
    *effects = EffectMask::READ;
    let before_impure = snapshot(&impure_call);
    assert!(matches!(
        prepare_legacy_phi_repair_candidate_v1(&impure_call),
        Err(PhiRepairPreflightErrorV1::ImpureSubstringCall { predecessor, .. })
            if predecessor == BasicBlockId::new(2)
    ));
    assert_eq!(snapshot(&impure_call), before_impure);

    let mut collision = foreign_const_phi(false);
    let foreign = match &collision.blocks[&BasicBlockId::new(3)].instructions[0] {
        MirInstruction::Phi { inputs, .. } => inputs[0].1,
        _ => panic!("expected Phi"),
    };
    collision.next_value_id = foreign.0;
    let before_collision = snapshot(&collision);
    assert!(matches!(
        prepare_legacy_phi_repair_candidate_v1(&collision),
        Err(PhiRepairPreflightErrorV1::AllocatorCursorCollision { value, next_value_id })
            if value == foreign && next_value_id == foreign.0
    ));
    assert_eq!(snapshot(&collision), before_collision);

    let mut overflow = foreign_const_phi(false);
    overflow.next_value_id = u32::MAX;
    let before_overflow = snapshot(&overflow);
    assert!(matches!(
        prepare_legacy_phi_repair_candidate_v1(&overflow),
        Err(PhiRepairPreflightErrorV1::AllocatorOverflow { next_value_id, planned })
            if next_value_id == u32::MAX && planned == 1
    ));
    assert_eq!(snapshot(&overflow), before_overflow);

    let mut exception = foreign_const_phi(false);
    let exception_value = exception.next_value_id();
    exception
        .get_block_mut(BasicBlockId::new(1))
        .unwrap()
        .add_instruction(MirInstruction::Catch {
            exception_type: None,
            exception_value,
            handler_bb: BasicBlockId::new(3),
        });
    let before_exception = snapshot(&exception);
    assert!(matches!(
        prepare_legacy_phi_repair_candidate_v1(&exception),
        Err(PhiRepairPreflightErrorV1::ExceptionInstruction { block })
            if block == BasicBlockId::new(1)
    ));
    assert_eq!(snapshot(&exception), before_exception);
}

#[test]
fn late_rhs_failure_is_preflight_only_and_leaves_live_state_unchanged() {
    let mut function = foreign_const_phi(false);
    let left = function.next_value_id();
    let foreign = function.next_value_id();
    let missing = ValueId::new(777);
    let block = function.get_block_mut(BasicBlockId::new(1)).unwrap();
    block.instructions.clear();
    block.instruction_spans.clear();
    block.add_instruction(MirInstruction::Const {
        dst: left,
        value: ConstValue::Integer(1),
    });
    block.add_instruction(MirInstruction::BinOp {
        dst: foreign,
        op: BinaryOp::Add,
        lhs: left,
        rhs: missing,
    });
    let merge = function.get_block_mut(BasicBlockId::new(3)).unwrap();
    let MirInstruction::Phi { inputs, .. } = &mut merge.instructions[0] else {
        panic!("expected Phi");
    };
    for (_, incoming) in inputs {
        *incoming = foreign;
    }
    let before = snapshot(&function);

    assert!(matches!(
        prepare_legacy_phi_repair_candidate_v1(&function),
        Err(PhiRepairPreflightErrorV1::UndefinedRematerializationOperand {
            predecessor,
            value,
        }) if predecessor == BasicBlockId::new(2) && value == missing
    ));
    assert_eq!(snapshot(&function), before);
}

#[test]
fn duplicate_definitions_reject_before_candidate_creation() {
    let mut function = function("candidate-duplicate-definition");
    function
        .get_block_mut(BasicBlockId::new(0))
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: ValueId::new(0),
            value: ConstValue::Integer(1),
        });
    function
        .get_block_mut(BasicBlockId::new(0))
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: ValueId::new(0),
            value: ConstValue::Integer(2),
        });

    assert!(matches!(
        prepare_legacy_phi_repair_candidate_v1(&function),
        Err(PhiRepairPreflightErrorV1::DuplicateDefinition {
            value,
            first_block,
            second_block,
        }) if value == ValueId::new(0)
            && first_block == BasicBlockId::new(0)
            && second_block == BasicBlockId::new(0)
    ));
}

#[test]
fn unused_phi_remains_blocked_without_artifact_closure() {
    let mut function = function("candidate-unused-phi");
    let phi = function.next_value_id();
    function
        .get_block_mut(BasicBlockId::new(0))
        .unwrap()
        .add_instruction(MirInstruction::Phi {
            dst: phi,
            inputs: Vec::new(),
            type_hint: None,
        });

    assert!(matches!(
        prepare_legacy_phi_repair_candidate_v1(&function),
        Err(PhiRepairPreflightErrorV1::UnusedPhi(_))
    ));
}

#[test]
fn fixture_artifact_closure_pairs_phi_and_span_and_rejects_unrewritten_references() {
    let mut function = function("candidate-fixture-artifact-closure");
    let phi = function.next_value_id();
    let const_value = function.next_value_id();
    let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
    block.add_instruction(MirInstruction::Phi {
        dst: phi,
        inputs: Vec::new(),
        type_hint: None,
    });
    block.add_instruction(MirInstruction::Const {
        dst: const_value,
        value: ConstValue::Integer(9),
    });
    let remaining_span = block.instruction_spans[1];

    let closed = remove_unused_phi_with_fixture_closure(
        &function,
        BasicBlockId::new(0),
        0,
        phi,
        &FixtureArtifactClosure {
            positional_references: Vec::new(),
            value_references: Vec::new(),
        },
    )
    .unwrap();
    let closed_block = &closed.blocks[&BasicBlockId::new(0)];
    assert_eq!(closed_block.instructions.len(), 1);
    assert_eq!(closed_block.instruction_spans, vec![remaining_span]);

    assert!(matches!(
        remove_unused_phi_with_fixture_closure(
            &function,
            BasicBlockId::new(0),
            0,
            phi,
            &FixtureArtifactClosure {
                positional_references: vec![1],
                value_references: Vec::new(),
            },
        ),
        Err(
            FixtureArtifactClosureError::PositionalReferenceWithoutRewrite {
                instruction_index: 1,
            }
        )
    ));
    assert!(matches!(
        remove_unused_phi_with_fixture_closure(
            &function,
            BasicBlockId::new(0),
            0,
            phi,
            &FixtureArtifactClosure {
                positional_references: Vec::new(),
                value_references: vec![phi],
            },
        ),
        Err(FixtureArtifactClosureError::ValueReferenceWithoutRewrite { value }) if value == phi
    ));
}

#[test]
fn candidate_rebuilds_its_cache_and_is_deterministic_across_block_insertion_order() {
    let mut left = foreign_const_phi(false);
    let right = foreign_const_phi(true);
    left.get_block_mut(BasicBlockId::new(0))
        .unwrap()
        .successors
        .clear();
    let left_before = snapshot(&left);
    let right_before = snapshot(&right);

    let left_candidate = prepare_legacy_phi_repair_candidate_v1(&left).unwrap();
    let right_candidate = prepare_legacy_phi_repair_candidate_v1(&right).unwrap();
    let left_repaired = left_candidate.execute().unwrap();
    let right_repaired = right_candidate.execute().unwrap();
    assert_eq!(
        snapshot(left_repaired.function()),
        snapshot(right_repaired.function())
    );
    assert_eq!(snapshot(&left), left_before);
    assert_eq!(snapshot(&right), right_before);
}
