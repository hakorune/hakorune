//! Borrowed/null equality publication: the source-admitted `==` against
//! the exact `null` literal becomes the dedicated `borrowed_null_compare`
//! physical row — never the integer `compare` — while `!=`, non-null
//! siblings and ordinary integer equality keep their own spellings.
use super::*;

/// TASK4-NULLCOMPARE-S0 publication witness: `if handle == null` on a
/// borrowed formal publishes the dedicated row in either operand order,
/// carries the exact `const_null` producer as its sibling, and keeps the
/// borrowed tagged representation on the callee parameter.
#[test]
fn null_compare_publishes_the_dedicated_physical_row() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for (suffix, condition, argument) in [
            ("hi", "handle == null", "5"),
            ("swapped", "null == handle", "5"),
            ("object", "handle == null", "s"),
        ] {
            let text = format!(
                "box Store {{ limit: i64 birth() {{ me.limit = 10 }} \
                 check(handle) {{ if {condition} {{ return 7 }} return 3 }} }} \
                 static box Main {{ main() {{ local s = new Store() return s.check({argument}) }} }}"
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
                        let callee = json["functions"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .find(|f| f["name"] == "Store.check/1")
                            .unwrap();
                        let tagged: Vec<u64> = callee["params"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .filter(|p| p["representation"] == "borrowed_kind_payload_v1")
                            .filter_map(|p| p["value"].as_u64())
                            .collect();
                        assert_eq!(tagged.len(), 1, "{suffix}: {wire}");
                        let rows: Vec<&serde_json::Value> = callee["blocks"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .flat_map(|b| b["instructions"].as_array().unwrap())
                            .map(|row| &row["instruction"])
                            .collect();
                        // The null equality never publishes as the integer
                        // `compare` row.
                        assert!(
                            rows.iter().all(|row| row["op"] != "compare"),
                            "{suffix}: {wire}"
                        );
                        let compares: Vec<&&serde_json::Value> = rows
                            .iter()
                            .filter(|row| row["op"] == "borrowed_null_compare")
                            .collect();
                        // One source compare; the edge-port model evaluates
                        // the same projection twice.
                        assert_eq!(compares.len(), 2, "{suffix}: {wire}");
                        let nulls: Vec<u64> = rows
                            .iter()
                            .filter(|row| row["op"] == "const_null")
                            .filter_map(|row| row["dst"].as_u64())
                            .collect();
                        for row in &compares {
                            assert_eq!(row["predicate"], "eq", "{suffix}: {wire}");
                            let lhs = row["lhs"].as_u64().unwrap();
                            let rhs = row["rhs"].as_u64().unwrap();
                            // Exactly one operand is the exact null
                            // producer; the other is the lent tagged view.
                            assert_eq!(
                                nulls.contains(&lhs) as usize + nulls.contains(&rhs) as usize,
                                1,
                                "{suffix}: {wire}"
                            );
                            let carrier = if nulls.contains(&lhs) { rhs } else { lhs };
                            assert_ne!(carrier, tagged[0], "{suffix}: {wire}");
                        }
                        reject_erased_carriers(&input);
                        std::fs::write(
                            std::env::temp_dir()
                                .join(format!("hako-issued-null-compare-{suffix}.json")),
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

/// `!=`, non-null siblings and formal-vs-formal equality stay outside the
/// admitted null-equality envelope: the used opaque formal fails the
/// unchanged artifact boundary — the same stop that every unadmitted
/// opaque use already takes — and no dedicated row is ever minted.
#[test]
fn non_null_equalities_stay_fail_closed_before_publication() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for (suffix, prefix, condition) in [
            ("ne", "", "handle != null"),
            ("zero", "", "handle == 0"),
            ("bool", "", "handle == false"),
            ("formal", "local q = handle ", "q == handle"),
        ] {
            let text = format!(
                "box Store {{ limit: i64 birth() {{ me.limit = 10 }} \
                 check(handle) {{ {prefix}if {condition} {{ return 7 }} return 3 }} }} \
                 static box Main {{ main() {{ local s = new Store() return s.check(5) }} }}"
            );
            let error = MirCompiler::with_options(false)
                .compile_normal_with_published(request(&text), |view, verification| {
                    classify_pretransform_report(verification);
                    view.issue_lifecycle_physical_abi_input().map(|_| ())
                })
                .err()
                .unwrap_or_else(|| panic!("{suffix}: unexpectedly admitted"));
            assert!(
                error.contains("artifact-source-unavailable"),
                "{suffix}: {error}"
            );
        }
    });
}
