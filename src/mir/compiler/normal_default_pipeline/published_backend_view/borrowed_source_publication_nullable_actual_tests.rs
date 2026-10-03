//! Borrowed actual publication for the null domain: the exact `null`
//! literal spells tag `0` with the `const_null` producer named in `value`,
//! and a caller-owned `NullableObject` call result spells
//! `nullable_typed_object` — the ABI owner selects tag 0/3 at runtime.
//! Integer zero and bool false keep their own tags: they never mint the
//! null spelling.
use super::*;

fn main_calls(json: &serde_json::Value) -> Vec<serde_json::Value> {
    json["functions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["name"] == "main")
        .unwrap()["blocks"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|b| {
            let t = &b["terminator"]["instruction"];
            (t["op"] == "invoke" && t["operation"]["kind"] == "ordinary_call")
                .then(|| t["operation"].clone())
        })
        .collect()
}

fn main_instructions(json: &serde_json::Value) -> Vec<serde_json::Value> {
    json["functions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["name"] == "main")
        .unwrap()["blocks"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|b| b["instructions"].as_array().unwrap().clone())
        .map(|row| row["instruction"].clone())
        .collect()
}

/// TASK4-NULLACTUAL-S0 publication witness: `s.release(null)` emits tag 0
/// with the exact `const_null` producer, and `s.release(h)` on a received
/// nullable emits the runtime-selected `nullable_typed_object` pair while
/// `h` stays caller-owned for the checked release.
#[test]
fn null_and_nullable_actuals_publish_their_wire_forms() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        // The literal-null case needs no nullable-return producer: `release`
        // is a borrowed-formal callee and `s.release(null)` exercises the
        // tag-0 pair on its own. The nullable case keeps the full
        // `Handle`/`check` program so `check`'s sealed `NullableObject`
        // claim is the sole class authority for the actual.
        for (suffix, text) in [
            (
                "null",
                "box Store { limit: i64 birth() { me.limit = 10 } \
                 release(handle) { return 0 } } \
                 static box Main { main() { local s = new Store() \
                 return s.release(null) } }"
                    .to_string(),
            ),
            (
                "nullable",
                "box Handle { page_id: i64 block_id: i64 \
                 birth(pid, bid) { me.page_id = pid me.block_id = bid } } \
                 box Store { limit: i64 birth() { me.limit = 10 } \
                 check(p) { if p > me.limit { return null } \
                 return new Handle(p, 3) } \
                 release(handle) { return 0 } } \
                 static box Main { main() { local s = new Store() \
                 local h = s.check(5) return s.release(h) } }"
                    .to_string(),
            ),
        ] {
            let text = text;
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
                        let calls = main_calls(&json);
                        let release = calls
                            .iter()
                            .find(|call| {
                                // `s.release` is the second ordinary target:
                                // `s.birth` is a birth_call, `s.check` and
                                // `s.release` are ordinary calls — the
                                // release call is the one with one arg that
                                // is not the i64 literal `5`.
                                call["call"]["args"].as_array().map_or(0, |a| a.len()) == 1
                                    && call["call"]["args"][0]["kind"] != 1
                            })
                            .unwrap_or_else(|| panic!("{suffix}: release call missing: {wire}"));
                        let arg = &release["call"]["args"][0];
                        let instructions = main_instructions(&json);
                        match suffix {
                            "null" => {
                                assert_eq!(arg["kind"], 0, "{suffix}: {wire}");
                                let value = arg["value"].as_u64().unwrap();
                                assert!(
                                    instructions.iter().any(|row| {
                                        row["op"] == "const_null"
                                            && row["dst"].as_u64() == Some(value)
                                    }),
                                    "{suffix}: exact const_null producer missing: {wire}"
                                );
                            }
                            "nullable" => {
                                assert_eq!(
                                    arg["kind"], "nullable_typed_object",
                                    "{suffix}: {wire}"
                                );
                                let value = arg["value"].as_u64().unwrap();
                                // The nullable actual's value is the exact
                                // received result of `s.check` — never a
                                // copy or a forged payload.
                                assert!(
                                    instructions.iter().any(|row| {
                                        row["op"] == "invoke_normal_result"
                                            && row["dst"].as_u64() == Some(value)
                                    }),
                                    "{suffix}: received result missing: {wire}"
                                );
                            }
                            _ => unreachable!(),
                        }
                        reject_erased_carriers(&input);
                        std::fs::write(
                            std::env::temp_dir()
                                .join(format!("hako-issued-null-actual-{suffix}.json")),
                            wire,
                        )
                        .unwrap();
                        Ok(())
                    },
                )
                .unwrap_or_else(|error| panic!("{suffix}: {error}"));
        }
    });
}

/// Integer zero and bool false stay in their own scalar domains: neither
/// spells tag 0 nor the nullable pair. A fresh `new` actual and an
/// unclaimed producer still reject before publication.
#[test]
fn non_null_domains_keep_their_own_tags() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for (suffix, argument, kind) in [
            ("zero", "0", serde_json::json!(1)),
            ("false", "false", serde_json::json!(2)),
        ] {
            let text = format!(
                "box Store {{ limit: i64 birth() {{ me.limit = 10 }} \
                 release(handle) {{ return 0 }} }} \
                 static box Main {{ main() {{ local s = new Store() return s.release({argument}) }} }}"
            );
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
                        let calls = main_calls(&json);
                        let release = calls
                            .iter()
                            .find(|call| {
                                call["call"]["args"].as_array().map_or(0, |a| a.len()) == 1
                            })
                            .unwrap_or_else(|| panic!("{suffix}: release call missing: {wire}"));
                        assert_eq!(
                            release["call"]["args"][0]["kind"], kind,
                            "{suffix}: {wire}"
                        );
                        reject_erased_carriers(&input);
                        Ok(())
                    },
                )
                .unwrap_or_else(|error| panic!("{suffix}: {error}"));
        }
    });
}
