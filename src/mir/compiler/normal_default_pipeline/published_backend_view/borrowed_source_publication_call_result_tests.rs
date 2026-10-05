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
            ("p: Item", "if p == null { return 7 } return p.value", "declared"),
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
