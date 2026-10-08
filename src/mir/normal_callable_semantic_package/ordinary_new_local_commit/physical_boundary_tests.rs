use super::*;
use crate::mir::{BasicBlock, ConstValue, EffectMask, FunctionSignature, MirType};

fn fixture() -> (MirFunction, Vec<(BasicBlockId, MirInstruction)>) {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "physical-boundary".into(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId(0),
    );
    let frame = MirInstruction::FaultFrameEnter {
        dst: ValueId(1),
        mode: crate::mir::instruction::FaultFrameMode::RootOwned,
    };
    let mut block = BasicBlock::new(BasicBlockId(0));
    block.add_instruction(frame.clone());
    for dst in [2, 3] {
        block.add_instruction(MirInstruction::Const {
            dst: ValueId(dst),
            value: ConstValue::Integer(i64::from(dst)),
        });
    }
    block.set_terminator(MirInstruction::Return { value: None });
    function.add_block(block);
    (function, vec![(BasicBlockId(0), frame)])
}

fn validate(boundary: &PhysicalBoundary, function: &MirFunction, bindings: &Bindings) -> bool {
    let mut projection = boundary.project(function).unwrap();
    boundary
        .validate_complete(function, &mut projection, bindings)
        .is_ok()
}

#[test]
fn only_original_unrecorded_unused_constants_may_disappear() {
    let (original, bindings) = fixture();
    let boundary = PhysicalBoundary::capture(&original, &bindings).unwrap();
    assert!(validate(&boundary, &original, &bindings));
    let mut omitted = original.clone();
    omitted
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .instructions
        .truncate(1);
    assert!(validate(&boundary, &omitted, &bindings));

    let mutations: [fn(&mut MirFunction); 4] = [
        |f| {
            f.blocks.get_mut(&BasicBlockId(0)).unwrap().instructions[1] = MirInstruction::Const {
                dst: ValueId(2),
                value: ConstValue::Integer(99),
            };
        },
        |f| {
            f.blocks
                .get_mut(&BasicBlockId(0))
                .unwrap()
                .instructions
                .swap(1, 2);
        },
        |f| {
            f.blocks
                .get_mut(&BasicBlockId(0))
                .unwrap()
                .instructions
                .push(MirInstruction::Const {
                    dst: ValueId(4),
                    value: ConstValue::Integer(4),
                });
        },
        |f| {
            f.blocks
                .get_mut(&BasicBlockId(0))
                .unwrap()
                .set_terminator(MirInstruction::Return {
                    value: Some(ValueId(2)),
                });
        },
    ];
    for mutate in mutations {
        let mut changed = original.clone();
        mutate(&mut changed);
        assert!(!validate(&boundary, &changed, &bindings));
    }
    for kind in ["recorded", "used", "duplicate"] {
        let mut protected = original.clone();
        let mut bindings = bindings.clone();
        let block = protected.blocks.get_mut(&BasicBlockId(0)).unwrap();
        match kind {
            "recorded" => bindings.push((BasicBlockId(0), block.instructions[1].clone())),
            "used" => block.set_terminator(MirInstruction::Return {
                value: Some(ValueId(2)),
            }),
            "duplicate" => block.instructions.push(block.instructions[1].clone()),
            _ => unreachable!(),
        }
        let boundary = PhysicalBoundary::capture(&protected, &bindings).unwrap();
        protected
            .blocks
            .get_mut(&BasicBlockId(0))
            .unwrap()
            .instructions
            .remove(1);
        assert!(!validate(&boundary, &protected, &bindings), "{kind}");
    }
}

