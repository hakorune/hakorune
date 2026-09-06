#[test]
fn pair_i64_add_return_is_issued_from_completion_and_existing_field_reads() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        "box Pair { left: i64 right: i64
        birth(left, right) { me.left = left me.right = right } }
        static box Main { main() {
        local pair = new Pair(10, 20)
        return pair.left + pair.right } }",
    )
    .expect("source package");
    let ledger = &package.ordinary_new_claim_ledger;
    let result = ledger
        .terminal_i64_add_return()
        .expect("Pair terminal result relation");
    let completion = ledger.root_completion_for_test();
    assert_eq!(result.owner(), completion.owner());
    assert_eq!(
        result.return_site(),
        completion.explicit_site().expect("return site")
    );
    let reads = ledger.field_reads.borrow();
    assert_eq!(reads.len(), 2);
    assert!(result
        .field_reads()
        .iter()
        .all(|site| reads.contains_key(site)));
    assert_ne!(result.field_reads()[0], result.field_reads()[1]);
    assert_eq!(result.add_site().owner(), result.owner());
}

#[test]
fn non_add_terminal_does_not_issue_pair_result_relation() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        "box Pair { left: i64 right: i64
        birth(left, right) { me.left = left me.right = right } }
        static box Main { main() {
        local pair = new Pair(10, 20)
        return pair.left } }",
    )
    .expect("source package");
    assert!(package
        .ordinary_new_claim_ledger
        .terminal_i64_add_return()
        .is_none());
}

#[test]
fn nested_add_keeps_scalar_completion_without_issuing_direct_pair_relation() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        "box Pair { left: i64 right: i64
        birth(left, right) { me.left = left me.right = right } }
        static box Main { main() {
        local pair = new Pair(10, 20)
        return (pair.left + pair.right) + 1 } }",
    )
    .expect("source package");
    let completion = package.ordinary_new_claim_ledger.root_completion_for_test();
    assert!(matches!(completion.cleanup().terminal_homes(), Some(Ok(_))));
    assert!(package
        .ordinary_new_claim_ledger
        .terminal_i64_add_return()
        .is_none());
}

#[test]
fn explicit_bare_return_issues_unit_relation_without_i64_specialization() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        "box Pair { left: i64 right: i64 birth(left, right) { me.left = left me.right = right } } static box Main { main() { local pair = new Pair(10, 20) return } }",
    ).expect("source package");
    let ledger = &package.ordinary_new_claim_ledger;
    let relation = ledger.terminal_unit_return().expect("explicit bare return relation");
    let completion = ledger.root_completion_for_test();
    assert_eq!(relation.owner(), completion.owner());
    assert_eq!(relation.return_site(), completion.explicit_site().expect("return site"));
    assert!(ledger.terminal_i64_add_return().is_none());
}
