#[test]
fn root_instance_call_uses_selected_result_contract() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        "box Pair { left: i64 right: i64
        birth(left, right) { me.left = left me.right = right }
        sum(): i64 { return me.left + me.right } }
        static box Main { main() {
        local pair = new Pair(10, 20)
        return pair.sum() } }",
    )
    .expect("annotated instance result must be source-admitted");
    let ledger = &package.ordinary_new_claim_ledger;
    assert!(ledger.root_instance_call_expected(ledger.root_completion_for_test().owner()));
    assert!(!ledger.root_instance_call_is_empty());
}

#[test]
fn root_instance_call_without_result_contract_stays_unavailable() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        "box Page { birth() { } }
        box Pair { left: i64 right: i64
        birth(left, right) { me.left = left me.right = right }
        sum() { return me.left + me.right } }
        static box Main { main() {
        local pair = new Pair(10, 20)
        return pair.sum() }
        helper() { local page = new Page() local m = %{\"v\" => 1} return 30 } }",
    )
    .expect("missing result contract is an unavailable source shape");
    let ledger = &package.ordinary_new_claim_ledger;
    assert!(ledger.root_instance_call_expected(ledger.root_completion_for_test().owner()));
    let child_owner = package
        .batch()
        .declarations()
        .find(|declaration| declaration.owner() != ledger.root_completion_for_test().owner())
        .map(|declaration| declaration.owner());
    if let Some(child_owner) = child_owner {
        assert!(!ledger.root_instance_call_expected(child_owner));
    }
    assert!(ledger.root_instance_call_is_empty());
}

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
    let relation = ledger
        .terminal_unit_return()
        .expect("explicit bare return relation");
    let completion = ledger.root_completion_for_test();
    assert_eq!(relation.owner(), completion.owner());
    assert_eq!(
        relation.return_site(),
        completion.explicit_site().expect("return site")
    );
    assert!(ledger.terminal_i64_add_return().is_none());
}

#[test]
fn direct_integer_literal_issues_exact_terminal_relation() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        "box Pair { left: i64 right: i64 birth(left, right) { me.left = left me.right = right } } static box Main { main() { local pair = new Pair(10, 20) return 30 } }",
    ).expect("source package");
    let relation = package
        .ordinary_new_claim_ledger
        .terminal_integer_literal_return()
        .expect("literal relation");
    let completion = package.ordinary_new_claim_ledger.root_completion_for_test();
    assert_eq!(relation.owner(), completion.owner());
    assert_eq!(
        relation.return_site(),
        completion.explicit_site().expect("return site")
    );
    assert_eq!(relation.value(), 30);
    assert!(package
        .ordinary_new_claim_ledger
        .terminal_i64_add_return()
        .is_none());
    assert!(package
        .ordinary_new_claim_ledger
        .terminal_unit_return()
        .is_none());
}

#[test]
fn ordinary_child_literal_relation_is_borrowed_by_owner() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        "box Page { birth() { } } static box Main {
            main() { return helper(0) }
            helper(value: i64): i64 { local page = new Page() local m = %{\"v\" => value} return 30 }
        }",
    )
    .expect("ordinary child source package");
    let owner = package
        .batch()
        .declarations()
        .find_map(|declaration| {
            package
                .batch()
                .with_lowering_input(declaration.batch_slot(), |input| {
                    input
                        .function()
                        .expression_source()
                        .initializers()
                        .next()
                        .map(|_| declaration.owner())
                })
                .expect("same batch loan")
        })
        .expect("helper owner");
    let ledger = &package.ordinary_new_claim_ledger;
    let relation = ledger
        .terminal_integer_literal_return_for_owner(owner)
        .expect("ordinary child literal relation");
    let completion = ledger.completion_index.get(&owner).unwrap().as_ref().unwrap();
    assert_eq!(relation.owner(), owner);
    assert_eq!(relation.value(), 30);
    assert_eq!(completion.explicit_site(), Some(relation.return_site()));
    let value = ledger
        .prepare_terminal_integer_literal_return(owner, relation.return_site().node())
        .unwrap()
        .expect("owner-indexed literal consumer");
    assert_eq!(value, 30);
    let foreign_owner = package
        .batch()
        .declarations()
        .find(|declaration| declaration.owner() != owner)
        .expect("app main owner")
        .owner();
    assert!(ledger
        .prepare_terminal_integer_literal_return(foreign_owner, relation.return_site().node())
        .unwrap()
        .is_none());
    ledger
        .record_terminal_integer_literal_return(owner, crate::mir::ValueId(900))
        .unwrap();
}

