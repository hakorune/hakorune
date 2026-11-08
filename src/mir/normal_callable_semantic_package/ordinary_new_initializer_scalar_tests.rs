//! Current scalar provenance and Loop basis cases for the initializer family.
//! Named test entrypoints stay in the parent; only their scalar responsibility moves.
use super::*;

pub(super) fn initializer_core_i64_reads_require_original_producers_in_the_real_home_walk() {
    use crate::mir::resolved_semantics::home_new_prefix::{
        CheckedExpressionReferenceV1 as Reference, CurrentScalarProducerV1 as Producer,
    };
    for actual_input in ["\"bytes\"", "true", "1"] {
        for (prefix, operand, annotation, expected) in [
            ("", "count", ": i64", Some("entry")),
            ("local alias = count", "alias", ": i64", Some("entry")),
            ("local start = 1", "start", ": i64", Some("literal")),
            (
                "local start = 1 local alias = start",
                "alias",
                ": i64",
                Some("literal"),
            ),
            (
                "local start = Values.find()",
                "start",
                ": i64",
                Some("call"),
            ),
            (
                "local start = Values.find() local alias = start",
                "alias",
                ": i64",
                Some("call"),
            ),
            (
                "local start = line.length()",
                "start",
                ": i64",
                Some("initializer"),
            ),
            (
                "local start = line.substring(0, 1).length()",
                "start",
                ": i64",
                Some("initializer"),
            ),
            (
                "local start = count + line.length()",
                "start",
                ": i64",
                Some("initializer"),
            ),
            (
                "local start = count + line.length() local alias = start",
                "alias",
                ": i64",
                Some("initializer"),
            ),
            (
                "local start = 1 start = 2",
                "start",
                ": i64",
                Some("rebind"),
            ),
            (
                "local start = 1 if count > 0 { start = 2 }",
                "start",
                ": i64",
                Some("branch"),
            ),
            (
                "local start = 1 loop count > 0 { start = start + 1 break }",
                "start",
                ": i64",
                None,
            ),
            ("", "count", ": usize", None),
        ] {
            let text = format!(
                r#"static box Values {{
            find() {{ return 1 }}
            caller(line, count{annotation}) {{ {prefix} local out = {operand} + line.length() return 0 }}
            seed() {{ return Values.caller({actual_input}, 1) }}
        }} static box Main {{ main() {{ return Values.seed() }} }}"#
            );
            let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog_selection(
            &text, crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable)
            .unwrap_or_else(|error| panic!("{text}: {error:?}"));
            let ledger = &package.ordinary_new_claim_ledger;
            let contract = package
                .parameter_contracts
                .iter()
                .find(|row| row.parameters.len() == 2)
                .unwrap();
            package
                .batch()
                .with_lowering_input(contract.batch_slot, |input| {
                    let initializer = input
                        .function()
                        .expression_source()
                        .initializers()
                        .last()
                        .unwrap();
                    let flow = ledger
                        .completion_for_owner(input.owner())
                        .and_then(|completion| completion.cleanup().root_flow());
                    let frame = flow.and_then(|flow| {
                        flow.expression_plans()
                            .iter()
                            .filter_map(|plan| plan.initializer_frame())
                            .find(|frame| frame.binding() == initializer.binding())
                    });
                    assert_eq!(
                        frame.is_some(),
                        expected.is_some() && actual_input == "\"bytes\"",
                        "{prefix}/{operand}/{annotation}/{actual_input}; completion={:?}",
                        ledger.completion_for_owner(input.owner())
                    );
                    let Some(frame) = frame else { return };
                    assert!(frame.is_i64());
                    assert!(!frame.is_text_residence());
                    assert!(!frame.owns_root());
                    assert_eq!(frame.evaluation().cores().len(), 1);
                    assert_eq!(
                        frame.entry_reads().len(),
                        1,
                        "the Core receiver remains a mandatory original Text read"
                    );
                    let selected = ledger
                        .initializer_expression_frame_v1(
                            input.owner(),
                            frame.source().root().statement().node(),
                        )
                        .unwrap()
                        .unwrap();
                    assert!(Rc::ptr_eq(frame, &selected));
                    let reads: Vec<_> = frame
                        .evaluation()
                        .references()
                        .values()
                        .filter_map(|proof| match proof {
                            Reference::Scalar(read) => read.original_read(),
                            _ => None,
                        })
                        .collect();
                    assert_eq!(reads.len(), 1);
                    let producer = reads[0].producer();
                    let actual = match producer {
                        Producer::SignedEntry(root) => {
                            assert_eq!(*root, contract.parameters[1].binding);
                            "entry"
                        }
                        Producer::FieldRead(_) => {
                            panic!("this fixture reads a formal scalar, not a field")
                        }
                        Producer::BindingRebind(original) => {
                            let retained = flow
                                .unwrap()
                                .expression_plans()
                                .iter()
                                .filter_map(|plan| plan.rebind_frame())
                                .find(|frame| {
                                    frame.source().root().statement()
                                        == original.source().root().statement()
                                })
                                .unwrap();
                            assert!(Rc::ptr_eq(original, retained));
                            "rebind"
                        }
                        Producer::BranchNormal(basis) => {
                            assert!(basis.corroborates_original());
                            "branch"
                        }
                        Producer::LoopHeader(_) | Producer::LoopAfter { .. } => {
                            panic!("this fixture has no Loop")
                        }
                        Producer::Integer { value, .. } => {
                            assert_eq!(*value, 1);
                            "literal"
                        }
                        Producer::Call(call) => {
                            assert!(flow.unwrap().local_calls().contains(call));
                            assert!(call.local_binding().is_some());
                            "call"
                        }
                        Producer::Initializer(original) => {
                            let retained = flow
                                .unwrap()
                                .expression_plans()
                                .iter()
                                .filter_map(|plan| plan.initializer_frame())
                                .find(|frame| frame.binding() == original.binding())
                                .unwrap();
                            assert!(Rc::ptr_eq(original, retained));
                            "initializer"
                        }
                    };
                    assert_eq!(Some(actual), expected);
                    assert!(
                        frame.prior_homes().is_empty(),
                        "scalar initializer never manufactures a Text Home"
                    );
                })
                .unwrap();
        }
    }
}

