use super::*;
use crate::mir::{BasicBlock, EffectMask, FunctionSignature, MirType, ValueId};

fn jump(target: u32) -> MirInstruction {
    MirInstruction::Jump {
        target: BasicBlockId(target),
        edge_args: None,
    }
}

fn original() -> (MirFunction, Vec<(BasicBlockId, MirInstruction)>) {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "cleanup".into(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId(0),
    );
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .set_terminator(jump(10));
    let bindings = vec![
        (
            BasicBlockId(11),
            MirInstruction::Return {
                value: Some(ValueId(11)),
            },
        ),
        (
            BasicBlockId(12),
            MirInstruction::ReturnFault {
                fault_frame: ValueId(6),
            },
        ),
        (BasicBlockId(14), jump(11)),
        (BasicBlockId(15), jump(12)),
        (
            BasicBlockId(13),
            MirInstruction::Invoke {
                operation: InvokeOperation::HomeRelease {
                    object: hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(0)
                        .unwrap(),
                    value: ValueId(8),
                },
                fault_frame: ValueId(6),
                normal_landing: BasicBlockId(14),
                fault_landing: BasicBlockId(15),
            },
        ),
        (BasicBlockId(10), jump(13)),
    ];
    for (id, terminal) in &bindings {
        let mut block = BasicBlock::new(*id);
        block.set_terminator(terminal.clone());
        function.add_block(block);
    }
    function
        .blocks
        .get_mut(&BasicBlockId(10))
        .unwrap()
        .instructions = vec![
        MirInstruction::Copy {
            dst: ValueId(8),
            src: ValueId(7),
        },
        MirInstruction::BinOp {
            dst: ValueId(11),
            op: crate::mir::BinaryOp::Add,
            lhs: ValueId(9),
            rhs: ValueId(10),
        },
    ];
    (function, bindings)
}

fn contracted(original: &MirFunction) -> MirFunction {
    let mut function = original.clone();
    for (from, into) in [(13, 10), (11, 14), (12, 15)] {
        let removed = function.blocks.remove(&BasicBlockId(from)).unwrap();
        function
            .blocks
            .get_mut(&BasicBlockId(into))
            .unwrap()
            .set_terminator(removed.terminator.unwrap());
    }
    function
}

#[test]
fn exact_original_and_three_contractions_preserve_full_release() {
    let (original, bindings) = original();
    let boundary = RootCleanupBoundary::capture(&original, &bindings, 1).unwrap();
    assert_eq!(
        boundary.project(&original, &bindings).unwrap().len(),
        bindings.len()
    );
    let result = boundary.project(&contracted(&original), &bindings).unwrap();
    assert_eq!(result.len(), bindings.len() - 3);
    assert!(result.contains(&(BasicBlockId(10), bindings[4].1.clone())));
    // Partial legal optimization is also a graph equivalence, not a retry.
    let mut partial = original.clone();
    let removed = partial.blocks.remove(&BasicBlockId(13)).unwrap();
    partial
        .blocks
        .get_mut(&BasicBlockId(10))
        .unwrap()
        .set_terminator(removed.terminator.unwrap());
    assert_eq!(
        boundary.project(&partial, &bindings).unwrap().len(),
        bindings.len() - 1
    );
}

#[test]
fn finished_cleanup_rejects_operation_path_and_prefix_mutations() {
    let (original, bindings) = original();
    let boundary = RootCleanupBoundary::capture(&original, &bindings, 1).unwrap();
    let valid = contracted(&original);
    let mutations: &[(&str, fn(&mut MirFunction))] =
        &[
            ("omit release", |f| {
                f.blocks
                    .get_mut(&BasicBlockId(10))
                    .unwrap()
                    .set_terminator(jump(14))
            }),
            ("duplicate release", |f| {
                let op = f.blocks[&BasicBlockId(10)].terminator.clone().unwrap();
                f.blocks
                    .get_mut(&BasicBlockId(14))
                    .unwrap()
                    .instructions
                    .push(op);
            }),
            ("receiver", |f| {
                if let Some(MirInstruction::Invoke {
                    operation: InvokeOperation::HomeRelease { value, .. },
                    ..
                }) = &mut f.blocks.get_mut(&BasicBlockId(10)).unwrap().terminator
                {
                    *value = ValueId(99);
                }
            }),
            ("frame", |f| {
                if let Some(MirInstruction::Invoke { fault_frame, .. }) =
                    &mut f.blocks.get_mut(&BasicBlockId(10)).unwrap().terminator
                {
                    *fault_frame = ValueId(99);
                }
            }),
            ("swap Normal/Fault", |f| {
                if let Some(MirInstruction::Invoke {
                    normal_landing,
                    fault_landing,
                    ..
                }) = &mut f.blocks.get_mut(&BasicBlockId(10)).unwrap().terminator
                {
                    std::mem::swap(normal_landing, fault_landing);
                }
            }),
            ("Fault becomes Normal", |f| {
                f.blocks.get_mut(&BasicBlockId(15)).unwrap().set_terminator(
                    MirInstruction::Return {
                        value: Some(ValueId(11)),
                    },
                )
            }),
            ("return value", |f| {
                f.blocks.get_mut(&BasicBlockId(14)).unwrap().set_terminator(
                    MirInstruction::Return {
                        value: Some(ValueId(99)),
                    },
                )
            }),
            ("prefix omitted", |f| {
                f.blocks
                    .get_mut(&BasicBlockId(10))
                    .unwrap()
                    .instructions
                    .pop();
            }),
            ("prefix reordered", |f| {
                f.blocks
                    .get_mut(&BasicBlockId(10))
                    .unwrap()
                    .instructions
                    .reverse()
            }),
            ("additional instruction", |f| {
                f.blocks
                    .get_mut(&BasicBlockId(14))
                    .unwrap()
                    .instructions
                    .push(MirInstruction::Copy {
                        dst: ValueId(99),
                        src: ValueId(8),
                    })
            }),
            ("entry bypass", |f| {
                f.blocks
                    .get_mut(&BasicBlockId(0))
                    .unwrap()
                    .set_terminator(jump(14))
            }),
            ("external internal incoming", |f| {
                let mut extra = BasicBlock::new(BasicBlockId(99));
                extra.set_terminator(jump(15));
                f.add_block(extra);
            }),
            ("external entry incoming", |f| {
                let mut extra = BasicBlock::new(BasicBlockId(99));
                extra.set_terminator(jump(10));
                f.add_block(extra);
            }),
            ("incoming slot changed", |f| {
                f.blocks
                    .get_mut(&BasicBlockId(0))
                    .unwrap()
                    .set_terminator(MirInstruction::Branch {
                        condition: ValueId(99),
                        then_bb: BasicBlockId(10),
                        else_bb: BasicBlockId(100),
                        then_edge_args: None,
                        else_edge_args: None,
                    })
            }),
            ("removed node restored", |f| {
                let mut block = BasicBlock::new(BasicBlockId(13));
                block.set_terminator(jump(14));
                f.add_block(block);
            }),
            ("entry removed", |f| {
                f.blocks.remove(&BasicBlockId(10));
            }),
        ];
    for (name, mutate) in mutations {
        let mut changed = valid.clone();
        mutate(&mut changed);
        assert!(
            boundary.project(&changed, &bindings).is_err(),
            "accepted {name}"
        );
    }
}

