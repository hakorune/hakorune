//! Composed I64 results keep the original caller/callee and cleanup authority.
use super::*;

#[test]
fn borrowed_local_call_results_publish_grounded_chain_in_both_orders() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for reverse in [false, true] {
            for annotated in [false, true] {
                let result = if annotated { ": i64" } else { "" };
                let read = "read(p: Item): i64 { if p == null { return 7 } return p.value }";
                let bridge = format!("bridge(p: Item){result} {{ local recv = new Transport() local out = recv.read(p) return out }}");
                let wrap = format!("wrap(p: Item){result} {{ local recv = new Transport() local out = recv.bridge(p) return out }}");
                let methods = if reverse {
                    format!("{wrap} {bridge} {read}")
                } else {
                    format!("{read} {bridge} {wrap}")
                };
                for (domain, prefix, actual) in [
                    ("object", "local item = new Item()", "item"),
                    ("null", "", "null"),
                ] {
                    let text = format!("box Item {{ value: i64 birth() {{ me.value = 5 }} }}
                        box Transport {{ flag: i64 birth() {{ me.flag = 0 }} {methods} }}
                        static box Main {{ main() {{ local recv = new Transport() {prefix} return recv.wrap({actual}) }} }}");
                    for optimize in [false, true] {
                        MirCompiler::with_options(optimize).compile_normal_with_published(
                            request(&text), |view, verification| -> Result<(), String> {
                                require_artifact_verification(verification);
                                let input = view.issue_lifecycle_physical_abi_input()?;
                                let wire = super::super::super::physical_program_json::emit_lifecycle_physical_abi_json(&input)?;
                                let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                                for target in ["Transport.bridge/1", "Transport.wrap/1"] {
                                    let function = json["functions"].as_array().unwrap().iter()
                                        .find(|function| function["name"] == target).unwrap();
                                    assert_eq!(function["role"], "ordinary_i64");
                                }
                                std::fs::write(std::env::temp_dir().join(format!(
                                    "hako-issued-borrowed-call-result-{domain}-reverse{reverse}-annotated{annotated}-opt{optimize}.json"
                                )), wire).unwrap();
                                Ok(())
                            },
                        ).unwrap_or_else(|error| panic!("{domain}/reverse{reverse}/annotated{annotated}/opt{optimize}: {error}"));
                    }
                }
            }
        }

    });
}

#[test]
fn borrowed_direct_call_results_publish_grounded_chain_in_both_orders() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for reverse in [false, true] {
            for annotated in [false, true] {
                let result = if annotated { ": i64" } else { "" };
                let read = format!(
                    "read(p: Item){result} {{ if p == null {{ return 7 }} return p.value }}"
                );
                let bridge = format!("bridge(p: Item){result} {{ local recv = new Transport() return recv.read(p) }}");
                let wrap = format!("wrap(p: Item){result} {{ local recv = new Transport() return recv.bridge(p) }}");
                let methods = if reverse {
                    format!("{wrap} {bridge} {read}")
                } else {
                    format!("{read} {bridge} {wrap}")
                };
                for (domain, prefix, actual) in [
                    ("object", "local item = new Item()", "item"),
                    ("null", "", "null"),
                ] {
                    let text = format!("box Item {{ value: i64 birth() {{ me.value = 5 }} }}
                        box Transport {{ flag: i64 birth() {{ me.flag = 0 }} {methods} }}
                        static box Main {{ main() {{ local recv = new Transport() {prefix} return recv.wrap({actual}) }} }}");
                    for optimize in [false, true] {
                        MirCompiler::with_options(optimize).compile_normal_with_published(
                            request(&text), |view, verification| -> Result<(), String> {
                                require_artifact_verification(verification);
                                let input = view.issue_lifecycle_physical_abi_input()?;
                                let wire = super::super::super::physical_program_json::emit_lifecycle_physical_abi_json(&input)?;
                                let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                                for target in ["Transport.bridge/1", "Transport.wrap/1"] {
                                    let function = json["functions"].as_array().unwrap().iter()
                                        .find(|function| function["name"] == target).unwrap();
                                    assert_eq!(function["role"], "ordinary_i64");
                                }
                                std::fs::write(std::env::temp_dir().join(format!(
                                    "hako-issued-borrowed-direct-call-result-{domain}-reverse{reverse}-annotated{annotated}-opt{optimize}.json"
                                )), wire).unwrap();
                                Ok(())
                            },
                        ).unwrap_or_else(|error| panic!("{domain}/reverse{reverse}/annotated{annotated}/opt{optimize}: {error}"));
                    }
                }
            }
        }
    });
}

