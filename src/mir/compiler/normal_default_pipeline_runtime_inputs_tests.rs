//! Existing ingress test owner, including source-backed wrapper lifecycle.
use super::*;
use crate::mir::{MirInstruction, ValueId};

#[test]
fn normal_ingress_snapshots_runtime_inputs_permissively_at_compile_time() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = "static box Main { main(args) { return 0 } }";
    let request = crate::test_support::with_env_vars(
        &[
            ("NYASH_SCRIPT_ARGS_JSON", Some(r#"["request-time"]"#)),
            ("HAKO_SCRIPT_ARGS_JSON", None),
            ("NYASH_BUILDER_SAFEPOINT_ENTRY", Some("0")),
        ],
        || {
            let ast = NyashParser::parse_from_string(source).expect("App source");
            NormalCompileRequestV1::for_mir_mode(ast, None, HashMap::new()).expect("App request")
        },
    );
    let selected = crate::test_support::with_env_vars(
        &[
            ("NYASH_SCRIPT_ARGS_JSON", Some(r#"["ingress-time"]"#)),
            ("HAKO_SCRIPT_ARGS_JSON", Some(r#"["must-not-win"]"#)),
            ("NYASH_BUILDER_SAFEPOINT_ENTRY", Some("On")),
        ],
        || {
            MirCompiler::with_options(false)
                .compile_normal(request)
                .expect("normal compile must snapshot at compile ingress")
        },
    );
    let selected_dump = MirPrinter::new().print_module(&selected.module);
    assert!(selected_dump.contains("safepoint"), "{selected_dump}");
    assert!(selected_dump.contains("ingress-time"), "{selected_dump}");
    assert!(!selected_dump.contains("request-time"), "{selected_dump}");
    assert!(!selected_dump.contains("must-not-win"), "{selected_dump}");

    let malformed = crate::test_support::with_env_vars(
        &[
            ("NYASH_SCRIPT_ARGS_JSON", Some("{malformed}")),
            ("HAKO_SCRIPT_ARGS_JSON", Some(r#"["must-stay-masked"]"#)),
            ("NYASH_BUILDER_SAFEPOINT_ENTRY", Some(" maybe ")),
        ],
        || {
            let ast = NyashParser::parse_from_string(source).expect("App source");
            MirCompiler::with_options(false)
                .compile_normal(
                    NormalCompileRequestV1::for_mir_mode(ast, None, HashMap::new())
                        .expect("App request"),
                )
                .expect("malformed normal inputs remain permissive")
        },
    );
    let malformed_dump = MirPrinter::new().print_module(&malformed.module);
    assert!(!malformed_dump.contains("safepoint"), "{malformed_dump}");
    assert!(
        !malformed_dump.contains("must-stay-masked"),
        "{malformed_dump}"
    );

    let hako_fallback = crate::test_support::with_env_vars(
        &[
            ("NYASH_SCRIPT_ARGS_JSON", None),
            ("HAKO_SCRIPT_ARGS_JSON", Some(r#"["hako-fallback"]"#)),
            ("NYASH_BUILDER_SAFEPOINT_ENTRY", Some("off")),
        ],
        || {
            let ast = NyashParser::parse_from_string(source).expect("App source");
            MirCompiler::with_options(false)
                .compile_normal(
                    NormalCompileRequestV1::for_mir_mode(ast, None, HashMap::new())
                        .expect("App request"),
                )
                .expect("HAKO fallback must remain normal-compatible")
        },
    );
    let hako_dump = MirPrinter::new().print_module(&hako_fallback.module);
    assert!(hako_dump.contains("hako-fallback"), "{hako_dump}");
    assert!(!hako_dump.contains("safepoint"), "{hako_dump}");
    // The source-backed artifact route retains the same ingress payload and
    // owns the wrapper residence independently of source-local Homes.
    for optimize in [false, true] {
        for artifact_source in [source, "static box Main { main(other, args) { return 0 } }",
            "box Token {} static box Main { main(args) { local token = new Token() print(\"argv-body\") return 0 } }"] {
            for snapshot in [r#"[]"#, r#"["alpha","猫","x\u0000y",""]"#] {
                crate::test_support::with_env_vars(
                    &[
                        ("NYASH_MACRO_DISABLE", Some("1")),
                        ("NYASH_SCRIPT_ARGS_JSON", Some(snapshot)),
                        ("HAKO_SCRIPT_ARGS_JSON", None),
                    ],
                    || {
                        let mut compiler = MirCompiler::with_options(optimize);
                        compiler.compile_normal_with_published(published_request(artifact_source), |view, verdict| {
                assert!(verdict.is_ok(), "{verdict:?}");
                let input = view.issue_lifecycle_physical_abi_input()?;
                let json = published_backend_view::emit_lifecycle_physical_abi_json(&input)?;
                let wire: serde_json::Value = serde_json::from_str(&json).unwrap();
                let root = view.retained_root().expect("same original Main");
                let source = view.retained_root_source().expect("original entry source");
                let rows = source.main_arguments_bindings_v1(view.module(), root)?;
                assert!(rows.iter().any(|(_, row)| matches!(row,
                    MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::ScriptArgumentsNew { values }, .. }
                    if serde_json::to_value(values).unwrap() == serde_json::from_str::<serde_json::Value>(snapshot).unwrap())));
                assert!(wire.to_string().contains("script_arguments_new"));
                let file = match (artifact_source.contains("argv-body"), snapshot == "[]") {
                    (false, true) => "hako-issued-main-arguments-empty.json",
                    (false, false) => "hako-issued-main-arguments-text.json",
                    (true, true) => "hako-issued-main-arguments-body-empty.json",
                    (true, false) => "hako-issued-main-arguments-body-text.json",
                };
                std::fs::write(std::env::temp_dir().join(file), &json).unwrap();
                for drift in 0..10 {
                    let mut changed = view.module().clone();
                    let body = changed.functions.get_mut(&root.signature.name).unwrap();
                    match drift {
                        0 => {
                            let fault = body.blocks.values().find_map(|block| match block.terminator.as_ref() {
                                Some(MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::ScriptArgumentsNew { .. }, fault_landing, .. }) => Some(*fault_landing), _ => None,
                            }).unwrap();
                            body.blocks.get_mut(&fault).unwrap().terminator = Some(MirInstruction::Return { value: None });
                        }
                        1 => for block in body.blocks.values_mut() { block.instructions.retain(|row|
                            !matches!(row, MirInstruction::ArrayResidenceRelease { .. })); },
                        2 => for block in body.blocks.values_mut() {
                            if let Some(MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::ScriptArgumentsNew { values }, .. }) = block.terminator.as_mut() {
                                *values = vec!["foreign".into()].into();
                            }
                        },
                        3 => {
                            let (_, block) = body.blocks.iter_mut().find(|(_, block)| block.instructions.iter()
                                .any(|row| matches!(row, MirInstruction::ArrayResidenceRelease { .. }))).unwrap();
                            let index = block.instructions.len() - 1;
                            let MirInstruction::ArrayResidenceRelease { value } = block.instructions[index] else { unreachable!() };
                            let alias = ValueId::new(900_000);
                            block.instructions.insert(index, MirInstruction::Copy { dst: alias, src: value });
                            block.instructions[index + 1] = MirInstruction::ArrayResidenceRelease { value: alias };
                        }
                        4 => {
                            let normal = body.blocks.values().find_map(|block| match block.terminator.as_ref() {
                                Some(MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::ScriptArgumentsNew { .. }, normal_landing, .. }) => Some(*normal_landing), _ => None,
                            }).unwrap();
                            body.entry_block = normal;
                        }
                        5 => for block in body.blocks.values_mut() { for row in &mut block.instructions {
                            if let MirInstruction::InvokeNormalResult { dst, .. } = row { *dst = ValueId::new(900_001); }
                        }},
                        6 => {
                            let block = body.blocks.values_mut().find(|block| block.instructions.iter()
                                .any(|row| matches!(row, MirInstruction::ArrayResidenceRelease { .. }))).unwrap();
                            let index = block.instructions.len() - 1;
                            let MirInstruction::ArrayResidenceRelease { value } = block.instructions[index] else { unreachable!() };
                            block.instructions.insert(index, MirInstruction::DestroyOwned { value });
                        }
                        7 => {
                            use crate::mir::{BasicBlock, BasicBlockId, ConstValue};
                            let block = body.blocks.values_mut().find(|block| block.instructions.iter()
                                .any(|row| matches!(row, MirInstruction::ArrayResidenceRelease { .. }))).unwrap();
                            let MirInstruction::ArrayResidenceRelease { value } = block.instructions.pop().unwrap() else { unreachable!() };
                            let (left, right, merge) = (BasicBlockId::new(900010), BasicBlockId::new(900011), BasicBlockId::new(900012));
                            let (condition, null, alias) = (ValueId::new(900010), ValueId::new(900011), ValueId::new(900012));
                            block.instructions.push(MirInstruction::Const { dst: condition, value: ConstValue::Bool(true) });
                            block.terminator = Some(MirInstruction::Branch { condition, then_bb: left, else_bb: right,
                                then_edge_args: None, else_edge_args: None });
                            let mut l = BasicBlock::new(left);
                            l.set_terminator(MirInstruction::Jump { target: merge, edge_args: None });
                            let mut r = BasicBlock::new(right);
                            r.add_instruction(MirInstruction::Const { dst: null, value: ConstValue::Null });
                            r.set_terminator(MirInstruction::Jump { target: merge, edge_args: None });
                            let mut m = BasicBlock::new(merge);
                            m.add_instruction(MirInstruction::Phi { dst: alias, inputs: vec![(left, value), (right, null)], type_hint: None });
                            m.add_instruction(MirInstruction::ArrayResidenceRelease { value });
                            m.set_terminator(MirInstruction::Return { value: Some(alias) });
                            body.add_block(l); body.add_block(r); body.add_block(m);
                        }
                        8 => {
                            let block = body.blocks.values_mut().find(|block| block.instructions.iter()
                                .any(|row| matches!(row, MirInstruction::ArrayResidenceRelease { .. }))).unwrap();
                            let release = block.instructions.last().unwrap().clone();
                            block.add_instruction_before_terminator(release);
                        }
                        _ => {
                            let release = body.blocks.values().flat_map(|block| block.instructions.iter())
                                .find(|row| matches!(row, MirInstruction::ArrayResidenceRelease { .. })).unwrap().clone();
                            let mut detached = crate::mir::BasicBlock::new(crate::mir::BasicBlockId::new(900020));
                            detached.add_instruction(release);
                            detached.set_terminator(MirInstruction::Return { value: None });
                            body.add_block(detached);
                        }
                    }
                    assert!(source.main_arguments_bindings_v1(&changed, &changed.functions[&root.signature.name]).is_err(),
                        "wrapper source drift{drift} must refuse");
                }
                Ok(())
            }).unwrap_or_else(|error| panic!("original argv source reaches final physical JSON: optimize={optimize} source={artifact_source} snapshot={snapshot}: {error:?}"));
                    },
                );
            }
        }
    }
}
