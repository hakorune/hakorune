use super::super::super::super::root_cleanup_graph::ordered_paths;
use super::super::super::super::root_home::RootHomeExitEntry;
use super::{origin, RootHomeCleanupOrderV1};
use crate::mir::{
    BasicBlock, BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirInstruction, MirType,
    ValueId,
};

fn fixture() -> (
    RootHomeCleanupOrderV1,
    MirFunction,
    Vec<(BasicBlockId, MirInstruction)>,
) {
    let full = vec![origin(1), origin(2)];
    let order =
        RootHomeCleanupOrderV1::from_sequences(full.clone(), &[full[1].clone()], &full).unwrap();
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "ordered".into(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId(10),
    );
    function
        .blocks
        .get_mut(&BasicBlockId(10))
        .unwrap()
        .set_terminator(MirInstruction::Jump {
            target: BasicBlockId(0),
            edge_args: None,
        });
    let bindings = vec![
        (
            BasicBlockId(1),
            MirInstruction::Return {
                value: Some(ValueId(0)),
            },
        ),
        (
            BasicBlockId(2),
            MirInstruction::ReturnFault {
                fault_frame: ValueId(9),
            },
        ),
        (
            BasicBlockId(3),
            MirInstruction::Invoke {
                operation: full[0].operation().clone(),
                fault_frame: ValueId(9),
                normal_landing: BasicBlockId(2),
                fault_landing: BasicBlockId(2),
            },
        ),
        (
            BasicBlockId(4),
            MirInstruction::Invoke {
                operation: full[1].operation().clone(),
                fault_frame: ValueId(9),
                normal_landing: BasicBlockId(1),
                fault_landing: BasicBlockId(3),
            },
        ),
        (
            BasicBlockId(0),
            MirInstruction::Jump {
                target: BasicBlockId(4),
                edge_args: None,
            },
        ),
    ];
    for (id, terminal) in &bindings {
        let mut block = BasicBlock::new(*id);
        block.set_terminator(terminal.clone());
        function.add_block(block);
    }
    (order, function, bindings)
}
#[test]
fn root_ordered_paths_preserve_fault_only_obligation() {
    let (order, function, bindings) = fixture();
    ordered_paths::validate(
        &function,
        &bindings,
        &RootHomeExitEntry::Plain {
            local_bindings: vec![],
        },
        &order,
        None,
    )
    .unwrap();
    // The same path owner also stops at an explicit Loop continuation instead
    // of requiring the root Return. Localized operands vary by actual origin.
    let normal = order.normal().iter().map(|row| row.operation().clone()).collect::<Vec<_>>();
    let residuals = vec![order.fault_after_normal_step(0).unwrap().iter()
        .map(|row| row.operation().clone()).collect::<Vec<_>>()];
    for localized in [false, true] {
        let mut loop_function = function.clone();
        loop_function.blocks.get_mut(&BasicBlockId(1)).unwrap().set_terminator(MirInstruction::Jump {
            target: BasicBlockId(10), edge_args: None });
        let mut loans = std::collections::BTreeMap::new();
        for (id, source) in [(BasicBlockId(4), normal[0].clone()), (BasicBlockId(3), residuals[0][0].clone())] {
            let actual = if localized {
                let crate::mir::instruction::InvokeOperation::Map(crate::mir::instruction::MapInvokeOperation::End { map }) = &source
                    else { panic!("source End"); };
                crate::mir::instruction::InvokeOperation::Map(crate::mir::instruction::MapInvokeOperation::End { map: ValueId(map.0 + 100) })
            } else { source.clone() };
            let MirInstruction::Invoke { operation, .. } = loop_function.blocks.get_mut(&id).unwrap().terminator.as_mut().unwrap()
                else { panic!("Invoke"); };
            *operation = actual.clone();
            loans.insert(id, (source, actual));
        }
        let mut resolve = |id, source: &crate::mir::instruction::InvokeOperation| {
            let (original, actual) = loans.get(&id).ok_or_else(|| "foreign origin".to_owned())?;
            if original != source { return Err("foreign source End".to_owned()); }
            Ok(actual.clone())
        };
        ordered_paths::validate_to_continuation_v2(&loop_function, BasicBlockId(0), BasicBlockId(1),
            &normal, &residuals, Some(ValueId(9)), &mut resolve).unwrap();
        assert!(ordered_paths::validate_to_continuation_v2(&loop_function, BasicBlockId(0), BasicBlockId(9),
            &normal, &residuals, Some(ValueId(9)), &mut resolve).is_err());
        assert!(ordered_paths::validate_to_continuation_v2(&loop_function, BasicBlockId(0), BasicBlockId(4),
            &normal, &residuals, Some(ValueId(9)), &mut resolve).unwrap_err().contains("early-continuation"));
        // A matching ReturnFault beyond the caller continuation cannot discharge
        // an obligation owned by this cleanup graph.
        let mut crossed = loop_function.clone();
        crossed.blocks.get_mut(&BasicBlockId(1)).unwrap().set_terminator(MirInstruction::Jump {
            target: BasicBlockId(20), edge_args: None });
        let mut outside = BasicBlock::new(BasicBlockId(20));
        outside.set_terminator(MirInstruction::ReturnFault { fault_frame: ValueId(9) });
        crossed.add_block(outside);
        let MirInstruction::Invoke { normal_landing, fault_landing, .. } = crossed.blocks
            .get_mut(&BasicBlockId(3)).unwrap().terminator.as_mut().unwrap() else { panic!("fault End"); };
        *normal_landing = BasicBlockId(1);
        *fault_landing = BasicBlockId(1);
        assert!(ordered_paths::validate_to_continuation_v2(&crossed, BasicBlockId(0), BasicBlockId(1),
            &normal, &residuals, Some(ValueId(9)), &mut resolve).unwrap_err().contains("fault-continuation"));
        let MirInstruction::Invoke { operation, .. } = loop_function.blocks.get_mut(&BasicBlockId(3)).unwrap().terminator.as_mut().unwrap()
            else { panic!("fault End"); };
        *operation = loans[&BasicBlockId(4)].1.clone();
        assert!(ordered_paths::validate_to_continuation_v2(&loop_function, BasicBlockId(0), BasicBlockId(1),
            &normal, &residuals, Some(ValueId(9)), &mut resolve).unwrap_err().contains("ordered-path/operation"));
    }
}
#[test]
fn root_ordered_paths_reject_joint_record_and_mir_fault_omission() {
    let (order, mut function, mut bindings) = fixture();
    function.blocks.remove(&BasicBlockId(3));
    bindings.retain(|(id, _)| *id != BasicBlockId(3));
    let MirInstruction::Invoke { fault_landing, .. } = &mut bindings[2].1 else {
        panic!("invoke")
    };
    *fault_landing = BasicBlockId(2);
    function
        .blocks
        .get_mut(&BasicBlockId(4))
        .unwrap()
        .set_terminator(bindings[2].1.clone());
    assert!(ordered_paths::validate(
        &function,
        &bindings,
        &RootHomeExitEntry::Plain {
            local_bindings: vec![]
        },
        &order,
        None
    )
    .unwrap_err()
    .contains("ordered-path/operation"));
}
#[test]
fn root_ordered_paths_reject_normal_substitution_fault_retry_and_wrong_outcome() {
    for mutation in 0..3 {
        let (order, mut function, bindings) = fixture();
        let id = if mutation == 0 { 4 } else { 3 };
        let MirInstruction::Invoke {
            operation,
            normal_landing,
            ..
        } = function
            .blocks
            .get_mut(&BasicBlockId(id))
            .unwrap()
            .terminator
            .as_mut()
            .unwrap()
        else {
            panic!("invoke")
        };
        match mutation {
            0 => *operation = order.full()[0].operation().clone(),
            1 => *operation = order.full()[1].operation().clone(),
            2 => *normal_landing = BasicBlockId(1),
            _ => unreachable!(),
        }
        assert!(ordered_paths::validate(
            &function,
            &bindings,
            &RootHomeExitEntry::Plain {
                local_bindings: vec![]
            },
            &order,
            None
        )
        .is_err());
    }
}