#[test]
fn borrowed_stored_child_call_results_publish_original_receivers() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for (parameter, body, variant) in [
            (
                "p: Item",
                "if p == null { return 7 } return p.value",
                "declared",
            ),
            ("p", "if p == null { return 7 } return p.value", "opaque"),
            ("p", "return 5", "scalar"),
        ] {
            for reverse in [false, true] {
                for annotated in [false, true] {
                    let result = if annotated { ": i64" } else { "" };
                    let leaf = format!(
                        "box Leaf {{ flag: i64 birth() {{ me.flag = 0 }}
                    read({parameter}){result} {{ {body} }} }}"
                    );
                    let parent = format!(
                        "box Parent {{ left: Leaf right: Leaf
                    birth() {{ me.left = new Leaf() me.right = new Leaf() }}
                    first(p: Item){result} {{ return me.left.read(p) }}
                    second(p: Item){result} {{ return me.right.read(p) }} }}"
                    );
                    let definitions = if reverse {
                        format!("{parent} {leaf}")
                    } else {
                        format!("{leaf} {parent}")
                    };
                    for (domain, prefix, actual) in [
                        ("object", "local item = new Item()", "item"),
                        ("null", "", "null"),
                    ] {
                        for method in ["first", "second"] {
                            let other = if method == "first" { "second" } else { "first" };
                            let text = format!(
                                "box Item {{ value: i64 birth() {{ me.value = 5 }} }}
                            {definitions} static box Main {{ main() {{ local parent = new Parent()
                                {prefix} local ignored = parent.{other}({actual})
                                return parent.{method}({actual}) }} }}"
                            );
                            for optimize in [false, true] {
                                MirCompiler::with_options(optimize).compile_normal_with_published(
                                request(&text), |view, verification| -> Result<(), String> {
                                    require_artifact_verification(verification);
                                    let input = view.issue_lifecycle_physical_abi_input()?;
                                    crate::mir::parameter_entry_backend_capability::enforce_lifecycle_parameter_entry_backend_supported(view.module(), &input)?;
                                    if !optimize && !reverse && !annotated && variant == "declared" && domain == "object" && method == "first" {
                                        crate::mir::parameter_entry_backend_capability::assert_declared_parameter_drift_rejected(view.module(), &input, "Parent.first/1");
                                    }
                                    let wire = super::super::super::physical_program_json::emit_lifecycle_physical_abi_json(&input)?;
                                    let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                                    for (name, ordinal) in [("Parent.first/1", 0), ("Parent.second/1", 1)] {
                                        let function = json["functions"].as_array().unwrap().iter()
                                            .find(|function| function["name"] == name).unwrap();
                                        assert_eq!(function["role"], "ordinary_i64");
                                        let blocks = function["blocks"].as_array().unwrap();
                                        let calls: Vec<_> = blocks.iter().filter(|block|
                                            block["terminator"]["instruction"]["operation"]["kind"] == "ordinary_call").collect();
                                        assert_eq!(calls.len(), 1, "one original terminal call");
                                        let call = &calls[0]["terminator"]["instruction"]["operation"]["call"];
                                        let target = &json["functions"][call["target"].as_u64().unwrap() as usize];
                                        assert_eq!(target["name"], "Leaf.read/1");
                                        let reads: Vec<_> = blocks.iter().flat_map(|block|
                                            block["instructions"].as_array().unwrap()).map(|row| &row["instruction"])
                                            .filter(|read| read["op"] == "object_field_get").collect();
                                        assert_eq!(reads.len(), 1, "one original receiver read");
                                        let read = reads[0];
                                        assert_eq!(read["field_ordinal"], ordinal);
                                        assert_eq!(read["object_id"], function["receiver_object"]);
                                        assert_eq!(read["base"], function["receiver"]);
                                        assert_eq!(call["receiver"], read["dst"]);
                                        assert_ne!(target["receiver_object"], function["receiver_object"]);
                                        let layout = json["layouts"].as_array().unwrap().iter()
                                            .find(|layout| layout["object_id"] == function["receiver_object"]).unwrap();
                                        let residence = layout["owned_object_residences"].as_array().unwrap().iter()
                                            .find(|row| row["field_ordinal"] == ordinal).unwrap();
                                        assert_eq!(residence["child_object_id"], target["receiver_object"]);
                                        assert!(blocks.iter().all(|block| {
                                            let op = &block["terminator"]["instruction"]["operation"];
                                            op["kind"] != "object_field_release" && op["kind"] != "object_release"
                                        }), "ordinary child borrow creates no receiver teardown");
                                    }
                                    let variant = if variant == "declared" { "".to_owned() } else { format!("{variant}-") };
                                    std::fs::write(std::env::temp_dir().join(format!(
                                        "hako-issued-borrowed-stored-child-{variant}{domain}-{method}-reverse{reverse}-annotated{annotated}-opt{optimize}.json"
                                    )), wire).unwrap();
                                    Ok(())
                                },
                            ).unwrap_or_else(|error| panic!("{variant}/{domain}/{method}/reverse{reverse}/annotated{annotated}/opt{optimize}: {error}"));
                            }
                        }
                    }
                }
            }
        }
    });
}

#[test]
fn borrowed_stored_child_callee_fault_publishes_original_scratch_birth() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let text = "box Leaf { flag: i64 birth() { me.flag = 0 }
            read(p: Item): i64 { local scratch = new Scratch() return 5 } }
            box Scratch { flag: i64 birth() { me.flag = 0 } }
            box Parent { child: Leaf birth() { me.child = new Leaf() }
                read(p: Item): i64 { return me.child.read(p) } }
            box Item { value: i64 birth() { me.value = 5 } }
            static box Main { main() { local parent = new Parent()
                local item = new Item() return parent.read(item) } }";
        for optimize in [false, true] {
            MirCompiler::with_options(optimize).compile_normal_with_published(
                request(text), |view, verification| -> Result<(), String> {
                    require_artifact_verification(verification);
                    let input = view.issue_lifecycle_physical_abi_input()?;
                    let wire = super::super::super::physical_program_json::emit_lifecycle_physical_abi_json(&input)?;
                    let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                    let leaf = json["functions"].as_array().unwrap().iter()
                        .find(|function| function["name"] == "Leaf.read/1").unwrap();
                    assert_eq!(leaf["receiver_object"], 0, "canonical child id zero is valid");
                    assert!(leaf["blocks"].as_array().unwrap().iter().any(|block|
                        block["terminator"]["instruction"]["operation"]["kind"] == "birth_call"));
                    std::fs::write(std::env::temp_dir().join(format!(
                        "hako-issued-stored-child-callee-fault-opt{optimize}.json"
                    )), wire).unwrap();
                    Ok(())
                },
            ).unwrap_or_else(|error| panic!("stored callee Fault/opt{optimize}: {error}"));
        }
    });
}