pub(super) fn initializer_core_i64_reaches_the_normal_compiler_with_original_reads_and_cleanup() {
    // Sources that now reach physical emission recurse deeper than a 2 MiB test thread.
    std::thread::Builder::new().name("initializer-core-i64".into())
        .stack_size(32 * 1024 * 1024).spawn(|| {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let mut failures = Vec::new();
        for (methods, body, returned) in [
            ("static box Values { make(text) { return text.substring(0,1) } }", "local out = Values.make(\"abc\") print(out)", "0"),
            ("static box Values { make() { local older = \"old\" + \"er\" return \"abc\" } }", "local out = Values.make() print(out)", "0"),
            ("static box Values { make() { return (\"a\" + \"bc\").substring(0,1) } }", "local out = Values.make() print(out)", "0"),
            ("static box Values { make() { local value = \"a\" + \"b\" return value } }", "local out = Values.make() print(out)", "0"),
            ("static box Values { make(text) { local value = \"!\" + text return value } }", "local out = Values.make(\"a\" + \"b\") print(out)", "0"),
            ("", "local n = \"abc\".length()", "0"),
            ("", "local text = \"abc\" local n = text.length()", "0"),
            ("", "local start = 1 local alias = start local n = alias + \"abc\".length()", "0"),
            ("", "local n = (\"a\" + \"b\").length()", "0"),
            ("", "local first = \"abc\".length() local next = first + \"d\".length()", "0"),
            ("static box Values { find(): i64 { return 1 } run(count: i64): i64 { local first = Values.find() local next = first + \"d\".length() return 0 } }", "local out = Values.run(1)", "0"),
            ("static box Values { find(value: i64): i64 { return value } run(count: i64): i64 { local first = Values.find(count) local next = first + \"d\".length() return 0 } }", "local out = Values.run(1)", "0"),
            ("static box Values { find(value: i64): i64 { return value } run(count: i64): i64 { local alias = count local first = Values.find(alias) local next = first + \"d\".length() return 0 } }", "local out = Values.run(1)", "0"),
            ("static box Values { find(value: i64): i64 { return value } run(count: i64): i64 { local first = Values.find(count + 1) local next = first + \"d\".length() return 0 } }", "local out = Values.run(1)", "0"),
            ("static box Values { find(value: i64): i64 { return value } run(count: i64): i64 { local first = Values.find(count + (\"a\" + \"b\").length()) local next = first + \"d\".length() return 0 } }", "local out = Values.run(1)", "0"),
            ("static box Values { find(value: i64): i64 { return value } run(count: i64): i64 { local computed = \"abc\".length() local first = Values.find(computed + 1) local next = first + \"d\".length() return 0 } }", "local out = Values.run(1)", "0"),
            ("static box Values { find(value: i64): i64 { return value } run(count: i64): i64 { local first = Values.find(\"abc\".length()) local next = first + \"d\".length() return 0 } }", "local out = Values.run(1)", "0"),
            ("static box Values { read(count: i64, other: i64): i64 { local n = count + \"d\".length() return 0 } }", "local out = Values.read(1, 2)", "0"),
            ("static box Values { sink(p, q) { return 0 } run(): i64 { local out = Values.sink(\"abc\".substring(0,2).substring(0,1), \"xy\".substring(0,1)) return 0 } }", "local out = Values.run()", "0"),
            ("static box Values { find(text, value: i64): i64 { local n = text.length() return value } } box Worker { accept(text) { return true } run(text) { local n = text.length() local i = 0 loop(i < n) { local end = Values.find(text, i) if end < 0 { end = n } me.accept(text.substring(i, end)) i = end + 1 } } }", "local worker = new Worker() worker.run(\"7\")", "0"),
            ("static box Values { find(text, value): i64 { local n = text.length() return 0 } } box Worker { accept(text) { return true } run(text) { local n = text.length() local i = 0 loop(i < n) { local end = Values.find(text, i) if end < 0 { end = n } me.accept(text.substring(i, end)) i = end + 1 } } }", "local worker = new Worker() worker.run(\"7\")", "0"),
        ] {
            // Declared Main args selects the existing ordinary body consumer;
            // arity-0 qualified Main has a distinct canonical trivial-body route.
            let main_parameters = if methods.is_empty() { "" } else { "args" };
            let main = format!("static box Main {{ main({main_parameters}) {{ {body} return {returned} }} }}");
            let text = if methods.contains("make(text) { return text.substring") {
                format!("{main} {methods}")
            } else { format!("{methods} {main}") };
            let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
                &text, crate::parser::ParserBuildConfig::default()).unwrap();
            let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
                crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap() else { panic!("original source") };
            let mut consumed = false;
            let result = crate::mir::MirCompiler::with_options(methods.contains("make(")).compile_normal_with_published(
                crate::mir::NormalCompileRequestV1::for_mir_mode_callable_source(source, None, Default::default()).for_closed_app_executable(),
                |view, verification| {
                    verification.as_ref().map_err(|errors| format!("{errors:?}"))?;
                    let input = view.issue_lifecycle_physical_abi_input()?;
                    let root = view.retained_root_source().ok_or_else(|| "missing original initializer source".to_owned())?;
                    root.visit_finalized_lexical_call_nodes_v1(view.module(), |_, _, _, _, _, _, _| Ok(()))?;
                    if methods.contains("find(") {
                        let function = view.module().functions.values().find(|function| function.signature.name.contains("run")).unwrap();
                        let call_sites: Vec<_> = function.blocks.iter().filter_map(|(block, body)| {
                            match body.terminator.as_ref() {
                                Some(crate::mir::MirInstruction::Invoke {
                                    operation: crate::mir::instruction::InvokeOperation::Call { result: crate::mir::instruction::InvokeCallResultKind::I64, .. },
                                    normal_landing, ..
                                }) => Some((*block, *normal_landing)),
                                _ => None,
                            }
                        }).collect();
                        assert_eq!(call_sites.len(), 1, "original scalar producer must remain in the real child");
                        for mutation in ["callee", "result", "arguments"] {
                            let mut changed = view.module().clone();
                            let (block, normal) = call_sites[0];
                            let changed_function = changed.functions.get_mut(&function.signature.name).unwrap();
                            if mutation == "result" {
                                let result = changed_function.blocks.get_mut(&normal).unwrap().instructions.iter_mut()
                                    .find(|row| matches!(row, crate::mir::MirInstruction::InvokeNormalResult { invoke_block, .. } if *invoke_block == block)).unwrap();
                                let crate::mir::MirInstruction::InvokeNormalResult { dst, .. } = result else { unreachable!() };
                                *dst = crate::mir::ValueId(99999);
                            } else {
                                let Some(crate::mir::MirInstruction::Invoke {
                                    operation: crate::mir::instruction::InvokeOperation::Call { call, .. }, ..
                                }) = changed_function.blocks.get_mut(&block).unwrap().terminator.as_mut() else { unreachable!() };
                                match mutation {
                                    "callee" => call.callee = crate::mir::Callee::Global(
                                        crate::mir::builder::CanonicalSameModuleCallableKeyV1::static_box_method("Values", "run", 1)
                                            .canonical_global_target_v1().unwrap()),
                                    "arguments" => call.args.push(crate::mir::ValueId(99999)),
                                    _ => unreachable!(),
                                }
                            }
                            assert!(root.visit_finalized_lexical_call_nodes_v1(&changed, |_, _, _, _, _, _, _| Ok(())).is_err(), "{mutation}");
                        }
                    }
                    if methods.contains("find(") || methods.contains("sink(") {
                        let function = view.module().functions.values().find(|function| function.signature.name.contains("run")).unwrap();
                        for (block, body) in &function.blocks {
                            for (index, instruction) in body.instructions.iter().enumerate() {
                                if !matches!(instruction, crate::mir::MirInstruction::BinOp { op: crate::mir::BinaryOp::Add, .. }) { continue; }
                                let mut changed = view.module().clone();
                                let crate::mir::MirInstruction::BinOp { op, .. } = &mut changed.functions.get_mut(&function.signature.name).unwrap().blocks.get_mut(block).unwrap().instructions[index] else { unreachable!() };
                                *op = crate::mir::BinaryOp::Sub;
                                assert!(root.visit_finalized_lexical_call_nodes_v1(&changed, |_, _, _, _, _, _, _| Ok(())).is_err(), "original integer operation cannot change");
                            }
                            if matches!(body.terminator, Some(crate::mir::MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::End { .. }), .. })) {
                                let mut changed = view.module().clone();
                                let Some(crate::mir::MirInstruction::Invoke { operation, .. }) = changed.functions.get_mut(&function.signature.name).unwrap().blocks.get_mut(block).unwrap().terminator.as_mut() else { unreachable!() };
                                *operation = crate::mir::instruction::InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::End { value: crate::mir::ValueId(99999) });
                                assert!(root.visit_finalized_lexical_call_nodes_v1(&changed, |_, _, _, _, _, _, _| Ok(())).is_err(), "argument child cleanup cannot change");
                            }
                        }
                    }
                    if let Some(function) = view.module().functions.values().find(|function| function.params.len() == 2) {
                        let mut changed = view.module().clone();
                        changed.functions.get_mut(&function.signature.name).unwrap().params.swap(0, 1);
                        let root = view.retained_root_source().ok_or_else(|| "missing original initializer source".to_owned())?;
                        assert!(root.visit_finalized_lexical_call_nodes_v1(&changed, |_, _, _, _, _, _, _| Ok(())).is_err(),
                            "equal-I64 parameters cannot exchange their original ordinal/value identity");
                    }
                    if methods.contains("sink(") {
                        let function = view.module().functions.values().find(|function| function.signature.name.contains("run")).unwrap();
                        let block = function.blocks.iter().find_map(|(block, body)| matches!(body.terminator,
                            Some(crate::mir::MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::Call { .. }, .. })).then_some(*block)).unwrap();
                        for mutation in ["argument-order", "argument-value", "call-fault"] {
                            let mut changed = view.module().clone();
                            let Some(crate::mir::MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::Call { call, .. }, normal_landing, fault_landing, .. }) =
                                changed.functions.get_mut(&function.signature.name).unwrap().blocks.get_mut(&block).unwrap().terminator.as_mut() else { unreachable!() };
                            match mutation {
                                "argument-order" => call.args.swap(0, 1),
                                "argument-value" => call.args[0] = crate::mir::ValueId(99999),
                                "call-fault" => *fault_landing = *normal_landing,
                                _ => unreachable!(),
                            }
                            assert!(root.visit_finalized_lexical_call_nodes_v1(&changed, |_, _, _, _, _, _, _| Ok(())).is_err(), "{mutation}");
                        }
                    }
                    if methods.contains("make() { return (") || methods.contains("make() { local older")
                        || methods.contains("make(text) { return text.substring") {
                        let root = view.retained_root_source().unwrap();
                        let function = view.module().functions.values().find(|function| function.signature.name.contains("make")).unwrap();
                        for mutation in ["missing-hold", "wrong-return", "missing-install"] {
                            let mut changed = view.module().clone();
                            for block in changed.functions.get_mut(&function.signature.name).unwrap().blocks.values_mut() {
                                match mutation {
                                    "missing-hold" => block.instructions.retain(|row| !matches!(row,
                                        crate::mir::MirInstruction::StringResidenceInstall { returning: true, .. })),
                                    "missing-install" => block.instructions.retain(|row| !matches!(row,
                                        crate::mir::MirInstruction::StringResidenceInstall { returning: false, .. })),
                                    "wrong-return" => if let Some(crate::mir::MirInstruction::Return { value }) = &mut block.terminator {
                                        *value = Some(crate::mir::ValueId(99999));
                                    },
                                    _ => unreachable!(),
                                }
                            }
                            assert!(root.visit_finalized_lexical_call_nodes_v1(&changed, |_, _, _, _, _, _, _| Ok(())).is_err(), "{mutation}");
                        }
                    }
                    crate::mir::emit_lifecycle_physical_abi_json(&input)?;
                    consumed = true;
                    Ok(())
                });
            match result {
                Err(error) => failures.push(format!("{text}: {error}")),
                Ok(_) if !consumed => failures.push(format!("original initializer must reach publication: {text}")),
                Ok(_) => (),
            }
        }
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    });
    }).unwrap().join().unwrap();
}

