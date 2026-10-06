//! Exact source outcomes are passive; legacy ownership projections stay closed.
use super::brand_catalog_tests::{issue_with_brand_catalog as issue, receiver_observations_for};
use super::{ResultExitOriginV1, ResultValueOriginV1};
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;

fn origins<'a>(
    package: &'a super::VerifiedNormalCallableSemanticPackageV1,
    owner: &str,
    name: &str,
    arity: u32,
) -> &'a [ResultExitOriginV1] {
    package
        .ordinary_new_claim_ledger
        .callable_result_origins(&CanonicalSameModuleCallableKeyV1::instance_box_method(
            owner, name, arity,
        ))
        .expect("exact source origin row")
}

fn contains(rows: &[ResultExitOriginV1], value: ResultValueOriginV1) -> bool {
    rows.iter().any(|row| row.alternatives().contains(&value))
}

#[test]
fn real_realloc_preserves_every_return_origin_without_owned_projection() {
    let package = issue(include_str!(
        "../../../lang/src/hako_alloc/memory/page_heap_box.hako"
    ))
    .unwrap();
    let rows = origins(&package, "HakoAllocHeap", "realloc", 2);
    assert_eq!(rows.len(), 8, "all Completion value exits are recorded");
    assert!(contains(rows, ResultValueOriginV1::Null));
    assert!(contains(
        rows,
        ResultValueOriginV1::Fresh("HakoAllocHandle".into())
    ));
    assert!(contains(
        rows,
        ResultValueOriginV1::ForwardFormal { ordinal: 0 }
    ));
    use crate::mir::resolved_semantics::SourcePathSegmentV1::{Body, IfThen, Value};
    use std::collections::{BTreeMap, BTreeSet};
    let null = ResultValueOriginV1::Null;
    let expected: BTreeMap<_, BTreeSet<_>> = [
        (vec![Body(0), IfThen(0), Value], vec![null.clone()]),
        (vec![Body(1), IfThen(0), Value], vec![null.clone()]),
        (vec![Body(2), IfThen(0), Value], vec![null.clone()]),
        (
            vec![Body(3), IfThen(1), IfThen(0), Value],
            vec![
                null.clone(),
                ResultValueOriginV1::ForwardFormal { ordinal: 0 },
            ],
        ),
        (
            vec![Body(4), IfThen(1), IfThen(0), Value],
            vec![
                null.clone(),
                ResultValueOriginV1::ForwardFormal { ordinal: 0 },
            ],
        ),
        (vec![Body(6), IfThen(0), Value], vec![null.clone()]),
        (
            vec![Body(7), IfThen(0), Value],
            vec![
                null.clone(),
                ResultValueOriginV1::Fresh("HakoAllocHandle".into()),
            ],
        ),
        (vec![Body(9), Value], vec![null]),
    ]
    .into_iter()
    .map(|(site, values)| (site, values.into_iter().collect()))
    .collect();
    let actual: BTreeMap<_, _> = rows
        .iter()
        .map(|row| {
            (
                row.site().site().node().segments().to_vec(),
                row.alternatives().clone(),
            )
        })
        .collect();
    assert_eq!(
        actual, expected,
        "each exact return retains its own alternatives"
    );
    let owner = rows[0].site().owner();
    assert!(rows.iter().all(|row| row.site().owner() == owner));
    let sites: std::collections::BTreeSet<_> = rows.iter().map(|row| row.site()).collect();
    assert_eq!(sites.len(), rows.len());
    let ledger = &package.ordinary_new_claim_ledger;
    let k = CanonicalSameModuleCallableKeyV1::instance_box_method("HakoAllocHeap", "realloc", 2);
    assert!(ledger
        .callable_result_class_claims_for_test()
        .get(&k)
        .is_none());
    assert!(!ledger
        .callable_result_class_claims_for_test()
        .contains_key(&k));
    assert!(receiver_observations_for(&package, "HakoAllocHeap", "realloc").is_empty());
    assert_eq!(ledger.callable_result_class(&k), None);
    assert_eq!(ledger.nullable_callable_result_class(&k), None);
    let null_exit = origins(&package, "HakoAllocPage", "resizeInPlace", 2);
    assert!(contains(
        null_exit,
        ResultValueOriginV1::ForwardFormal { ordinal: 0 }
    ));
    assert!(!null_exit.iter().any(|row| row
        .alternatives()
        .iter()
        .any(|origin| matches!(origin, ResultValueOriginV1::Fresh(_)))));
}