#[test]
fn borrowed_static_local_original_cohort_publishes_receiver_free_and_forwarded_calls() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for (label, actual) in [
            ("integer", "7"),
            ("negative", "-7"),
            ("bool", "true"),
            ("null", "null"),
        ] {
            let text = format!("static box Layout {{ pick(p) {{ return 0 }} }} box Heap {{ lookup(size) {{ local k = Layout.pick(size) return 0 }} }} static box Main {{ main() {{ local heap = new Heap() local k = heap.lookup({actual}) local a = Layout.pick({actual}) return 0 }} }}");
            for optimize in [false, true] {
                MirCompiler::with_options(optimize).compile_normal_with_published(request(&text), |view, verification| -> Result<(), String> {
                    require_artifact_verification(verification);
                    let input = view.issue_lifecycle_physical_abi_input()?;
                    let wire = super::super::super::physical_program_json::emit_lifecycle_physical_abi_json(&input)?;
                    let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                    let functions = json["functions"].as_array().unwrap();
                    let layout = functions.iter().find(|f| f["name"] == "Layout.pick/1").expect("original Static definition");
                    assert_eq!(layout["role"], "ordinary_i64");
                    assert_eq!(layout["params"].as_array().unwrap().len(), 1);
                    assert_eq!(layout["params"][0]["representation"], "borrowed_kind_payload_v1");
                    std::fs::write(std::env::temp_dir().join(format!(
                        "hako-issued-borrowed-static-local-cohort-{label}-opt{optimize}.json"
                    )), &wire).unwrap();
                    if label == "integer" && !optimize {
                        std::fs::write(std::env::temp_dir().join("hako-issued-borrowed-static-local-cohort.json"), &wire).unwrap();
                    }
                    Ok(())
                }).unwrap_or_else(|error| panic!("Static {label}/opt{optimize}: {error}"));
            }
        }
    });
}

#[test]
fn borrowed_static_mixed_scalar_original_cohort_publishes_exact_ordinal_carriers() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let text = "static box Layout { pick(p, q: i64) { return q } } box Heap { lookup(size) { local q = 9 local k = Layout.pick(size, q) return 0 } } static box Main { main() { local heap = new Heap() local k = heap.lookup(7) local a = Layout.pick(8, 9) return 0 } }";
        for optimize in [false, true] {
            MirCompiler::with_options(optimize).compile_normal_with_published(request(text), |view, verification| -> Result<(), String> {
                require_artifact_verification(verification);
                let input = view.issue_lifecycle_physical_abi_input()?;
                let wire = super::super::super::physical_program_json::emit_lifecycle_physical_abi_json(&input)?;
                let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                let layout = json["functions"].as_array().unwrap().iter().find(|f| f["name"] == "Layout.pick/2").unwrap();
                assert_eq!(layout["params"].as_array().unwrap().len(), 2);
                assert_eq!(layout["params"][0]["representation"], "borrowed_kind_payload_v1");
                assert_ne!(layout["params"][1]["representation"], "borrowed_kind_payload_v1");
                std::fs::write(std::env::temp_dir().join(format!("hako-issued-static-mixed-opt{optimize}.json")), wire).unwrap();
                Ok(())
            }).unwrap_or_else(|error| panic!("mixed scalar Static/opt{optimize}: {error}"));
        }
    });
}

#[test]
fn borrowed_static_guard_original_cohort_publishes_all_calls_with_prior_home_cleanup() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let text = "static box Layout { pick(p) { if p > 0 { return 1 } return 0 } } box Heap { lookup(size) { local k = Layout.pick(size) return 0 } } static box Main { main() { local heap = new Heap() local k = heap.lookup(7) local a = Layout.pick(8) return 0 } }";
        for optimize in [false, true] {
            MirCompiler::with_options(optimize).compile_normal_with_published(request(text), |view, verification| -> Result<(), String> {
                require_artifact_verification(verification);
                let input = view.issue_lifecycle_physical_abi_input()?;
                let wire = super::super::super::physical_program_json::emit_lifecycle_physical_abi_json(&input)?;
                let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                let functions = json["functions"].as_array().unwrap();
                let mut calls = 0;
                for function in functions {
                    for block in function["blocks"].as_array().unwrap() {
                        let operation = &block["terminator"]["instruction"]["operation"];
                        calls += usize::from(operation["kind"] == "ordinary_call");
                    }
                }
                assert_eq!(calls, 3, "all three original source calls publish");
                std::fs::write(std::env::temp_dir().join(format!("hako-issued-static-guard-opt{optimize}.json")), wire).unwrap();
                Ok(())
            }).unwrap_or_else(|error| panic!("guarded Static/opt{optimize}: {error}"));
        }
    });
}

