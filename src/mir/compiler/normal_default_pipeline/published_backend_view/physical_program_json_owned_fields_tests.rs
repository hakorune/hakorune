/// An `OwnedArrayFieldsNoHook` object emits one `field_residence_release`
/// per sealed `ArrayBox` field — reverse declaration order — before the
/// containing `home_release`, all bound to the same base value.
#[test]
fn owned_array_fields_release_in_reverse_order_before_home_release() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let mut compiler = MirCompiler::with_options(false);
        compiler
            .compile_normal_with_published(
                request(
                    "box Page {
                       left: i64 = 0
                       items: ArrayBox = new ArrayBox()
                       children: ArrayBox = new ArrayBox()
                       birth() { }
                     }
                     static box Main {
                       main() {
                         local p = new Page()
                         return 0
                       }
                     }",
                ),
                |view, verification| -> Result<(), String> {
                    assert!(verification.is_ok(), "{verification:?}");
                    let input = view.issue_lifecycle_physical_abi_input()?;
                    let json = emit_lifecycle_physical_abi_json(&input)?;
                    let decoded: serde_json::Value = serde_json::from_str(&json).unwrap();
                    let functions = decoded["functions"].as_array().unwrap();
                    let caller = functions
                        .iter()
                        .find(|row| row["name"] == "main")
                        .expect("main row");
                    let blocks: std::collections::BTreeMap<u64, &serde_json::Value> =
                        caller["blocks"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|block| (block["id"].as_u64().unwrap(), block))
                            .collect();
                    // Walk one cleanup path: Jump/Invoke normal landings until
                    // return — collecting (kind, field_ordinal, operand) of
                    // each checked operation plus its fault-chain head.
                    fn walk(
                        blocks: &std::collections::BTreeMap<u64, &serde_json::Value>,
                        mut head: u64,
                    ) -> Vec<(String, Option<u64>, Option<u64>, Option<u64>)> {
                        let mut path = Vec::new();
                        for _ in 0..=blocks.len() {
                            let t = &blocks[&head]["terminator"]["instruction"];
                            match t["op"].as_str().unwrap() {
                                "invoke" => {
                                    let op = &t["operation"];
                                    path.push((
                                        op["kind"].as_str().unwrap().to_owned(),
                                        op["field_ordinal"].as_u64(),
                                        op["base"].as_u64().or(op["value"].as_u64()),
                                        t["fault"].as_u64(),
                                    ));
                                    head = t["normal"].as_u64().unwrap();
                                }
                                "jump" => head = t["target"].as_u64().unwrap(),
                                "return" | "return_fault" => {
                                    path.push((
                                        t["op"].as_str().unwrap().to_owned(),
                                        None,
                                        None,
                                        None,
                                    ));
                                    return path;
                                }
                                other => panic!("unexpected op {other} on cleanup path"),
                            }
                        }
                        panic!("cyclic cleanup path");
                    }
                    let birth = caller["blocks"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .find(|block| {
                            block["terminator"]["instruction"]["operation"]["kind"]
                                == "birth_call"
                        })
                        .expect("birth call block");
                    // Normal teardown: children in reverse declaration order,
                    // then the containing Home release — one path only.
                    let normal = walk(
                        &blocks,
                        birth["terminator"]["instruction"]["normal"]
                            .as_u64()
                            .unwrap(),
                    );
                    let (path, ordinals, bases): (Vec<_>, Vec<_>, Vec<_>) = normal
                        .iter()
                        .map(|(kind, ordinal, operand, _)| {
                            (kind.as_str(), *ordinal, *operand)
                        })
                        .fold((vec![], vec![], vec![]), |(mut p, mut o, mut b), row| {
                            p.push(row.0);
                            o.push(row.1);
                            b.push(row.2);
                            (p, o, b)
                        });
                    let base = normal
                        .iter()
                        .find_map(|(_, _, operand, _)| *operand)
                        .expect("release operand");
                    assert_eq!(
                        path.as_slice(),
                        [
                            "field_residence_release",
                            "field_residence_release",
                            "home_release",
                            "return",
                        ],
                        "normal exit: children reversed, then the parent Home"
                    );
                    assert_eq!(
                        ordinals.as_slice(),
                        [Some(2), Some(1), None, None],
                        "child residences release in reverse declaration order"
                    );
                    assert!(
                        bases.iter().all(|operand| *operand == Some(base) || operand.is_none()),
                        "children release off the same owning base value"
                    );
                    // Exit-fault edge: the faulted op never re-runs — the
                    // remaining children and the parent Home still release.
                    let fault_head = normal[0].3.expect("first child fault edge");
                    let fault = walk(&blocks, fault_head);
                    let fault_kinds: Vec<&str> =
                        fault.iter().map(|(kind, ..)| kind.as_str()).collect();
                    let fault_ordinals: Vec<Option<u64>> =
                        fault.iter().map(|(_, ordinal, ..)| *ordinal).collect();
                    assert_eq!(
                        fault_kinds.as_slice(),
                        ["field_residence_release", "home_release", "return_fault"]
                    );
                    assert_eq!(fault_ordinals.as_slice(), [Some(1), None, None]);
                    // Birth-fault edge: children first (reverse order), then
                    // the unpublished storage reclaim.
                    let birth_fault = walk(
                        &blocks,
                        birth["terminator"]["instruction"]["fault"]
                            .as_u64()
                            .unwrap(),
                    );
                    let birth_fault_kinds: Vec<&str> = birth_fault
                        .iter()
                        .map(|(kind, ..)| kind.as_str())
                        .collect();
                    assert_eq!(
                        birth_fault_kinds.as_slice(),
                        [
                            "field_residence_release",
                            "field_residence_release",
                            "reclaim_unpublished",
                            "return_fault",
                        ],
                        "construction fault releases children before storage"
                    );
                    Ok(())
                },
            )
            .unwrap();
    });
}

