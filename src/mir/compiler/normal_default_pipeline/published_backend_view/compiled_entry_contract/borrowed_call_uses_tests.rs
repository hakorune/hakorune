//! Original source/entry/Forwarded proofs with synthetic physical instructions.
//! This is operand-closure evidence, not production publication or tagged ABI.
use super::*;
use crate::mir::CompareOp;

pub(super) fn fixture() -> (FunctionUses, MirFunction, MirInstruction) {
    let text = "box Transport { birth() {} probe(p): i64 { return 0 } forward(q): i64 { local alias = q local recv = new Transport() local out = recv.probe(alias) return 0 } } static box Main { main() { local recv = new Transport() local out = recv.forward(true) return 0 } }";
    let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
        text,
        crate::parser::ParserBuildConfig::default(),
    )
    .unwrap();
    let transformed = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap()
    });
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed else {
        panic!("source-backed");
    };
    let brands = crate::analysis::brand_program_declaration_catalog::issue_brand_program_declaration_catalog_v1(source.ast()).unwrap();
    let source = crate::mir::builder::NormalRootExecutionConsumerV1::consume_once(source)
        .unwrap()
        .into_consumed_source();
    let mut resolver =
        crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1::new(993).unwrap();
    let package = crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_with_brand_catalog_v1(
        &mut resolver, source, Some(&brands),
    ).unwrap();
    let (prepared, row, arguments, ledger, _, copies) =
        crate::mir::builder::lexical_call_projection_forwarded_fixture(package, BasicBlockId(0));
    let owner = row.call_site().owner();
    let call = prepared
        .materialize_with_ledger(owner, &row, &arguments, &ledger)
        .unwrap();
    let entry = ledger.borrowed_ordinary_entry_values_v1(owner).unwrap();
    let mut function = MirFunction::new(
        crate::mir::FunctionSignature {
            name: "Transport.forward/1".into(),
            params: vec![crate::mir::MirType::Unknown],
            return_type: crate::mir::MirType::Integer,
            effects: crate::mir::EffectMask::PURE,
        },
        BasicBlockId(0),
    );
    function.params = entry.iter().map(|(_, _, value)| *value).collect();
    let mut state = FunctionUses::default();
    for (_, formal, value) in entry.iter() {
        state.roots.insert(*value, *formal);
    }
    for (index, original) in copies.iter().enumerate() {
        state
            .copy(original, Some((BasicBlockId(0), index)))
            .unwrap();
        function
            .blocks
            .get_mut(&BasicBlockId(0))
            .unwrap()
            .instructions
            .push(original.1.clone());
    }
    let coordinate = (BasicBlockId(0), copies.len());
    let mut uses = BorrowedCallUses {
        functions: [(function.signature.name.clone(), state)].into(),
    };
    uses.call(
        &function,
        coordinate,
        ledger.borrowed_call_actuals_v1(&row).unwrap().unwrap(),
        &copies
            .iter()
            .enumerate()
            .map(|(index, original)| (original.clone(), (BasicBlockId(0), index)))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    let instruction = MirInstruction::Invoke {
        operation: InvokeOperation::Call {
            call,
            result: InvokeCallResultKind::I64,
        },
        fault_frame: ValueId(900),
        normal_landing: BasicBlockId(1),
        fault_landing: BasicBlockId(2),
    };
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .terminator = Some(instruction.clone());
    (
        uses.functions.remove(&function.signature.name).unwrap(),
        function,
        instruction,
    )
}

fn verify(state: &mut FunctionUses, function: &MirFunction) -> Result<(), String> {
    // The call's expected coordinate follows the terminator, which shifts
    // when tests append instructions before it.
    let index = function.blocks[&BasicBlockId(0)].instructions.len();
    state.arguments = state
        .arguments
        .iter()
        .map(|((block, _, ordinal), formal)| ((*block, index, *ordinal), *formal))
        .collect();
    let tracked = state.tracked()?;
    let mut definitions = Scan::default();
    let mut successors: BTreeMap<BasicBlockId, Vec<BasicBlockId>> = BTreeMap::new();
    let mut indexed = Vec::new();
    for (block, row) in &function.blocks {
        successors
            .entry(*block)
            .or_default()
            .extend(row.successors.iter().copied());
        for (index, instruction) in row.all_instructions().enumerate() {
            indexed.push(((*block, index), instruction));
        }
    }
    let views = state.scan_views(&tracked, &indexed)?;
    let dominates = |from: BasicBlockId, to: BasicBlockId| {
        path_dominates(function.entry_block, &successors, from, to)
    };
    for (coordinate, instruction) in &indexed {
        state.instruction(
            &tracked,
            &views,
            *coordinate,
            instruction,
            &mut definitions,
            &dominates,
        )?;
    }
    state.definitions(&tracked, &function.params, &definitions)
}