#[test]
fn owned_call_results_publish_original_child_handle_contracts() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for typed in [true, false] {
            let formal = if typed { "size: i64" } else { "size" };
            for received in [false, true] {
                for nullable in [false, true] {
                    let make = if nullable {
                        "if size > 0 { return new Token() } return null"
                    } else {
                        "return new Token()"
                    };
                    let relay = if received {
                        "local item = me.make(7) return item"
                    } else {
                        "return me.make(7)"
                    };
                    let text = format!(
                        "box Token {{}} box Spare {{}}
                    box Maker {{ make({formal}) {{ {make} }}
                        relay() {{ local spare = new Spare() {relay} }} }}
                    static box Main {{ main() {{ local maker = new Maker()
                        local item = maker.relay() return 0 }} }}"
                    );
                    for optimize in [false, true] {
                        MirCompiler::with_options(optimize).compile_normal_with_published(
                        request(&text), |view, verification| -> Result<(), String> {
                            require_artifact_verification(verification);
                            let input = view.issue_lifecycle_physical_abi_input()?;
                            let wire = super::super::super::physical_program_json::emit_lifecycle_physical_abi_json(&input)?;
                            let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                            let role = if nullable { "ordinary_nullable_handle" } else { "ordinary_handle" };
                            for name in ["Maker.make/1", "Maker.relay/0"] {
                                let function = json["functions"].as_array().unwrap().iter()
                                    .find(|function| function["name"] == name).unwrap();
                                assert_eq!(function["role"], role);
                            }
                            assert_eq!(input.entry().root_result(), super::super::super::CompiledEntryRootResultV1::I64);
                            if !typed {
                                let mut changed = view.issue_lifecycle_physical_program()?;
                                let make = changed.functions.iter_mut()
                                    .find(|function| function.name() == "Maker.make/1").unwrap();
                                assert!(make.param_carriers().unwrap().contains(
                                    &crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1::BorrowedTaggedValue
                                ), "opaque actual must select the original borrowed checker");
                                make.role = match make.role.clone() {
                                    PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryHandle { key, receiver_object, returns } =>
                                        PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryNullableHandle { key, receiver_object, returns },
                                    PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryNullableHandle { key, receiver_object, returns } =>
                                        PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryHandle { key, receiver_object, returns },
                                    _ => panic!("make keeps its original object role"),
                                };
                                assert!(super::super::super::compiled_entry_contract::verify_borrowed_call_incoming(
                                    &changed, view.module()
                                ).unwrap_err().contains("borrowed-incoming/result-mismatch"),
                                    "Handle/Nullable role drift must reject the unchanged call");
                            }
                            let suffix = if typed { "" } else { "-opaque" };
                            std::fs::write(std::env::temp_dir().join(format!(
                                "hako-issued-owned-call-result-received{received}-nullable{nullable}-opt{optimize}{suffix}.json"
                            )), wire).unwrap();
                            Ok(())
                        },
                    ).unwrap_or_else(|error| panic!("typed{typed}/received{received}/nullable{nullable}/opt{optimize}: {error}"));
                    }
                }
            }
        }
    });
    // Reuse the unchanged app's production loader, import lineage and
        // closed executable selection; the reduced source is a different entry.
        let env_updates: Vec<_> = crate::test_support::JOINIR_DEFAULT_MODE.into_iter().chain([
            ("NYASH_ALLOW_USING_FILE", Some("1")), ("NYASH_ENABLE_USING", Some("1")),
            ("NYASH_OPERATOR_BOX_ALL", Some("0")), ("NYASH_MACRO_DISABLE", Some("1")),
        ]).collect();
        crate::test_support::with_env_vars(&env_updates, || {
        for optimize in [false, true] {
            let filename = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("apps/mimalloc-lite/main.hako");
            let code = std::fs::read_to_string(&filename).unwrap();
            let runner = crate::runner::NyashRunner::new(Default::default());
            let prepared = crate::runner::modes::common_util::source_hint::prepare_normal_source_with_imports(
                &runner, filename.to_str().unwrap(), &code,
            ).expect("same production import preparation");
            let transformed = crate::runner::modes::common_util::normal_callable::materialize_normal_callable_program_with_identity_and_lineage_v1(
                &prepared.code, runner.parser_build_config(), filename.to_str().unwrap(), prepared.lineage,
            ).expect("same original imported source");
            let crate::runner::modes::common_util::normal_callable::NormalCallableMaterializationOutcomeV1::SourceBacked(source) = transformed
                else { panic!("unchanged app remains source-backed"); };
            let request = crate::mir::NormalCompileRequestV1::for_mir_mode_callable_source(
                source, filename.to_str(), prepared.imports,
            ).for_closed_app_executable();
            MirCompiler::with_options(optimize).compile_normal_with_published(request, |view, verification| -> Result<(), String> {
                require_artifact_verification(verification);
                let source = view.retained_root_source().expect("same original Mixed source");
                // This source loan borrows the original retained actuals.
                // Whole-program compiled-entry admission has independent scans.
                let handoff = view.retained_handoff.unwrap();
                let callables = handoff.callables().unwrap();
                for (symbol, expected) in [
                    ("HakoAllocHeap.allocateResult/1", crate::mir::instruction::InvokeCallResultKind::NullableHandle),
                    ("HakoAllocHeap.reallocResult/2", crate::mir::instruction::InvokeCallResultKind::MixedHandle { formal_ordinal: 0 }),
                ] {
                    let key = view.module().canonical_callable_definitions.keys()
                        .find(|key| key.mir_symbol_projection() == symbol).unwrap();
                    let owner = callables.owner_for_canonical_key(key).unwrap();
                    let function = &view.module().functions[symbol];
                    let mut proved = 0;
                    for actual in handoff.birth_actuals().unwrap() {
                        if actual.owner() != owner
                            || actual.target().mir_symbol_projection() != "HakoAllocHandleResult.birth/3" { continue; }
                        let Some(argument) = actual.arguments().iter().find(|arg| arg.source().ordinal() == 2
                            && matches!(arg.source().kind(), crate::mir::normal_callable_semantic_package::OrdinaryNewTrivialArgumentKindV1::BoundValue { .. }))
                            else { continue };
                        assert_eq!(source.received_birth_actual_result_v1(actual, 2, function)?, Some(expected));
                        let drift = actual.with_argument_value_for_test(2, crate::mir::ValueId::new(9999));
                        assert!(source.received_birth_actual_result_v1(&drift, 2, function).is_err());
                        assert!(source.received_birth_actual_result_v1(actual, 99, function).is_err());
                        assert_eq!(argument.source().owner(), actual.owner());
                        if matches!(expected, crate::mir::instruction::InvokeCallResultKind::MixedHandle { .. }) {
                            let mut changed = function.clone();
                            changed.params[1] = crate::mir::ValueId::new(9999);
                            assert!(source.received_birth_actual_result_v1(actual, 2, &changed).is_err(),
                                "the same field support must retain its original caller formal");
                        }
                        proved += 1;
                    }
                    assert_eq!(proved, 1, "same original received result -> Birth argument: {symbol}");
                }
                let page_key = view.module().canonical_callable_definitions.keys()
                    .find(|key| key.mir_symbol_projection() == "HakoAllocPage.allocate/1").unwrap();
                let page_owner = callables.owner_for_canonical_key(page_key).unwrap();
                let page = &view.module().functions["HakoAllocPage.allocate/1"];
                let actual = handoff.birth_actuals().unwrap().iter().find(|row| row.owner() == page_owner
                    && row.target().mir_symbol_projection() == "HakoAllocHandle.birth/3").unwrap();
                let argument = &actual.arguments()[1];
                assert!(argument.source().field_array_read().is_some(), "same original block_id Get loan");
                source.validate_new_argument_reads_for_test(page_owner, page)?;
                for drift in 0..4 {
                    let mut changed = page.clone();
                    let mut mutations = 0;
                    for block in changed.blocks.values_mut() {
                        for row in block.instructions.iter_mut().chain(block.terminator.iter_mut()) {
                            match row {
                                MirInstruction::ArrayRead { operation: crate::mir::instruction::ArrayReadOperationV1::Element { index }, .. } if drift == 0 => { *index = crate::mir::ValueId(99991); mutations += 1; },
                                MirInstruction::Copy { dst, src } if drift == 1 && *dst == argument.value() => { *src = crate::mir::ValueId(99992); mutations += 1; },
                                MirInstruction::FieldGet { field, .. } if drift == 2 && field == "free_stack" => { *field = "foreign".into(); mutations += 1; },
                                MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::Birth(call), .. }
                                    if drift == 3 => {
                                        call.arguments[1] = crate::mir::instruction::BirthArgumentV1::Value(crate::mir::ValueId(99993)); mutations += 1;
                                    },
                                _ => {},
                            }
                        }
                    }
                    assert!(mutations > 0, "original Get/local/Birth boundary must mutate drift{drift}");
                    assert!(source.validate_new_argument_reads_for_test(page_owner, &changed).is_err());
                }
                // The independent whole-program consumer must also accept the
                // original operands; source-local positives cannot waive it.
                let compiled = view.issue_lifecycle_compiled_entry_contract()?;
                let input = view.issue_lifecycle_physical_abi_input()?;
                crate::mir::backend_capability::enforce_published_lifecycle_backend_supported(view, &input)?;
                let generic = PublishedMirBackendView::try_new_selected_normal(view.module())
                    .map_err(|error| error.to_string())?;
                assert!(generic.validate_lifecycle_input_handoff(&input).is_err());
                assert!(generic.validated_named_arrays().is_err());
                let wrapper = compiled.program().functions().iter()
                    .find(|function| function.name() == "HakoAllocHeap.reallocResult/2").unwrap();
                assert!(wrapper.role().object_return_outcomes().unwrap().iter().any(|row| row.has_field_supports()),
                    "the uncalled wrapper retains its original outgoing field support");
                let support_groups: Vec<_> = wrapper.role().object_return_outcomes().unwrap().iter()
                    .flat_map(|row| row.field_support_witness_counts()).collect();
                assert_eq!(support_groups, vec![2],
                    "both original Forward witnesses share one physical dependency, without losing either loan");
                for drift in 0..3 {
                    let mut changed = view.module().functions[wrapper.name()].clone();
                    let mut mutations = 0;
                    if drift == 2 { changed.params[1] = crate::mir::ValueId::new(99991); mutations += 1; }
                    for block in changed.blocks.values_mut() {
                        match block.terminator.as_mut() {
                            Some(MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::Call { call, .. }, .. })
                                if drift == 0 && matches!(&call.callee, crate::mir::Callee::SameModuleInstance { key, .. }
                                    if key.mir_symbol_projection() == "HakoAllocHeap.realloc/2") => {
                                call.args[0] = crate::mir::ValueId::new(99992); mutations += 1;
                            }
                            Some(MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::Birth(call), .. })
                                if drift == 1 && call.key.mir_symbol_projection() == "HakoAllocHandleResult.birth/3"
                                    && call.arguments.get(2).is_some_and(|value| !matches!(value,
                                        crate::mir::instruction::BirthArgumentV1::Value(value) if changed.params.get(1) == Some(value))) => {
                                call.arguments[2] = crate::mir::instruction::BirthArgumentV1::Value(crate::mir::ValueId::new(99992)); mutations += 1;
                            }
                            _ => {},
                        }
                    }
                    assert!(mutations > 0, "original field-support drift{drift} must change its own source binding");
                    assert!(super::super::super::physical_program::validate_object_return_census(
                        &changed, wrapper.role()).is_err(), "retained field support rejects physical drift{drift}");
                }
                let mut argument_trace = Vec::new();
                source.visit_finalized_borrowed_call_nodes_v1(view.module(), |packet, _, caller, coordinate, _| {
                    if caller.signature.name == "HakoAllocHeap.realloc/2"
                        && packet.target().mir_symbol_projection() == "HakoAllocHeap.isLiveHandle/1" {
                        let actuals = source.borrowed_finalized_call_actuals_v1(packet)?.unwrap();
                        argument_trace.push(format!("call={:?} coordinate={coordinate:?} actuals={actuals:?}", packet.call_site()));
                    }
                    Ok(())
                })?;
                assert_eq!(argument_trace.len(), 1, "same original realloc -> isLiveHandle source packet");
                std::fs::write(std::env::temp_dir().join(format!("hako-issued-mimalloc-realloc-argument-v1-opt{optimize}.trace")),
                    argument_trace.join("\n")).unwrap();
                // Reuse the original app's source and completed cohort. The
                // Array read joins its own residence and finishing projection.
                let arrays = view.validated_named_arrays()?;
                let callables = view.retained_handoff.unwrap().callables().unwrap();
                let seed_key = view.module().canonical_callable_definitions.keys()
                    .find(|key| key.mir_symbol_projection() == "HakoAllocPage.seedBlocks/0").unwrap();
                let seed_owner = callables.owner_for_canonical_key(seed_key).unwrap();
                let seed = &view.module().functions["HakoAllocPage.seedBlocks/0"];
                let reads = source.finished_array_field_reads_v1(seed_owner, view.module(), seed, arrays)?;
                assert_eq!(reads.len(), 4, "all original field-resident Array aliases");
                for mutation in 0..5 {
                    let mut changed = view.module().clone();
                    let seed = changed.functions.get_mut("HakoAllocPage.seedBlocks/0").unwrap();
                    let (binding, _) = &reads[0];
                    let block = seed.blocks.get_mut(&binding.0).unwrap();
                    let index = block.instructions.iter().position(|row| row == &binding.1).unwrap();
                    match mutation {
                        0 => { block.instructions.remove(index); block.instruction_spans.remove(index); },
                        1 => block.add_instruction(binding.1.clone()),
                        _ => {
                            let MirInstruction::FieldGet { base, field, declared_type, .. } = &mut block.instructions[index] else { unreachable!() };
                            match mutation {
                                2 => *base = crate::mir::ValueId(99999),
                                3 => *field = "foreign".into(),
                                _ => *declared_type = Some(crate::mir::MirType::Integer),
                            }
                        }
                    }
                    assert!(source.finished_array_field_reads_v1(seed_owner, &changed,
                        &changed.functions["HakoAllocPage.seedBlocks/0"], arrays).is_err(), "Array read drift {mutation}");
                }
                assert!(source.finished_array_field_reads_v1(seed_owner, view.module(), seed, &[]).is_err(),
                    "a passive residence descriptor cannot replace the emitted write");
                // The unchanged app supplies the source-owned nullable signature;
                // the old generic Null/Unknown merge must not turn it into Unit.
                assert_eq!(view.module().functions["HakoAllocPage.resizeInPlace/2"].signature.return_type,
                    crate::mir::MirType::Unknown);
                let mut wrong_signature = view.module().clone();
                wrong_signature.functions.get_mut("HakoAllocPage.resizeInPlace/2").unwrap()
                    .signature.return_type = crate::mir::MirType::Void;
                let mut drift = PublishedMirBackendView::try_new_selected_normal(&wrong_signature)
                    .map_err(|issue| issue.to_string())?
                    .bind_finalized_root_handoff(view.retained_handoff)?;
                drift.lifecycle_storage_profile = view.lifecycle_storage_profile;
                assert!(drift.issue_lifecycle_physical_program().unwrap_err()
                    .contains("ordinary-membership-drift"), "Void cannot replace the original borrowed result");
                let retain = |key: &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
                    rows: &[crate::mir::normal_callable_semantic_package::FinalizedObjectReturnOutcomeV1],
                    result: crate::mir::instruction::InvokeCallResultKind| -> Result<PublishedLifecyclePhysicalFunctionRoleV1, String> {
                    use super::super::super::{physical_program as physical, physical_program_json as wire};
                    let function = &view.module().functions[&key.mir_symbol_projection()];
                    let receiver_object = physical::object_identity::ordinary_receiver_object(view.module(), key)?;
                    let role = match result {
                        crate::mir::instruction::InvokeCallResultKind::MixedHandle { formal_ordinal } =>
                            PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryMixedHandle {
                                key: key.clone(), receiver_object, formal_ordinal, returns: rows.into(),
                            },
                        crate::mir::instruction::InvokeCallResultKind::BorrowedNullableHandle { support } =>
                            PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryBorrowedNullableHandle {
                                key: key.clone(), receiver_object, support, returns: rows.into(),
                            },
                        _ => panic!("exact original object result"),
                    };
                    assert_eq!(role.ordinary_result(), Some(result));
                    let calls: Vec<_> = function.blocks.values().flat_map(|block| block.all_instructions())
                        .filter_map(|row| physical::ordinary_call_site_v1(row)
                            .map(|(call, result)| physical::OrdinaryCallSite { call: call.clone(), result }))
                        .collect();
                    let issued = physical::issue_function_with_module(Some(view.module()), function, role, false, &calls, Some(view))?;
                    assert_eq!(issued.role().object_return_outcomes().unwrap(), rows);
                    if matches!(result, crate::mir::instruction::InvokeCallResultKind::BorrowedNullableHandle { .. }) {
                        super::super::super::compiled_entry_contract::verify_original_formal_return(
                            source, view.module(), &issued,
                        )?;
                    }
                    for row in rows {
                        row.validate_physical_function_v1(function, result, true)?;
                        let changed = match result {
                            crate::mir::instruction::InvokeCallResultKind::MixedHandle { .. } =>
                                crate::mir::instruction::InvokeCallResultKind::MixedHandle { formal_ordinal: 1 },
                            _ => crate::mir::instruction::InvokeCallResultKind::borrowed_nullable_formal(1),
                        };
                        assert!(row.validate_physical_function_v1(function, changed, true)
                            .unwrap_err().contains("object-return/physical-result"),
                            "the source result ordinal stays authoritative before physical lane checks");
                        let block = issued.blocks().iter().find(|block| block.id() == row.returned().0).unwrap();
                        let at = (block.id(), block.terminator().index());
                        let json = wire::encode_object_return(&issued, at, &row.returned().1)?.unwrap();
                        assert_eq!(json["kind"], row.origin_name());
                        assert_eq!(json["copies"].as_array().unwrap().len(), row.transport_bindings().len());
                        assert!(wire::encode_object_return(&issued, (at.0, at.1 + 1), &row.returned().1).is_err());
                    }
                    let mut wrong_ordinal = issued.role().clone();
                    match &mut wrong_ordinal {
                        PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryMixedHandle { formal_ordinal, .. } => *formal_ordinal = 99,
                        PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryBorrowedNullableHandle { support, .. } =>
                            *support = crate::mir::instruction::BorrowedResultSupportV1::Formal(99),
                        _ => unreachable!(),
                    }
                    assert!(physical::issue_function_with_module(Some(view.module()), function, wrong_ordinal, false, &calls, Some(view)).is_err());
                    let mut missing = issued.role().clone();
                    match &mut missing {
                        PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryMixedHandle { returns, .. }
                        | PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryBorrowedNullableHandle { returns, .. } => *returns = Box::new([]),
                        _ => unreachable!(),
                    }
                    assert!(physical::issue_function_with_module(Some(view.module()), function, missing, false, &calls, Some(view)).unwrap_err().contains("object-return/census"));
                    Ok(issued.role().clone())
                };
                let key = hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::instance_box_method("HakoAllocHeap", "realloc", 2);
                let rows = source.mixed_return_projections_v1(view.module(), &key)?;
                let retained_role = retain(&key, &rows, crate::mir::instruction::InvokeCallResultKind::MixedHandle { formal_ordinal: 0 })?;
                let origins: std::collections::BTreeSet<_> = rows.iter().map(|row| row.origin_name()).collect();
                assert_eq!(origins, std::collections::BTreeSet::from(["null", "owned_call", "borrowed_call"]));
                assert_eq!(rows.len(), 8, "all original realloc exits, without duplicate witness roots");
                let copy = rows.iter().flat_map(|row| row.transport_bindings()).next()
                    .expect("original borrowed result retains its LOCAL publication Copy").clone();
                for drift in 0..7 {
                    let mut changed = view.module().clone();
                    let function = changed.functions.get_mut("HakoAllocHeap.realloc/2").unwrap();
                    match drift {
                        0 => for block in function.blocks.values_mut() {
                            if let Some(MirInstruction::Return { value }) = block.terminator.as_mut() {
                                *value = Some(crate::mir::ValueId::new(900_001));
                            }
                        },
                        1 => for block in function.blocks.values_mut() {
                            for row in &mut block.instructions {
                                if let MirInstruction::InvokeNormalResult { dst, .. } = row { *dst = crate::mir::ValueId::new(900_002); }
                            }
                        },
                        2 => {
                            let mut block = crate::mir::BasicBlock::new(crate::mir::BasicBlockId::new(900_003));
                            block.set_terminator(MirInstruction::Return { value: None });
                            function.add_block(block);
                        }
                        3 => function.params[1] = crate::mir::ValueId::new(900_004),
                        4 => function.metadata.physical_param_carriers.as_mut().unwrap()[1] =
                            crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1::ExistingCallableI64,
                        _ => {
                            let row = function.blocks.get_mut(&copy.0).unwrap().instructions.iter_mut()
                                .find(|row| **row == copy.1).unwrap();
                            let MirInstruction::Copy { dst, src } = row else { unreachable!() };
                            if drift == 5 { *src = crate::mir::ValueId::new(900_005); }
                            else { *dst = crate::mir::ValueId::new(900_006); }
                        }
                    }
                    if drift == 3 {
                        // Mixed rows retain inner Call origins, whereas the
                        // original source entry owns the outer formal ValueId.
                        assert!(source.mixed_return_projections_v1(&changed, &key).is_err(),
                            "Mixed source entry drift must refuse");
                    } else {
                        assert!(super::super::super::physical_program::validate_object_return_census(
                            &changed.functions["HakoAllocHeap.realloc/2"], &retained_role,
                        ).is_err(), "Mixed final drift{drift} must refuse");
                    }
                }
                // The same unchanged app supplies complete conditional End
                // bindings; no synthetic instruction or source receipt is used.
                let name = "HakoAllocHeap.reallocResult/2";
                let function = &view.module().functions[name];
                let ends = source.object_end_bindings_v1(view.module(), function)?;
                assert!(!ends.is_empty(), "original mixed result has conditional cleanup");
                let actual = super::super::super::physical_program::object_end::bindings(
                    Some(view.module()), function, Some(view),
                )?;
                assert_eq!(actual, ends);
                assert!(super::super::super::physical_program::object_end::bindings(
                    None, function, None,
                ).is_err(), "an End instruction cannot authorize itself");
                for drift in ["missing", "body", "operand"] {
                    let mut changed = view.module().clone();
                    let function = changed.functions.get_mut(name).unwrap();
                    for block in function.blocks.values_mut() {
                        for instruction in block.instructions.iter_mut().chain(block.terminator.iter_mut()) {
                            if let MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::ObjectEndIfOwned { end, value, .. }, .. } = instruction {
                                match drift {
                                    "body" => *end = Box::new([]),
                                    "operand" => *value = crate::mir::ValueId(999999),
                                    _ => *instruction = MirInstruction::Return { value: None },
                                }
                            }
                        }
                    }
                    assert!(source.object_end_bindings_v1(&changed, &changed.functions[name]).is_err(),
                        "conditional End {drift} must not inherit source proof");
                }

                let key = hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::instance_box_method("HakoAllocPage", "resizeInPlace", 2);
                let rows = source.borrowed_nullable_return_projections_v1(view.module(), &key)?;
                let retained_role = retain(&key, &rows, crate::mir::instruction::InvokeCallResultKind::borrowed_nullable_formal(0))?;
                assert_eq!(rows.len(), 8, "seven explicit null exits and one nullable formal Return");
                let origins: std::collections::BTreeSet<_> = rows.iter().map(|row| row.origin_name()).collect();
                assert_eq!(origins, std::collections::BTreeSet::from(["null", "borrowed_formal"]));
                for drift in 0..5 {
                    let mut changed = view.module().clone();
                    let function = changed.functions.get_mut("HakoAllocPage.resizeInPlace/2").unwrap();
                    match drift {
                        0 => for block in function.blocks.values_mut() {
                            if let Some(MirInstruction::Return { value }) = block.terminator.as_mut() {
                                *value = Some(crate::mir::ValueId::new(900_010));
                            }
                        },
                        1 => function.params[1] = crate::mir::ValueId::new(900_011),
                        2 => function.metadata.physical_param_carriers.as_mut().unwrap()[1] =
                            crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1::ExistingCallableI64,
                        3 => {
                            let mut changed_null = false;
                            for block in function.blocks.values_mut() {
                                for row in &mut block.instructions {
                                    if let MirInstruction::Const { value, .. } = row {
                                        if *value == crate::mir::ConstValue::Null {
                                            *value = crate::mir::ConstValue::Integer(0);
                                            changed_null = true;
                                        }
                                    }
                                }
                            }
                            assert!(changed_null, "original explicit null producer exists");
                        },
                        _ => {
                            let mut block = crate::mir::BasicBlock::new(crate::mir::BasicBlockId::new(900_012));
                            block.set_terminator(MirInstruction::Return { value: None });
                            function.add_block(block);
                        }
                    }
                    assert!(super::super::super::physical_program::validate_object_return_census(
                        &changed.functions["HakoAllocPage.resizeInPlace/2"], &retained_role,
                    ).is_err(), "borrowed final drift{drift} must refuse");
                }
                Ok(())
            }).unwrap_or_else(|error| panic!("unchanged app Mixed original Return projection opt{optimize}: {error}"));
        }
        });
}

