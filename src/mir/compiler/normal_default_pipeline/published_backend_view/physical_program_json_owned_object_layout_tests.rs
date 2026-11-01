//! A class layout alone is not an owned-child inventory.
use super::*;

#[test]
fn owned_object_layout_keeps_ignored_null_only_declared_parent() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for optimize in [false, true] {
            for scratch in [true, false] {
                let body = if scratch {
                    "local scratch = new Transport() return 7"
                } else {
                    "return 7"
                };
                let text = "box Leaf { flag: i64 birth() { me.flag = 0 } }
                box Parent { child: Leaf birth() { me.child = new Leaf() } }
                box Transport { flag: i64 birth() { me.flag = 0 }
                    probe(p: Parent): i64 { local scratch = new Transport() return 7 } }
                static box Main { main() { local recv = new Transport()
                    return recv.probe(null) } }"
                    .replace("local scratch = new Transport() return 7", body);
                MirCompiler::with_options(optimize)
                    .compile_normal_with_published(
                        request(&text),
                        |view, verification| -> Result<(), String> {
                            assert!(verification.is_ok(), "final artifact verification: {verification:?}");
                            let input = view.issue_lifecycle_physical_abi_input()?;
                            let wire = emit_lifecycle_physical_abi_json(&input)?;
                            let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                            let callee = json["functions"]
                                .as_array()
                                .unwrap()
                                .iter()
                                .find(|row| row["name"] == "Transport.probe/1")
                                .unwrap();
                            let object = &callee["params"][0]["object_view"];
                            assert!(
                                object.is_number(),
                                "the declared Parent class view is retained"
                            );
                            let layout = json["layouts"]
                                .as_array()
                                .unwrap()
                                .iter()
                                .find(|row| &row["object_id"] == object)
                                .unwrap();
                            assert_eq!(layout["field_count"], 1);
                            assert!(
                                layout["owned_object_residences"]
                                    .as_array()
                                    .unwrap()
                                    .is_empty(),
                                "a class/null view must not mint owned child permission"
                            );
                            std::fs::write(
                                std::env::temp_dir().join(format!(
                                    "hako-owned-layout-null-only-opt{optimize}.json"
                                )),
                                wire,
                            )
                            .unwrap();
                            Ok(())
                        },
                    )
                    .unwrap();
            }
        }
    });
    // Direct placement preserves the original declaration/slot plan and has
    // a distinct wire ABI, while unselected runtime stores remain v1.
    crate::test_support::with_env_vars(
        &[("NYASH_MACRO_DISABLE", Some("1")), ("HAKO_TYPED_OBJECT_STORE", Some("direct_v0:Pair"))],
        || {
            let source = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/apps/ownership-move-min/main.hako"));
            for update in [false, true] {
                let source = if update {
                    source.replace("return me.left + me.right", "me.left = me.left + 1\nreturn me.left + me.right")
                } else { source.to_owned() };
                MirCompiler::with_options(false).compile_normal_with_published(
                    request(&source),
                    |view, verification| -> Result<(), String> {
                        assert!(verification.is_ok(), "final artifact verification: {verification:?}");
                        let input = view.issue_lifecycle_physical_abi_input()?;
                        assert!(matches!(input.runtime_requirements(), crate::mir::PublishedLifecycleRuntimeRequirementsV1::TypedObject { storage_profile: 1 }));
                        let wire = emit_lifecycle_physical_abi_json(&input)?;
                        let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                        assert_eq!(json["storage_profile"], 3);
                        assert_eq!(json["layouts"][0]["placement"]["kind"], "direct_v0");
                        std::fs::write(std::env::temp_dir().join(format!("hako-issued-direct-scalar-update{update}.json")), wire).unwrap();
                        Ok(())
                    }).unwrap();
            }
        },
    );
}

#[test]
fn owned_object_layout_publishes_self_referential_child_tuple() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for optimize in [false, true] {
            let text = "box Node { left: Node\nright: Node\nvalue: i64
                birth(left, right, value) {
                    me.left = left
                    me.right = right
                    me.value = value
                } }
                static box Main { main() {
                    local item = new Node(null, null, 1)
                    return 0
                } }";
            MirCompiler::with_options(optimize)
                .compile_normal_with_published(
                    request(text),
                    |view, verification| -> Result<(), String> {
                        assert!(verification.is_ok(), "{verification:?}");
                        let input = view.issue_lifecycle_physical_abi_input()?;
                        let wire = emit_lifecycle_physical_abi_json(&input)?;
                        let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                        let birth = json["functions"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .find(|row| row["name"] == "Node.birth/3")
                            .unwrap();
                        let stores: Vec<_> = birth["blocks"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|row| &row["terminator"]["instruction"]["operation"])
                            .filter(|row| row["kind"] == "object_field_set")
                            .collect();
                        assert_eq!(stores.len(), 2, "both nullable fields store");
                        let layout = json["layouts"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .find(|row| {
                                row["object_id"] == stores[0]["object_id"]
                            })
                            .unwrap();
                        // The self-referential census seals the object's
                        // own identity in both mark rows — the emitted
                        // teardown calls its own helper on each live child.
                        assert_eq!(
                            layout["owned_object_residences"],
                            serde_json::json!([
                                {"field_ordinal": 0,
                                 "child_object_id": layout["object_id"]},
                                {"field_ordinal": 1,
                                 "child_object_id": layout["object_id"]},
                            ])
                        );
                        std::fs::write(
                            std::env::temp_dir().join(format!(
                                "hako-owned-layout-selfref-opt{optimize}.json"
                            )),
                            wire,
                        )
                        .unwrap();
                        Ok(())
                    },
                )
                .unwrap();
        }
    });
}