#[test]
fn borrowed_use_original_forwarded_copy_and_exact_call_position_pass() {
    let (mut state, function, _) = fixture();
    verify(&mut state, &function).unwrap();
    assert_eq!(state.roots.len(), 1);
    assert_eq!(state.copies.len(), 1);
    assert_eq!(state.arguments.len(), 1);
}

#[test]
fn borrowed_use_rejects_receiver_fault_frame_wrong_ordinal_and_argument_loss() {
    for mutation in 0..4 {
        let (mut state, mut function, mut instruction) = fixture();
        let MirInstruction::Invoke {
            operation: InvokeOperation::Call { call, .. },
            fault_frame,
            ..
        } = &mut instruction
        else {
            panic!("Invoke");
        };
        let value = call.args[0];
        match mutation {
            0 => {
                let Callee::SameModuleInstance { receiver, .. } = &mut call.callee else {
                    panic!("instance");
                };
                *receiver = value;
            }
            1 => *fault_frame = value,
            2 => call.args.insert(0, ValueId(999)),
            _ => call.args.clear(),
        }
        function
            .blocks
            .get_mut(&BasicBlockId(0))
            .unwrap()
            .terminator = Some(instruction);
        assert!(verify(&mut state, &function)
            .unwrap_err()
            .contains("borrowed-use/"));
    }
}

#[test]
fn borrowed_use_rejects_unproved_copy_phi_return_and_edge_transport() {
    for mutation in 0..4 {
        let (mut state, mut function, _) = fixture();
        let value = *state.roots.keys().next().unwrap();
        let instruction = match mutation {
            0 => MirInstruction::Copy {
                dst: ValueId(999),
                src: value,
            },
            1 => MirInstruction::Phi {
                dst: ValueId(999),
                inputs: vec![(BasicBlockId(9), value)],
                type_hint: None,
            },
            2 => MirInstruction::Return { value: Some(value) },
            _ => MirInstruction::Jump {
                target: BasicBlockId(9),
                edge_args: Some(crate::mir::EdgeArgs {
                    layout: crate::mir::edge_args::JumpArgsLayout::CarriersOnly,
                    values: vec![value],
                }),
            },
        };
        function
            .blocks
            .get_mut(&BasicBlockId(0))
            .unwrap()
            .instructions
            .push(instruction);
        assert!(verify(&mut state, &function)
            .unwrap_err()
            .contains("borrowed-use/"));
    }
}

#[test]
fn borrowed_use_rejects_root_copy_definition_and_parameter_drift() {
    for mutation in 0..4 {
        let (mut state, mut function, _) = fixture();
        let value = *state.roots.keys().next().unwrap();
        let copy = state.copies.values().next().unwrap().0 .1.clone();
        match mutation {
            0 => function
                .blocks
                .get_mut(&BasicBlockId(0))
                .unwrap()
                .instructions
                .push(copy),
            1 => function.params.push(copy.dst_value().unwrap()),
            2 => function
                .blocks
                .get_mut(&BasicBlockId(0))
                .unwrap()
                .instructions
                .push(MirInstruction::Const {
                    dst: value,
                    value: crate::mir::ConstValue::Integer(1),
                }),
            _ => function.params.clear(),
        }
        assert!(verify(&mut state, &function)
            .unwrap_err()
            .contains("borrowed-use/"));
    }
}

#[test]
fn borrowed_use_rejects_missing_call_omitted_alias_use_and_duplicate_coordinate() {
    let (mut state, mut function, instruction) = fixture();
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .terminator = Some(MirInstruction::Return { value: None });
    assert!(verify(&mut state, &function)
        .unwrap_err()
        .contains("argument-coverage"));
    let dst = *state.copies.keys().next().unwrap();
    state.copies.get_mut(&dst).unwrap().1 = None;
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .instructions
        .clear();
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .terminator = Some(instruction.clone());
    state.arguments = [(
        (BasicBlockId(0), 0, 0),
        *state.roots.values().next().unwrap(),
    )]
    .into();
    assert!(verify(&mut state, &function)
        .unwrap_err()
        .contains("omitted-copy-argument"));
    let tracked = state.tracked().unwrap();
    let mut scan = Scan::default();
    let views = ViewScan::default();
    let dominates = |_: BasicBlockId, _: BasicBlockId| false;
    let unused = MirInstruction::Return { value: None };
    state
        .instruction(
            &tracked,
            &views,
            (BasicBlockId(0), 0),
            &unused,
            &mut scan,
            &dominates,
        )
        .unwrap();
    assert!(state
        .instruction(
            &tracked,
            &views,
            (BasicBlockId(0), 0),
            &unused,
            &mut scan,
            &dominates,
        )
        .unwrap_err()
        .contains("coordinate-duplicate"));
}

