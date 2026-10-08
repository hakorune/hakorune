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
    let sites = claims
        .iter()
        .map(|claim| claim.site().clone())
        .collect::<Vec<_>>();
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
        let crate::mir::resolved_semantics::SourceBindingSiteV1::Local { statement, ordinal } =
            &declarations[index].1
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
fn root_instance_call_stays_unissued_for_unreleasable_terminal_home() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        "box Page { birth() { } }
        box Holder { init { inner }
        birth() { me.inner = new Page() }
        run(): i64 { return 0 } }
        static box Main { main() {
        local holder = new Holder()
        return holder.run() } }",
    )
    .expect("unreleasable receiver home source");
    let ledger = &package.ordinary_new_claim_ledger;
    let owner = ledger.root_completion_for_test().owner();
    // The terminal call is still a known instance call ...
    assert!(ledger.root_instance_call_expected(owner));
    // ... but `holder`'s claim is unreleasable (`init`-declared fields give
    // `Destruction::Unavailable(FieldType)`), so `prepare_root_home_exit`
    // can never take the row. The issuer must withhold it instead of
    // stranding `Ready` rows.
    assert!(ledger.root_instance_call_is_empty());
}

#[test]
fn unreleasable_root_call_receiver_passes_finishing_but_not_the_seal() {
    // Same shape as the withhold pin above: `holder`'s sealed claim is
    // `Destruction::Unavailable(FieldType)`, so the issuer withholds the
    // `Ready` row and `prepare_root_home_exit` records `Unavailable`.
    // Non-artifact finishing must treat that recorded disposition as the
    // deliberate absence of a physical call entry — while the sealing
    // lane stays the sole rejection authority (`root-call-entry-unavailable`).
    let mut package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        "box Page { birth() { } }
        box Holder { init { inner }
        birth() { me.inner = new Page() }
        run(): i64 { return 0 } }
        static box Main { main() {
        local holder = new Holder()
        return holder.run() } }",
    )
    .expect("unreleasable receiver home source");
    let mut loans = package.direct_call_loans.take();
    let main = package
        .declaration_catalog()
        .source_backed_app_main()
        .expect("app main");
    let declaration = package
        .batch()
        .declarations()
        .find(|row| row.identity().same_as(main.parser_identity()))
        .expect("main declaration");
    let mut builder = crate::mir::MirBuilder::new();
    let mut function = package
        .batch()
        .with_lowering_input_and_source_identity(declaration.batch_slot(), |input, identity| {
            builder.lower_map_dependency_for_test(
                input,
                crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(
                    main.catalog_key().clone(),
                ),
                main.parser_identity(),
                identity.method_source_observation().cloned(),
                std::rc::Rc::clone(&package.ordinary_new_claim_ledger),
                loans.as_mut(),
            )
        })
        .expect("lowering input")
        .unwrap_or_else(|e| panic!("unreleasable root call lowers generically: {e}"));
    if let Some(loans) = loans {
        loans.finish_empty().expect("no direct-call rows owed");
    }
    let ledger = &package.ordinary_new_claim_ledger;
    let observation = ledger
        .validate_finalized_new_root(&function)
        .expect("draft validation tolerates the Unavailable exit");
    function
        .install_root_ordinary_new_observation(observation)
        .expect("observation install");
    ledger
        .validate_after_compiler_finishing(&function)
        .unwrap_or_else(|e| panic!("Unavailable exit must pass non-artifact finishing: {e}"));
    let mut module = crate::mir::MirModule::new("unavailable-root-call".into());
    module
        .functions
        .insert(function.signature.name.clone(), function);
    let error = ledger
        .seal_finalized_root_birth_handoff(
            "Main.main/0".into(),
            &module,
            &std::collections::BTreeSet::new(),
            None,
        )
        .expect_err("document sealing stays the sole rejection authority");
    assert!(error.contains("root-call-entry-unavailable"), "{error}");
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
        .record_terminal_integer_literal_return(
            owner,
            relation.return_site().node(),
            crate::mir::ValueId(900),
        )
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

/// TASK4-ROOTSOURCE-S0: a proven-i64 bound local — stored literal or a
/// classified call result — returns its own scalar terminal relation.
/// Proven mixes (bound i64 + i64 field) join the same row; the stored
/// `SourceScalarKind` is the sole class authority.
#[test]
fn bound_i64_scalar_issues_exact_terminal_relation() {
    for (label, suffix) in [
        ("bound-literal", "local value = 1 return value"),
        ("bound-copy", "local value = 1 local copy = value return copy"),
        ("bound-field-add", "local value = 1 return value + pair.left"),
        ("bound-field-add-rev", "local value = 1 return pair.left + value"),
    ] {
        let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
            &format!(
                "box Pair {{ left: i64 right: i64 birth(left, right) {{ me.left = left me.right = right }} }} static box Main {{ main() {{ local pair = new Pair(10, 20) {suffix} }} }}"
            ),
        )
        .expect("source package");
        let ledger = &package.ordinary_new_claim_ledger;
        let completion = ledger.root_completion_for_test();
        let relation = ledger
            .terminal_relation
            .values()
            .find_map(|relation| match relation {
                crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1::I64Scalar(row) => Some(row),
                _ => None,
            })
            .unwrap_or_else(|| panic!("{label}: scalar relation"));
        assert_eq!(relation.owner(), completion.owner(), "{label}");
        assert_eq!(
            completion.explicit_site(),
            Some(relation.return_site()),
            "{label}"
        );
    }
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
        assert!(ledger.terminal_relation.is_empty(), "{suffix}");
        assert!(ledger.field_reads.borrow().is_empty(), "{suffix}");
    }
}