/// An `OwnedArrayFieldsNoHook` object whose `ArrayBox` field never proved
/// a birth-side provider keeps its lifecycle claim unavailable — the
/// pipeline fails closed instead of emitting a plain Home release over
/// live field residences.
#[test]
fn unproven_owned_array_field_never_publishes_a_teardown() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let mut compiler = MirCompiler::with_options(false);
        let mut verified = None;
        let outcome = compiler.compile_normal_with_published(
            request(
                "box Page {
                   items: ArrayBox = new ArrayBox()
                   birth() { }
                   touch() { me.items = new ArrayBox() }
                 }
                 static box Main {
                   main() {
                     local p = new Page()
                     return 0
                   }
                 }",
            ),
            |_view, verification| -> Result<(), String> {
                verified = Some(verification.is_ok());
                Ok(())
            },
        );
        assert!(
            outcome.is_err() || verified == Some(false),
            "unproven owned teardown must fail closed (outcome_ok={})",
            outcome.is_ok()
        );
    });
}

/// An installed user-object child carrying a sealed `ArrayBox` residence
/// publishes `object_field_release` on the containing field while the
/// child's own layout row carries the `owned_residences` mark — the
/// physical consumer walks those slots newest-first before the child's
/// plain Home release.
#[test]
fn installed_owned_array_child_publishes_residence_marks() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let mut compiler = MirCompiler::with_options(false);
        compiler
            .compile_normal_with_published(
                request(
                    "box Page {
                       items: ArrayBox = new ArrayBox()
                       birth() { }
                     }
                     box Parent {
                       items: ArrayBox = new ArrayBox()
                       child: Page = new Page()
                       birth() { }
                     }
                     static box Main {
                       main() {
                         local p = new Parent()
                         return 0
                       }
                     }",
                ),
                |view, verification| -> Result<(), String> {
                    assert!(verification.is_ok(), "{verification:?}");
                    let input = view.issue_lifecycle_physical_abi_input()?;
                    let json = emit_lifecycle_physical_abi_json(&input)?;
                    let decoded: serde_json::Value = serde_json::from_str(&json).unwrap();
                    // The child's layout row marks its owned `ArrayBox`
                    // residence — declaration ordinal zero here.
                    let page_layout = input
                        .layouts()
                        .iter()
                        .find(|layout| !layout.owned_residences().is_empty())
                        .expect("an owned-array layout mark");
                    assert_eq!(page_layout.owned_residences(), &[0]);
                    let object_id = page_layout.object_id() as u64;
                    let decoded_layouts = decoded["layouts"].as_array().unwrap();
                    let marked = decoded_layouts
                        .iter()
                        .find(|layout| layout["object_id"].as_u64() == Some(object_id))
                        .expect("layout row on the wire");
                    assert_eq!(marked["owned_residences"].as_array().unwrap().len(), 1);
                    // The caller's teardown releases the installed child
                    // through `object_field_release` — the op naming the
                    // marked child — before the parent's own residence and
                    // Home release, all newest-first.
                    let caller = decoded["functions"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .find(|row| row["name"] == "main")
                        .expect("caller row");
                    let operations: Vec<&serde_json::Value> = caller["blocks"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .filter_map(|block| {
                            block["terminator"]["instruction"]["operation"]
                                .as_object()
                                .map(|_| &block["terminator"]["instruction"]["operation"])
                        })
                        .collect();
                    let release = operations
                        .iter()
                        .find(|op| op["kind"] == "object_field_release")
                        .expect("installed child releases through object_field_release");
                    assert_eq!(
                        release["child_object_id"].as_u64(),
                        Some(object_id),
                        "the release names the owned-array child"
                    );
                    Ok(())
                },
            )
            .unwrap();
    });
}

