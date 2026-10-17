//! Independent original Print frame / physical membership and topology checks.
use super::*;

#[test]
fn original_print_source_reaches_normal_compiler_and_finished_string_lifetimes() {
    let _ring0 = crate::runtime::ring0::ensure_global_ring0_initialized();
    const DEEP_PROVIDER: &str = "box Leaf { n: i64 birth() { me.n = 7 } }
        box Middle { child: Leaf birth() { me.child = new Leaf() } }
        box Parent { child: Middle birth() { me.child = new Middle() } }
        static box Main { main() { local p = new Parent()
            print(\"n=\" + p.child.child.n) return 0 } }";
    let mut compile_errors = Vec::new();
    crate::test_support::with_env_var("NYASH_MACRO_DERIVE", "", || {
        for (text, additions) in [
            ("static box Main { main() { print(\"猫\") return 0 } }", 0),
            ("static box Main { main() { if (1 == 1) && (2 < 3) { print(\"yes\") return 1 } return 0 } }", 0),
            ("box Counter { n: i64 birth() { me.n = 7 } read() { return me.n } }
              static box Main { main() { local c = new Counter()
                if (c.read() == 7) && (c.read() != 0) { print(\"yes\") return 1 } return 0 } }", 0),
            ("box Counter { n: i64 birth() { me.n = 7 } read() { return me.n } }
              static box Main { main() { local c = new Counter()
                if (c.n > 0) || (c.read() == 7) { print(\"yes\") return 1 } return 0 } }", 0),
            ("box Counter { n: i64 birth() { me.n = 7 } read() { return me.n } }
              static box Main { main() { local c = new Counter()
                if c.read() == 7 { print(\"yes\") return 1 } return 0 } }", 0),
            ("box Counter { n: i64 birth() { me.n = 7 } read() { return me.n } }
              static box Main { main() { local c = new Counter()
                if 7 < c.read() + 1 { print(\"yes\") return 1 } return 0 } }", 0),
            ("box Counter { n: i64 birth() { me.n = 7 } value() { return me.n }
                read() { if me.value() == 7 { return me.n } return me.n } }
              static box Main { main() { local c = new Counter()
                if c.read() == 7 { print(\"yes\") return 1 } return 0 } }", 0),
            ("box Counter { n: i64 birth() { me.n = 7 } value() { return me.n }
                read() { print(\"n=\" + me.value()) return me.n } }
              static box Main { main() { local c = new Counter()
                print(c.read()) return 0 } }", 0),
            ("static box Main { main() { print(\"\") return 0 } }", 0),
            ("static box Main { main() { print(7) return 0 } }", 0),
            ("box Counter { n: i64 birth() { me.n = 7 } }
              box Maker { fetch() { return new Counter() } }
              static box Main { main() { local maker = new Maker() local c = maker.fetch()
                print(\"n=\" + c.n) return 0 } }", 1),
            ("static box Main { main() { print(\"sum=\" + (7 + 9)) return 0 } }", 1),
            ("box Counter { n: i64 other: i64 birth() { me.n = 7 me.other = 0 } }
              static box Main { main() { local c = new Counter()
                print((\"n=\" + c.n) + (\" m=\" + c.other)) return 0 } }", 3),
            ("box Counter { n: i64 birth() { me.n = 7 } read() { return me.n } }
              static box Main { main() { local c = new Counter()
                print((\"n=\" + \"x\") + c.read()) return 0 } }", 2),
            ("box Leaf { n: i64 birth() { me.n = 7 } read() { return me.n } }
              box Parent { child: Leaf birth() { me.child = new Leaf() } }
              static box Main { main() { local p = new Parent() local alias = p.child
                print(\"n=\" + alias.read()) return 0 } }", 1),
            ("box Leaf { n: i64 birth() { me.n = 7 } }
              box Parent { child: Leaf birth() { me.child = new Leaf() } }
              static box Main { main() { local p = new Parent()
                print(\"n=\" + p.child.n) return 0 } }", 1),
            (DEEP_PROVIDER, 1),
        ] {
            let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
                text, crate::parser::ParserBuildConfig::default()).unwrap();
            let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
                crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap() else {
                    panic!("original source authority lost");
                };
            let result = match crate::mir::MirCompiler::with_options(false).compile_normal(
                crate::mir::NormalCompileRequestV1::for_mir_mode_callable_source(source, None, Default::default())) {
                Ok(result) => result,
                Err(error) => { compile_errors.push(format!("original Print failed: {text}\n{error}")); continue; },
            };
            assert!(result.verification_result.is_ok(), "{text}: {:?}", result.verification_result);
            let main = result.module.get_function("main").unwrap();
            let instructions: Vec<_> = main.blocks.values().flat_map(|block| block.all_instructions()).collect();
            assert_eq!(instructions.iter().filter(|row| matches!(row,
                MirInstruction::Invoke { operation: InvokeOperation::String(Operation::Print { .. }), .. })).count(), 1,
                "canonical Print must survive: {text}");
            assert_eq!(instructions.iter().filter(|row| matches!(row,
                MirInstruction::Invoke { operation: InvokeOperation::String(Operation::Add { .. }), .. })).count(), additions);
            assert!(instructions.iter().filter(|row| matches!(row,
                MirInstruction::Invoke { operation: InvokeOperation::String(Operation::End { .. }), .. })).count() >= additions);
            assert!(!instructions.iter().any(|row| matches!(row, MirInstruction::Call(_))), "legacy Call in checked Print module");
        }
        let publish_alias = |text: &str, output: Option<&str>| -> Result<(), String> {
            let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
                text, crate::parser::ParserBuildConfig::default()).map_err(|error| error.to_string())?;
            let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
                crate::r#macro::transform_normal_callable_program_v1(parsed).map_err(|error| format!("{error:?}"))?
                else { return Err("original source lost".into()) };
            let outcome = crate::mir::MirCompiler::with_options(false).compile_normal_with_published(
                crate::mir::NormalCompileRequestV1::for_mir_mode_callable_source(source, None, Default::default()),
                |view, verification| {
                    verification.as_ref().map_err(|errors| format!("{errors:?}"))?;
                    let input = view.issue_lifecycle_physical_abi_input()?;
                    let wire = crate::mir::emit_lifecycle_physical_abi_json(&input)?;
                    let main = view.module().get_function("main").unwrap();
                    if let Some((invoke_block, field, base, normal)) = main.blocks.iter().find_map(|(id, block)|
                        match block.terminator.as_ref() {
                            Some(MirInstruction::Invoke { operation: InvokeOperation::FieldGet { field, base }, normal_landing, .. }) =>
                                Some((*id, *field, *base, *normal_landing)),
                            _ => None,
                        }) {
                        let root = view.retained_root_source().expect("checked Print retains its source");
                        for mutation in ["base", "raw", "normal", "fault"] {
                            let mut changed = view.module().clone();
                            let main = changed.functions.get_mut("main").unwrap();
                            match mutation {
                                "base" => match main.blocks.get_mut(&invoke_block).unwrap().terminator.as_mut().unwrap() {
                                    MirInstruction::Invoke { operation: InvokeOperation::FieldGet { base, .. }, .. } =>
                                        *base = crate::mir::ValueId(99999),
                                    _ => unreachable!(),
                                },
                                "raw" => {
                                    *main.blocks.get_mut(&invoke_block).unwrap().terminator.as_mut().unwrap() =
                                        MirInstruction::Jump { target: normal, edge_args: None };
                                    let projection = main.blocks.get_mut(&normal).unwrap().instructions.iter_mut()
                                        .find(|row| matches!(row, MirInstruction::InvokeNormalResult { invoke_block: source, .. } if *source == invoke_block)).unwrap();
                                    let dst = projection.dst_value().unwrap();
                                    *projection = MirInstruction::ObjectFieldGet { dst, base, field };
                                },
                                "normal" => {
                                    let projection = main.blocks.get_mut(&normal).unwrap().instructions.iter_mut()
                                        .find(|row| matches!(row, MirInstruction::InvokeNormalResult { invoke_block: source, .. } if *source == invoke_block)).unwrap();
                                    if let MirInstruction::InvokeNormalResult { invoke_block, .. } = projection {
                                        *invoke_block = BasicBlockId(99999);
                                    }
                                },
                                "fault" => if let Some(MirInstruction::Invoke { fault_landing, .. }) =
                                    &mut main.blocks.get_mut(&invoke_block).unwrap().terminator { *fault_landing = normal; },
                                _ => unreachable!(),
                            }
                            assert!(root.visit_finalized_lexical_call_nodes_v1(&changed, |_, _, _, _, _, _, _| Ok(())).is_err(),
                                "final checked Print consumer admitted {mutation}");
                        }
                    }
                    if let Some(path) = output { std::fs::write(path, wire).map_err(|error| error.to_string())?; }
                    Ok(())
                })?;
            match outcome {
                crate::mir::NormalPublishedCompileOutcome::Consumed(()) => Ok(()),
                crate::mir::NormalPublishedCompileOutcome::ExplicitCompatibility(_) => Err("no canonical alias artifact".into()),
            }
        };
        publish_alias(DEEP_PROVIDER, Some("/tmp/hako-issued-print-deep-provider.json"))
            .unwrap_or_else(|error| panic!("deep provider Print publication: {error}"));
        publish_alias("box Leaf { n: i64 birth() { me.n = 7 } read() { return me.n } }
            box Parent { child: Leaf birth() { me.child = new Leaf() } }
            static box Main { main() { local p = new Parent() local alias = p.child
                print(\"n=\" + alias.read()) return 0 } }", Some("/tmp/hako-issued-print-alias.json")).unwrap();
        for text in [
            "box Counter { n: i64 birth() { me.n = 7 } read() { return me.n } }
             static box Main { main() { local c = new Counter()
               if c.read() == 7 { print(\"yes\") return 1 } return 0 } }",
            "static box Main { main() { if (1 == 1) && (2 < 3) { print(\"yes\") return 1 } return 0 } }",
            "box Counter { n: i64 birth() { me.n = 7 } read() { return me.n } }
             static box Main { main() { local c = new Counter()
               if (c.read() == 7) && (c.read() != 0) { print(\"yes\") return 1 } return 0 } }",
        ] { publish_alias(text, None).unwrap(); }
        publish_alias("box Leaf { n: i64 birth() { me.n = 7 } }
            box Wrapper { child: Leaf birth(child) { me.child = child } }
            box Maker { make(flag: i64) { if flag == 0 { return null } return new Leaf() }
                wrap(flag: i64) { local h = me.make(flag) return new Wrapper(h) } }
            static box Main { main() { local maker = new Maker() local w = maker.wrap(1)
                print(\"n=\" + w.child.n) return 0 } }", None)
            .unwrap_or_else(|error| panic!("received nested Print publication: {error}"));
        for flag in [0, 1] {
            let text = format!("box Counter {{ n: i64 birth() {{ me.n = 37 }} }}
                box Maker {{ fetch(flag: i64) {{ if flag == 0 {{ return null }} return new Counter() }} }}
                static box Main {{ main() {{ local maker = new Maker() local c = maker.fetch({flag})
                    print(\"n=\" + c.n) return 0 }} }}");
            let path = format!("/tmp/hako-issued-received-field-{flag}.json");
            publish_alias(&text, Some(&path)).unwrap_or_else(|error| panic!("received Print source {flag}: {error}"));
            let wire: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
            let reads = wire["functions"].as_array().unwrap().iter().flat_map(|function| function["blocks"].as_array().unwrap())
                .filter(|block| block["terminator"]["instruction"]["operation"]["kind"] == "field_get").count();
            assert_eq!(reads, 1, "same original source must publish its checked read");
        }
        // A fieldless literal-return method has no existing scalar-result
        // source loan; Bool kind alone must not authorize this call target.
        assert!(publish_alias("box Counter { n: i64 birth() { me.n = 7 } read() { return me.n } ok() { return true } }
            static box Main { main() { local c = new Counter()
              if (c.read() == 7) && c.ok() { print(\"yes\") return 1 } return 0 } }", None).is_err());
        for body in [
            "local p = new Parent() local alias = p.child p = new Parent() print(alias.read())",
            "local p = new Parent() local alias = p.child alias = p.child print(alias.read())",
        ] {
            let text = format!("box Leaf {{ n: i64 birth() {{ me.n = 7 }} read() {{ return me.n }} }}
                box Parent {{ child: Leaf birth() {{ me.child = new Leaf() }} }}
                static box Main {{ main() {{ {body} return 0 }} }}");
            assert!(publish_alias(&text, None).is_err(), "rebound alias/root admitted at publication: {body}");
        }
        let text = "box Leaf { n: i64 birth() { me.n = 7 } read() { return me.n } }
            box Parent { child: Leaf birth() { me.child = new Leaf() }
                replace() { me.child = new Leaf() } }
            static box Main { main() { local p = new Parent() local alias = p.child
                print(alias.read()) return 0 } }";
        assert!(publish_alias(text, None).is_err(), "same-class replacement admitted at publication");
    });
    // The document fixture owns its process-state guard; do not nest it.
        let text = "box Leaf { n: i64 birth() { me.n = 7 } read() { return me.n } }
            box Parent { child: Leaf other: Leaf birth() { me.child = new Leaf() me.other = new Leaf() } }
            static box Main { main() { local p = new Parent() local alias = p.child local sibling = p.other
                print(\"n=\" + alias.read() + \" other=\" + sibling.n) return 0 } }";
        let (completed, ledger) = crate::mir::builder::lexical_call_projection_document_completion_fixture(text);
        let (key, module, same, keys, cohort) = completed.document_preflight_parts_for_test(true).unwrap();
        assert!(Rc::ptr_eq(&same, &ledger));
        let main_key = key.clone();
        let handoff = ledger.seal_finalized_root_birth_handoff(key, &module, &keys, cohort, Vec::new()).unwrap();
        let root = handoff.root_source().unwrap();
        root.visit_finalized_lexical_call_nodes_v1(&module, |_, _, _, _, _, _, _| Ok(())).unwrap();
        let retained: Vec<_> = root.local_calls.get(&root.owner()).into_iter().flatten()
            .flat_map(|group| group.lexical_packets()).collect();
        assert_eq!(retained.len(), 1);
        let original = retained[0];
        let final_context = crate::mir::normal_callable_semantic_package::CallValidationContextV1::Finalized(root);
        final_context.require_ledger(&ledger).unwrap();
        final_context.with_taken_original_row_v1(original.call_site(), |row| {
            assert!(std::ptr::eq(row, original.original_row()?));
            Ok(())
        }).unwrap();
        assert!(crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(&ledger)
            .with_taken_original_row_v1(original.call_site(), |_| Ok(())).is_err(),
            "the consumed ledger pool cannot stand in for the final source owner");
        let foreign = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
            "static box Main { main() { return 0 } }").unwrap();
        assert!(final_context.require_ledger(&foreign.ordinary_new_claim_ledger).is_err());
        let main = module.get_function(&main_key).unwrap();
        let instructions: Vec<_> = main.blocks.values().flat_map(|b| b.all_instructions()).collect();
        let receiver = instructions.iter().find_map(|i| match i {
            MirInstruction::Invoke { operation: InvokeOperation::Call { call, .. }, .. } => match &call.callee {
                crate::mir::definitions::Callee::SameModuleInstance { key, receiver } if key.owner() == "Leaf" => Some(*receiver),
                _ => None,
            }, _ => None,
        }).unwrap();
        let alias_copy = instructions.iter().find_map(|i| match i {
            MirInstruction::Copy { dst, src } if *dst == receiver => Some((**i).clone()), _ => None,
        }).unwrap();
        let MirInstruction::Copy { src: alias_read, .. } = alias_copy else { unreachable!() };
        let alias_field = instructions.iter().find_map(|i| match i {
            MirInstruction::ObjectFieldGet { dst, field, .. } if *dst == alias_read => Some(*field), _ => None,
        }).unwrap();
        let sibling_read = instructions.iter().find_map(|i| match i {
            MirInstruction::ObjectFieldGet { dst, field, .. }
                if field.object() == alias_field.object() && *field != alias_field => Some(*dst), _ => None,
        }).unwrap();
        let sibling = instructions.iter().find_map(|i| match i {
            MirInstruction::Copy { dst, src } if *src == sibling_read => Some(*dst), _ => None,
        }).unwrap();
        assert_ne!(sibling, receiver);
        for mutation in ["receiver", "copy-source", "missing-copy", "duplicate-copy", "read-base"] {
            let mut changed = module.clone();
            let function = changed.functions.get_mut(&main_key).unwrap();
            let mut count = 0;
            for block in function.blocks.values_mut() {
                if mutation == "missing-copy" {
                    block.instructions.retain(|i| { let remove = i == &alias_copy; count += usize::from(remove); !remove });
                    continue;
                }
                if mutation == "duplicate-copy" && block.instructions.contains(&alias_copy) {
                    block.instructions.push(alias_copy.clone()); count += 1; continue;
                }
                for instruction in block.instructions.iter_mut().chain(block.terminator.iter_mut()) {
                    match (mutation, instruction) {
                        ("receiver", MirInstruction::Invoke { operation: InvokeOperation::Call { call, .. }, .. }) => {
                            if let crate::mir::definitions::Callee::SameModuleInstance { receiver: actual, .. } = &mut call.callee {
                                if *actual == receiver { *actual = sibling; count += 1; }
                            }
                        }
                        ("copy-source", MirInstruction::Copy { dst, src }) if *dst == receiver => { *src = sibling_read; count += 1; }
                        ("read-base", MirInstruction::ObjectFieldGet { dst, base, .. }) if *dst == alias_read => { *base = sibling; count += 1; }
                        _ => {}
                    }
                }
            }
            assert_eq!(count, 1, "{mutation}");
            assert!(root.visit_finalized_lexical_call_nodes_v1(&changed, |_, _, _, _, _, _, _| Ok(())).is_err(),
                "final consumer admitted {mutation}");
        }

        let text = "box Pool { size: i64 = 0 birth() {} }
            static box Main { main() { local pool = new Pool()
              if (pool.size == 0) && (pool.size != 1) { return 1 } return 0 } }";
        let (completed, ledger) = crate::mir::builder::lexical_call_projection_document_completion_fixture(text);
        let (key, module, _, keys, cohort) = completed.document_preflight_parts_for_test(true).unwrap();
        let main_key = key.clone();
        let handoff = ledger.seal_finalized_root_birth_handoff(key, &module, &keys, cohort, Vec::new()).unwrap();
        let root = handoff.root_source().unwrap();
        let mut visits = 0;
        root.visit_finalized_lexical_call_nodes_v1(&module, |_, _, _, _, _, _, _| { visits += 1; Ok(()) }).unwrap();
        assert_eq!(visits, 0, "call-free condition still has a final source proof");
        let main = &module.functions[&main_key];
        let mut branches: Vec<_> = main.blocks.iter().filter_map(|(id, block)| match block.terminator.as_ref() {
            Some(row @ MirInstruction::Branch { then_edge_args: None, else_edge_args: None, .. }) => Some((*id, row.clone())),
            _ => None,
        }).collect();
        assert_eq!(branches.len(), 2);
        let lhs = branches.iter().position(|(_, row)| matches!(row,
            MirInstruction::Branch { then_bb, else_bb, .. } if branches.iter()
                .any(|(id, _)| id == then_bb || id == else_bb))).expect("source LHS reaches source RHS branch");
        branches.swap(0, lhs);
        let outer = main.blocks.iter().find_map(|(id, block)| match block.terminator.as_ref() {
            Some(row @ MirInstruction::Branch { then_edge_args: Some(_), .. }) => Some((*id, row.clone())),
            _ => None,
        }).unwrap();
        let result = main.blocks.values().flat_map(|block| &block.instructions).find_map(|row| match row {
            MirInstruction::Phi { dst, inputs, .. } if inputs.len() == 3
                && inputs.iter().all(|(_, value)| main.blocks.values().flat_map(|block| &block.instructions)
                    .any(|row| matches!(row, MirInstruction::Const { dst, value: crate::mir::ConstValue::Bool(_) } if dst == value))) => Some(*dst),
            _ => None,
        }).unwrap();
        let MirInstruction::Branch { condition: outer_condition, .. } = outer.1 else { unreachable!() };
        let field = main.blocks.values().flat_map(|block| &block.instructions)
            .find(|row| matches!(row, MirInstruction::ObjectFieldGet { .. })).unwrap().clone();
        for mutation in ["lhs-direction", "rhs-edge", "phi-value", "phi-pred", "missing-phi", "duplicate-phi",
            "copy-source", "outer-condition", "outer-target", "outer-args", "missing-field", "duplicate-field"] {
            let mut changed = module.clone();
            let main = changed.functions.get_mut(&main_key).unwrap();
            let mut edits = 0;
            if mutation == "lhs-direction" || mutation == "rhs-edge" {
                let target = if mutation == "lhs-direction" { &branches[0] } else { &branches[1] };
                let MirInstruction::Branch { then_bb, else_bb, .. } = main.blocks.get_mut(&target.0)
                    .unwrap().terminator.as_mut().unwrap() else { unreachable!() };
                if mutation == "lhs-direction" { std::mem::swap(then_bb, else_bb); } else { *then_bb = *else_bb; }
                edits += 1;
            } else if mutation.starts_with("outer-") {
                let MirInstruction::Branch { condition, then_bb, else_bb, then_edge_args, .. } = main.blocks
                    .get_mut(&outer.0).unwrap().terminator.as_mut().unwrap() else { unreachable!() };
                match mutation {
                    "outer-condition" => *condition = result,
                    "outer-target" => *then_bb = *else_bb,
                    "outer-args" => *then_edge_args = None,
                    _ => unreachable!(),
                }
                edits += 1;
            } else {
                for block in main.blocks.values_mut() {
                    let mut replacement = Vec::new();
                    for mut row in std::mem::take(&mut block.instructions) {
                        match (mutation, &mut row) {
                            ("missing-field", actual) if *actual == field => { edits += 1; continue; },
                            ("duplicate-field", actual) if *actual == field => { replacement.push(actual.clone()); edits += 1; },
                            ("missing-phi", MirInstruction::Phi { dst, .. }) if *dst == result => { edits += 1; continue; },
                            ("duplicate-phi", actual @ MirInstruction::Phi { .. }) if actual.dst_value() == Some(result) => {
                                replacement.push(actual.clone()); edits += 1;
                            },
                            ("phi-value", MirInstruction::Phi { dst, inputs, .. }) if *dst == result => {
                                inputs[0].1 = inputs[1].1; edits += 1;
                            },
                            ("phi-pred", MirInstruction::Phi { dst, inputs, .. }) if *dst == result => {
                                inputs[0].0 = inputs[1].0; edits += 1;
                            },
                            ("copy-source", MirInstruction::Copy { dst, src }) if *dst == outer_condition => {
                                *src = *dst; edits += 1;
                            },
                            _ => {},
                        }
                        replacement.push(row);
                    }
                    block.instructions = replacement;
                }
            }
            assert_eq!(edits, 1, "one independent final MIR edit: {mutation}");
            assert!(root.visit_finalized_lexical_call_nodes_v1(&changed, |_, _, _, _, _, _, _| Ok(())).is_err(),
                "final source consumer admitted {mutation}");
        }
    assert!(compile_errors.is_empty(), "{}", compile_errors.join("\n\n"));
}

#[test]
fn nested_print_paths_retain_original_fields_and_root_reads_in_final_consumer() {
    let text = "box Leaf { n: i64 birth() { me.n = 7 } }
        box Parent { left: Leaf right: Leaf birth() { me.left = new Leaf() me.right = new Leaf() } }
        static box Main { main() { local p = new Parent()
            print(\"n=\" + p.right.n + \" other=\" + p.left.n) return 0 } }";
    let (completed, ledger) = crate::mir::builder::lexical_call_projection_document_completion_fixture(text);
    let (key, module, _, keys, cohort) = completed.document_preflight_parts_for_test(true).unwrap();
    let main_key = key.clone();
    let handoff = ledger.seal_finalized_root_birth_handoff(key, &module, &keys, cohort, Vec::new()).unwrap();
    let root = handoff.root_source().unwrap();
    root.visit_finalized_lexical_call_nodes_v1(&module, |_, _, _, _, _, _, _| Ok(())).unwrap();
    let main = &module.functions[&main_key];
    let instructions: Vec<_> = main.blocks.values().flat_map(|block| block.all_instructions()).collect();
    assert_eq!(instructions.iter().filter(|row| matches!(row,
        MirInstruction::Invoke { operation: InvokeOperation::String(Operation::Print { .. }), .. })).count(), 1);
    let reads: Vec<_> = instructions.iter().filter_map(|row| match row {
        MirInstruction::ObjectFieldGet { dst, base, field } => Some((*dst, *base, *field)), _ => None,
    }).collect();
    assert_eq!(reads.len(), 4, "two original paths each retain their intermediate and terminal read");
    let &(terminal, parent, _) = reads.iter().find(|(_, base, _)| reads.iter().any(|(dst, _, _)| dst == base)).unwrap();
    let &(_, base, field) = reads.iter().find(|(dst, _, _)| *dst == parent).unwrap();
    let &(sibling, _, sibling_field) = reads.iter().find(|(_, _, other)| other.object() == field.object() && *other != field).unwrap();
    let copy = instructions.iter().find_map(|row| match row {
        MirInstruction::Copy { dst, .. } if *dst == base => Some((**row).clone()), _ => None,
    }).expect("original root lexical read retains its Copy");
    for mutation in ["sibling-field", "terminal-base", "root-base", "missing-intermediate", "duplicate-intermediate",
        "root-copy-source", "missing-root-copy", "duplicate-root-copy"] {
        let mut changed = module.clone();
        let main = changed.functions.get_mut(&main_key).unwrap();
        let mut edits = 0;
        for block in main.blocks.values_mut() {
            let mut replacement = Vec::new();
            for mut instruction in std::mem::take(&mut block.instructions) {
                match (mutation, &mut instruction) {
                    ("sibling-field", MirInstruction::ObjectFieldGet { dst, field, .. }) if *dst == parent => {
                        *field = sibling_field; edits += 1;
                    },
                    ("terminal-base", MirInstruction::ObjectFieldGet { dst, base, .. }) if *dst == terminal => {
                        *base = sibling; edits += 1;
                    },
                    ("root-base", MirInstruction::ObjectFieldGet { dst, base, .. }) if *dst == parent => {
                        *base = sibling; edits += 1;
                    },
                    ("missing-intermediate", MirInstruction::ObjectFieldGet { dst, .. }) if *dst == parent => {
                        edits += 1; continue;
                    },
                    ("duplicate-intermediate", actual @ MirInstruction::ObjectFieldGet { .. }) if actual.dst_value() == Some(parent) => {
                        replacement.push(actual.clone()); edits += 1;
                    },
                    ("root-copy-source", MirInstruction::Copy { dst, src }) if *dst == base => {
                        *src = *dst; edits += 1;
                    },
                    ("missing-root-copy", actual) if *actual == copy => { edits += 1; continue; },
                    ("duplicate-root-copy", actual) if *actual == copy => { replacement.push(actual.clone()); edits += 1; },
                    _ => {},
                }
                replacement.push(instruction);
            }
            block.instructions = replacement;
        }
        assert_eq!(edits, 1, "one independent final MIR mutation: {mutation}");
        assert!(root.visit_finalized_lexical_call_nodes_v1(&changed, |_, _, _, _, _, _, _| Ok(())).is_err(),
            "final source consumer admitted {mutation}");
    }
}

#[test]
fn local_new_stored_print_calls_recheck_root_and_physical_receiver_in_final_consumer() {
    let text = "box Leaf { n: i64 birth() { me.n = 7 } read() { return me.n } }
        box Parent { left: Leaf right: Leaf birth() { me.left = new Leaf() me.right = new Leaf() } }
        static box Main { main() { local p = new Parent() local q = new Parent()
            print(\"n=\" + p.left.read() + \" other=\" + q.right.read()) return 0 } }";
    let (completed, ledger) = crate::mir::builder::lexical_call_projection_document_completion_fixture(text);
    let (key, module, _, keys, cohort) = completed.document_preflight_parts_for_test(true).unwrap();
    let main_key = key.clone();
    let handoff = ledger.seal_finalized_root_birth_handoff(key, &module, &keys, cohort, Vec::new()).unwrap();
    let root = handoff.root_source().unwrap();
    let mut visits = 0;
    root.visit_finalized_lexical_call_nodes_v1(&module, |_, _, _, _, _, _, _| { visits += 1; Ok(()) }).unwrap();
    assert_eq!(visits, 2, "both original stored scalar calls reach the independent final consumer");
    let main = &module.functions[&main_key];
    let instructions: Vec<_> = main.blocks.values().flat_map(|block| block.all_instructions()).collect();
    let reads: Vec<_> = instructions.iter().filter_map(|row| match row {
        MirInstruction::ObjectFieldGet { dst, base, field } => Some((*dst, *base, *field)), _ => None,
    }).collect();
    assert_eq!(reads.len(), 2);
    let (receiver, parent, field) = reads[0];
    let (sibling, other_parent, other_field) = reads[1];
    let root_copy = instructions.iter().find_map(|row| match row {
        MirInstruction::Copy { dst, .. } if *dst == parent => Some((**row).clone()), _ => None,
    }).unwrap();
    let other_initializer = instructions.iter().find_map(|row| match row {
        MirInstruction::Copy { dst, src } if *dst == other_parent => Some(*src), _ => None,
    }).unwrap();
    assert_eq!(field.object(), other_field.object());
    assert_ne!(field, other_field);
    for mutation in ["sibling-field", "parent-value", "call-receiver", "root-copy-source",
        "missing-root-copy", "duplicate-root-copy", "missing-read", "duplicate-read"] {
        let mut changed = module.clone();
        let main = changed.functions.get_mut(&main_key).unwrap();
        let mut edits = 0;
        for block in main.blocks.values_mut() {
            let mut replacement = Vec::new();
            for mut row in std::mem::take(&mut block.instructions) {
                match (mutation, &mut row) {
                    ("sibling-field", MirInstruction::ObjectFieldGet { dst, field, .. }) if *dst == receiver => { *field = other_field; edits += 1; },
                    ("parent-value", MirInstruction::ObjectFieldGet { dst, base, .. }) if *dst == receiver => { *base = other_parent; edits += 1; },
                    ("root-copy-source", MirInstruction::Copy { dst, src }) if *dst == parent => { *src = other_initializer; edits += 1; },
                    ("missing-root-copy", actual) if *actual == root_copy => { edits += 1; continue; },
                    ("duplicate-root-copy", actual) if *actual == root_copy => { replacement.push(actual.clone()); edits += 1; },
                    ("missing-read", MirInstruction::ObjectFieldGet { dst, .. }) if *dst == receiver => { edits += 1; continue; },
                    ("duplicate-read", actual @ MirInstruction::ObjectFieldGet { .. }) if actual.dst_value() == Some(receiver) => {
                        replacement.push(actual.clone()); edits += 1;
                    },
                    _ => {},
                }
                replacement.push(row);
            }
            block.instructions = replacement;
            if mutation == "call-receiver" {
                if let Some(MirInstruction::Invoke { operation: InvokeOperation::Call { call, .. }, .. }) = block.terminator.as_mut() {
                    if let crate::mir::definitions::Callee::SameModuleInstance { receiver: actual, .. } = &mut call.callee {
                        if *actual == receiver { *actual = sibling; edits += 1; }
                    }
                }
            }
        }
        assert_eq!(edits, 1, "one independent final edit: {mutation}");
        assert!(root.visit_finalized_lexical_call_nodes_v1(&changed, |_, _, _, _, _, _, _| Ok(())).is_err(), "admitted {mutation}");
    }
}

#[test]
fn literal_print_retains_original_frame_and_requires_physical_membership_without_a_call() {
    for integer in [false, true] {
        let text = if integer {
            "static box Main { main() { print(7) return 0 } }"
        } else {
            "static box Main { main() { print(\"猫\") return 0 } }"
        };
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(text).unwrap();
        let operand = if integer {
            Operand::NormalInteger(ValueId(85))
        } else {
            Operand::StaticText { utf8: "猫".into() }
        };
        let ledger = &package.ordinary_new_claim_ledger;
        let owner = ledger.root_owner().unwrap();
        let declaration = package
            .batch()
            .declarations()
            .find(|row| row.owner() == owner)
            .unwrap();
        let frame = package.batch().with_lowering_input(declaration.batch_slot(), |input| {
            let shape = input.body_shape().unwrap();
            let statement = shape.statements().iter().find(|row|
                matches!(row, crate::mir::resolved_semantics::BodyStatementShapeV1::Print { .. })).unwrap();
            let source_ledger = input.forest().callable_source_ledger(owner).unwrap();
            let source = crate::mir::resolved_value_profile::expression_plan::statement_expression_source_plan_v1(
                &source_ledger, shape, statement.site()).unwrap().unwrap();
            let plan = crate::mir::resolved_semantics::home_new_prefix::qualify_print_expression_v1::<()>(
                source, &[], &mut |_| None, &mut |_| Ok(None), &mut |_| true,
                &mut |fields, _| { assert!(fields.is_empty()); Ok(true) }).unwrap().unwrap();
            Rc::clone(plan.print_frame().unwrap())
        }).unwrap();
        assert!(frame.calls().is_empty());
        assert!(frame.owned_result_nodes().is_empty());
        let print = (
            BasicBlockId(0),
            MirInstruction::Invoke {
                operation: InvokeOperation::String(Operation::Print {
                    value: operand.clone(),
                }),
                fault_frame: ValueId(90),
                normal_landing: BasicBlockId(1),
                fault_landing: BasicBlockId(2),
            },
        );
        let continuation = (
            BasicBlockId(1),
            MirInstruction::Jump {
                target: BasicBlockId(3),
                edge_args: None,
            },
        );
        let frame_enter = (
            BasicBlockId(0),
            MirInstruction::FaultFrameEnter {
                dst: ValueId(90),
                mode: crate::mir::instruction::FaultFrameMode::RootOwned,
            },
        );
        let fault = (
            BasicBlockId(2),
            MirInstruction::ReturnFault {
                fault_frame: ValueId(90),
            },
        );
        let frame_enter = (BasicBlockId(10), frame_enter.1);
        let entry = (
            BasicBlockId(10),
            MirInstruction::Jump {
                target: BasicBlockId(0),
                edge_args: None,
            },
        );
        let mut bindings = vec![
            frame_enter,
            entry.clone(),
            print.clone(),
            continuation.clone(),
            fault,
        ];
        let scalar = (
            BasicBlockId(0),
            MirInstruction::Const {
                dst: ValueId(85),
                value: crate::mir::ConstValue::Integer(7),
            },
        );
        if integer {
            bindings.insert(2, scalar.clone());
        }
        let mut function = MirFunction::new(
            crate::mir::FunctionSignature {
                name: "literal-print-correspondence".into(),
                params: vec![],
                return_type: crate::mir::MirType::Integer,
                effects: crate::mir::EffectMask::PURE,
            },
            BasicBlockId(10),
        );
        for (id, instruction) in &bindings {
            let block = function
                .blocks
                .entry(*id)
                .or_insert_with(|| crate::mir::BasicBlock::new(*id));
            if matches!(
                instruction,
                MirInstruction::Invoke { .. }
                    | MirInstruction::Jump { .. }
                    | MirInstruction::ReturnFault { .. }
            ) {
                block.set_terminator(instruction.clone());
            } else {
                block.add_instruction(instruction.clone());
            }
        }
        let mut after = crate::mir::BasicBlock::new(BasicBlockId(3));
        after.add_instruction(MirInstruction::Const {
            dst: ValueId(91),
            value: crate::mir::ConstValue::Integer(0),
        });
        after.set_terminator(MirInstruction::Return {
            value: Some(ValueId(91)),
        });
        function.add_block(after);
        let values = vec![Some(operand)];
        let packet = Rc::new(
            EmittedPrintExpressionProjectionV1::new(
                Rc::clone(&frame),
                values.clone(),
                vec![],
                vec![],
                Default::default(),
                entry.clone(),
                print.clone(),
                continuation.clone(),
                bindings.clone(),
                ledger,
                &function,
            )
            .unwrap(),
        );
        let group =
            RootLifecyclePrefixBindingGroupV1::new_print(Rc::clone(&packet), bindings.clone())
                .unwrap();
        let expected = LifecyclePrefixSourceV1::PrintExpression {
            site: group.site().clone(),
            frame: Rc::clone(&frame),
        };
        assert!(group.matches_source(&expected));
        assert!(group.clone().matches_source(&expected));
        assert!(group
            .with_bindings(bindings.clone())
            .matches_source(&expected));
        assert!(group.lexical().is_none());
        assert_eq!(group.lexical_packets().count(), 0);
        let impostor =
            RootLifecyclePrefixBindingGroupV1::new(group.site().clone(), bindings.clone(), None)
                .unwrap();
        assert!(
            !impostor.matches_source(&expected),
            "a Call-shaped group cannot consume a Print frame"
        );
        let rebuilt = LifecyclePrefixSourceV1::PrintExpression {
            site: group.site().clone(),
            frame: Rc::new((*frame).clone()),
        };
        assert!(
            !group.matches_source(&rebuilt),
            "equal facts cannot replace the original frame Rc"
        );
        for retained in [
            vec![],
            vec![print.clone()],
            vec![continuation.clone()],
            vec![print.clone(), print.clone(), continuation.clone()],
        ] {
            assert!(
                RootLifecyclePrefixBindingGroupV1::new_print(Rc::clone(&packet), retained).is_err()
            );
        }
        assert!(EmittedPrintExpressionProjectionV1::new(
            frame,
            vec![Some(Operand::StaticText { utf8: "別".into() })],
            vec![],
            vec![],
            Default::default(),
            entry.clone(),
            print.clone(),
            continuation.clone(),
            bindings.clone(),
            ledger,
            &function
        )
        .is_err());
        let mut injected = function.clone();
        injected
            .blocks
            .get_mut(&BasicBlockId(0))
            .unwrap()
            .add_instruction(MirInstruction::Const {
                dst: ValueId(95),
                value: crate::mir::ConstValue::Integer(123),
            });
        assert!(
            EmittedPrintExpressionProjectionV1::new(
                Rc::clone(packet.frame()),
                values.clone(),
                vec![],
                vec![],
                Default::default(),
                entry.clone(),
                print.clone(),
                continuation.clone(),
                bindings.clone(),
                ledger,
                &injected
            )
            .is_err(),
            "unowned instruction inside expression scope"
        );
        if integer {
            let mut outside = function.clone();
            outside
                .blocks
                .get_mut(&BasicBlockId(0))
                .unwrap()
                .instructions
                .retain(|row| row != &scalar.1);
            outside
                .blocks
                .get_mut(&BasicBlockId(10))
                .unwrap()
                .add_instruction(scalar.1.clone());
            let mut observed = bindings.clone();
            let original = observed
                .iter_mut()
                .find(|binding| **binding == scalar)
                .unwrap();
            original.0 = BasicBlockId(10);
            let error = EmittedPrintExpressionProjectionV1::new(
                Rc::clone(packet.frame()),
                values.clone(),
                vec![],
                vec![],
                Default::default(),
                entry.clone(),
                print.clone(),
                continuation.clone(),
                observed,
                ledger,
                &outside,
            )
            .unwrap_err();
            assert!(
                error.contains("scope-disconnected"),
                "producer before expression entry: {error}"
            );
        }
        for mutation in 0..3 {
            let mut changed = function.clone();
            if mutation < 2 {
                let MirInstruction::Invoke {
                    normal_landing,
                    fault_frame,
                    ..
                } = changed
                    .blocks
                    .get_mut(&BasicBlockId(0))
                    .unwrap()
                    .terminator
                    .as_mut()
                    .unwrap()
                else {
                    unreachable!()
                };
                if mutation == 0 {
                    *normal_landing = BasicBlockId(3);
                } else {
                    *fault_frame = ValueId(92);
                }
            } else {
                changed
                    .blocks
                    .get_mut(&BasicBlockId(2))
                    .unwrap()
                    .set_terminator(MirInstruction::Return { value: None });
            }
            assert!(
                packet.validate_paths(ledger, &changed, None).is_err(),
                "topology drift {mutation}"
            );
        }
    }
}
