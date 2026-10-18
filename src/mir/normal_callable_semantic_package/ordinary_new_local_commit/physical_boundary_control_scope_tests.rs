//! Governing control is independent of a Call's unchanged data arguments.
use super::*;

fn scalar_control_fixture(with_cycle: bool) -> (MirFunction, Vec<(BasicBlockId, MirInstruction)>) {
    let mut original = MirFunction::new(FunctionSignature {
        name: "control-boundary".into(), params: vec![MirType::Bool, MirType::Bool],
        return_type: MirType::Integer, effects: EffectMask::PURE,
    }, BasicBlockId(0));
    original.params = vec![ValueId(1), ValueId(2)];
    let mut entry = BasicBlock::new(BasicBlockId(0));
    entry.add_instruction(MirInstruction::Copy { dst: ValueId(5), src: ValueId(1) });
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId(5), then_bb: BasicBlockId(1), else_bb: BasicBlockId(2),
        then_edge_args: None, else_edge_args: None,
    });
    original.add_block(entry);
    let target = crate::mir::builder::CanonicalSameModuleCallableKeyV1::static_box_method("Values", "find", 0)
        .canonical_global_target_v1().unwrap();
    let call = MirInstruction::Call(crate::mir::definitions::MirCall::new(
        Some(ValueId(3)), crate::mir::Callee::Global(target), vec![]));
    let mut selected = BasicBlock::new(BasicBlockId(1));
    selected.add_instruction(call.clone());
    selected.set_terminator(MirInstruction::Return { value: Some(ValueId(3)) });
    original.add_block(selected);
    let mut other = BasicBlock::new(BasicBlockId(2));
    other.set_terminator(if with_cycle {
        MirInstruction::Jump { target: BasicBlockId(0), edge_args: None }
    } else { MirInstruction::Return { value: None } });
    original.add_block(other);
    original.update_cfg();
    let bindings = vec![(BasicBlockId(1), call)];
    (original, bindings)
}

#[test]
fn recorded_scalar_call_rechecks_external_branch_and_condition_producer() {
    for with_cycle in [false, true] {
        let (original, bindings) = scalar_control_fixture(with_cycle);
        let boundary = PhysicalBoundary::capture(&original, &bindings).unwrap();
        let mut projection = boundary.project(&original).unwrap();
        boundary.validate_complete(&original, &mut projection, &bindings).unwrap();
        projection.validate_finished_sequences_v1(&original).unwrap();
        for mutation in ["condition", "producer", "entry", "duplicate", "parameter-collision", "incoming"] {
            let mut changed = original.clone();
            match mutation {
                "condition" => {
                    let Some(MirInstruction::Branch { condition, .. }) =
                        &mut changed.blocks.get_mut(&BasicBlockId(0)).unwrap().terminator else { unreachable!() };
                    *condition = ValueId(2); // genuine Bool, same arms and edge args
                }
                "producer" => changed.blocks.get_mut(&BasicBlockId(0)).unwrap().instructions[0] =
                    MirInstruction::Copy { dst: ValueId(5), src: ValueId(2) },
                "entry" => changed.entry_block = BasicBlockId(1),
                "incoming" if !with_cycle => changed.blocks.get_mut(&BasicBlockId(2)).unwrap()
                    .set_terminator(MirInstruction::Jump { target: BasicBlockId(1), edge_args: None }),
                "incoming" => continue, // both cycle blocks already belong to the scope

                "duplicate" | "parameter-collision" => changed.blocks.get_mut(&BasicBlockId(2)).unwrap()
                    .add_instruction(MirInstruction::Copy { dst: if mutation == "duplicate" { ValueId(5) } else { ValueId(1) }, src: ValueId(2) }),
                _ => unreachable!(),
            }
            assert!(projection.validate_finished_sequences_v1(&changed).is_err(), "{mutation}");
            assert!(boundary.project(&changed).and_then(|mut projected|
                boundary.validate_complete(&changed, &mut projected, &bindings)).is_err(), "{mutation}");
        }
    }
    // Identical metadata occurrences can be removed independently; the final
    // consumer replays the sealed count, never tuple-wide omission membership.
    for omitted in 0..=2 {
        let (mut original, mut bindings) = scalar_control_fixture(false);
        let hint = MirInstruction::KeepAlive { values: vec![ValueId(1)] };
        original.blocks.get_mut(&BasicBlockId(1)).unwrap().instructions.extend([hint.clone(), hint.clone()]);
        bindings.extend([(BasicBlockId(1), hint.clone()), (BasicBlockId(1), hint.clone())]);
        let boundary = PhysicalBoundary::capture(&original, &bindings).unwrap();
        let mut finished = original.clone();
        finished.blocks.get_mut(&BasicBlockId(1)).unwrap().instructions.truncate(3 - omitted);
        let mut projection = boundary.project(&finished).unwrap();
        assert!(projection.validate_metadata_correspondence_v2(&finished, &(BasicBlockId(1), hint.clone()), true).is_err());
        boundary.validate_complete(&finished, &mut projection, &bindings).unwrap();
        projection.validate_metadata_correspondence_v2(&finished, &(BasicBlockId(1), hint.clone()), true).unwrap();
        let mut extra = finished.clone();
        extra.blocks.get_mut(&BasicBlockId(1)).unwrap().instructions.push(hint.clone());
        assert!(projection.validate_finished_sequences_v1(&extra).is_err());
    }
}

