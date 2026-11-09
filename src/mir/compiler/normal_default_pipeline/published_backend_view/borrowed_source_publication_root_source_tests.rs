//! Root-source retention for bound i64 returns: the finalized root handoff
//! keeps the verified main source for checked birth actuals and local-call
//! inventories independently of terminal-map presence, and a proven-i64
//! bound return carries its own source relation. A retained loan without a
//! result ABI stays fail-closed at the unchanged result boundary.
use super::*;

/// TASK4-ROOTSOURCE-S0 publication witness: `return <bound-i64-local>` and
/// trivial integer returns publish through the retained root source. The
/// scalar classifier's stored i64 kind is the sole proof; the handoff only
/// transports the exact owner/sites relation.
#[test]
fn bound_i64_returns_publish_through_root_source() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for (label, tail, expect_invoke) in [
            (
                "bound-call-result",
                "local h = new Item(1, 2) local r = s.release(h) return r",
                true,
            ),
            (
                "bound-literal",
                "local h = new Item(1, 2) local x = 5 return x",
                false,
            ),
            (
                "bound-copy-chain",
                "local h = new Item(1, 2) local x = 5 local y = x return y",
                false,
            ),
            (
                "bound-trivial-add",
                "local h = new Item(1, 2) local x = 5 return x + 1",
                false,
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
                    require_artifact_verification(verification);
                    let input = view.issue_lifecycle_physical_abi_input()?;
                    let wire = super::super::super::physical_program_json::emit_lifecycle_physical_abi_json(&input)?;
                    let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                    let functions = json["functions"].as_array().unwrap();
                    let caller = functions
                        .iter()
                        .find(|f| f["name"] == "main")
                        .expect("caller row");
                    assert_eq!(caller["role"], "root_i64", "{label}: {wire}");
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
                    assert!(
                        instructions.iter().any(|row| row["op"] == "return"),
                        "{label}: main publishes its return edge: {wire}"
                    );
                    if expect_invoke {
                        assert!(
                            instructions.iter().any(|row| row["op"] == "invoke_normal_result"),
                            "{label}: bound call result carries its invoke: {wire}"
                        );
                        let call = instructions
                            .iter()
                            .find(|row| row["operation"]["kind"] == "ordinary_call")
                            .expect("call edge");
                        assert!(
                            call["operation"]["call"]["args"].as_array().unwrap().iter().any(|arg| {
                                arg["kind"] == 3
                            }),
                            "{label}: typed-home actual spells the object payload tag: {wire}"
                        );
                    }
                    Ok(())
                })
                .unwrap_or_else(|error| panic!("{label}: {error}"));
        }
        // The bound call result without a `new` argument exercises the same
        // path through the plain `new Store` actual.
        let text = "box Store { limit: i64 birth() { me.limit = 10 } release(p) { return 0 } } \
            static box Main { main() { local s = new Store() local r = s.release(5) return r } }";
        MirCompiler::with_options(false)
            .compile_normal_with_published(request(text), |view, verification| -> Result<(), String> {
                require_artifact_verification(verification);
                let input = view.issue_lifecycle_physical_abi_input()?;
                let wire = super::super::super::physical_program_json::emit_lifecycle_physical_abi_json(&input)?;
                let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                let caller = json["functions"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|f| f["name"] == "main")
                    .expect("caller row");
                assert_eq!(caller["role"], "root_i64", "{wire}");
                Ok(())
            })
            .unwrap_or_else(|error| panic!("bound-call-noarg: {error}"));
    });
}

