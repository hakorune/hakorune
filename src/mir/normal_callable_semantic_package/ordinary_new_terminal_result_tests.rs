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
fn root_instance_call_retains_initializer_and_claim_object_identity() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        "box Pair { left: i64 right: i64
        birth(left, right) { me.left = left me.right = right }
        sum(): i64 { return me.left + me.right } }
        static box Main { main() {
        local first = new Pair(1, 2)
        local second = new Pair(3, 4)
        return second.sum() } }",
    )
    .expect("two same-typed receiver source");
    let ledger = &package.ordinary_new_claim_ledger;
    let owner = ledger.root_completion_for_test().owner();
    let call_site = ledger
        .call_source_completion()
        .expect("root call relation")
        .1
        .call_site()
        .clone();
    let row = ledger
        .take_root_instance_call(owner, &call_site)
        .expect("take source row")
        .expect("instance call row");
    let claims = ledger.pending_claims_for_test();
    let claim = claims
        .get(row.receiver_initializer())
        .expect("initializer claim remains source-owned");
    assert_eq!(row.receiver_initializer().owner(), owner);
    assert_eq!(row.receiver_binding().owner(), owner);
    assert_eq!(row.receiver_object(), claim.object());
}

#[test]
fn root_instance_call_rejects_emitted_receiver_mutation() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        "box Page { birth() { } sum(): i64 { return 30 } }
        static box Main { main() {
        local first = new Page()
        local second = new Page()
        return second.sum() } }",
    )
    .expect("two same-typed receiver source");
    let ledger = &package.ordinary_new_claim_ledger;
    let owner = ledger.root_completion_for_test().owner();
    let claim_rows = ledger.pending_claims_for_test();
    let claims = claim_rows.values().collect::<Vec<_>>();
    assert_eq!(claims.len(), 2);
    let sites = claims.iter().map(|claim| claim.site().clone()).collect::<Vec<_>>();
    let declarations = sites
        .iter()
        .map(|site| {
            let declaration = package
                .batch()
                .declarations()
                .find(|row| row.owner() == owner)
                .expect("root declaration");
            package
                .batch()
                .with_lowering_input(declaration.batch_slot(), |input| {
                    input
                        .function()
                        .expression_source()
                        .initializers()
                        .find(|row| row.initializer_site() == Some(site.site()))
                        .map(|row| (row.binding(), row.declaration_site().clone()))
                })
                .expect("lowering input")
                .expect("local declaration")
        })
        .collect::<Vec<_>>();
    drop(claims);
    drop(claim_rows);
    ledger.register_new_root(owner).expect("root registration");
    for (index, site) in sites.iter().enumerate() {
        let claim = ledger
            .try_take(site, "Page", 0)
            .expect("claim take")
            .expect("selected claim");
        assert!(ledger.prepare_new_emission(&claim).expect("prepare"));
        ledger.begin_new_emission(site).expect("begin");
        let initializer = crate::mir::ValueId(100 + index as u32 * 2);
        let local = crate::mir::ValueId(101 + index as u32 * 2);
        let block = crate::mir::BasicBlockId::new(index as u32);
        let binding = crate::mir::MirInstruction::InvokeNormalResult {
            invoke_block: block,
            dst: initializer,
        };
        ledger
            .record_new_emission(site, initializer, Vec::new(), None, vec![(block, binding)])
            .expect("record");
        ledger
            .complete_new_expression(site, "Page", initializer)
            .expect("complete expression");
        let crate::mir::resolved_semantics::SourceBindingSiteV1::Local {
            statement,
            ordinal,
        } = &declarations[index].1
        else {
            panic!("local declaration");
        };
        ledger
            .complete_local_installation(
                owner,
                statement.node(),
                &[(declarations[index].0, *ordinal, initializer, local)],
            )
            .expect("complete local");
    }
    let call_site = ledger
        .call_source_completion()
        .expect("root call relation")
        .1
        .call_site()
        .clone();
    let row = ledger
        .take_root_instance_call(owner, &call_site)
        .expect("take source row")
        .expect("instance call row");
    assert_ne!(row.receiver_initializer(), &sites[0]);
    let error = ledger
        .resolve_root_instance_receiver_for_test(owner, &row, crate::mir::ValueId(101))
        .expect_err("emitted receiver mutation must fail at the source/local boundary");
    assert!(error.contains("instance-call-receiver"), "{error}");
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
fn root_instance_call_without_result_contract_is_owner_scoped_in_inverse_order() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        "box Page { birth() { } }
        box Pair { left: i64 right: i64
        birth(left, right) { me.left = left me.right = right }
        sum() { return me.left + me.right } }
        static box Main {
        helper() { local page = new Page() local m = %{\"v\" => 1} return 30 }
        main() { local pair = new Pair(10, 20) return pair.sum() } }",
    )
    .expect("inverse declaration order keeps the missing result contract unavailable");
    let ledger = &package.ordinary_new_claim_ledger;
    let root_owner = ledger.root_completion_for_test().owner();
    assert!(ledger.root_instance_call_expected(root_owner));
    let child_owner = package
        .batch()
        .declarations()
        .find(|declaration| declaration.owner() != root_owner)
        .map(|declaration| declaration.owner())
        .expect("inverse-order child owner");
    assert!(!ledger.root_instance_call_expected(child_owner));
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
    let completion = ledger
        .completion_index
        .get(&owner)
        .unwrap()
        .as_ref()
        .unwrap();
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
fn child_and_root_add_field_reads_merge_in_declaration_order() {
    for methods in [
        r#"main() { local pair = new Pair(10, 20) local m = %{"root" => 1} return pair.left + pair.right }
           helper(value: i64): i64 { local pair = new Pair(value, 20) local m = %{"child" => value} return pair.left + pair.right }"#,
        r#"helper(value: i64): i64 { local pair = new Pair(value, 20) local m = %{"child" => value} return pair.left + pair.right }
           main() { local pair = new Pair(10, 20) local m = %{"root" => 1} return pair.left + pair.right }"#,
    ] {
        let source = format!(
            "box Pair {{ left: i64 right: i64 birth(left, right) {{ me.left = left me.right = right }} }} static box Main {{ {methods} }}"
        );
        let package = super::super::brand_catalog_tests::issue_with_brand_catalog(&source)
            .expect("root and child add source package");
        let ledger = &package.ordinary_new_claim_ledger;
        let root_owner = ledger.root_completion_for_test().owner();
        let child_owner = package
            .batch()
            .declarations()
            .find_map(|declaration| {
                let owner = declaration.owner();
                (owner != root_owner && ledger.terminal_i64_add_return_for_owner(owner).is_some())
                    .then_some(owner)
            })
            .expect("child add relation");
        let root = ledger
            .terminal_i64_add_return_for_owner(root_owner)
            .expect("root add relation");
        let child = ledger
            .terminal_i64_add_return_for_owner(child_owner)
            .expect("child add relation");
        let reads = ledger.field_reads.borrow();
        assert_eq!(root.field_reads().len(), 2);
        assert_eq!(child.field_reads().len(), 2);
        assert_eq!(reads.len(), 4);
        assert!(root
            .field_reads()
            .iter()
            .chain(child.field_reads())
            .all(|site| reads.contains_key(site)));
    }
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
