//! Guarded-formal field-read publication (PARAMFIELD): the dominated
//! `formal.field` receiver admits through the co-sealed borrowed object
//! view, publishes one `object_field_get`, and stays fail-closed at named
//! stops for every out-of-scope cohort.
use super::*;

/// A dominated `handle.page_id` read on a guarded `OpaqueHandle` formal
/// publishes: the callee param carries the sealed `object_view`, exactly
/// one `object_field_get` reads the borrowed payload, and the formal base
/// never picks up a release or end obligation — the caller still owns it.
#[test]
fn guarded_formal_field_read_publishes_object_view() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for (label, caller) in [(
            "handle",
            "local h = s.check(5) return s.release(h)",
        )] {
            let text = "box Handle { page_id: i64 block_id: i64 birth(pid, bid) { me.page_id = pid me.block_id = bid } } \
                box Store { limit: i64 birth() { me.limit = 10 } \
                check(p) { if p > me.limit { return null } return new Handle(p, 3) } \
                release(handle) { if handle == null { return 0 } local id = handle.page_id if id > 0 { return 1 } return 0 } } \
                static box Main { main() { local s = new Store() ".to_string()
                + caller
                + " } }";
            MirCompiler::with_options(false)
                .compile_normal_with_published(
                    request(&text),
                    |view, verification| -> Result<(), String> {
                        classify_pretransform_report(verification);
                        let input = view.issue_lifecycle_physical_abi_input()?;
                        let wire =
                            super::super::super::physical_program_json::emit_lifecycle_physical_abi_json(
                                &input,
                            )?;
                        let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                        let functions = json["functions"].as_array().unwrap();
                        let callee = functions
                            .iter()
                            .find(|f| f["name"] == "Store.release/1")
                            .expect("Store.release/1 published");
                        let params = callee["params"].as_array().unwrap();
                        assert_eq!(params.len(), 1, "{label}: {wire}");
                        assert_eq!(
                            params[0]["representation"], "borrowed_kind_payload_v1",
                            "{label}: {wire}"
                        );
                        let object_view = params[0]["object_view"]
                            .as_u64()
                            .expect("sealed object view rides the param row");
                        let layouts = json["layouts"].as_array().unwrap();
                        assert!(
                            layouts
                                .iter()
                                .any(|row| row["object_id"].as_u64() == Some(object_view)),
                            "{label}: object_view names a declared layout: {wire}"
                        );
                        let instructions: Vec<&serde_json::Value> = callee["blocks"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .flat_map(|block| {
                                block["instructions"]
                                    .as_array()
                                    .unwrap()
                                    .iter()
                                    .map(|row| &row["instruction"])
                                    .chain(std::iter::once(
                                        &block["terminator"]["instruction"],
                                    ))
                            })
                            .collect();
                        let formal = params[0]["value"].as_u64().unwrap();
                        let reads: Vec<&serde_json::Value> = instructions
                            .iter()
                            .filter(|row| {
                                row["op"] == "object_field_get"
                                    && row["base"].as_u64() == Some(formal)
                                    && row["object_id"].as_u64() == Some(object_view)
                            })
                            .copied()
                            .collect();
                        assert_eq!(
                            reads.len(),
                            1,
                            "{label}: exactly one field read on the guarded formal: {wire}"
                        );
                        assert!(
                            !instructions.iter().any(|row| {
                                matches!(
                                    row["operation"]["kind"].as_str(),
                                    Some("home_release") | Some("home_release_if_live")
                                        | Some("reclaim_unpublished")
                                        | Some("object_field_release")
                                ) && row["operation"]["value"].as_u64() == Some(formal)
                            }),
                            "{label}: a borrowed formal never owes a release: {wire}"
                        );
                        std::fs::write(
                            std::env::temp_dir()
                                .join(format!("hako-issued-param-field-{label}.json")),
                            wire,
                        )
                        .unwrap();
                        Ok(())
                    },
                )
                .unwrap_or_else(|error| panic!("{label}: {error}"));
        }
        // A `null`-only caller gives the formal no object-class claim: no
        // view is minted, so the callee stays on the bounded sibling lane —
        // the read is unclaimed and the legacy `FieldGet` projection fails
        // closed on its physical `unproved-copy` arm.
        let text = "box Handle { page_id: i64 block_id: i64 birth(pid, bid) { me.page_id = pid me.block_id = bid } } \
            box Store { limit: i64 birth() { me.limit = 10 } \
            check(p) { if p > me.limit { return null } return new Handle(p, 3) } \
            release(handle) { if handle == null { return 0 } local id = handle.page_id if id > 0 { return 1 } return 0 } } \
            static box Main { main() { local s = new Store() local h = s.check(5) return s.release(null) } }";
        let error = MirCompiler::with_options(false)
            .compile_normal_with_published(request(text), |view, verification| {
                classify_pretransform_report(verification);
                view.issue_lifecycle_physical_abi_input().map(|_| ())
            })
            .err()
            .unwrap_or_else(|| panic!("null-actual: unexpectedly admitted"));
        assert!(error.contains("unproved-copy"), "null-actual: {error}");
    });
}

