//! Original source/entry/Forwarded proofs with synthetic physical instructions.
//! This is operand-closure evidence, not production publication or tagged ABI.
use super::*;

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

fn verify(state: &FunctionUses, function: &MirFunction) -> Result<(), String> {
    let tracked = state.tracked()?;
    let mut definitions = Scan::default();
    for (block, row) in &function.blocks {
        for (index, instruction) in row.all_instructions().enumerate() {
            state.instruction(&tracked, (*block, index), instruction, &mut definitions)?;
        }
    }
    state.definitions(&tracked, &function.params, &definitions)
}

#[test]
fn borrowed_use_original_forwarded_copy_and_exact_call_position_pass() {
    let (state, function, _) = fixture();
    verify(&state, &function).unwrap();
    assert_eq!(state.roots.len(), 1);
    assert_eq!(state.copies.len(), 1);
    assert_eq!(state.arguments.len(), 1);
}

#[test]
fn borrowed_use_rejects_receiver_fault_frame_wrong_ordinal_and_argument_loss() {
    for mutation in 0..4 {
        let (state, mut function, mut instruction) = fixture();
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
        assert!(verify(&state, &function)
            .unwrap_err()
            .contains("borrowed-use/"));
    }
}

#[test]
fn borrowed_use_rejects_unproved_copy_phi_return_and_edge_transport() {
    for mutation in 0..4 {
        let (state, mut function, _) = fixture();
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
        assert!(verify(&state, &function)
            .unwrap_err()
            .contains("borrowed-use/"));
    }
}

#[test]
fn borrowed_use_rejects_root_copy_definition_and_parameter_drift() {
    for mutation in 0..4 {
        let (state, mut function, _) = fixture();
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
        assert!(verify(&state, &function)
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
    assert!(verify(&state, &function)
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
    assert!(verify(&state, &function)
        .unwrap_err()
        .contains("omitted-copy-argument"));
    let tracked = state.tracked().unwrap();
    let mut scan = Scan::default();
    let unused = MirInstruction::Return { value: None };
    state
        .instruction(&tracked, (BasicBlockId(0), 0), &unused, &mut scan)
        .unwrap();
    assert!(state
        .instruction(&tracked, (BasicBlockId(0), 0), &unused, &mut scan)
        .unwrap_err()
        .contains("coordinate-duplicate"));
}