#[test]
fn root_ordered_structure_accepts_fault_only_release_without_fixed_count() {
    let (_, mut function, bindings) = fixture();
    super::super::super::super::root_cleanup_graph::ordered_structure::validate_original(
        &function,
        &bindings,
        &RootHomeExitEntry::Plain {
            local_bindings: vec![],
        },
        crate::mir::instruction::InvokeCallResultKind::I64,
    )
    .unwrap();
    // The same structural owner recognizes one explicit caller continuation,
    // while all other outgoing edges and all unreachable owned nodes fail.
    let loop_bindings = bindings.iter().filter(|(id, _)| *id != BasicBlockId(1)).cloned().collect::<Vec<_>>();
    let validate = |function: &MirFunction, rows: &Vec<(BasicBlockId, MirInstruction)>, stop| {
        super::super::super::super::root_cleanup_graph::ordered_structure::validate_to_continuation_v2(
            function, rows, BasicBlockId(0), stop)
    };
    validate(&function, &loop_bindings, BasicBlockId(1)).unwrap();
    for mutation in 0..6 {
        let mut changed = function.clone();
        let mut rows = loop_bindings.clone();
        let stop = if mutation == 5 { BasicBlockId(4) } else { BasicBlockId(1) };
        match mutation {
            0 => {
                let terminal = MirInstruction::ReturnFault { fault_frame: ValueId(9) };
                let mut extra = BasicBlock::new(BasicBlockId(8));
                extra.set_terminator(terminal.clone());
                changed.add_block(extra);
                rows.push((BasicBlockId(8), terminal));
            },
            1 | 3 => {
                let terminal = rows.iter_mut().find(|(id, _)| *id == BasicBlockId(3)).unwrap();
                let MirInstruction::Invoke { normal_landing, .. } = &mut terminal.1 else { panic!("fault End"); };
                *normal_landing = BasicBlockId(if mutation == 1 { 9 } else { 3 });
                changed.blocks.get_mut(&terminal.0).unwrap().set_terminator(terminal.1.clone());
            },
            2 => {
                let mut extra = BasicBlock::new(BasicBlockId(8));
                extra.set_terminator(MirInstruction::Jump { target: BasicBlockId(3), edge_args: None });
                changed.add_block(extra);
            },
            4 => rows.push(bindings[0].clone()),
            5 => {},
            _ => unreachable!(),
        }
        assert!(validate(&changed, &rows, stop).is_err(), "mutation {mutation}");
    }
    // The Plain entry is an original source continuation, unlike cleanup nodes.
    let phi = MirInstruction::Phi {
        dst: ValueId(30), inputs: vec![(BasicBlockId(10), ValueId(31))], type_hint: None,
    };
    function.blocks.get_mut(&BasicBlockId(0)).unwrap().instructions.push(phi.clone());
    super::super::super::super::root_cleanup_graph::ordered_structure::validate_original(
        &function, &bindings, &RootHomeExitEntry::Plain { local_bindings: vec![] },
        crate::mir::instruction::InvokeCallResultKind::I64,
    ).expect("source prefix is outside the cleanup instruction owner");
    let boundary = super::super::super::super::physical_boundary::PhysicalBoundary::capture(
        &function, &bindings,
    ).unwrap_err();
    assert!(boundary.contains("phi-in-recorded-block"), "{boundary}");
    let roles = std::collections::BTreeMap::from([
        (BasicBlockId(0), (vec![phi].into_boxed_slice(), None)),
    ]);
    super::super::super::super::physical_boundary::PhysicalBoundary::capture_with_source_phi_roles(
        &function, &bindings, &[], &[], &roles, None,
    ).expect("independent physical owner still requires the exact source prefix role");

}
#[test]
fn root_ordered_structure_rejects_extra_missing_foreign_cycle_and_duplicate_nodes() {
    for mutation in 0..7 {
        let (_, mut function, mut bindings) = fixture();
        match mutation {
            0 => function
                .blocks
                .get_mut(&BasicBlockId(3))
                .unwrap()
                .instructions
                .push(MirInstruction::Copy {
                    dst: ValueId(30),
                    src: ValueId(31),
                }),
            1 => {
                function.blocks.remove(&BasicBlockId(3));
            }
            2 => {
                bindings.insert(0, (BasicBlockId(8), MirInstruction::Return { value: None }));
            }
            3 => {
                let MirInstruction::Invoke { normal_landing, .. } = &mut bindings[2].1 else {
                    panic!("invoke")
                };
                *normal_landing = BasicBlockId(3);
                function
                    .blocks
                    .get_mut(&BasicBlockId(3))
                    .unwrap()
                    .set_terminator(bindings[2].1.clone());
            }
            4 => {
                bindings.insert(0, bindings[0].clone());
            }
            5 => {
                let mut block = BasicBlock::new(BasicBlockId(8));
                block.set_terminator(MirInstruction::Jump {
                    target: BasicBlockId(3),
                    edge_args: None,
                });
                function.add_block(block);
            }
            6 => function.blocks.get_mut(&BasicBlockId(3)).unwrap().instructions.push(
                MirInstruction::Phi {
                    dst: ValueId(30), inputs: vec![(BasicBlockId(4), ValueId(31))], type_hint: None,
                },
            ),
            _ => unreachable!(),
        }
        assert!(
            super::super::super::super::root_cleanup_graph::ordered_structure::validate_original(
                &function,
                &bindings,
                &RootHomeExitEntry::Plain {
                    local_bindings: vec![]
                },
                crate::mir::instruction::InvokeCallResultKind::I64
            )
            .is_err(),
            "mutation {mutation}"
        );
    }
}

#[test]
fn root_ordered_structure_and_paths_accept_finished_plain_entry_contraction() {
    let (order, original, bindings) = fixture();
    let boundary = super::super::super::super::physical_boundary::PhysicalBoundary::capture(
        &original, &bindings,
    )
    .unwrap();
    let mut function = original.clone();
    let removed = function.blocks.remove(&BasicBlockId(4)).unwrap();
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .set_terminator(removed.terminator.unwrap());
    let mut projection = boundary.project(&function).unwrap();
    let entry = RootHomeExitEntry::Plain {
        local_bindings: vec![],
    };
    super::super::super::super::root_cleanup_graph::ordered_structure::validate_projected(
        &function,
        &bindings,
        &entry,
        &projection,
        crate::mir::instruction::InvokeCallResultKind::I64,
    )
    .unwrap();
    ordered_paths::validate(&function, &bindings, &entry, &order, Some(&projection)).unwrap();
    boundary
        .validate_complete(&function, &mut projection, &bindings)
        .unwrap();
}
