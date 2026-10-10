//! Unannotated borrowed-I64 result publication: the source-proven I64
//! return set of a declaration-unannotated callee projects the I64 invoke
//! through the existing physical owner — no annotation is manufactured.
use super::*;

/// TASK4-I64RESULT-S0 publication witness: an unannotated borrowed callee's
/// source-proven I64 result publishes the I64 invoke through the existing
/// local/direct-return forms — the declaration stays unannotated and no
/// result annotation is manufactured.
#[test]
fn unannotated_borrowed_i64_result_publishes_call_forms() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for (label, callee_body, tail, kind) in [
            ("direct-return", "return 0", "return s.release(5)", 1),
            (
                "bound-forward",
                "return 0",
                "local r = s.release(5) return s.release(r)",
                1,
            ),
            ("discard", "return 0", "s.release(5) return 1", 1),
            ("multi-exit", "if 1 > 0 { return 0 } return 1", "return s.release(5)", 1),
        ] {
            let text = format!(
                "box Store {{ limit: i64 birth() {{ me.limit = 10 }} \
                release(handle) {{ {callee_body} }} }} \
                static box Main {{ main() {{ local s = new Store() {tail} }} }}",
            );
            MirCompiler::with_options(false)
                .compile_normal_with_published(request(&text), |view, verification| -> Result<(), String> {
                    classify_pretransform_report(verification);
                    let input = view.issue_lifecycle_physical_abi_input()?;
                    let wire = super::super::super::physical_program_json::emit_lifecycle_physical_abi_json(&input)?;
                    let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                    let functions = json["functions"].as_array().unwrap();
                    let callee = functions
                        .iter()
                        .find(|f| f["name"] == "Store.release/1")
                        .expect("Store.release/1 published");
                    assert_eq!(callee["role"], "ordinary_i64", "{label}: {wire}");
                    assert!(
                        callee["params"].as_array().unwrap().iter().any(|p| {
                            p["representation"] == "borrowed_kind_payload_v1"
                        }),
                        "{label}: opaque formal keeps the borrowed carrier: {wire}"
                    );
                    let caller = functions
                        .iter()
                        .find(|f| f["name"] == "main")
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
                    let calls: Vec<&serde_json::Value> = instructions
                        .iter()
                        .filter(|row| {
                            row["operation"]["kind"] == "ordinary_call"
                                && row["operation"]["result"] == "i64"
                        })
                        .copied()
                        .collect();
                    assert!(
                        !calls.is_empty(),
                        "{label}: at least one I64 ordinary call edge: {wire}"
                    );
                    for call in &calls {
                        assert!(
                            call["operation"]["call"]["args"].as_array().unwrap().iter().any(|arg| {
                                arg["kind"] == kind
                            }),
                            "{label}: borrowed actual spells payload tag {kind}: {wire}"
                        );
                    }
                    assert!(
                        instructions.iter().any(|row| row["op"] == "invoke_normal_result"),
                        "{label}: invoke mints its normal result: {wire}"
                    );
                    Ok(())
                })
                .unwrap_or_else(|error| panic!("{label}: {error}"));
        }
        // The accepted local-new actual keeps the tagged-object carrier.
        for (label, tail) in [
            (
                "typed-home-return",
                "local h = new Item(1, 2) return s.release(h)",
            ),
            (
                "typed-home-discard",
                "local h = new Item(1, 2) s.release(h) return 1",
            ),
        ] {
            let text = format!(
                "box Item {{ id: i64 serial: i64 birth(id, serial) {{ me.id = id me.serial = serial }} }} \
                box Store {{ limit: i64 birth() {{ me.limit = 10 }} \
                release(handle) {{ return 0 }} }} \
                static box Main {{ main() {{ local s = new Store() {tail} }} }}",
            );
            MirCompiler::with_options(false)
                .compile_normal_with_published(request(&text), |view, verification| -> Result<(), String> {
                    classify_pretransform_report(verification);
                    let input = view.issue_lifecycle_physical_abi_input()?;
                    let wire = super::super::super::physical_program_json::emit_lifecycle_physical_abi_json(&input)?;
                    let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                    let functions = json["functions"].as_array().unwrap();
                    let caller = functions
                        .iter()
                        .find(|f| f["name"] == "main")
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
                        .expect("call edge");
                    assert_eq!(call["operation"]["result"], "i64", "{label}: {wire}");
                    assert!(
                        call["operation"]["call"]["args"].as_array().unwrap().iter().any(|arg| {
                            arg["kind"] == 3
                        }),
                        "{label}: typed-home actual spells the object payload tag: {wire}"
                    );
                    Ok(())
                })
                .unwrap_or_else(|error| panic!("{label}: {error}"));
        }
    });
}

#[test]
fn static_direct_return_records_one_root_invoke() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let text = "static box Layout { word() { return 8 } relay() { return me.word() } } static box Main { main() { return 0 } }";
        MirCompiler::with_options(false)
            .compile_normal_for_mir_json(request(text), |view, verification| -> Result<(), String> {
                classify_pretransform_report(verification);
                let relay = &view.module().functions["Layout.relay/0"];
                let instructions: Vec<_> = relay.blocks.values()
                    .flat_map(|block| block.instructions.iter().chain(block.terminator.iter()))
                    .collect();
                assert_eq!(instructions.iter().filter(|row| matches!(row,
                    crate::mir::MirInstruction::Invoke { operation:
                        crate::mir::instruction::InvokeOperation::Call { result:
                            crate::mir::instruction::InvokeCallResultKind::I64, .. }, .. })).count(), 1);
                assert_eq!(instructions.iter().filter(|row| matches!(row,
                    crate::mir::MirInstruction::InvokeNormalResult { .. })).count(), 1);
                Ok(())
            })
            .unwrap();
    });
}