pub(super) fn original_loop_header_body_break_and_backedge_keep_one_basis_and_exact_home_scopes() {
    // Sources that now reach physical emission recurse deeper than a 2 MiB test thread.
    std::thread::Builder::new().name("original-loop-header".into())
        .stack_size(32 * 1024 * 1024).spawn(|| {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let text = r#"static box Main { main() {
        local n = 2
        local text = "7"
        local i = 0
        loop(i < n) {
            local ch = text.substring(i, i + 1)
            if ch < "0" || ch > "9" { break }
            i = i + 1
        }
        return i
    } }"#;
    // A fresh issuance of this SAME source exercises the actual one-port
    // writer; the source-only assertions below consume a different handoff.
    for optimize in [false, true] {
        let physical = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog_selection(
            text, crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable).unwrap();
        let main = physical
            .declaration_catalog()
            .source_backed_app_main()
            .unwrap();
        let declaration = physical
            .batch()
            .declarations()
            .find(|row| row.identity().same_as(main.parser_identity()))
            .unwrap();
        let mut builder = crate::mir::MirBuilder::new();
        let function = physical
            .batch()
            .with_lowering_input_and_source_identity(declaration.batch_slot(), |input, identity| {
                builder.lower_map_dependency_for_test(
                    input,
                    crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(
                        main.catalog_key().clone(),
                    ),
                    main.parser_identity(),
                    identity.method_source_observation().cloned(),
                    Rc::clone(&physical.ordinary_new_claim_ledger),
                    None,
                )
            })
            .unwrap()
            .expect("whole original source Loop through SAME Raw port");
        crate::mir::verification::MirVerifier::new_strict()
            .verify_function(&function)
            .unwrap_or_else(|error| {
                panic!(
                    "{error:?}\n{}",
                    crate::mir::MirPrinter::new().print_function(&function)
                )
            });
        let observation = physical
            .ordinary_new_claim_ledger
            .validate_finalized_new_root(&function)
            .expect("whole Loop original final consumer");
        let mut function = function;
        function
            .install_root_ordinary_new_observation(observation)
            .unwrap();
        let mut module = crate::mir::MirModule::new("source-loop-finishing".into());
        module.functions.insert("Main.main/0".into(), function);
        if optimize {
            let roots = physical
                .ordinary_new_claim_ledger
                .scalar_finishing_liveness_v1(&module, "Main.main/0")
                .unwrap();
            crate::mir::optimizer::MirOptimizer::new()
                .optimize_module_with_required_roots(&mut module, &roots);
            let finished = &module.functions["Main.main/0"];
            crate::mir::verification::MirVerifier::new_strict()
                .verify_function(finished)
                .unwrap_or_else(|error| {
                    panic!(
                        "optimize={optimize}: {error:?}\n{}",
                        crate::mir::MirPrinter::new().print_function(finished)
                    )
                });
        }
        let finished = &module.functions["Main.main/0"];
        physical
            .ordinary_new_claim_ledger
            .validate_artifact_after_compiler_finishing(finished)
            .unwrap_or_else(|error| {
                panic!(
                    "whole Loop finishing consumer optimize={optimize}: {error}\n{}",
                    crate::mir::MirPrinter::new().print_function(finished)
                )
            });
    }
    for expected_control in [true, false] {
        let text = if expected_control {
            text.to_owned()
        } else {
            text.replace("i = i + 1", "i = i + 1 local unused = null")
        };
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog_selection(
        &text, crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable).unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        let source = ledger.borrowed_formal_source.as_ref().unwrap();
        assert_eq!(package.parameter_contracts.len(), 1);
        package.batch().with_lowering_input(package.parameter_contracts[0].batch_slot, |input| {
        let site = crate::mir::resolved_semantics::SourcePathV1::root_body(3).stmt();
        let retained = ledger.retained_source_loop_recipe_v2(input.owner(), &site);
        if expected_control {
            let product = retained.unwrap().expect("production selected row keeps its source Recipe");
            assert_eq!(product.source.source.site(), &site);
            let flow = ledger.completion_for_owner(input.owner()).unwrap().cleanup().root_flow().unwrap();
            let slot = flow.expression_plans().iter().find(|plan| Rc::ptr_eq(plan.source(), product.source.condition.source())).unwrap();
            assert!(Rc::ptr_eq(slot.loop_recipe_v2().unwrap(), product), "same product in the original condition slot");
            let prefixes = flow.lifecycle_prefixes(&[]).unwrap();
            assert_eq!(prefixes.len(), 1, "one Loop owner, not flat conditional body prefixes");
            assert!(matches!(&prefixes[0], crate::mir::resolved_semantics::home_new_prefix::LifecyclePrefixSourceV1::Loop { product: original, .. }
                if Rc::ptr_eq(original, product)));
            product.assert_schedule_for_test(input);
            // The physical input bridge borrows the SAME executable JoinSig
            // payload, and cannot substitute another logical key or edge role.
            for carrier in &product.recipe.as_recipe().carriers {
                let binding = *product.bindings.iter().find(|(_, key)| **key == carrier.binding).unwrap().0;
                use crate::mir::resolved_semantics::source_value_flow::SourceValueEdgeV1 as Edge;
                for edge in [Edge::LoopEntry, Edge::ConditionFalse] {
                    product.validate_incoming_value_v2(&edge, binding, carrier.entry_value).unwrap();
                    assert!(product.validate_incoming_value_v2(&edge, binding,
                        crate::mir::loop_recipe_contract::LoopValueKeyV1::new(u32::MAX)).is_err());
                }
                assert!(product.validate_incoming_value_v2(&Edge::ThenNormal, binding, carrier.entry_value).is_err());
            }
            // The production selectors borrow the same shared source inventory
            // as the schedule; scoped execution must never fall back to raw.
            for scheduled in product.schedule().frames() {
                use crate::mir::resolved_semantics::home_new_prefix::LifecyclePrefixSourceV1 as Prefix;
                let statement = scheduled.prefix().statement();
                match scheduled.prefix() {
                    Prefix::InitializerExpression { frame, .. } => {
                        let selected = ledger.initializer_expression_frame_v1(input.owner(), statement.node()).unwrap().unwrap();
                        assert!(Rc::ptr_eq(frame, &selected));
                    }
                    Prefix::BindingRebind { frame, .. } => {
                        let selected = ledger.rebind_expression_frame_v1(input.owner(), statement.node()).unwrap().unwrap();
                        assert!(Rc::ptr_eq(frame, &selected));
                    }
                    Prefix::BoolCondition { frame, .. } => {
                        let selected = ledger.bool_condition_frame_v1(input.owner(), statement).unwrap().unwrap();
                        assert!(Rc::ptr_eq(frame, &selected));
                    }
                    _ => panic!("same projector schedules only executable source frames"),
                }
            }
            let context = crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(ledger);
            assert!(context.validate_source_loop_bool_frame_v2(product, 0, &product.source.condition).is_err(),
                "the original pending Loop group is required before condition execution");
            for basis in product.source.headers.values() {
                let point = crate::mir::resolved_semantics::home_new_prefix::CurrentScalarProducerV1::LoopHeader(Rc::clone(basis));
                assert!(context.source_loop_point_product_v2(&point).is_err(), "the same affine Loop parent is required");
            }
            ledger.open_source_loop_prefix_v2(Rc::clone(product)).unwrap();
            crate::mir::builder::BindingValueObservationV2::assert_checked_loop_points_for_test(input, Rc::clone(ledger), product);
            for basis in product.source.headers.values() {
                use crate::mir::resolved_semantics::home_new_prefix::CurrentScalarProducerV1 as Producer;
                let point = Producer::LoopHeader(Rc::clone(basis));
                assert!(Rc::ptr_eq(&context.source_loop_point_product_v2(&point).unwrap(), product));
                crate::mir::builder::LocalProvenanceV1::assert_indexed_phi_loan_for_test(point);
                let after = Producer::LoopAfter { product: Rc::clone(product), binding: basis.binding() };
                assert!(context.source_loop_point_product_v2(&after).unwrap_err().contains("physical-closure-pending"),
                    "logical After cannot borrow a pending physical Loop");
            }
            context.validate_source_loop_bool_frame_v2(product, 0, &product.source.condition).unwrap();
            assert!(context.validate_source_loop_bool_frame_v2(product, 1, &product.source.condition).is_err(),
                "a header frame cannot borrow a body frame's schedule position");
            assert!(context.validate_source_loop_bool_frame_v2(product, usize::MAX, &product.source.condition).is_err());
            for (index, step) in product.schedule().frames().iter().enumerate().skip(1) {
                if let crate::mir::resolved_semantics::home_new_prefix::LifecyclePrefixSourceV1::BoolCondition { frame, .. } = step.prefix() {
                    context.validate_source_loop_bool_frame_v2(product, index, frame).unwrap();
                    assert!(context.validate_source_loop_bool_frame_v2(product, 0, frame).is_err(),
                        "body If retains its own source purpose, never Header");
                }
            }
            assert!(ledger.open_source_loop_prefix_v2(Rc::clone(product)).is_err(), "one original Loop group");
            ledger.with_local_call_binding_groups_for_test(input.owner(), |groups| {
                assert_eq!(groups.len(), 1, "conditional children never become root prefixes");
                assert!(groups[0].matches_source(&prefixes[0]));
                assert!(groups[0].require_sealed_condition_v1().unwrap_err().contains("physical-closure-pending"),
                    "collection cannot grant final physical completion");
            });
            ledger.validate_bool_condition_source_v1(&product.source.condition).unwrap();
            for frame in product.source.body_expressions.iter().filter_map(|plan| plan.initializer_frame()) {
                assert!(ledger.validate_initializer_expression_source_v1(frame).is_err(), "condition must be collected first");
            }
            let exit_site = crate::mir::resolved_semantics::SourcePathV1::root_body(4).stmt();
            let exit = flow.exit_row(&exit_site).unwrap().unwrap();
            assert!(prefixes[0].covered_by(exit));
            assert!(product.source.body_expressions.iter().all(|frame| !exit.covers_statement(frame.source().root().statement())),
                "outer exit does not assert that zero-iteration/conditional children executed");
            assert!(product.source.body_expressions.iter().all(|plan| plan.loop_recipe_v2().is_none()),
                "body/If slots cannot borrow this Loop condition's attachment");
            assert!(Rc::ptr_eq(&product.source.source, ledger.retained_loop_source_v1(input.owner(), &site).unwrap().unwrap()));
            let policy = crate::mir::builder::LoopFactsPolicyFrameV1::from_values(false, false, false, false, false, true);
            let foreign = crate::mir::builder::issue_preissued_callable_loop_source_plan_v1(input, &site, policy).unwrap();
            assert!(foreign.validate_source_recipe_attachment_v2(product).is_err(),
                "equal source reissued by another planner cannot borrow this product");
            let mut paired = ledger.take_preissued_loop_source_handoff_v2(input.owner(), &site).unwrap().unwrap();
            assert!(Rc::ptr_eq(paired.original_source_v1(), &product.source.source));
            assert!(Rc::ptr_eq(paired.source_recipe_for_test_v2().unwrap(), product));
            assert!(paired.attach_source_recipe_v2(Rc::clone(product)).is_err(), "attachment is affine");
            assert!(ledger.take_preissued_loop_source_handoff_v2(input.owner(), &site).unwrap_err()
                .contains("already-taken"));
            assert!(Rc::ptr_eq(ledger.retained_source_loop_recipe_v2(input.owner(), &site).unwrap().unwrap(), product),
                "physical take does not replace or remove the source proof");
        } else {
            assert!(retained.is_err(), "selected incomplete tree is terminal");
            assert!(ledger.take_preissued_loop_source_handoff_v2(input.owner(), &site).is_err(),
                "a refused logical product never grants an affine physical handoff");
        }
        let mut pending = ledger.borrowed_formal_actuals.clone();
        crate::mir::resolved_semantics::home_new_prefix::assert_original_loop_source_walk_for_test(input, expected_control,
            &mut |site, request| borrowed_call_arguments_callback_v1(package.batch(), &package.selected, &Ok(Vec::new()), &[],
                &package.parameter_contracts, &[], None, source, &Ok(Default::default()), &mut pending,
                &ledger.borrowed_i64_results, &mut |_| None, site, request,
                &ledger.callable_result_classes, &ledger.receiver_call_observations, &ledger.field_write_claims, None));
    }).unwrap();
    }
    }).unwrap().join().unwrap();
}