#[test]
fn ordinary_child_field_relation_is_retained_by_owner() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        "box Page { slot: i64 birth() { me.slot = 7 } } static box Main {
            main() { return 30 }
            helper(value: i64): i64 { local page = new Page() local m = %{\"v\" => value} return page.slot }
        }",
    )
    .expect("ordinary child field source package");
    let owner = package
        .batch()
        .declarations()
        .find_map(|declaration| {
            package
                .batch()
                .with_lowering_input(declaration.batch_slot(), |input| {
                    input
                        .function()
                        .expression_source()
                        .initializers()
                        .next()
                        .map(|_| declaration.owner())
                })
                .expect("same batch loan")
        })
        .expect("helper owner");
    let ledger = &package.ordinary_new_claim_ledger;
    let relation = ledger
        .terminal_i64_field_return_for_owner(owner)
        .expect("ordinary child field relation");
    assert_eq!(relation.owner(), owner);
    assert_eq!(relation.field_read_site().owner(), owner);
    assert!(ledger
        .field_reads
        .borrow()
        .contains_key(relation.field_read_site()));
    let foreign_owner = package
        .batch()
        .declarations()
        .find(|declaration| declaration.owner() != owner)
        .expect("app main owner")
        .owner();
    assert!(ledger
        .prepare_terminal_i64_field_return(foreign_owner, relation.return_site().node(), |_, _| {
            Ok(crate::mir::ValueId(1))
        })
        .unwrap()
        .is_none());
}

#[test]
fn direct_i64_field_issues_exact_terminal_relation() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        "box Pair { left: i64 right: i64 birth(left, right) { me.left = left me.right = right } } static box Main { main() { local pair = new Pair(10, 20) return pair.left } }",
    )
    .expect("source package");
    let ledger = &package.ordinary_new_claim_ledger;
    let relation = ledger
        .terminal_i64_field_return()
        .expect("direct field terminal relation");
    let completion = ledger.root_completion_for_test();
    assert_eq!(relation.owner(), completion.owner());
    assert_eq!(
        relation.return_site(),
        completion.explicit_site().expect("return site")
    );
    assert_eq!(relation.field_read_site().owner(), relation.owner());
    assert!(ledger
        .field_reads
        .borrow()
        .contains_key(relation.field_read_site()));
    assert!(ledger.terminal_i64_add_return().is_none());
    assert!(ledger.terminal_integer_literal_return().is_none());
    assert!(ledger.terminal_unit_return().is_none());
}

#[test]
fn direct_bool_does_not_issue_i64_field_terminal_relation() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        "box Pair { left: i64 right: i64 birth(left, right) { me.left = left me.right = right } } static box Main { main() { local pair = new Pair(10, 20) return true } }",
    )
    .expect("source package");
    let ledger = &package.ordinary_new_claim_ledger;
    assert!(ledger.terminal_i64_field_return().is_none());
    assert!(ledger.terminal_i64_add_return().is_none());
    assert!(ledger.terminal_integer_literal_return().is_none());
}

#[test]
fn mixed_or_unproven_add_discards_terminal_and_all_staged_reads() {
    for suffix in [
        "return true + 1",
        "return 1 + true",
        "return pair.left + true",
        "return false + pair.right",
        "return (pair.left + pair.right) + true",
        "return true + (pair.left + pair.right)",
        "return (pair.left + true) + pair.right",
        "return pair.left + (true + pair.right)",
        "local value = true return value + pair.left",
        "local value = true return pair.left + value",
        "local value = 1 return value + pair.left",
        "local value = 1 return pair.left + value",
    ] {
        let source = format!(
            "box Pair {{ left: i64 right: i64
             birth(left, right) {{ me.left = left me.right = right }} }}
             static box Main {{ main() {{ local pair = new Pair(10, 20) {suffix} }} }}"
        );
        let package = super::super::brand_catalog_tests::issue_with_brand_catalog(&source)
            .expect("source package retains explicit unsupported completion");
        let ledger = &package.ordinary_new_claim_ledger;
        let completion = ledger.root_completion_for_test();
        assert!(matches!(completion.cleanup().terminal_homes(),
            Some(Err(crate::mir::resolved_semantics::home_new_prefix::HomePrefixUnavailableV1::ReturnValueNotCovered(_)))),
            "{suffix}");
        assert!(ledger.terminal_relation.is_none(), "{suffix}");
        assert!(ledger.field_reads.borrow().is_empty(), "{suffix}");
    }
}