fn compare_fixture() -> (FunctionUses, MirFunction, ValueId, BindingRefV1) {
    let (mut state, mut function, _) = fixture();
    let carrier = *state.roots.keys().next().unwrap();
    let formal = *state.roots.values().next().unwrap();
    state.compare_admissions.insert(formal, 1);
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Copy {
            dst: ValueId(700),
            src: carrier,
        });
    (state, function, ValueId(700), formal)
}

#[test]
fn borrowed_use_checked_compare_view_counts_distinct_operands_once() {
    // The edge-port model may evaluate the same projection twice; the lent
    // view is used once per admitted source operand.
    let (mut state, mut function, view, _) = compare_fixture();
    for (index, dst) in [(0usize, ValueId(701)), (1, ValueId(702))] {
        let _ = index;
        function
            .blocks
            .get_mut(&BasicBlockId(0))
            .unwrap()
            .instructions
            .push(MirInstruction::Compare {
                dst,
                op: CompareOp::Gt,
                lhs: view,
                rhs: ValueId(800),
            });
    }
    verify(&mut state, &function).unwrap();
}

#[test]
fn borrowed_use_rejects_compare_view_escape_and_coverage_drift() {
    // View operand reaching a non-compare instruction.
    let (mut state, mut function, view, _) = compare_fixture();
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .instructions
        .extend([
            MirInstruction::Compare {
                dst: ValueId(701),
                op: CompareOp::Gt,
                lhs: view,
                rhs: ValueId(800),
            },
            MirInstruction::Copy {
                dst: ValueId(703),
                src: view,
            },
        ]);
    assert!(verify(&mut state, &function)
        .unwrap_err()
        .contains("forbidden-operand"));
    // View operand leaving through an edge argument.
    let (mut state, mut function, view, _) = compare_fixture();
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .instructions
        .extend([
            MirInstruction::Compare {
                dst: ValueId(701),
                op: CompareOp::Gt,
                lhs: view,
                rhs: ValueId(800),
            },
            MirInstruction::Jump {
                target: BasicBlockId(9),
                edge_args: Some(crate::mir::EdgeArgs {
                    layout: crate::mir::edge_args::JumpArgsLayout::CarriersOnly,
                    values: vec![view],
                }),
            },
        ]);
    assert!(verify(&mut state, &function)
        .unwrap_err()
        .contains("forbidden-operand"));
    // The admitted use never observed: without the compare the view copy
    // itself is only an unproved carrier copy.
    let (mut state, mut function, _, _) = compare_fixture();
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .instructions
        .pop();
    assert!(verify(&mut state, &function)
        .unwrap_err()
        .contains("compare-coverage"));
    // A physical compare without the source admission.
    let (mut state, mut function, view, formal) = compare_fixture();
    state.compare_admissions.remove(&formal);
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Compare {
            dst: ValueId(701),
            op: CompareOp::Gt,
            lhs: view,
            rhs: ValueId(800),
        });
    assert!(verify(&mut state, &function)
        .unwrap_err()
        .contains("compare-coverage"));
}

#[test]
fn borrowed_use_rejects_view_shape_drift() {
    // A compare operand copied from anything but the tracked carrier is no
    // view; the stray copy of an untracked value stays legal, the admitted
    // use is then missing.
    let (mut state, mut function, _, _) = compare_fixture();
    let block = function.blocks.get_mut(&BasicBlockId(0)).unwrap();
    block.instructions.pop();
    block.instructions.extend([
        MirInstruction::Copy {
            dst: ValueId(700),
            src: ValueId(801),
        },
        MirInstruction::Compare {
            dst: ValueId(701),
            op: CompareOp::Gt,
            lhs: ValueId(700),
            rhs: ValueId(800),
        },
    ]);
    assert!(verify(&mut state, &function)
        .unwrap_err()
        .contains("compare-coverage"));
    // A tracked operand used directly still counts toward the admission.
    let (mut state, mut function, _, _) = compare_fixture();
    let carrier = *state.roots.keys().next().unwrap();
    let block = function.blocks.get_mut(&BasicBlockId(0)).unwrap();
    block.instructions.pop();
    block.instructions.push(MirInstruction::Compare {
        dst: ValueId(701),
        op: CompareOp::Gt,
        lhs: carrier,
        rhs: ValueId(800),
    });
    verify(&mut state, &function).unwrap();
}