const TOKEN: &str = "box Token { value: i64 birth(value) { me.value = value } }";

#[test]
fn mixed_forward_composes_caller_ordinal_and_keeps_caller_sites() {
    for methods in [
        "mixed(h: Token, choose: i64) { if choose == 0 { return new Token(1) } return h } relay(unused, h: Token, choose: i64) { return me.mixed(h, choose) } top(h: Token, choose: i64) { return me.relay(0, h, choose) }",
        "top(h: Token, choose: i64) { return me.relay(0, h, choose) } relay(unused, h: Token, choose: i64) { return me.mixed(h, choose) } mixed(h: Token, choose: i64) { if choose == 0 { return new Token(1) } return h }",
    ] {
        let package = issue(&format!("{TOKEN} box Door {{ {methods} }} static box Main {{ main() {{ return 0 }} }}")).unwrap();
        let mixed = origins(&package, "Door", "mixed", 2);
        let relay = origins(&package, "Door", "relay", 3);
        let top = origins(&package, "Door", "top", 2);
        assert_eq!(mixed.len(), 2);
        assert_eq!(relay.len(), 1);
        assert!(contains(relay, ResultValueOriginV1::ForwardFormal { ordinal: 1 }));
        assert!(contains(top, ResultValueOriginV1::ForwardFormal { ordinal: 0 }));
        assert!(contains(top, ResultValueOriginV1::Fresh("Token".into())));
        assert_ne!(mixed[0].site().owner(), relay[0].site().owner());
        assert_ne!(relay[0].site().owner(), top[0].site().owner());
        for (name, arity) in [("mixed",2), ("relay",3), ("top",2)] {
            assert!(!package.ordinary_new_claim_ledger.callable_result_class_claims_for_test()
                .contains_key(&CanonicalSameModuleCallableKeyV1::instance_box_method("Door", name, arity)));
        }
    }
}

#[test]
fn null_only_and_forwarded_null_never_gain_legacy_membership() {
    let package = issue(&format!("{TOKEN} box Door {{ nil(choose: i64) {{ if choose == 0 {{ return null }} return null }} relay(choose: i64) {{ return me.nil(choose) }} would_upgrade(choose: i64) {{ if choose == 0 {{ return new Token(1) }} return me.nil(choose) }} }} static box Main {{ main() {{ return 0 }} }}")).unwrap();
    for name in ["nil", "relay", "would_upgrade"] {
        let rows = origins(&package, "Door", name, 1);
        assert!(contains(rows, ResultValueOriginV1::Null));
        let k = CanonicalSameModuleCallableKeyV1::instance_box_method("Door", name, 1);
        assert!(package
            .ordinary_new_claim_ledger
            .callable_result_class_claims_for_test()
            .get(&k)
            .is_none());
    }
    assert!(origins(&package, "Door", "nil", 1)
        .iter()
        .all(|row| row.alternatives().len() == 1));
}

#[test]
fn origin_rejects_rebind_unknown_actual_class_and_ungrounded_cycle() {
    let methods = "give(h: Token) { return h } rebound(h: Token) { h = null return h } local_rebound(h: Token) { local same = me.give(h) same = null return same } unknown() { return me.give(null) } multi(a: Token, b: Token, choose: i64) { if choose == 0 { return a } return b } cycle(h: Token) { return me.cycle(h) } mismatch(h: Other, choose: i64) { if choose == 0 { return new Token(1) } return h } builtin() { return new ArrayBox() }";
    let package = issue(&format!("{TOKEN} box Other {{ value: i64 birth(value) {{ me.value = value }} }} box Door {{ {methods} }} static box Main {{ main() {{ return 0 }} }}")).unwrap();
    for (name, arity) in [
        ("rebound", 1),
        ("local_rebound", 1),
        ("unknown", 0),
        ("multi", 3),
        ("cycle", 1),
        ("mismatch", 2),
        ("builtin", 0),
    ] {
        let k = CanonicalSameModuleCallableKeyV1::instance_box_method("Door", name, arity);
        assert!(
            package
                .ordinary_new_claim_ledger
                .callable_result_origins(&k)
                .is_none(),
            "{name} remains unavailable"
        );
    }
}