#[test]
fn owned_object_layout_publishes_constructed_child_canonical_tuple() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let text = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/apps/ownership-child-typed-min/main.hako"));
    let compile = |text: &str, optimize| MirCompiler::with_options(optimize)
        .compile_normal_with_published(request(text), |view, verification| -> Result<String, String> {
            assert!(verification.is_ok(), "final artifact: {verification:?}");
            let input = view.issue_lifecycle_physical_abi_input()?;
            // The canonical CLI checks this boundary after optimization;
            // unoptimized publication still contains unused declaration strings.
            if optimize { crate::mir::backend_capability::enforce_published_lifecycle_backend_supported(view, &input)?; }
            emit_lifecycle_physical_abi_json(&input)
        });
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for optimize in [false, true] {
            let text = "box Leaf { flag: i64 birth() { me.flag = 0 } }
                box Parent { child: Leaf birth() { me.child = new Leaf() } }
                static box Main { main() { local parent = new Parent() return 5 } }";
            MirCompiler::with_options(optimize)
                .compile_normal_with_published(
                    request(text),
                    |view, verification| -> Result<(), String> {
                        assert!(verification.is_ok(), "{verification:?}");
                        let input = view.issue_lifecycle_physical_abi_input()?;
                        let wire = emit_lifecycle_physical_abi_json(&input)?;
                        let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                        let birth = json["functions"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .find(|row| row["name"] == "Parent.birth/0")
                            .unwrap();
                        let store = birth["blocks"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|row| &row["terminator"]["instruction"]["operation"])
                            .find(|row| row["kind"] == "object_field_set")
                            .unwrap();
                        let layout = json["layouts"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .find(|row| row["object_id"] == store["object_id"])
                            .unwrap();
                        assert_eq!(
                            layout["owned_object_residences"],
                            serde_json::json!([
                                {"field_ordinal": 0, "child_object_id": store["child_object_id"]}
                            ])
                        );
                        std::fs::write(
                            std::env::temp_dir()
                                .join(format!("hako-owned-layout-constructed-opt{optimize}.json")),
                            wire,
                        )
                        .unwrap();
                        Ok(())
                    },
                )
                .unwrap();
        }
        for optimize in [false, true] {
            let crate::mir::compiler::normal_default_pipeline::NormalPublishedCompileOutcome::Consumed(wire) = compile(text, optimize).unwrap()
                else { panic!("canonical publication required"); };
            std::fs::write(std::env::temp_dir().join("hako-issued-nonrecursive-typed-child.json"), wire).unwrap();
            // Both transferred child and explicitly moved parent lose their
            // original binding; a source rewrite must never revive either.
            for stale in ["leaf.read()", "full.read()"] {
                let invalid = text.replace("moved.read() + empty.read()", stale);
                assert!(compile(&invalid, optimize).is_err(), "stale receiver {stale}");
            }
        }
    });
    crate::test_support::with_env_vars(&[("NYASH_MACRO_DISABLE", Some("1")),
        ("HAKO_TYPED_OBJECT_STORE", None)], || {
        let text = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/apps/ownership-tree-bench-typed/main.hako"));
        let crate::mir::compiler::normal_default_pipeline::NormalPublishedCompileOutcome::Consumed(wire) = compile(text, true).unwrap()
            else { panic!("canonical typed factory publication required"); };
        std::fs::write(std::env::temp_dir().join("hako-issued-typed-binary-trees.json"), wire).unwrap();
    });
    crate::test_support::with_env_vars(&[("NYASH_MACRO_DISABLE", Some("1")),
        ("HAKO_TYPED_OBJECT_STORE", Some("direct_v0:Leaf,Parent"))], || {
        let provider = "box Leaf { flag: i64 birth() { me.flag = 0 } }
            box Parent { first: Leaf second: Leaf birth() { me.first = new Leaf() me.second = new Leaf() } }
            static box Main { main() { local parent = new Parent() return 5 } }";
        for (name, source) in [("child", text), ("birth-prefix", provider)] {
            let crate::mir::compiler::normal_default_pipeline::NormalPublishedCompileOutcome::Consumed(wire) = compile(source, true).unwrap()
                else { panic!("canonical direct publication required"); };
            std::fs::write(std::env::temp_dir().join(format!("hako-issued-direct-{name}.json")), wire).unwrap();
        }
    });
}