/// A retained root-source loan whose exits name no result ABI stays
/// fail-closed at the unchanged result boundary: unproven bound returns
/// (bool/object/text) never mint an i64 terminal; proven pure scalar roots
/// publish through their source-certified Static V2 route.
#[test]
fn unproven_bound_returns_stay_fail_closed() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for (label, tail, stop) in [
            (
                "bound-bool",
                "local b = true return b",
                "artifact-root-bool-result-unsupported",
            ),
            (
                "bound-object",
                "local g = new Store() return g",
                "root-result/source-unsupported",
            ),
            (
                "bound-text",
                "local t = \"hi\" return t",
                "admission-retained-root-result-missing",
            ),
        ] {
            let text = format!(
                "box Store {{ limit: i64 birth() {{ me.limit = 10 }} \
                release(p) {{ return 0 }} }} \
                static box Main {{ main() {{ local s = new Store() {tail} }} }}",
            );
            let error = MirCompiler::with_options(false)
                .compile_normal_with_published(request(&text), |view, verification| {
                    require_artifact_verification(verification);
                    view.issue_lifecycle_physical_abi_input().map(|_| ())
                })
                .err()
                .unwrap_or_else(|| panic!("{label}: unexpectedly admitted"));
            assert!(error.contains(stop), "{label}: {error}");
        }
        // B36: the original literal/read receipt authorizes Static V2, not a lifecycle ABI.
        let called = std::cell::Cell::new(false);
        MirCompiler::with_options(false)
            .compile_normal_with_published(
                request("static box Main { main() { local x = 5 return x } }"),
                |view, verification| -> Result<(), String> {
                    called.set(true);
                    require_artifact_verification(verification);
                    assert_eq!(view.route(), PublishedStaticMethodRouteV1::CanonicalTyped);
                    assert!(!view.has_lifecycle_instructions());
                    assert!(view
                        .retained_root_source()
                        .expect("original scalar source")
                        .validate_plain_scalar_read_root_v1(view.module())?);
                    Ok(())
                },
            )
            .unwrap_or_else(|error| panic!("pure-bound-lit: {error}"));
        assert!(
            called.get(),
            "pure-bound-lit must reach the canonical consumer"
        );
    });
}

#[test]
fn explicit_share_source_rejects_an_unrequested_physical_clone() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let text = "box Page { value: i64 birth() { me.value = 7 } }
        static box Main { main() { local page = new Page()
        local first = share move page local second = share first return 7 } }";
        MirCompiler::with_options(false)
            .compile_normal_with_published(
                request(text),
                |view, verification| -> Result<(), String> {
                    require_artifact_verification(verification);
                    view.issue_lifecycle_physical_abi_input()?;
                    let main = &view.module().functions["main"];
                    let original = super::super::shared_initializer::bindings(
                        Some(view.module()),
                        main,
                        Some(&view),
                    )?;
                    let extra = original
                        .iter()
                        .find_map(|(_, instruction)| {
                            matches!(
                                instruction,
                                MirInstruction::Invoke {
                                    operation: InvokeOperation::Shared(
                                        crate::mir::instruction::SharedInvokeOperation::Clone { .. }
                                    ),
                                    ..
                                }
                            )
                            .then(|| instruction.clone())
                        })
                        .ok_or("explicit share must select Clone")?;
                    let mut changed = view.module().clone();
                    let id = crate::mir::BasicBlockId::new(900_101);
                    let mut block = crate::mir::BasicBlock::new(id);
                    block.terminator = Some(extra);
                    changed
                        .functions
                        .get_mut("main")
                        .unwrap()
                        .blocks
                        .insert(id, block);
                    // Check the publication coverage owner directly: a generic CFG/SSA
                    // refusal must not substitute for the original-source obligation.
                    let error = super::super::shared_initializer::bindings(
                        Some(&changed),
                        &changed.functions["main"],
                        Some(&view),
                    )
                    .expect_err("no source share authorizes the extra Clone");
                    assert!(
                        error.contains("shared-initializer/source-coverage"),
                        "{error}"
                    );
                    Ok(())
                },
            )
            .unwrap();
        for tail in [
            "local v = s.right return v",
            "local t = share s local v = t.right return v",
            "local t = move s local v = t.right return v",
            "return s.right",
        ] {
            let text = format!("box Pair {{ left: i64 right: i64 birth(left, right) {{ me.left = left me.right = right }} }}
                static box Main {{ main() {{ local s = share new Pair(4, 5) {tail} }} }}");
            MirCompiler::with_options(false)
                .compile_normal_with_published(
                    request(&text),
                    |view, verification| -> Result<(), String> {
                        require_artifact_verification(verification);
                        view.issue_lifecycle_physical_abi_input()?;
                        Ok(())
                    },
                )
                .unwrap_or_else(|error| panic!("fresh Shared payload read {tail}: {error}"));
        }
    });
}
