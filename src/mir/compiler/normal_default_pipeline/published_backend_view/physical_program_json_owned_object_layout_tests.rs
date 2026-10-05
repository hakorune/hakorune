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
                            // The existing pretransform Document report still uses scalar
                            // formal types for the already-sealed borrowed carrier. Keep
                            // only this exact reference observation; ABI admission below
                            // must corroborate the original null/class view independently.
                            if let Err(errors) = verification {
                                assert_eq!(errors.len(), 1, "{errors:?}");
                                let error = errors[0].to_string();
                                assert!(
                                    error.contains("[mir/invoke/call-argument-type-drift]")
                                        && error.contains(
                                            "caller=main callee=Transport.probe/1 argument=0"
                                        )
                                        && error.contains("actual=Some(Void) formal=Integer")
                                        && error.contains("carrier=Some(BorrowedTaggedValue)"),
                                    "{error}"
                                );
                            }
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
}

#[test]
fn owned_object_layout_publishes_constructed_child_canonical_tuple() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
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
    });
}
