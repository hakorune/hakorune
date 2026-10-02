//! Data-correspondence tests; these do not issue source transport authority.
use super::*;
use crate::mir::{BasicBlock, ConstValue, EffectMask, FunctionSignature, MirType};

fn fixture(
    looping: bool,
) -> (
    MirFunction,
    Vec<(BasicBlockId, MirInstruction)>,
    Vec<(BasicBlockId, MirInstruction)>,
) {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "borrowed-copy-boundary".into(),
            params: vec![MirType::Integer, MirType::Bool],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId(0),
    );
    function.params = vec![ValueId(50), ValueId(51)];
    let frame = MirInstruction::FaultFrameEnter {
        dst: ValueId(100),
        mode: crate::mir::instruction::FaultFrameMode::RootOwned,
    };
    let mut entry = BasicBlock::new(BasicBlockId(0));
    entry.add_instruction(frame.clone());
    entry.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(1),
        edge_args: None,
    });
    function.add_block(entry);
    let copies = vec![
        (
            BasicBlockId(1),
            MirInstruction::Copy {
                src: ValueId(50),
                dst: ValueId(60),
            },
        ),
        (
            BasicBlockId(1),
            MirInstruction::Copy {
                src: ValueId(60),
                dst: ValueId(61),
            },
        ),
    ];
    let mut body = BasicBlock::new(BasicBlockId(1));
    for (_, copy) in &copies {
        body.add_instruction(copy.clone());
    }
    body.set_terminator(if looping {
        MirInstruction::Branch {
            condition: ValueId(51),
            then_bb: BasicBlockId(1),
            else_bb: BasicBlockId(2),
            then_edge_args: None,
            else_edge_args: None,
        }
    } else {
        MirInstruction::Jump {
            target: BasicBlockId(2),
            edge_args: None,
        }
    });
    function.add_block(body);
    let mut exit = BasicBlock::new(BasicBlockId(2));
    exit.set_terminator(MirInstruction::Return { value: None });
    function.add_block(exit);
    (function, vec![(BasicBlockId(0), frame)], copies)
}

fn capture(function: &MirFunction, mandatory: &Bindings, aliases: &Bindings) -> PhysicalBoundary {
    PhysicalBoundary::capture_with_source_copies(function, mandatory, &[], aliases).unwrap()
}

fn validate(
    boundary: &PhysicalBoundary,
    function: &MirFunction,
    mandatory: &Bindings,
) -> Result<FinishedBindings, String> {
    let mut projection = boundary.project(function)?;
    boundary.validate_complete(function, &mut projection, mandatory)?;
    Ok(projection)
}

#[test]
fn borrowed_copy_boundary_keeps_preserved_full_and_partial_unused_chains() {
    let (original, mandatory, copies) = fixture(false);
    let boundary = capture(&original, &mandatory, &copies);
    let kept = validate(&boundary, &original, &mandatory).unwrap();
    assert_eq!(
        kept.borrowed_copy_coordinate(&original, &copies[0])
            .unwrap(),
        Some((BasicBlockId(1), 0))
    );
    for retained in [0, 1] {
        let mut finished = original.clone();
        finished
            .blocks
            .get_mut(&BasicBlockId(1))
            .unwrap()
            .instructions
            .truncate(retained);
        let projection = validate(&boundary, &finished, &mandatory).unwrap();
        assert_eq!(
            projection
                .borrowed_copy_coordinate(&finished, &copies[1])
                .unwrap(),
            None
        );
        assert_eq!(
            projection
                .borrowed_copy_coordinate(&finished, &copies[0])
                .unwrap()
                .is_some(),
            retained == 1
        );
    }
}

#[test]
fn borrowed_copy_boundary_checks_empty_lifecycle_and_loop_aliases_without_cycle_waiver() {
    let (original, _, copies) = fixture(true);
    let boundary = capture(&original, &[], &copies);
    assert!(boundary.nodes.is_empty());
    validate(&boundary, &original, &[]).unwrap();
    let mut omitted = original.clone();
    omitted
        .blocks
        .get_mut(&BasicBlockId(1))
        .unwrap()
        .instructions
        .clear();
    validate(&boundary, &omitted, &[]).unwrap();
    omitted
        .blocks
        .get_mut(&BasicBlockId(2))
        .unwrap()
        .set_terminator(MirInstruction::Return {
            value: Some(ValueId(60)),
        });
    assert!(validate(&boundary, &omitted, &[]).is_err());
    // Registering the same loop block as lifecycle control still rejects its
    // real cycle. Alias membership never expands that contraction DAG.
    assert!(
        PhysicalBoundary::capture_with_source_copies(&original, &copies[..1], &[], &copies)
            .unwrap_err()
            .contains("cycle")
    );
}

#[test]
fn borrowed_copy_boundary_never_omits_a_mandatory_shared_prefix() {
    let (original, mut mandatory, copies) = fixture(false);
    mandatory.push(copies[0].clone());
    let boundary = capture(&original, &mandatory, &copies);
    let mut leaf_omitted = original.clone();
    leaf_omitted
        .blocks
        .get_mut(&BasicBlockId(1))
        .unwrap()
        .instructions
        .truncate(1);
    validate(&boundary, &leaf_omitted, &mandatory).unwrap();
    leaf_omitted
        .blocks
        .get_mut(&BasicBlockId(1))
        .unwrap()
        .instructions
        .clear();
    assert!(validate(&boundary, &leaf_omitted, &mandatory).is_err());
}