#[test]
fn root_instance_call_admits_owned_field_receiver_with_sealed_children() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        "box Page { birth() { } }
        box Holder { inner: Page
        birth() { me.inner = new Page() }
        run(): i64 { return 0 } }
        static box Main { main() {
        local holder = new Holder()
        return holder.run() } }",
    )
    .expect("sealed owned-field receiver home source");
    let ledger = &package.ordinary_new_claim_ledger;
    let owner = ledger.root_completion_for_test().owner();
    assert!(ledger.root_instance_call_expected(owner));
    // `inner: Page` makes `Holder` `OwnedObjectFieldsNoHook`; its sole
    // birth write is a `Provider` store of the declared class, so the
    // children inventory seals and the home is releasable — the issuer
    // keeps the `Ready` row for `emit_instance`.
    assert!(!ledger.root_instance_call_is_empty());
}

#[test]
fn root_instance_call_stays_unissued_for_owned_field_without_sealed_children() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        "box Page { birth() { } }
        box Holder { inner: Page
        birth() { }
        run(): i64 { return 0 } }
        static box Main { main() {
        local holder = new Holder()
        return holder.run() } }",
    )
    .expect("unsealed owned-field receiver home source");
    let ledger = &package.ordinary_new_claim_ledger;
    let owner = ledger.root_completion_for_test().owner();
    assert!(ledger.root_instance_call_expected(owner));
    // `inner` is declared typed but never written in birth — no sealed
    // residence, `children` is `None`, `end_available` can never hold,
    // so the issuer still withholds the `Ready` row.
    assert!(ledger.root_instance_call_is_empty());
}

#[test]
fn owned_field_receiver_home_seals_call_entry_and_residence_release() {
    // Mirror of `unreleasable_root_call_receiver_passes_finishing_but_not_
    // the_seal` with a typed `inner: Page` field: the receiver home now
    // reaches `end_available`, so the `Ready` row issues, `emit_instance`
    // records the Call entry and the seal must accept — the whole chain,
    // not just issue-time admission.
    let mut package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        "box Page { birth() { } }
        box Holder { inner: Page
        birth() { me.inner = new Page() }
        run(): i64 { return 0 } }
        static box Main { main(args) {
        local holder = new Holder()
        return holder.run() } }",
    )
    .expect("sealed owned-field receiver home source");
    let mut loans = package.direct_call_loans.take();
    let main = package
        .declaration_catalog()
        .source_backed_app_main()
        .expect("app main");
    let declaration = package
        .batch()
        .declarations()
        .find(|row| row.identity().same_as(main.parser_identity()))
        .expect("main declaration");
    let mut builder = crate::mir::MirBuilder::new();
    let mut function = package
        .batch()
        .with_lowering_input_and_source_identity(declaration.batch_slot(), |input, identity| {
            builder.lower_map_dependency_for_test(
                input,
                crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(
                    main.catalog_key().clone(),
                ),
                main.parser_identity(),
                identity.method_source_observation().cloned(),
                std::rc::Rc::clone(&package.ordinary_new_claim_ledger),
                loans.as_mut(),
            )
        })
        .expect("lowering input")
        .unwrap_or_else(|e| panic!("owned-field root call lowers: {e}"));
    if let Some(loans) = loans {
        loans.finish_empty().expect("no direct-call rows owed");
    }
    let ledger = &package.ordinary_new_claim_ledger;
    let observation = ledger
        .validate_finalized_new_root(&function)
        .expect("draft validation");
    function
        .install_root_ordinary_new_observation(observation)
        .expect("observation install");
    ledger
        .validate_after_compiler_finishing(&function)
        .unwrap_or_else(|e| panic!("finishing: {e}"));
    let birth_keys = std::collections::BTreeSet::from([
        hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::birth_constructor("Holder", 0),
        hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::birth_constructor("Page", 0),
    ]);
    let mut module = crate::mir::MirModule::new("owned-field-root-call".into());
    module
        .functions
        .insert(function.signature.name.clone(), function);
    ledger
        .seal_finalized_root_birth_handoff("Main.main/1".into(), &module, &birth_keys, None)
        .unwrap_or_else(|e| panic!("seal must accept the owned-field Call entry: {e}"));
}
