#[test]
fn map_result_local_call_publishes_ordinary_map_callee_and_map_edge() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let source = r#"static box Main {
            main() { local m = make_map() return 30 }
            make_map() { return %{"a" => 1} }
        }"#;
        for optimize in [false, true] {
            MirCompiler::with_options(optimize)
                .compile_normal_with_published(request(source), |view, verification| {
                    assert!(verification.is_ok(), "{verification:?}");
                    let input = view.issue_lifecycle_physical_abi_input()?;
                    // The map-returning callee is emitted under the
                    // `ordinary_map` role (module-wide edge walk, C6-1/C6-2).
                    let map_function = input
                        .program()
                        .functions()
                        .iter()
                        .find(|function| {
                            matches!(
                                function.role(),
                                PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryMap { .. }
                            )
                        })
                        .ok_or_else(|| "ordinary_map callee missing".to_owned())?;
                    let text = emit_lifecycle_physical_abi_json(&input)?;
                    let decoded: Value = serde_json::from_str(&text).unwrap();
                    let rows = decoded["functions"].as_array().unwrap();
                    let map_row = rows
                        .iter()
                        .find(|row| row["role"] == "ordinary_map")
                        .ok_or_else(|| "ordinary_map JSON row missing".to_owned())?;
                    assert_eq!(map_row["name"].as_str().unwrap(), map_function.name());
                    let root_row = rows
                        .iter()
                        .find(|row| row["role"] == "root_i64")
                        .ok_or_else(|| "root JSON row missing".to_owned())?;
                    let calls: Vec<_> = root_row["blocks"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .flat_map(|block| {
                            block["instructions"]
                                .as_array()
                                .into_iter()
                                .flatten()
                                .map(|row| &row["instruction"])
                                .chain(std::iter::once(&block["terminator"]["instruction"]))
                        })
                        .filter(|instruction| instruction["operation"]["kind"] == "ordinary_call")
                        .collect();
                    assert_eq!(calls.len(), 1, "one ordinary_call edge");
                    assert_eq!(
                        calls[0]["operation"]["result"], "map",
                        "map-result call edge"
                    );
                    // The wire target is the callee's ordinal in the
                    // published functions array.
                    let map_ordinal = rows
                        .iter()
                        .position(|row| row["name"] == map_row["name"])
                        .ok_or_else(|| "ordinary_map ordinal missing".to_owned())?;
                    assert_eq!(
                        calls[0]["operation"]["call"]["target"].as_u64(),
                        Some(map_ordinal as u64),
                        "call edge targets the ordinary_map callee"
                    );
                    // The received lease is released by one map_end before the
                    // plain return.
                    let ends: Vec<_> = root_row["blocks"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .flat_map(|block| {
                            block["instructions"]
                                .as_array()
                                .into_iter()
                                .flatten()
                                .map(|row| &row["instruction"])
                                .chain(std::iter::once(&block["terminator"]["instruction"]))
                        })
                        .filter(|instruction| instruction["operation"]["kind"] == "map_end")
                        .collect();
                    assert_eq!(ends.len(), 1, "one map_end on the received map");
                    std::fs::write(
                        std::env::temp_dir().join(if optimize {
                            "hako-issued-physical-v2-map-call-optimized.json"
                        } else {
                            "hako-issued-physical-v2-map-call.json"
                        }),
                        text,
                    )
                    .unwrap();
                    Ok::<(), String>(())
                })
                .unwrap();
        }
    });
}

#[test]
fn map_value_wire_kind_is_explicit_and_has_no_object_identity() {
    use crate::mir::instruction::{MapInvokeOperation as Map, MapValueKind};
    for (kind, wire) in [
        (MapValueKind::I64, 1),
        (MapValueKind::Bool, 2),
        (MapValueKind::BorrowedHandle, 3),
    ] {
        let op = InvokeOperation::Map(Map::InstallValue {
            map: ValueId(1),
            key: ValueId(2),
            value: ValueId(3),
            kind,
        });
        let encoded =
            encode_invoke(&op, &BTreeMap::new(), &BTreeMap::new(), 0, Some(42), None, None).unwrap();
        assert_eq!(
            encoded,
            json!({"kind": "map_install_value", "map": 1,
            "key": 2, "value": 3, "value_kind": wire, "site": 42})
        );
    }
}

#[test]
fn map_install_text_publishes_inline_utf8_without_a_value_operand() {
    use crate::mir::instruction::MapInvokeOperation as Map;
    let op = InvokeOperation::Map(Map::InstallText {
        map: ValueId(1),
        key: ValueId(2),
        utf8: "sealed payload".into(),
    });
    let encoded =
        encode_invoke(&op, &BTreeMap::new(), &BTreeMap::new(), 0, Some(42), None, None).unwrap();
    assert_eq!(
        encoded,
        json!({"kind": "map_install_text", "map": 1, "key": 2,
        "utf8": "sealed payload", "site": 42})
    );
}

