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
