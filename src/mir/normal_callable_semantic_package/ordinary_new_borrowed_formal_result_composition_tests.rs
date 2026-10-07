use super::*;
use crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1;
use std::rc::Rc;

fn issue(leaf: &str, body: &str) -> Result<VerifiedNormalCallableSemanticPackageV1, String> {
    let source = format!("box Transport {{ birth() {{ }} leaf(p) {{ {leaf} }}
        bridge(p) {{ local recv = new Transport() {body} }}
        wrap(p) {{ local recv = new Transport() local out = recv.bridge(p) return out }} }}
        static box Main {{ main() {{ local recv = new Transport() local out = recv.wrap(null) return 0 }} }}");
    crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        &source,
    )
    .map_err(|error| format!("{error:?}"))
}

#[test]
fn call_result_source_rejects_rebind_non_i64_and_ungrounded_cycles() {
    assert!(issue("return 7", "local out = recv.leaf(p) return out").is_ok());
    for (leaf, body) in [
        ("return 7", "local out = recv.leaf(p) out = 8 return out"),
        ("return true", "local out = recv.leaf(p) return out"),
        ("return null", "local out = recv.leaf(p) return out"),
        (
            "if p == null { return null } return new Transport()",
            "local out = recv.leaf(p) return out",
        ),
        (
            "local recv = new Transport() local out = recv.bridge(p) return out",
            "local out = recv.leaf(p) return out",
        ),
        (
            "return 7",
            "if p == null { return 0 } local out = recv.bridge(p) return out",
        ),
        ("return", "local out = recv.leaf(p) return out"),
        ("return 7", "local out = recv.bridge(p) return out"),
    ] {
        let error = issue(leaf, body)
            .err()
            .expect("selected source result is unproven");
        assert!(error.contains("borrowed-result/"), "{leaf}/{body}: {error}");
    }
}

#[test]
fn composed_result_corroboration_rejects_foreign_dependencies_and_propagates_callee_drift() {
    for change in 0..5 {
        let mut package = issue("return 7", "local out = recv.leaf(p) return out").unwrap();
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let original: Vec<_> = ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap()
            .incoming
            .iter()
            .map(|row| Ok(Some(row.source.require_instance().unwrap().clone())))
            .collect();
        ledger.corroborate_borrowed_result_cohort_v1(&original, &package.result_contracts);
        let callers: Vec<_> = ledger
            .borrowed_i64_results
            .iter()
            .filter_map(|(owner, row)| {
                row.as_ref()
                    .ok()
                    .filter(|row| !row.dependencies.is_empty())
                    .map(|_| *owner)
            })
            .collect();
        assert_eq!(callers.len(), 2, "two-hop chain");
        let caller = *callers
            .iter()
            .find(|owner| {
                let dependency = &ledger.borrowed_i64_results[owner]
                    .as_ref()
                    .unwrap()
                    .dependencies[0];
                ledger.borrowed_i64_results[&dependency.callee_owner()]
                    .as_ref()
                    .unwrap()
                    .dependencies
                    .is_empty()
            })
            .expect("bridge directly depends on leaf");
        assert!(
            ledger.borrowed_i64_results[&caller]
                .as_ref()
                .unwrap()
                .contract_corroborated
        );
        let proof = ledger
            .borrowed_i64_results
            .get_mut(&caller)
            .unwrap()
            .as_mut()
            .unwrap();
        let callee = proof.dependencies[0].callee_owner();
        match change {
            0 => proof.dependencies[0].target_batch_slot += 1,
            1 => {
                proof.dependencies[0].argument_sites[0] =
                    proof.dependencies[0].receiver_site.clone()
            }
            2 => proof.dependencies[0].callee_owner = caller,
            3 => {
                let leaf = ledger
                    .borrowed_i64_results
                    .get_mut(&callee)
                    .unwrap()
                    .as_mut()
                    .unwrap();
                leaf.returns[0] = original[0]
                    .as_ref()
                    .unwrap()
                    .as_ref()
                    .unwrap()
                    .call_site()
                    .clone();
            }
            _ => {
                ledger
                    .borrowed_i64_results
                    .insert(callee, Err(freeze("test/missing-result")));
            }
        }
        ledger.corroborate_borrowed_result_cohort_v1(&original, &package.result_contracts);
        for owner in callers {
            assert!(ledger.borrowed_i64_results[&owner]
                .as_ref()
                .unwrap_err()
                .contains("borrowed-result/result-contract-mismatch"));
        }
        assert!(ledger.borrowed_i64_results[&caller]
            .as_ref()
            .unwrap_err()
            .contains("borrowed-result/result-contract-mismatch"));
    }
}

#[test]
fn direct_call_result_source_rejects_non_i64_and_ungrounded_cycles() {
    assert!(issue("return 7", "return recv.leaf(p)").is_ok());
    for (leaf, body) in [
        ("return true", "return recv.leaf(p)"),
        ("return null", "return recv.leaf(p)"),
        ("return", "return recv.leaf(p)"),
        (
            "if p == null { return null } return new Transport()",
            "return recv.leaf(p)",
        ),
        ("return 7", "return recv.bridge(p)"),
        (
            "local recv = new Transport() return recv.bridge(p)",
            "return recv.leaf(p)",
        ),
        (
            "return 7",
            "if p == null { return 0 } return recv.bridge(p)",
        ),
    ] {
        let error = issue(leaf, body)
            .err()
            .expect("direct result lacks an I64 proof");
        assert!(error.contains("borrowed-result/"), "{leaf}/{body}: {error}");
    }
}

#[test]
fn direct_call_result_source_rejects_nonborrowed_callee_annotation() {
    let source = "box Transport { birth() { } leaf(q: i64): i64 { return q }
        bridge(p) { local recv = new Transport() return recv.leaf(7) } }
        static box Main { main() { local recv = new Transport() local out = recv.bridge(null) return 0 } }";
    let error = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(source)
        .err().expect("nonborrowed callee has no borrowed incoming source proof");
    assert!(
        format!("{error:?}").contains("borrowed-result/source-not-i64"),
        "{error:?}"
    );
}

#[test]
fn direct_call_result_source_retains_terminal_relations_for_declared_borrows() {
    use crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1;
    for annotated in [true, false] {
        let result = if annotated { ": i64" } else { "" };
        let source = format!("box Item {{ value: i64 birth() {{ me.value = 5 }} }}
            box Transport {{ flag: i64 birth() {{ me.flag = 0 }}
                read(p: Item){result} {{ if p == null {{ return 7 }} return p.value }}
                bridge(p: Item){result} {{ local recv = new Transport() return recv.read(p) }}
                wrap(p: Item){result} {{ local recv = new Transport() return recv.bridge(p) }} }}
            static box Main {{ main() {{ local recv = new Transport() local item = new Item() return recv.wrap(item) }} }}");
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&source).unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        for (owner, proof) in &ledger.borrowed_i64_results {
            if proof.as_ref().unwrap().dependencies.is_empty() {
                continue;
            }
            let relation = ledger.sole_terminal_relation_for_owner(*owner);
            let completion = ledger.completion_for_owner(*owner);
            assert!(
                matches!(relation, Some(TerminalRelationV1::Call(_))),
                "annotated={annotated} owner={owner:?} relation={relation:?} completion={completion:?}"
            );
            let flow = ledger
                .completion_for_owner(*owner)
                .unwrap()
                .cleanup()
                .root_flow()
                .unwrap();
            assert!(
                flow.all_exits_ready(),
                "annotated={annotated} owner={owner:?} flow={flow:?}"
            );
        }
    }
}