fn literal_control_fixture(shape: &str) -> (MirFunction, Vec<(BasicBlockId, MirInstruction)>) {
    let (mut function, _) = fixture();
    let entry = function.blocks.get_mut(&BasicBlockId(0)).unwrap();
    entry.instructions.clear();
    for (dst, value) in [(1, 1), (2, 0)] {
        entry.add_instruction(MirInstruction::Const {
            dst: ValueId(dst),
            value: ConstValue::Integer(value),
        });
    }
    let control = if shape == "split" {
        BasicBlockId(6)
    } else {
        BasicBlockId(0)
    };
    if control != BasicBlockId(0) {
        entry.set_terminator(MirInstruction::Jump {
            target: control,
            edge_args: None,
        });
        function.add_block(BasicBlock::new(control));
    }
    let predicate = function.blocks.get_mut(&control).unwrap();
    predicate.add_instruction(MirInstruction::Copy {
        dst: ValueId(6),
        src: ValueId(1),
    });
    predicate.add_instruction(MirInstruction::Compare {
        dst: ValueId(3),
        op: crate::mir::CompareOp::Gt,
        lhs: ValueId(6),
        rhs: ValueId(2),
    });
    predicate.add_instruction(MirInstruction::Copy {
        dst: ValueId(7),
        src: ValueId(3),
    });
    predicate.set_terminator(MirInstruction::Branch {
        condition: ValueId(7),
        then_bb: BasicBlockId(1),
        else_bb: BasicBlockId(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut bindings = Vec::new();
    for (id, dst) in [(1, 4), (2, 5)] {
        let mut block = BasicBlock::new(BasicBlockId(id));
        let frame = MirInstruction::FaultFrameEnter {
            dst: ValueId(dst),
            mode: crate::mir::instruction::FaultFrameMode::RootOwned,
        };
        block.add_instruction(frame.clone());
        block.set_terminator(if shape == "nonmerged" && id == 2 {
            MirInstruction::Jump {
                target: BasicBlockId(1),
                edge_args: None,
            }
        } else {
            MirInstruction::Return { value: None }
        });
        function.add_block(block);
        bindings.push((BasicBlockId(id), frame));
    }
    (function, bindings)
}

fn boundary_result(
    boundary: &PhysicalBoundary,
    function: &MirFunction,
    bindings: &Bindings,
) -> Result<(), String> {
    let mut projection = boundary.project(function)?;
    boundary.validate_complete(function, &mut projection, bindings)
}

fn replace_definition(function: &mut MirFunction, dst: ValueId, replacement: MirInstruction) {
    let original = function
        .blocks
        .values_mut()
        .flat_map(|b| &mut b.instructions)
        .find(|i| i.dst_value() == Some(dst))
        .unwrap();
    *original = replacement;
}

#[test]
fn literal_integer_control_folds_preserve_all_recorded_arms() {
    for shape in ["merged", "nonmerged", "split"] {
        let (original, bindings) = literal_control_fixture(shape);
        let boundary = PhysicalBoundary::capture(&original, &bindings).unwrap();
        assert!(
            boundary_result(&boundary, &original, &bindings).is_ok(),
            "{shape} original"
        );
        let mut compare_only = original.clone();
        replace_definition(
            &mut compare_only,
            ValueId(3),
            MirInstruction::Const {
                dst: ValueId(3),
                value: ConstValue::Bool(true),
            },
        );
        assert!(
            boundary_result(&boundary, &compare_only, &bindings).is_ok(),
            "{shape} Compare only"
        );
        for mutation in [
            "retained-branch-wrong-bool",
            "operand-literal",
            "condition-copy",
        ] {
            let mut changed = if mutation == "retained-branch-wrong-bool" {
                compare_only.clone()
            } else {
                original.clone()
            };
            let replacement = match mutation {
                "retained-branch-wrong-bool" => MirInstruction::Const {
                    dst: ValueId(3),
                    value: ConstValue::Bool(false),
                },
                "operand-literal" => MirInstruction::Const {
                    dst: ValueId(1),
                    value: ConstValue::Integer(-1),
                },
                "condition-copy" => MirInstruction::Copy {
                    dst: ValueId(7),
                    src: ValueId(2),
                },
                _ => unreachable!(),
            };
            replace_definition(&mut changed, replacement.dst_value().unwrap(), replacement);
            assert!(
                boundary_result(&boundary, &changed, &bindings).is_err(),
                "{shape} {mutation}"
            );
        }
        let mut module = crate::mir::MirModule::new("literal-control".into());
        module
            .functions
            .insert(original.signature.name.clone(), original);
        assert!(crate::mir::passes::simplify_cfg::simplify(&mut module) > 0);
        let finished = module.functions.values().next().unwrap();
        assert_eq!(
            finished.blocks.contains_key(&BasicBlockId(1)),
            shape == "nonmerged"
        );
        assert!(
            boundary_result(&boundary, finished, &bindings).is_ok(),
            "{shape} canonical fold"
        );
        let mut omitted = finished.clone();
        for block in omitted.blocks.values_mut() {
            block
                .instructions
                .retain(|i| !matches!(i.dst_value(), Some(ValueId(1 | 2 | 3 | 6 | 7))));
        }
        assert!(
            boundary_result(&boundary, &omitted, &bindings).is_ok(),
            "{shape} unused cone DCE"
        );
        for mutation in [
            "opposite-bool",
            "foreign-dst",
            "extra-const",
            "wrong-target",
            "missing-arm",
            "unreachable-duplicate",
            "foreign-definition",
            "unreachable-use",
            "return-env-use",
            "opposite-entry",
        ] {
            let mut changed = if matches!(mutation, "unreachable-use" | "return-env-use") {
                omitted.clone()
            } else {
                finished.clone()
            };
            match mutation {
                "opposite-bool" | "foreign-dst" => replace_definition(
                    &mut changed,
                    ValueId(3),
                    MirInstruction::Const {
                        dst: ValueId(if mutation == "foreign-dst" { 99 } else { 3 }),
                        value: ConstValue::Bool(mutation != "opposite-bool"),
                    },
                ),
                "extra-const" => changed
                    .blocks
                    .get_mut(&BasicBlockId(0))
                    .unwrap()
                    .add_instruction(MirInstruction::Const {
                        dst: ValueId(99),
                        value: ConstValue::Integer(9),
                    }),
                "wrong-target" => changed
                    .blocks
                    .get_mut(&BasicBlockId(0))
                    .unwrap()
                    .set_terminator(MirInstruction::Jump {
                        target: BasicBlockId(2),
                        edge_args: None,
                    }),
                "missing-arm" => {
                    changed.blocks.remove(&BasicBlockId(2));
                }
                "unreachable-duplicate"
                | "foreign-definition"
                | "unreachable-use"
                | "return-env-use" => {
                    let mut foreign = BasicBlock::new(BasicBlockId(90));
                    if mutation == "foreign-definition" {
                        for block in changed.blocks.values_mut() {
                            block
                                .instructions
                                .retain(|i| i.dst_value() != Some(ValueId(3)));
                        }
                    }
                    if matches!(mutation, "unreachable-duplicate" | "foreign-definition") {
                        foreign.add_instruction(MirInstruction::Const {
                            dst: ValueId(3),
                            value: ConstValue::Bool(true),
                        });
                    }
                    foreign.set_terminator(MirInstruction::Return {
                        value: (mutation == "unreachable-use").then_some(ValueId(3)),
                    });
                    if mutation == "return-env-use" {
                        foreign.return_env = Some(vec![ValueId(3)]);
                    }
                    changed.add_block(foreign);
                }
                "opposite-entry" => {
                    changed.blocks.remove(&BasicBlockId(0));
                    changed.entry_block = BasicBlockId(2);
                }
                _ => unreachable!(),
            }
            assert!(
                boundary_result(&boundary, &changed, &bindings).is_err(),
                "{shape} {mutation}"
            );
        }
    }
    // Dead carriers may disappear, but identical definitions/Branches cannot
    // relocate outside the original physical correspondence scope.
    for kind in ["definition", "branch"] {
        let (mut original, bindings) = literal_control_fixture("merged");
        let mut carrier = BasicBlock::new(BasicBlockId(9));
        if kind == "definition" {
            carrier.add_instruction(MirInstruction::Compare {
                dst: ValueId(20),
                op: crate::mir::CompareOp::Gt,
                lhs: ValueId(1),
                rhs: ValueId(2),
            });
            carrier.set_terminator(MirInstruction::Return { value: None });
        } else {
            original
                .blocks
                .get_mut(&BasicBlockId(0))
                .unwrap()
                .add_instruction(MirInstruction::Copy {
                    dst: ValueId(8),
                    src: ValueId(3),
                });
            carrier.set_terminator(MirInstruction::Branch {
                condition: ValueId(8),
                then_bb: BasicBlockId(10),
                else_bb: BasicBlockId(11),
                then_edge_args: None,
                else_edge_args: None,
            });
            for id in [10, 11] {
                let mut arm = BasicBlock::new(BasicBlockId(id));
                arm.set_terminator(MirInstruction::Return { value: None });
                original.add_block(arm);
            }
        }
        original.add_block(carrier);
        let boundary = PhysicalBoundary::capture(&original, &bindings).unwrap();
        assert!(boundary_result(&boundary, &original, &bindings).is_ok());
        let mut removed = original.clone();
        let removed_carrier = removed.blocks.remove(&BasicBlockId(9)).unwrap();
        assert!(
            boundary_result(&boundary, &removed, &bindings).is_ok(),
            "dead {kind} removal"
        );
        let mut relocated = removed;
        let mut foreign = BasicBlock::new(BasicBlockId(90));
        foreign.instructions = removed_carrier.instructions;
        foreign.terminator = removed_carrier.terminator;
        relocated.add_block(foreign);
        assert!(
            boundary_result(&boundary, &relocated, &bindings).is_err(),
            "dead {kind} relocation"
        );
        if kind == "branch" {
            let mut duplicated = original.clone();
            let mut foreign = duplicated.blocks[&BasicBlockId(9)].clone();
            foreign.id = BasicBlockId(90);
            duplicated.add_block(foreign);
            assert!(
                boundary_result(&boundary, &duplicated, &bindings).is_err(),
                "dead Branch duplicate"
            );
        }
    }
}

/// A `new`-style join fixture: entry branches into two arms that both
/// jump into a shared block where a recorded instruction lives. `phi`
/// controls whether the recorded block carries a leading phi, `arms`
/// controls how many arms reach it, and `non_phi_first` moves a Const
/// ahead of the phi to break the leading run.
fn join_fixture(
    arms: usize,
    phi: bool,
    non_phi_first: bool,
) -> (MirFunction, Vec<(BasicBlockId, MirInstruction)>) {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "physical-boundary-join".into(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId(0),
    );
    let mut entry = BasicBlock::new(BasicBlockId(0));
    entry.add_instruction(MirInstruction::Const {
        dst: ValueId(1),
        value: ConstValue::Integer(1),
    });
    let mut phi_inputs = vec![(BasicBlockId(0), ValueId(10))];
    if arms == 2 {
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId(1),
            then_bb: BasicBlockId(1),
            else_bb: BasicBlockId(2),
            then_edge_args: None,
            else_edge_args: None,
        });
        phi_inputs = vec![
            (BasicBlockId(1), ValueId(11)),
            (BasicBlockId(2), ValueId(12)),
        ];
    } else {
        entry.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(3),
            edge_args: None,
        });
    }
    function.add_block(entry);
    if arms == 2 {
        for arm in 1..=2u32 {
            let mut block = BasicBlock::new(BasicBlockId(arm));
            block.set_terminator(MirInstruction::Jump {
                target: BasicBlockId(3),
                edge_args: None,
            });
            function.add_block(block);
        }
    }
    let recorded = MirInstruction::Const {
        dst: ValueId(4),
        value: ConstValue::Integer(4),
    };
    let mut join = BasicBlock::new(BasicBlockId(3));
    if non_phi_first {
        join.add_instruction(MirInstruction::Const {
            dst: ValueId(6),
            value: ConstValue::Integer(6),
        });
    }
    if phi {
        join.add_instruction(MirInstruction::Phi {
            dst: ValueId(5),
            inputs: phi_inputs,
            type_hint: None,
        });
    }
    join.add_instruction(recorded.clone());
    join.set_terminator(MirInstruction::Return { value: None });
    function.add_block(join);
    (function, vec![(BasicBlockId(3), recorded)])
}

#[test]
fn leading_phi_in_a_genuine_join_recorded_block_captures_and_validates() {
    let (function, bindings) = join_fixture(2, true, false);
    let boundary = PhysicalBoundary::capture(&function, &bindings)
        .expect("a leading phi on a two-edge join is legal recorded context");
    assert!(validate(&boundary, &function, &bindings));
}

#[test]
fn phi_in_a_sole_predecessor_recorded_block_still_rejects() {
    let (function, bindings) = join_fixture(1, true, false);
    let error = PhysicalBoundary::capture(&function, &bindings).unwrap_err();
    assert!(
        error.contains("phi-in-recorded-block"),
        "sole-predecessor phi must stay fail-closed: {error}"
    );
}

#[test]
fn non_leading_phi_in_a_join_recorded_block_still_rejects() {
    let (function, bindings) = join_fixture(2, true, true);
    let error = PhysicalBoundary::capture(&function, &bindings).unwrap_err();
    assert!(
        error.contains("phi-in-recorded-block"),
        "a phi after a non-phi instruction is not a join head: {error}"
    );
}