#[test]
fn current_static_zero_original_source_publishes_invoke_without_generic_call() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let text = "static box Layout { word() { return 8 } run(p) { local a = me.word() local b = Layout.word() return 0 } } static box Main { main() { local r = Layout.run(7) return r } }";
        for optimize in [false, true] {
            MirCompiler::with_options(optimize).compile_normal_with_published(request(text), |view, verification| -> Result<(), String> {
                require_artifact_verification(verification);
                let input = view.issue_lifecycle_physical_abi_input()?;
                let wire = super::super::super::physical_program_json::emit_lifecycle_physical_abi_json(&input)?;
                let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                let run = json["functions"].as_array().unwrap().iter().find(|f| f["name"] == "Layout.run/1").unwrap();
                let calls: Vec<_> = run["blocks"].as_array().unwrap().iter().filter_map(|block| {
                    let instruction = &block["terminator"]["instruction"];
                    (instruction["operation"]["kind"] == "ordinary_call").then_some(instruction)
                }).collect();
                assert_eq!(calls.len(), 2, "{wire}");
                assert_eq!(calls[0]["operation"]["result"], "i64");
                assert!(calls[0]["operation"]["call"]["args"].as_array().unwrap().is_empty());
                std::fs::write(std::env::temp_dir().join(format!("hako-issued-current-static-zero-opt{optimize}.json")), wire).unwrap();
                Ok(())
            }).unwrap_or_else(|error| panic!("Current zero/opt{optimize}: {error}"));
        }
    });
}