#[test]
fn control_phi_preservation_uses_finishing_roots_and_leaves_dead_temporaries_removable() {
    let (mut original, mut bindings) = scalar_control_fixture(false);
    let Some(MirInstruction::Branch { then_bb, .. }) = &mut original.blocks.get_mut(&BasicBlockId(0)).unwrap().terminator
        else { unreachable!() };
    *then_bb = BasicBlockId(3);
    let entry = original.blocks.get_mut(&BasicBlockId(0)).unwrap();
    entry.set_terminator(entry.terminator.clone().unwrap());
    let mut predecessor = BasicBlock::new(BasicBlockId(3));
    predecessor.add_instruction(MirInstruction::Const { dst: ValueId(7), value: ConstValue::Integer(42) });
    predecessor.add_instruction(MirInstruction::BinOp { dst: ValueId(8), op: crate::mir::BinaryOp::Add, lhs: ValueId(7), rhs: ValueId(7) });
    predecessor.set_terminator(MirInstruction::Jump { target: BasicBlockId(4), edge_args: None });
    original.add_block(predecessor);
    let mut middle = BasicBlock::new(BasicBlockId(4));
    middle.add_instruction(MirInstruction::Phi { dst: ValueId(6), inputs: vec![(BasicBlockId(3), ValueId(1))], type_hint: None });
    middle.set_terminator(MirInstruction::Jump { target: BasicBlockId(1), edge_args: None });
    original.add_block(middle);
    let MirInstruction::Call(call) = &mut bindings[0].1 else { unreachable!() };
    call.args = vec![ValueId(6)];
    call.callee = crate::mir::Callee::Global(crate::mir::builder::CanonicalSameModuleCallableKeyV1::static_box_method("Values", "find", 1)
        .canonical_global_target_v1().unwrap());
    original.blocks.get_mut(&BasicBlockId(1)).unwrap().instructions[0] = bindings[0].1.clone();
    original.update_cfg();
    let boundary = PhysicalBoundary::capture(&original, &bindings).unwrap();
    let required: BTreeSet<_> = boundary.control_producer_bindings_v1(&original).unwrap().iter().filter_map(|(_, row)| row.dst_value()).collect();
    assert!(required.contains(&ValueId(6)));
    assert!(!required.contains(&ValueId(7)) && !required.contains(&ValueId(8)));
    let symbol = original.signature.name.clone();
    let mut module = crate::mir::MirModule::new("control-preservation".into());
    module.functions.insert(symbol.clone(), original);
    let mut unprotected = module.clone();
    crate::mir::passes::simplify_cfg::simplify(&mut unprotected);
    assert!(unprotected.functions[&symbol].blocks.values().all(|block|
        block.instructions.iter().all(|row| row.dst_value() != Some(ValueId(6)))));
    let roots = BTreeMap::from([(symbol.clone(), required)]);
    crate::mir::passes::simplify_cfg::simplify_with_preserved_producers(&mut module, &roots);
    crate::mir::passes::dce::eliminate_dead_code_with_required_roots(&mut module, &roots);
    let finished = &module.functions[&symbol];
    assert!(finished.blocks.values().flat_map(|block| &block.instructions).any(|row| row.dst_value() == Some(ValueId(6))));
    assert!(finished.blocks.values().flat_map(|block| &block.instructions).all(|row| !matches!(row.dst_value(), Some(ValueId(7) | ValueId(8)))));
    let mut projection = boundary.project(finished).unwrap();
    boundary.validate_complete(finished, &mut projection, &bindings).unwrap();
    projection.validate_finished_sequences_v1(finished).unwrap();
}
