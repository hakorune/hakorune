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
                    classify_pretransform_report(verification);
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
                classify_pretransform_report(verification);
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
/// (bool/object) never mint an i64 terminal, and pure mains still stop at
/// the unchanged completion gate.
#[test]
fn unproven_bound_returns_stay_fail_closed() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for (label, tail, stop) in [
            ("bound-bool", "local b = true return b", "retained-root-result-missing"),
            (
                "bound-object",
                "local g = new Store() return g",
                "retained-root-result-missing",
            ),
            (
                "bound-text",
                "local t = \"hi\" return t",
                "artifact-source-unavailable",
            ),
        ] {
            let text = format!(
                "box Store {{ limit: i64 birth() {{ me.limit = 10 }} \
                release(p) {{ return 0 }} }} \
                static box Main {{ main() {{ local s = new Store() {tail} }} }}",
            );
            let error = MirCompiler::with_options(false)
                .compile_normal_with_published(request(&text), |view, verification| {
                    classify_pretransform_report(verification);
                    view.issue_lifecycle_physical_abi_input().map(|_| ())
                })
                .err()
                .unwrap_or_else(|| panic!("{label}: unexpectedly admitted"));
            assert!(error.contains(stop), "{label}: {error}");
        }
        // A pure main without any checked `new` has no Completion artifact
        // at all — the unchanged root-completion gate still stops it.
        let error = MirCompiler::with_options(false)
            .compile_normal_with_published(
                request("static box Main { main() { local x = 5 return x } }"),
                |view, verification| {
                    classify_pretransform_report(verification);
                    view.issue_lifecycle_physical_abi_input().map(|_| ())
                },
            )
            .err()
            .unwrap_or_else(|| panic!("pure-bound-lit: unexpectedly admitted"));
        assert!(
            error.contains("artifact-root-completion-unavailable"),
            "pure-bound-lit: {error}"
        );
    });
}