#[test]
fn borrowed_copy_boundary_capture_rejects_foreign_tuple_definitions_and_parameters() {
    let (original, mandatory, copies) = fixture(false);
    let mut shared = copies.clone();
    shared.extend(copies.clone());
    assert_eq!(
        capture(&original, &mandatory, &shared)
            .borrowed_copies
            .originals
            .len(),
        2
    );
    let mut foreign = copies.clone();
    foreign[0].1 = MirInstruction::Copy {
        dst: ValueId(60),
        src: ValueId(999),
    };
    assert!(
        PhysicalBoundary::capture_with_source_copies(&original, &mandatory, &[], &foreign).is_err()
    );
    for parameter in [false, true] {
        let mut changed = original.clone();
        if parameter {
            changed.params.push(ValueId(60));
        } else {
            changed
                .blocks
                .get_mut(&BasicBlockId(2))
                .unwrap()
                .add_instruction(copies[0].1.clone());
        }
        assert!(
            PhysicalBoundary::capture_with_source_copies(&changed, &mandatory, &[], &copies)
                .is_err()
        );
    }
}

#[test]
fn borrowed_copy_boundary_rejects_relocation_changed_copy_and_redefinition() {
    let (original, mandatory, copies) = fixture(false);
    let boundary = capture(&original, &mandatory, &copies);
    for mutation in 0..4 {
        let mut changed = original.clone();
        let copy = changed
            .blocks
            .get_mut(&BasicBlockId(1))
            .unwrap()
            .instructions
            .remove(0);
        match mutation {
            0 => changed
                .blocks
                .get_mut(&BasicBlockId(2))
                .unwrap()
                .add_instruction(copy),
            1 => changed
                .blocks
                .get_mut(&BasicBlockId(1))
                .unwrap()
                .instructions
                .insert(
                    0,
                    MirInstruction::Copy {
                        dst: ValueId(60),
                        src: ValueId(999),
                    },
                ),
            2 => changed
                .blocks
                .get_mut(&BasicBlockId(2))
                .unwrap()
                .add_instruction(MirInstruction::Const {
                    dst: ValueId(60),
                    value: ConstValue::Integer(0),
                }),
            _ => changed.params.push(ValueId(60)),
        }
        assert!(
            validate(&boundary, &changed, &mandatory).is_err(),
            "mutation {mutation}"
        );
    }
    let mut duplicate = original.clone();
    duplicate
        .blocks
        .get_mut(&BasicBlockId(2))
        .unwrap()
        .add_instruction(copies[0].1.clone());
    assert!(validate(&boundary, &duplicate, &mandatory).is_err());
}

#[test]
fn borrowed_copy_boundary_rejects_phi_edge_and_unreachable_return_env_uses() {
    let (original, mandatory, copies) = fixture(false);
    let boundary = capture(&original, &mandatory, &copies);
    for mutation in 0..3 {
        let mut omitted = original.clone();
        omitted
            .blocks
            .get_mut(&BasicBlockId(1))
            .unwrap()
            .instructions
            .clear();
        let mut external = BasicBlock::new(BasicBlockId(9));
        external.set_terminator(MirInstruction::Return { value: None });
        match mutation {
            0 => external.add_instruction(MirInstruction::Phi {
                type_hint: None,
                dst: ValueId(70),
                inputs: vec![(BasicBlockId(1), ValueId(60))],
            }),
            1 => external.set_terminator(MirInstruction::Jump {
                target: BasicBlockId(2),
                edge_args: Some(crate::mir::EdgeArgs {
                    values: vec![ValueId(60)],
                    layout: crate::mir::edge_args::JumpArgsLayout::CarriersOnly,
                }),
            }),
            _ => external.return_env = Some(vec![ValueId(60)]),
        }
        omitted.add_block(external);
        assert!(
            validate(&boundary, &omitted, &mandatory).is_err(),
            "mutation {mutation}"
        );
    }
    let mut originally_used = original.clone();
    originally_used
        .blocks
        .get_mut(&BasicBlockId(2))
        .unwrap()
        .set_terminator(MirInstruction::Return {
            value: Some(ValueId(60)),
        });
    let protected = capture(&originally_used, &mandatory, &copies);
    originally_used
        .blocks
        .get_mut(&BasicBlockId(1))
        .unwrap()
        .instructions
        .clear();
    originally_used
        .blocks
        .get_mut(&BasicBlockId(2))
        .unwrap()
        .set_terminator(MirInstruction::Return { value: None });
    assert!(validate(&protected, &originally_used, &mandatory).is_err());
}

#[test]
fn borrowed_copy_boundary_uses_existing_contraction_mapping_for_live_copies() {
    let (original, mut mandatory, copies) = fixture(false);
    mandatory.push((
        BasicBlockId(0),
        original.blocks[&BasicBlockId(0)]
            .terminator
            .clone()
            .unwrap(),
    ));
    mandatory.push((
        BasicBlockId(1),
        original.blocks[&BasicBlockId(1)]
            .terminator
            .clone()
            .unwrap(),
    ));
    let boundary = capture(&original, &mandatory, &copies);
    let mut finished = original.clone();
    let removed = finished.blocks.remove(&BasicBlockId(1)).unwrap();
    let entry = finished.blocks.get_mut(&BasicBlockId(0)).unwrap();
    entry.instructions.extend(removed.instructions);
    entry.terminator = removed.terminator;
    let projection = validate(&boundary, &finished, &mandatory).unwrap();
    assert_eq!(
        projection
            .borrowed_copy_coordinate(&finished, &copies[0])
            .unwrap(),
        Some((BasicBlockId(0), 1))
    );
    assert_eq!(
        projection
            .borrowed_copy_coordinate(&finished, &copies[1])
            .unwrap(),
        Some((BasicBlockId(0), 2))
    );
}
