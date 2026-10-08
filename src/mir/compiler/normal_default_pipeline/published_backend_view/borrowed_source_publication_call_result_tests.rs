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
                                classify_pretransform_report(verification);
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
                                classify_pretransform_report(verification);
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
                                    classify_pretransform_report(verification);
                                    let input = view.issue_lifecycle_physical_abi_input()?;
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
                    classify_pretransform_report(verification);
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
                    classify_pretransform_report(verification);
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
                classify_pretransform_report(verification);
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
                classify_pretransform_report(verification);
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
                            classify_pretransform_report(verification);
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
                                    PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryHandle { key, receiver_object } =>
                                        PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryNullableHandle { key, receiver_object },
                                    PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryNullableHandle { key, receiver_object } =>
                                        PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryHandle { key, receiver_object },
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
}