#[test]
fn map_install_empty_array_publishes_no_payload_operand() {
    use crate::mir::instruction::MapInvokeOperation as Map;
    let op = InvokeOperation::Map(Map::InstallEmptyArray {
        map: ValueId(1),
        key: ValueId(2),
    });
    let encoded =
        encode_invoke(&op, &BTreeMap::new(), &BTreeMap::new(), 0, Some(42), None, None).unwrap();
    assert_eq!(
        encoded,
        json!({"kind": "map_install_empty_array", "map": 1, "key": 2, "site": 42})
    );
}

#[test]
fn map_install_borrowed_array_publishes_ordered_map_elements() {
    use crate::mir::instruction::MapInvokeOperation as Map;
    let op = InvokeOperation::Map(Map::InstallBorrowedArray {
        map: ValueId(1),
        key: ValueId(2),
        elements: vec![ValueId(7), ValueId(7), ValueId(9)].into_boxed_slice(),
    });
    let encoded =
        encode_invoke(&op, &BTreeMap::new(), &BTreeMap::new(), 0, Some(42), None, None).unwrap();
    assert_eq!(
        encoded,
        json!({"kind": "map_install_borrowed_array", "map": 1, "key": 2,
        "elements": [7, 7, 9], "site": 42})
    );
}

#[test]
fn map_view_reads_publish_typed_index_and_text_operations() {
    use crate::mir::instruction::MapInvokeOperation as Map;
    let index = InvokeOperation::Map(Map::ArrayIndexMap {
        map: ValueId(1),
        utf8: "functions".into(),
        index: 0,
    });
    let text = InvokeOperation::Map(Map::MapGetText {
        map: ValueId(2),
        utf8: "name".into(),
    });
    assert_eq!(
        encode_invoke(
            &index,
            &BTreeMap::new(),
            &BTreeMap::new(),
            0,
            Some(42),
            None,
            None,
        )
        .unwrap(),
        json!({"kind": "map_array_index_map", "map": 1,
            "utf8": "functions", "index": 0, "site": 42})
    );
    assert_eq!(
        encode_invoke(
            &text,
            &BTreeMap::new(),
            &BTreeMap::new(),
            0,
            Some(43),
            None,
            None,
        )
        .unwrap(),
        json!({"kind": "map_get_text", "map": 2,
            "utf8": "name", "site": 43})
    );
}

#[test]
fn map_array_length_publishes_typed_i64_operation() {
    use crate::mir::instruction::MapInvokeOperation as Map;
    let operation = InvokeOperation::Map(Map::ArrayLength {
        map: ValueId(1),
        utf8: "params".into(),
    });
    let encoded = encode_invoke(
        &operation,
        &BTreeMap::new(),
        &BTreeMap::new(),
        0,
        Some(44),
        None,
        None,
    )
    .unwrap();
    assert_eq!(
        encoded,
        json!({"kind": "map_array_length", "map": 1,
            "utf8": "params", "site": 44})
    );
}