#[test]
fn capture_rejects_incomplete_or_foreign_cleanup_boundary() {
    let (original, bindings) = original();
    let mut duplicate = bindings.clone();
    duplicate.insert(0, bindings[0].clone());
    assert!(RootCleanupBoundary::capture(&original, &duplicate, 1).is_err());
    let mut missing = bindings.clone();
    missing.remove(0);
    assert!(RootCleanupBoundary::capture(&original, &missing, 1).is_err());
    let mut foreign = original.clone();
    let mut extra = BasicBlock::new(BasicBlockId(99));
    extra.set_terminator(jump(13));
    foreign.add_block(extra);
    assert!(RootCleanupBoundary::capture(&foreign, &bindings, 1).is_err());
    let mut prefix = original.clone();
    prefix
        .blocks
        .get_mut(&BasicBlockId(13))
        .unwrap()
        .instructions
        .push(MirInstruction::Copy {
            dst: ValueId(99),
            src: ValueId(8),
        });
    assert!(RootCleanupBoundary::capture(&prefix, &bindings, 1).is_err());
}

#[test]
fn capture_rejects_reachable_release_cycle_even_without_optimization() {
    let (mut function, mut bindings) = original();
    bindings.retain(|(id, _)| *id != BasicBlockId(11));
    function.blocks.remove(&BasicBlockId(11));
    bindings
        .iter_mut()
        .find(|(id, _)| *id == BasicBlockId(14))
        .unwrap()
        .1 = jump(13);
    function
        .blocks
        .get_mut(&BasicBlockId(14))
        .unwrap()
        .set_terminator(jump(13));
    assert!(RootCleanupBoundary::capture(&function, &bindings, 1)
        .unwrap_err()
        .contains("original-cycle"));
}

#[test]
fn multiple_homes_preserve_clean_and_fault_suffixes_after_contraction() {
    let (mut function, mut bindings) = original();
    let position = bindings.iter().position(|(id, _)| *id == BasicBlockId(13)).unwrap();
    let template = bindings[position].1.clone();
    let MirInstruction::Invoke { normal_landing, fault_landing, .. } = &mut bindings[position].1 else { unreachable!() };
    *normal_landing = BasicBlockId(16);
    *fault_landing = BasicBlockId(17);
    // Both paths consume the remaining Home; a pending Fault never becomes clean.
    for (id, normal, fault) in [(16, 14, 15), (17, 15, 15)] {
        let mut instruction = template.clone();
        let MirInstruction::Invoke { operation: InvokeOperation::HomeRelease { value, .. }, normal_landing, fault_landing, .. } = &mut instruction else { unreachable!() };
        *value = ValueId(9);
        *normal_landing = BasicBlockId(normal);
        *fault_landing = BasicBlockId(fault);
        bindings.insert(0, (BasicBlockId(id), instruction));
    }
    for (id, terminal) in &bindings {
        if !function.blocks.contains_key(id) { function.add_block(BasicBlock::new(*id)); }
        function.blocks.get_mut(id).unwrap().set_terminator(terminal.clone());
    }
    assert!(RootCleanupBoundary::capture(&function, &bindings, 1).is_err());
    let boundary = RootCleanupBoundary::capture(&function, &bindings, 2).unwrap();
    let finished = contracted(&function);
    assert!(boundary.project(&finished, &bindings).is_ok());
    let mut drift = finished.clone();
    let Some(MirInstruction::Invoke { normal_landing, .. }) = &mut drift.blocks.get_mut(&BasicBlockId(17)).unwrap().terminator else { unreachable!() };
    *normal_landing = BasicBlockId(14);
    assert!(boundary.project(&drift, &bindings).is_err());
}
