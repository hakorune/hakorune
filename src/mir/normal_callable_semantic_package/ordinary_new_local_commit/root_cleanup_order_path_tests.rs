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
    let (_, function, bindings) = fixture();
    super::super::super::super::root_cleanup_graph::ordered_structure::validate_original(
        &function,
        &bindings,
        &RootHomeExitEntry::Plain {
            local_bindings: vec![],
        },
    )
    .unwrap();
}
#[test]
fn root_ordered_structure_rejects_extra_missing_foreign_cycle_and_duplicate_nodes() {
    for mutation in 0..6 {
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
            _ => unreachable!(),
        }
        assert!(
            super::super::super::super::root_cleanup_graph::ordered_structure::validate_original(
                &function,
                &bindings,
                &RootHomeExitEntry::Plain {
                    local_bindings: vec![]
                },
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
    )
    .unwrap();
    ordered_paths::validate(&function, &bindings, &entry, &order, Some(&projection)).unwrap();
    boundary
        .validate_complete(&function, &mut projection, &bindings)
        .unwrap();
}