/// PARAMFIELD census pin: a nullable/object formal's guarded use inside an
/// instance callee stays fail-closed at its named stop until the ordered
/// task-4 rows land. The guarded field-read slice admitted the dominated
/// `handle.page_id` receiver and moved its terminal-return variants to the
/// FIELDRESULT stop (`borrowed-result/source-not-i64`); the remaining
/// variants re-pin their named stop — the callee-side use envelope
/// (`artifact-source-unavailable`) and the literal-null/inline-new/
/// received-nullable actual arm
/// (`borrowed-actual/unsupported-or-unavailable`).
#[test]
fn parameter_field_frontiers_stay_fail_closed() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for (label, callee_sig, callee_body, caller_tail, stop) in [
            (
                "pa-guarded-nullarg",
                "",
                "if handle == null { return 0 } return handle.page_id",
                "return s.release(null)",
                "borrowed-result/source-not-i64",
            ),
            (
                "pa-guarded-nullarg-cmp",
                "",
                "if handle == null { return 0 } if handle.page_id < 0 { return 0 } return handle.page_id",
                "return s.release(null)",
                "borrowed-result/source-not-i64",
            ),
            (
                "pa-unguarded-nullarg",
                "",
                "return handle.page_id",
                "return s.release(null)",
                "artifact-source-unavailable",
            ),
            (
                "pa-guarded-handlearg",
                "",
                "if handle == null { return 0 } return handle.page_id",
                "local h = s.check(5) return s.release(h)",
                "borrowed-result/source-not-i64",
            ),
            (
                "pa-newarg-unguarded",
                "",
                "return handle.page_id",
                "return s.release(new Handle(1, 2))",
                "artifact-source-unavailable",
            ),
            (
                "pa-newarg-guarded",
                "",
                "if handle == null { return 0 } return handle.page_id",
                "return s.release(new Handle(1, 2))",
                "borrowed-actual/unsupported-or-unavailable",
            ),
            (
                "pa-trivial-newarg",
                "",
                "return 0",
                "return s.release(new Handle(1, 2))",
                "borrowed-actual/unsupported-or-unavailable",
            ),
            (
                "pa-trivial-handlearg-alias",
                "",
                "return 0",
                // A rebound alias is not the nullable binding itself:
                // `q`'s bound value names `h`'s binding, not `q`'s, so the
                // actual keeps rejecting before publication.
                "local h = s.check(5) local q = h return s.release(q)",
                "borrowed-actual/unsupported-or-unavailable",
            ),
            (
                "pa-eqnull-newarg",
                "",
                "if handle == null { return 0 } return 1",
                "return s.release(new Handle(1, 2))",
                "borrowed-actual/unsupported-or-unavailable",
            ),
            (
                "pa-eqnull-localnewarg",
                "",
                "if handle == null { return 0 } return 1",
                "local h = new Handle(1, 2) return s.release(h)",
                "admission-function-not-birth",
            ),
        ] {
            let text = format!(
                "box Handle {{ page_id: i64 block_id: i64 birth(pid, bid) {{ me.page_id = pid me.block_id = bid }} }} \
                box Store {{ limit: i64 birth() {{ me.limit = 10 }} \
                check(p) {{ if p > me.limit {{ return null }} return new Handle(p, 3) }} \
                release(handle){callee_sig} {{ {callee_body} }} }} \
                static box Main {{ main() {{ local s = new Store() {caller_tail} }} }}",
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
    });
}