#[test]
fn borrowed_use_dominated_add_view_passes() {
    // Same block, ordered after the compare's site check: the lent view
    // serves the ordered `+` operand.
    let (mut state, mut function, view, formal) = compare_fixture();
    state.add_admissions.insert(formal, 1);
    let carrier = *state.roots.keys().next().unwrap();
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .instructions
        .extend([
            MirInstruction::Compare {
                dst: ValueId(701),
                op: CompareOp::Gt,
                lhs: view,
                rhs: ValueId(800),
            },
            MirInstruction::Copy {
                dst: ValueId(704),
                src: carrier,
            },
            MirInstruction::BinOp {
                dst: ValueId(705),
                op: crate::mir::BinaryOp::Add,
                lhs: ValueId(704),
                rhs: ValueId(800),
            },
        ]);
    verify(&mut state, &function).unwrap();
}

#[test]
fn borrowed_use_rejects_undominated_and_drifting_add_view() {
    // Ordered `+` before the compare's site check in the same block.
    let (mut state, mut function, view, formal) = compare_fixture();
    state.add_admissions.insert(formal, 1);
    let carrier = *state.roots.keys().next().unwrap();
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .instructions
        .extend([
            MirInstruction::Copy {
                dst: ValueId(704),
                src: carrier,
            },
            MirInstruction::BinOp {
                dst: ValueId(705),
                op: crate::mir::BinaryOp::Add,
                lhs: ValueId(704),
                rhs: ValueId(800),
            },
            MirInstruction::Compare {
                dst: ValueId(701),
                op: CompareOp::Gt,
                lhs: view,
                rhs: ValueId(800),
            },
        ]);
    assert!(verify(&mut state, &function)
        .unwrap_err()
        .contains("undominated-view"));

    // A compare in a sibling arm never reaches the `+` use.
    let (mut state, mut function, view, formal) = compare_fixture();
    state.add_admissions.insert(formal, 1);
    let carrier = *state.roots.keys().next().unwrap();
    {
        let entry = function.blocks.get_mut(&BasicBlockId(0)).unwrap();
        entry.successors.insert(BasicBlockId(1));
        entry.successors.insert(BasicBlockId(2));
    }
    let mut arm = crate::mir::basic_block::BasicBlock::new(BasicBlockId(1));
    arm.instructions.push(MirInstruction::Compare {
        dst: ValueId(701),
        op: CompareOp::Gt,
        lhs: view,
        rhs: ValueId(800),
    });
    arm.terminator = Some(MirInstruction::Return { value: None });
    function.blocks.insert(BasicBlockId(1), arm);
    let mut other = crate::mir::basic_block::BasicBlock::new(BasicBlockId(2));
    other.instructions.extend([
        MirInstruction::Copy {
            dst: ValueId(704),
            src: carrier,
        },
        MirInstruction::BinOp {
            dst: ValueId(705),
            op: crate::mir::BinaryOp::Add,
            lhs: ValueId(704),
            rhs: ValueId(800),
        },
    ]);
    other.terminator = Some(MirInstruction::Return { value: None });
    function.blocks.insert(BasicBlockId(2), other);
    assert!(verify(&mut state, &function)
        .unwrap_err()
        .contains("undominated-view"));

    // The admitted `+` use never observed: coverage stays per admission.
    let (mut state, mut function, view, formal) = compare_fixture();
    state.add_admissions.insert(formal, 1);
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Compare {
            dst: ValueId(701),
            op: CompareOp::Gt,
            lhs: view,
            rhs: ValueId(800),
        });
    assert!(verify(&mut state, &function)
        .unwrap_err()
        .contains("add-coverage"));

    // A different arithmetic operator is no lent-view site: its carrier
    // copy stays an unproved copy.
    let (mut state, mut function, view, formal) = compare_fixture();
    state.add_admissions.insert(formal, 1);
    let carrier = *state.roots.keys().next().unwrap();
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .instructions
        .extend([
            MirInstruction::Compare {
                dst: ValueId(701),
                op: CompareOp::Gt,
                lhs: view,
                rhs: ValueId(800),
            },
            MirInstruction::Copy {
                dst: ValueId(704),
                src: carrier,
            },
            MirInstruction::BinOp {
                dst: ValueId(705),
                op: crate::mir::BinaryOp::Sub,
                lhs: ValueId(704),
                rhs: ValueId(800),
            },
        ]);
    assert!(verify(&mut state, &function)
        .unwrap_err()
        .contains("unproved-copy"));
}

