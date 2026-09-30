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
        phi_inputs = vec![(BasicBlockId(1), ValueId(11)), (BasicBlockId(2), ValueId(12))];
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