/// The nullable receiver call publishes its sealed wire shape: the callee
/// row carries the `ordinary_nullable_handle` role, the call site spells
/// `"nullable_handle"`, the callee materializes the Void null sentinel as
/// `const_null`, and the caller's cleanup owes `home_release_if_live` —
/// never an unconditional release on a maybe-null handle.
#[test]
fn nullable_receiver_call_serializes_nullable_handle_and_checked_release() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let mut compiler = MirCompiler::with_options(false);
        compiler
            .compile_normal_with_published(
                request(
                    "box Probe {
                       v: i64
                       birth(v) { me.v = v }
                       fetch(flag) {
                         if flag == 0 { return null }
                         return new Probe(7)
                       }
                       run(): i64 {
                         local h = me.fetch(0)
                         return 0
                       }
                     }
                     static box Main {
                       main() {
                         local p = new Probe(1)
                         return p.run()
                       }
                     }",
                ),
                |view, verification| -> Result<(), String> {
                    assert!(verification.is_ok(), "{verification:?}");
                    let input = view.issue_lifecycle_physical_abi_input()?;
                    let json = emit_lifecycle_physical_abi_json(&input)?;
                    let decoded: serde_json::Value = serde_json::from_str(&json).unwrap();
                    let functions = decoded["functions"].as_array().unwrap();
                    let callee = functions
                        .iter()
                        .find(|row| row["name"] == "Probe.fetch/1")
                        .expect("nullable callee row");
                    assert_eq!(callee["role"], "ordinary_nullable_handle");
                    let callee_instructions: Vec<&serde_json::Value> = callee["blocks"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .flat_map(|block| {
                            block["instructions"].as_array().unwrap().iter().chain(
                                std::iter::once(&block["terminator"]["instruction"]),
                            )
                        })
                        .map(|row| row.get("instruction").unwrap_or(row))
                        .collect();
                    assert!(
                        callee_instructions
                            .iter()
                            .any(|row| row["op"] == "const_null"),
                        "callee materializes the Void null sentinel: {callee_instructions:?}"
                    );
                    let caller = functions
                        .iter()
                        .find(|row| row["name"] == "Probe.run/0")
                        .expect("caller row");
                    let instructions: Vec<&serde_json::Value> = caller["blocks"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .flat_map(|block| {
                            block["instructions"].as_array().unwrap().iter().chain(
                                std::iter::once(&block["terminator"]["instruction"]),
                            )
                        })
                        .map(|row| row.get("instruction").unwrap_or(row))
                        .collect();
                    let call = instructions
                        .iter()
                        .find(|row| row["operation"]["kind"] == "ordinary_call")
                        .expect("nullable call edge");
                    assert_eq!(call["operation"]["result"], "nullable_handle");
                    let releases: Vec<&serde_json::Value> = instructions
                        .iter()
                        .filter(|row| row["operation"]["kind"] == "home_release_if_live")
                        .copied()
                        .collect();
                    assert_eq!(
                        releases.len(),
                        1,
                        "exactly one checked release for the nullable result"
                    );
                    assert!(
                        !instructions
                            .iter()
                            .any(|row| row["operation"]["kind"] == "home_release"),
                        "a maybe-null result never owes an unconditional release"
                    );
                    Ok(())
                },
            )
            .unwrap();
    });
}

#[test]
fn checked_string_transport_preserves_bytes_operand_roles_and_sites() {
    use crate::mir::instruction::{StringInvokeOperand as Operand, StringInvokeOperation as Op};
    let encode = |operation| encode_invoke(&InvokeOperation::String(operation),
        &BTreeMap::new(), &BTreeMap::new(), 0, Some(42), None, None).unwrap();
    let add = Op::Add { left: Operand::StaticText { utf8: "猫\0".into() },
        right: Operand::NormalInteger(ValueId(7)) };
    assert_eq!(encode(add), json!({"kind":"string_add", "site":42,
        "left":{"kind":"static_text", "utf8":"猫\0"},
        "right":{"kind":"normal_integer", "value":7}}));
    assert_eq!(encode(Op::Print { value: Operand::OwnedString(ValueId(9)) }),
        json!({"kind":"string_print", "site":42, "value":{"kind":"owned_string", "value":9}}));
    assert_eq!(encode(Op::End { value:ValueId(9) }),
        json!({"kind":"string_end", "site":42, "value":9}));
}


#[test]
fn owned_array_transport_preserves_descriptor_and_borrowed_result_role() {
    use crate::mir::instruction::{
        InvokeNormalResultKind, OwnedArrayInvokeOperation as Op, OwnedArraySlotDescriptorV1,
    };
    let object = hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(7).unwrap();
    for nullable in [false, true] {
        let descriptor = OwnedArraySlotDescriptorV1::new(object, nullable);
        for (operation, kind, extra) in [
            (Op::Claim { array: ValueId(1), descriptor }, "array_owned_claim", None),
            (Op::Append { array: ValueId(1), descriptor, candidate: ValueId(2) }, "array_owned_append", Some(("value", 2))),
            (Op::Get { array: ValueId(1), descriptor, index: ValueId(3) }, "array_owned_get", Some(("index", 3))),
            (Op::End { array: ValueId(1), descriptor }, "array_owned_end", None),
        ] {
            let operation = InvokeOperation::OwnedArray(operation);
            let mut expected = json!({"kind":kind, "array":1, "object_id":7, "nullable":nullable, "site":42});
            if let Some((key, value)) = extra { expected[key] = json!(value); }
            assert_eq!(encode_invoke(&operation, &BTreeMap::new(), &BTreeMap::new(),
                0, Some(42), None, None).unwrap(), expected);
            assert_eq!(operation.normal_result_kind(), if kind == "array_owned_get" {
                Some(InvokeNormalResultKind::BorrowedNullableHandle)
            } else { None });
            assert_ne!(operation.normal_result_kind(), Some(InvokeNormalResultKind::NullableHandle));
            // Missing diagnostic identity cannot be serialized into a valid
            // checked operation, even when no final ABI view was supplied.
            assert!(encode_invoke(&operation, &BTreeMap::new(), &BTreeMap::new(),
                0, None, None, None).is_err());
        }
    }
}