/// A sealed qualified-static provider argument (`LayoutBox.class_size(0)`)
/// emits exactly one `ordinary_call` invoke inside the provider's Birth
/// unit — i64 result projected through `invoke_normal_result` — and the
/// projected value reaches the `birth_call` actuals with the i64 tag.
#[test]
fn provider_static_call_argument_serializes_inside_birth_unit() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let mut compiler = MirCompiler::with_options(false);
        compiler
            .compile_normal_with_published(
                request(
                    "static box LayoutBox {
                       class_size(unused) { return 8 }
                     }
                     box Page {
                       items: ArrayBox = new ArrayBox()
                       birth(size) { }
                     }
                     box Parent {
                       child: Page = new Page(LayoutBox.class_size(0))
                       birth() { }
                     }
                     static box Main {
                       main() { local p = new Parent() return 0 }
                     }",
                ),
                |view, verification| -> Result<(), String> {
                    assert!(verification.is_ok(), "{verification:?}");
                    let input = view.issue_lifecycle_physical_abi_input()?;
                    let json = emit_lifecycle_physical_abi_json(&input)?;
                    std::fs::write(
                        std::env::temp_dir().join("hako-provider-static-arg-wire.json"),
                        &json,
                    )
                    .unwrap();
                    let decoded: serde_json::Value = serde_json::from_str(&json).unwrap();
                    let functions = decoded["functions"].as_array().unwrap();
                    let callee_index = functions
                        .iter()
                        .position(|row| row["name"] == "LayoutBox.class_size/1")
                        .expect("static callee row") as u64;
                    let birth = functions
                        .iter()
                        .find(|row| row["role"] == "birth_unit" && row["name"] == "Parent.birth/0")
                        .expect("parent birth row");
                    let rows: Vec<&serde_json::Value> = birth["blocks"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .flat_map(|block| {
                            block["instructions"]
                                .as_array()
                                .unwrap()
                                .iter()
                                .chain(std::iter::once(&block["terminator"]["instruction"]))
                        })
                        .collect();
                    let calls: Vec<&&serde_json::Value> = rows
                        .iter()
                        .filter(|row| {
                            row["op"] == "invoke" && row["operation"]["kind"] == "ordinary_call"
                        })
                        .collect();
                    assert_eq!(calls.len(), 1, "exactly one argument call");
                    let call = &calls[0]["operation"];
                    assert_eq!(call["call"]["target"].as_u64(), Some(callee_index));
                    assert_eq!(call["result"], "i64");
                    assert_eq!(call["call"]["args"][0]["kind"], "i64");
                    // The projection sits in the call's normal landing and
                    // its value is the i64 actual on the birth_call edge.
                    let landing = birth["blocks"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .find(|block| block["id"].as_u64() == calls[0]["normal"].as_u64())
                        .expect("normal landing block");
                    let projection = &landing["instructions"][0]["instruction"];
                    assert_eq!(projection["op"], "invoke_normal_result");
                    let projected = projection["dst"].as_u64().unwrap();
                    let birth_call = rows
                        .iter()
                        .find(|row| row["operation"]["kind"] == "birth_call")
                        .expect("provider birth call");
                    let actual = birth_call["operation"]["call"]["args"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .find(|arg| arg["value"].as_u64() == Some(projected))
                        .expect("projected value is a birth actual");
                    assert_eq!(actual["kind"].as_u64(), Some(1));
                    Ok(())
                },
            )
            .unwrap();
    });
}

/// Independent owned-slot consumer witness: no Unit call or Array helper local.
#[test]
fn birth_owned_slot_consumer_publishes_original_provider_only_graph() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for optimize in [false, true] {
            let mut compiler = MirCompiler::with_options(optimize);
            compiler
                .compile_normal_with_published(
                    request(
                        "box Child {
                           left: ArrayBox = new ArrayBox()
                           right: ArrayBox = new ArrayBox()
                           birth() { }
                         }
                         box Parent {
                           items: ArrayBox = new ArrayBox()
                           first: Child = new Child()
                           second: Child = new Child()
                           birth() { }
                         }
                         static box Main {
                           main() { local p = new Parent() return 0 }
                         }",
                    ),
                    |view, verification| -> Result<(), String> {
                        assert!(verification.is_ok(), "{verification:?}");
                        let input = view.issue_lifecycle_physical_abi_input()?;
                        let json = emit_lifecycle_physical_abi_json(&input)?;
                        let decoded: serde_json::Value = serde_json::from_str(&json).unwrap();
                        let functions = decoded["functions"].as_array().unwrap();
                        assert_eq!(functions.len(), 3);
                        assert_eq!(
                            functions
                                .iter()
                                .filter(|f| f["role"] == "birth_unit")
                                .count(),
                            2
                        );
                        assert!(functions
                            .iter()
                            .all(|f| f["role"] == "birth_unit" || f["role"] == "root_i64"));
                        for f in functions {
                            for b in f["blocks"].as_array().unwrap() {
                                assert!(b["instructions"]
                                    .as_array()
                                    .unwrap()
                                    .iter()
                                    .all(|r| r["instruction"]["op"] != "array_residence_release"));
                            }
                        }
                        std::fs::write(
                            std::env::temp_dir()
                                .join(format!("hako-issued-owned-slot-consumer-{optimize}.json")),
                            json,
                        )
                        .unwrap();
                        Ok(())
                    },
                )
                .unwrap();
        }
    });
}