fn set_write(
    dst: ValueId,
    receiver: ValueId,
    index: ValueId,
    value: ValueId,
) -> MirInstruction {
    MirInstruction::ArrayElementWrite {
        site_id: crate::mir::ArrayWriteSiteId(0),
        dst: Some(dst),
        kind: crate::mir::ArrayElementWriteKind::Set,
        producer: crate::mir::ArrayWriteProducerKind::MethodCall,
        receiver,
        index: Some(index),
        value,
    }
}

#[path = "borrowed_call_uses_ctor_view_tests.rs"]
mod ctor_view_tests;

#[path = "borrowed_call_uses_null_compare_tests.rs"]
mod null_compare_tests;

#[test]
fn borrowed_use_dominated_set_view_passes() {
    // Same block, ordered after the compare's site check: the lent view
    // serves the `.set` element value; receiver and index never carry it.
    let (mut state, mut function, view, formal) = compare_fixture();
    state.set_admissions.insert(formal, 1);
    let carrier = *state.roots.keys().next().unwrap();
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .instructions
        .extend([
            MirInstruction::Compare {
                dst: ValueId(701),
                op: CompareOp::Gt,
                lhs: view,
                rhs: ValueId(800),
            },
            MirInstruction::Copy {
                dst: ValueId(704),
                src: carrier,
            },
            set_write(
                ValueId(706),
                ValueId(810),
                ValueId(811),
                ValueId(704),
            ),
        ]);
    verify(&mut state, &function).unwrap();
    // The tracked carrier itself may serve the element value directly.
    let (mut state, mut function, view, formal) = compare_fixture();
    state.set_admissions.insert(formal, 1);
    let carrier = *state.roots.keys().next().unwrap();
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .instructions
        .extend([
            MirInstruction::Compare {
                dst: ValueId(701),
                op: CompareOp::Gt,
                lhs: view,
                rhs: ValueId(800),
            },
            set_write(ValueId(706), ValueId(810), ValueId(811), carrier),
        ]);
    verify(&mut state, &function).unwrap();
}

#[test]
fn borrowed_use_rejects_undominated_and_drifting_set_view() {
    // Ordered `.set` before the compare's site check in the same block.
    let (mut state, mut function, view, formal) = compare_fixture();
    state.set_admissions.insert(formal, 1);
    let carrier = *state.roots.keys().next().unwrap();
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .instructions
        .extend([
            MirInstruction::Copy {
                dst: ValueId(704),
                src: carrier,
            },
            set_write(ValueId(706), ValueId(810), ValueId(811), ValueId(704)),
            MirInstruction::Compare {
                dst: ValueId(701),
                op: CompareOp::Gt,
                lhs: view,
                rhs: ValueId(800),
            },
        ]);
    assert!(verify(&mut state, &function)
        .unwrap_err()
        .contains("undominated-view"));

    // The admitted `.set` use never observed: coverage stays per admission.
    let (mut state, mut function, view, formal) = compare_fixture();
    state.set_admissions.insert(formal, 1);
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Compare {
            dst: ValueId(701),
            op: CompareOp::Gt,
            lhs: view,
            rhs: ValueId(800),
        });
    assert!(verify(&mut state, &function)
        .unwrap_err()
        .contains("set-coverage"));

    // The lent view may not move into the receiver or index lanes.
    for (receiver, index) in [(true, false), (false, true)] {
        let (mut state, mut function, view, formal) = compare_fixture();
        state.set_admissions.insert(formal, 1);
        let carrier = *state.roots.keys().next().unwrap();
        function
            .blocks
            .get_mut(&BasicBlockId(0))
            .unwrap()
            .instructions
            .extend([
                MirInstruction::Compare {
                    dst: ValueId(701),
                    op: CompareOp::Gt,
                    lhs: view,
                    rhs: ValueId(800),
                },
                set_write(
                    ValueId(706),
                    if receiver { carrier } else { ValueId(810) },
                    if index { carrier } else { ValueId(811) },
                    carrier,
                ),
            ]);
        assert!(verify(&mut state, &function)
            .unwrap_err()
            .contains("forbidden-operand"));
    }
}
