//! Focused boundary tests for `HANDLE-FIELD-READ-S0` — local-initializer
//! `receiver.field` reads.
//!
//! Positive membership requires the sealed chain: a `local x = recv.field`
//! FieldAccess initializer → a receiver binding with a proven provenance
//! (claim-local `new` Home, the entry loan's `me` root, or an earlier
//! proven field-read alias) → a unique declared field on that
//! provenance's own class authority → a declared result type that is
//! either numeric-integer (`Scalar`) or an ordinary registered box
//! (`Alias`). Anything else leaves the statement unclaimed — the `new`
//! claims that follow keep `PrefixNotCovered`, and no ledger row is
//! staged for the read site.

use super::brand_catalog_tests::issue_with_brand_catalog as issue;
use crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1;
use crate::mir::resolved_semantics::{BodyExpressionShapeV1, OwnedExprSiteV1};

/// Exact owned site of the first `FieldAccess` reading `name` anywhere
/// in the package — fixtures keep read field names unique per test.
fn field_access_site(
    package: &super::VerifiedNormalCallableSemanticPackageV1,
    name: &str,
) -> OwnedExprSiteV1 {
    package
        .batch()
        .declarations()
        .find_map(|declaration| {
            package
                .batch()
                .with_lowering_input(declaration.batch_slot(), |input| {
                    input
                        .body_shape()
                        .expect("body shape")
                        .expressions()
                        .iter()
                        .find_map(|row| {
                        match row {
                            BodyExpressionShapeV1::FieldAccess { site, field, .. }
                                if field.as_ref() == name =>
                            {
                                Some(OwnedExprSiteV1::new(
                                    declaration.owner(),
                                    site.clone(),
                                ))
                            }
                            _ => None,
                        }
                    })
                })
                .expect("batch loan")
        })
        .unwrap_or_else(|| panic!("field-access site for {name} not found"))
}

/// The `new` claim's prefix record — `Err(PrefixNotCovered(_))` is the
/// named unavailability an unclaimed statement leaves behind.
fn new_claim_prefix_covered(
    package: &super::VerifiedNormalCallableSemanticPackageV1,
) -> bool {
    package
        .ordinary_new_claim_ledger
        .pending_claims_for_test()
        .values()
        .all(|claim| claim.home_prefix().is_ok())
}

#[test]
fn local_field_read_claims_scalar_on_selected_new_home() {
    for declared in ["i64", "usize", "u64"] {
        let package = issue(&format!(
            "box Pool {{ size: {declared} = 0 birth() {{ }} }}
             static box Main {{ main() {{ local pool = new Pool() local n = pool.size return n }} }}",
        )).expect("existing numeric field profile");
        let site = field_access_site(&package, "size");
        let source = package.ordinary_new_claim_ledger.staged_local_field_source(&site).unwrap();
        assert_eq!(source.result(), &LocalFieldReadResultV1::Scalar);
        assert_eq!(source.declared_type_name(), Some(declared));
        assert_eq!(source.field().declaration_ordinal(), 0);
        assert!(new_claim_prefix_covered(&package));
    }
}

#[test]
fn local_field_read_claims_object_alias_on_selected_new_home() {
    use super::instance_construction::ConstructionUnavailableV1;
    use crate::mir::resolved_semantics::home_new_prefix::HomePrefixUnavailableV1;

    // Classify the source field without granting constructor execution.
    for unavailable in 0..3 {
        let birth = if unavailable == 1 {
            "birth() { me.page = new Page() me.page.seed() }"
        } else {
            "birth() { me.page = new Page() }"
        };
        let init = if unavailable == 2 {
            "new Pool() { page: null }"
        } else {
            "new Pool()"
        };
        let text = format!("box Page {{ id: i64 = 0 birth() {{}} seed(): i64 {{ return 0 }} }} box Pool {{ page: Page {birth} }} static box Main {{ main() {{ local pool = {init} local page = pool.page return 0 }} }}");
        let package = issue(&text).expect("object alias field read package");
        let claims = package.ordinary_new_claim_ledger.pending_claims_for_test();
        let mut pool_claims = claims.values().filter(|claim| claim.class() == "Pool");
        let claim = pool_claims.next().expect("original Pool claim");
        assert!(pool_claims.next().is_none());
        let claim_site = claim.site().clone();
        if unavailable == 1 {
            let constructor = package.instance_constructors().rows().iter()
                .find(|row| row.box_name() == "Pool").unwrap();
            let source = constructor.construction_source().as_ref().expect("original stored-child call structure");
            assert_eq!(source.steps().len(), 2);
            assert_eq!(source.stores().len(), 1);
            assert!(matches!(source.steps()[0], super::instance_construction::ConstructionSourceStepV1::Store(_)));
            assert!(matches!(source.steps()[1], super::instance_construction::ConstructionSourceStepV1::StoredChildCall(_)));
            assert_eq!(constructor.construction(), &Err(ConstructionUnavailableV1::BodyCoverageUnsupported),
                "retained source structure grants no execution eligibility");
            // Same source structure survives the shared caller census. A
            // structurally equal reissued Rc must never impersonate its loan.
            let super::instance_construction::ConstructionSourceStepV1::StoredChildCall(request) = &source.steps()[1] else { unreachable!() };
            let site = request.site();
            let input = constructor.lowering_input(package.batch().source_ast()).unwrap();
            let Some(BodyExpressionShapeV1::FieldAccess { object, field, .. }) = input.body_shape().unwrap().expression_shape(request.receiver_site()) else { panic!("original field receiver") };
            use super::ordinary_new_coseal::{StoredReceiverSourceV1, StoredReceiverSourceOriginV1};
            let mut receiver = StoredReceiverSourceV1 {
                origin: StoredReceiverSourceOriginV1::Constructor(source.child_call_source_v1(site).unwrap().unwrap()),
                parent_new: None,
                parent_binding: request.receiver_binding(), parent_site: object.clone(),
                parent_class: "Pool".into(), field_name: field.clone(), child_class: "Page".into(),
            };
            let expected = Some((request.field(), request.child()));
            assert!(package.ordinary_new_claim_ledger.take_lexical_instance_call(site.owner(), site.site()).unwrap().is_none(),
                "original constructor source is not a qualified scalar caller");
            assert_eq!(receiver.corroborate_constructor_origin_v1(package.batch(), package.instance_constructors(), site).unwrap(), expected);
            let foreign = std::rc::Rc::new(source.as_ref().clone());
            assert_eq!(foreign.as_ref(), source.as_ref());
            receiver.origin = StoredReceiverSourceOriginV1::Constructor(foreign.child_call_source_v1(site).unwrap().unwrap());
            assert!(receiver.corroborate_constructor_origin_v1(package.batch(), package.instance_constructors(), site).is_err());
            receiver.origin = StoredReceiverSourceOriginV1::Constructor(source.child_call_source_v1(site).unwrap().unwrap());
            receiver.field_name = "wrong".into();
            assert!(receiver.corroborate_constructor_origin_v1(package.batch(), package.instance_constructors(), site).is_err());

        }
        // Birth also reads page. Select the caller's actual local initializer,
        // rather than the first matching spelling in the entire package.
        let declaration = package.batch().declarations()
            .find(|declaration| declaration.owner() == claim_site.owner()).unwrap();
        let site = package.batch().with_lowering_input(declaration.batch_slot(), |input| {
            let mut reads = input.body_shape().unwrap().expressions().iter().filter_map(|row| {
                let BodyExpressionShapeV1::FieldAccess { site, field, .. } = row else {
                    return None;
                };
                (field.as_ref() == "page" && input.function().expression_source().initializers()
                    .any(|initializer| initializer.initializer_site() == Some(site)))
                    .then(|| OwnedExprSiteV1::new(input.owner(), site.clone()))
            });
            let site = reads.next().expect("caller local field initializer");
            assert!(reads.next().is_none());
            site
        }).unwrap();
        let (field, result) = package.ordinary_new_claim_ledger.staged_local_field_read(&site)
            .expect("source declaration stages the original field read");
        assert_eq!(result, LocalFieldReadResultV1::Alias("Page".into()));
        assert_eq!(field.declaration_ordinal(), 0);
        match unavailable {
            0 => assert!(claim.construction().is_ok()),
            1 => assert_eq!(claim.construction(), &Err(ConstructionUnavailableV1::BodyCoverageUnsupported)),
            _ => assert_eq!(claim.construction(), &Err(ConstructionUnavailableV1::OverrideUnsupported)),
        }
        if unavailable == 2 {
            assert!(matches!(claim.home_prefix(), Err(HomePrefixUnavailableV1::OverridesNotCovered(_))));
        } else {
            assert!(claim.home_prefix().is_ok(), "the bare Pool prefix is independently covered");
        }
        drop(claims);
        if unavailable != 0 {
            let taken = package.ordinary_new_claim_ledger.try_take(&claim_site, "Pool", 0).unwrap().unwrap();
            assert!(!package.ordinary_new_claim_ledger.prepare_new_emission(&taken).unwrap(),
                "source alias cannot grant New execution");
        }
    }
}

#[test]
fn local_field_read_claims_read_on_field_alias() {
    // The alias's retained declared class is the receiver class
    // authority for the next read — `page.alloc` resolves on Page.
    let package = issue(
        "box Page {
            alloc: i64 = 0
            birth() { }
        }
        box Pool {
            page: Page = new Page()
            birth() { }
        }
        static box Main { main() {
            local pool = new Pool()
            local page = pool.page
            local n = page.alloc
            return n
        } }",
    )
    .expect("read-on-alias package");
    let alias_site = field_access_site(&package, "page");
    let (_, alias_result) = package
        .ordinary_new_claim_ledger
        .staged_local_field_read(&alias_site)
        .expect("staged alias read");
    assert_eq!(alias_result, LocalFieldReadResultV1::Alias("Page".into()));
    let scalar_site = field_access_site(&package, "alloc");
    let (field, result) = package
        .ordinary_new_claim_ledger
        .staged_local_field_read(&scalar_site)
        .expect("staged read on alias");
    assert_eq!(result, LocalFieldReadResultV1::Scalar);
    assert_eq!(field.declaration_ordinal(), 0);
    assert!(new_claim_prefix_covered(&package));
}

#[test]
fn local_field_read_claims_entry_receiver_read() {
    // `me.size` inside a selected instance method — the entry loan's
    // receiver root proves the field on the declaration's own box. The
    // method carries its own `new` claim so the verified walk (not the
    // bounded sibling) scans it.
    let package = issue(
        "box Page {
            birth() { }
        }
        box Pool {
            size: i64 = 0
            birth() { }
            get(): i64 {
                local t = new Page()
                local n = me.size
                return 0
            }
        }
        static box Main { main() {
            local pool = new Pool()
            local r = pool.get()
            return r
        } }",
    )
    .expect("entry receiver field read package");
    // `pool.get()` must claim through the lexical i64 lane so `get` is
    // a selected callable carrying its entry loan into the walk.
    let (owner, call_site) = package
        .batch()
        .declarations()
        .find_map(|declaration| {
            package
                .batch()
                .with_lowering_input(declaration.batch_slot(), |input| {
                    input
                        .function()
                        .method_calls()
                        .find(|(_, call)| call.selector() == "get")
                        .map(|(site, _)| (declaration.owner(), site.clone()))
                })
                .expect("batch loan")
        })
        .expect("get call site");
    assert!(
        package
            .ordinary_new_claim_ledger
            .lexical_instance_call_covered(owner, &call_site),
        "the `pool.get()` call must claim so `get` carries an entry loan"
    );
    let site = field_access_site(&package, "size");
    let (field, result) = package
        .ordinary_new_claim_ledger
        .staged_local_field_read(&site)
        .expect("staged entry-receiver read");
    assert_eq!(result, LocalFieldReadResultV1::Scalar);
    assert_eq!(field.declaration_ordinal(), 0);
}

#[test]
fn local_field_read_alias_is_not_a_movable_argument() {
    // `page` is a borrowed FieldAlias — it must not enter a selected
    // `new` argument slot (moving the receiver root through the alias
    // would double-own the storage).
    let package = issue(
        "box Page {
            id: i64 = 0
            birth() { }
        }
        box Pool {
            page: Page = new Page()
            birth() { }
        }
        box Shell {
            inner: Page = new Page()
            birth(x) { }
        }
        static box Main { main() {
            local pool = new Pool()
            local page = pool.page
            local shell = new Shell(page)
            return 0
        } }",
    )
    .expect("alias-argument package");
    let site = field_access_site(&package, "page");
    assert_eq!(
        package
            .ordinary_new_claim_ledger
            .staged_local_field_read(&site)
            .map(|(_, result)| result),
        Some(LocalFieldReadResultV1::Alias("Page".into())),
        "the alias-producing read itself is claimed"
    );
    assert!(
        !new_claim_prefix_covered(&package),
        "the `new Shell(page)` claim stays uncovered — the alias is not a movable value"
    );
}

#[test]
fn local_field_read_stays_fail_closed() {
    for (label, member) in [
        // No declared field by that name — declaration authority fails.
        ("missing-field", "pool.missing"),
        // A `MapBox` field is neither a numeric integer nor an ordinary
        // registered box — the read is out of this lane's scope.
        ("container-field", "pool.map"),
    ] {
        let source = format!(
            "box Pool {{
                size: i64 = 0
                map: MapBox = new MapBox()
                birth() {{ }}
            }}
            box Page {{
                birth() {{ }}
            }}
            static box Main {{ main() {{
                local pool = new Pool()
                local n = {member}
                local page = new Page()
                return 0
            }} }}"
        );
        let package = issue(&source)
            .unwrap_or_else(|issue| panic!("{label} package: {issue:?}"));
        assert!(
            !new_claim_prefix_covered(&package),
            "{label}: an unclaimed read keeps the following `new` uncovered"
        );
    }
}

#[test]
fn local_field_read_claims_object_alias_on_entry_receiver() {
    // OBJECT-FIELD-READ-S0: `me.page` inside a selected instance method —
    // the entry loan's borrowed `me` root proves the object-typed field
    // on the declaration's own box source through the same staged
    // `local_read_field` issuer arm, and the result binds as a borrowed
    // `Page` alias. The method holds no `new` claim — the object-field
    // read itself selects the verified walk, and the alias-vs-null
    // compare claims beside it.
    let package = issue(
        "box Page {
            id: i64 = 0
            birth() { }
        }
        box Pool {
            page: Page = new Page()
            size: i64 = 0
            birth() { }
            probe(): i64 {
                local l = me.page
                local c = l == null
                return 0
            }
        }
        static box Main { main() {
            local pool = new Pool()
            local r = pool.probe()
            return r
        } }",
    )
    .expect("entry-receiver object field read package");
    // `pool.probe()` must claim through the lexical i64 lane so `probe`
    // carries its entry loan into the walk.
    let (owner, call_site) = package
        .batch()
        .declarations()
        .find_map(|declaration| {
            package
                .batch()
                .with_lowering_input(declaration.batch_slot(), |input| {
                    input
                        .function()
                        .method_calls()
                        .find(|(_, call)| call.selector() == "probe")
                        .map(|(site, _)| (declaration.owner(), site.clone()))
                })
                .expect("batch loan")
        })
        .expect("probe call site");
    assert!(
        package
            .ordinary_new_claim_ledger
            .lexical_instance_call_covered(owner, &call_site),
        "the `pool.probe()` call must claim so `probe` carries an entry loan"
    );
    let site = field_access_site(&package, "page");
    let (field, result) = package
        .ordinary_new_claim_ledger
        .staged_local_field_read(&site)
        .expect("staged `me.` object field read");
    assert_eq!(result, LocalFieldReadResultV1::Alias("Page".into()));
    assert_eq!(field.declaration_ordinal(), 0);
    assert!(new_claim_prefix_covered(&package));

    let source = "box Pool {
        free_stack: ArrayBox = new ArrayBox() block_used: ArrayBox = new ArrayBox()
        use_counts: ArrayBox = new ArrayBox() requested_sizes: ArrayBox = new ArrayBox()
        birth() {} seed() {
            local free_stack = me.free_stack local block_used = me.block_used
            local use_counts = me.use_counts local requested_sizes = me.requested_sizes
            local i = 0 loop(i < 4) {
                free_stack.push(i) block_used.push(0) use_counts.push(0) requested_sizes.push(0) i = i + 1
            }
        }
    } static box Main { main() { return 0 } }";
    let package = issue(source).expect("same original Array field alias census");
    let ledger = &package.ordinary_new_claim_ledger;
    for field_name in ["free_stack", "block_used", "use_counts", "requested_sizes"] {
        let site = field_access_site(&package, field_name);
        let (field, result) = ledger.staged_local_field_read(&site).expect("source field alias staged");
        let LocalFieldReadResultV1::BorrowedArray(loan) = result else { panic!("borrowed Array, no ordinary alias"); };
        assert_eq!(loan.requirement.field(), field);
        assert_eq!(loan.requirement.field_site(), site.site());
        assert_eq!(loan.requirement.owner(), site.owner());
        let originals: Vec<_> = package.source_core_method_calls.values().flat_map(|rows| rows.values())
            .filter_map(|row| match row.contract().named_array_requirement() {
                Some(crate::mir::resolved_semantics::NamedArrayRequirementV1::FieldResidence(source))
                    if source.field_site() == site.site() && source.owner() == site.owner() => Some(source),
                _ => None,
            }).collect();
        assert_eq!(originals.len(), 1);
        assert!(std::rc::Rc::ptr_eq(&loan.requirement, originals[0]));
        let completion = ledger.completion_for_owner(site.owner()).expect("single canonical Completion");
        let flow = completion.cleanup().root_flow().expect("original alias Home walk");
        let exit = flow.implicit_exit().expect("original body end").expect("same Recipe proves original loop After");
        assert!(exit.homes().is_empty(), "Array aliases create no owner");
    }
    let site = field_access_site(&package, "free_stack");
    let declaration = package.batch().declarations().find(|row| row.owner() == site.owner()).unwrap();
    let loop_site = package.batch().with_lowering_input(declaration.batch_slot(), |input| {
        input.forest().callable_source_ledger(input.owner()).unwrap().loop_sites().next().unwrap().clone()
    }).unwrap();
    assert!(ledger.validate_preissued_loop_consumption_v1(site.owner()).is_err(), "Home success is not physical consumption");
    let home = std::rc::Rc::clone(ledger.no_exit_array_home_source_v1(site.owner(), &loop_site).unwrap());
    assert_eq!(home.appends().len(), 4, "every original Push, in source order");
    assert!(home.prior_homes().is_empty(), "source aliases create no owner");
    let flow = ledger.completion_for_owner(site.owner()).unwrap().cleanup().root_flow().unwrap();
    let mut installed = flow.field_array_loop_sources();
    assert!(std::rc::Rc::ptr_eq(&home, installed.next().unwrap()), "canonical Home walk retains the SAME proof");
    assert!(installed.next().is_none());
    let originals = package.source_core_method_calls.values().find(|rows|
        rows.values().any(|row| row.contract().owner() == site.owner())).unwrap();
    for row in home.appends() {
        let requirement = row.requirement();
        let fault = row.fault();
        assert_eq!(fault.call().site(), requirement.call());
        assert_eq!(fault.loop_site(), &loop_site);
        assert!(!fault.crossed_scopes().is_empty(), "the original Loop scope is crossed");
        let original = originals.values().find_map(|row| match row.contract().named_array_requirement() {
            Some(crate::mir::resolved_semantics::NamedArrayRequirementV1::FieldResidence(original))
                if original.call() == requirement.call() => Some(original), _ => None,
        }).unwrap();
        assert!(std::rc::Rc::ptr_eq(original, requirement));
        let contract = originals.values().find(|row| row.contract().call_site() == requirement.call()).unwrap().contract();
        let append = ledger.field_array_append_source_v1(contract).expect("same original source and canonical Home participation");
        assert!(std::rc::Rc::ptr_eq(append.proof(), &home));
        assert!(std::rc::Rc::ptr_eq(append.requirement(), requirement));
        assert_eq!(append.site(), fault.call());
        append.validate_source(ledger).unwrap();
        assert!(std::ptr::eq(append.argument(), row.argument()));
        assert_eq!(append.argument().site(), requirement.argument());
        if requirement.field_name() == "free_stack" {
            assert!(append.argument().binding().is_some());
        } else {
            assert_eq!(append.argument().literal(), Some(0));
        }
    }
    package.batch().with_lowering_input(declaration.batch_slot(), |input| {
        assert!(home.recheck_source(input, originals.values().map(|row| row.contract())));
        assert!(!home.recheck_source(input, originals.values().skip(1).map(|row| row.contract())), "missing original Call refuses");
    }).unwrap();
    let plan = ledger.take_preissued_loop_source_plan_v1(site.owner(), &loop_site).unwrap().unwrap();
    let original = std::rc::Rc::clone(ledger.retained_loop_source_v1(site.owner(), &loop_site).unwrap().unwrap());
    assert!(std::rc::Rc::ptr_eq(&original, plan.original_source_v1()));
    let roles = std::rc::Rc::clone(original.bindings());
    assert_eq!(roles.owner(), site.owner());
    assert_eq!(roles.loop_site(), loop_site.node());
    assert!(std::rc::Rc::ptr_eq(&original, home.original_source_v1()), "Home and affine plan share original source");
    assert!(std::rc::Rc::ptr_eq(&home, ledger.no_exit_array_home_source_v1(site.owner(), &loop_site).unwrap()),
        "affine Recipe take retains the SAME source/Home proof");
    package.batch().with_lowering_input(declaration.batch_slot(), |input| {
        assert_eq!(plan.flat_no_exit_statement_sites(input).unwrap().len(), 5);
        let condition = plan.condition_source_v1(input).unwrap();
        let root = condition.nodes()[0].site().node();
        let reads: Vec<_> = input.function().variable_refs().filter_map(|(site, reference)| {
            let crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding) = reference else { return None; };
            site.node().segments().starts_with(root.segments()).then(|| (site.node().clone(), *binding))
        }).collect();
        assert_eq!(reads.len(), 1, "original carrier read; bound is a literal");
        assert!(original.matches_condition_reads_v1(root, reads.clone()));
        assert!(!original.matches_condition_reads_v1(root, reads.iter().skip(1).cloned()));
        let other = input.function().variable_refs().find_map(|(_, reference)| match reference {
            crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding) if *binding != reads[0].1 => Some(*binding),
            _ => None,
        }).expect("distinct original local binding");
        let mut wrong_binding = reads.clone(); wrong_binding[0].1 = other;
        assert!(!original.matches_condition_reads_v1(root, wrong_binding));
        let mut duplicate = reads.clone(); duplicate.push(reads[0].clone());
        assert!(!original.matches_condition_reads_v1(root, duplicate));
        let mut extra = reads.clone(); extra.push((loop_site.node().clone(), reads[0].1));
        assert!(!original.matches_condition_reads_v1(root, extra));
        assert!(!original.matches_condition_reads_v1(loop_site.node(), reads));
        assert!(std::rc::Rc::ptr_eq(condition, plan.condition_source_v1(input).unwrap()), "one retained original condition plan");
        assert!(matches!(condition.root(), crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::LoopCondition {
            statement, condition: original, bundle, frame,
        } if statement == &loop_site && original == condition.nodes()[0].site()
            && Some(bundle) == input.function().loop_region_bundle(&loop_site).ok()
            && frame.matches(plan.frame())));
        assert!(crate::mir::resolved_value_profile::expression_plan::condition_expression_source_plan_v1(input, &loop_site).is_err(), "Loop is not an If purpose");
        assert!(home.recheck_source(input, originals.values().map(|row| row.contract())), "source proof survives Recipe take");
    }).unwrap();
    let main = package.batch().declarations().find(|row| row.owner() != site.owner()).unwrap();
    package.batch().with_lowering_input(main.batch_slot(), |input| {
        assert!(plan.condition_source_v1(input).is_none(), "foreign function cannot borrow the retained condition");
        assert!(plan.flat_no_exit_statement_sites(input).is_none(), "foreign source cannot borrow this Recipe");
    }).unwrap();
    assert!(ledger.take_preissued_loop_source_plan_v1(site.owner(), &loop_site).unwrap_err().contains("already-taken"));
    assert!(ledger.validate_preissued_loop_consumption_v1(site.owner()).unwrap_err()
        .contains("binding-handoff-unconsumed"), "taking only the planner does not consume its affine binding handoff");
    drop(plan);
    assert!(std::rc::Rc::ptr_eq(&original, ledger.retained_loop_source_v1(site.owner(), &loop_site).unwrap().unwrap()),
        "original source survives destruction of the consumed planner holder");

    // Same Recipe verification: repeated pushes retain distinct call sources;
    // zero iterations and unrelated live Homes preserve the same ownership law.
    for (prefix, bound, body, ready, homes) in [
        ("", "1", "a.push(i) a.push(0) i = i + 1", true, 0),
        ("", "0", "a.push(i) i = i + 1", true, 0),
        ("", "1", "a.push(i + (1 + i)) i = i + 1", true, 0),
        ("local held = new Atom()", "1", "a.push(i) i = i + 1", true, 1),
        ("", "true", "a.push(i) i = i + 1", false, 0),
        ("", "1", "a.push(i) print(7) i = i + 1", false, 0),
        ("", "1", "a.push(i) me.count = 0 i = i + 1", false, 0),
    ] {
        let source = format!("box Atom {{ birth() {{}} }} box Pool {{ values: ArrayBox = new ArrayBox() count: i64 = 0 birth() {{}} seed() {{
            {prefix} local a = me.values local limit = {bound} local i = 0 loop(i < limit) {{ {body} }}
        }} }} static box Main {{ main() {{ return 0 }} }}");
        let package = issue(&source).expect("original source proof case");
        let alias = field_access_site(&package, "values");
        let completion = package.ordinary_new_claim_ledger.completion_for_owner(alias.owner()).unwrap();
        let exit = completion.cleanup().root_flow().unwrap().implicit_exit().unwrap();
        assert_eq!(exit.is_ok(), ready, "{source}");
        if let Ok(exit) = exit {
            assert_eq!(exit.homes().len(), homes, "aliases acquire no Home; existing Homes survive");
            let declaration = package.batch().declarations().find(|row| row.owner() == alias.owner()).unwrap();
            package.batch().with_lowering_input(declaration.batch_slot(), |input| {
                let loop_site = input.forest().callable_source_ledger(input.owner()).unwrap().loop_sites().next().unwrap().clone();
                let proof = package.ordinary_new_claim_ledger.no_exit_array_home_source_v1(input.owner(), &loop_site).unwrap();
                assert_eq!(proof.prior_homes(), exit.homes(), "Fault owes the SAME original pre-Loop live Homes");
                let flow = completion.cleanup().root_flow().unwrap();
                assert!(std::rc::Rc::ptr_eq(proof, flow.field_array_loop_sources().next().unwrap()),
                    "the complete Home law owns this original Loop participation");
                assert_eq!(proof.appends().len(), body.matches(".push(").count());
            }).unwrap();
        }
    }
}

#[test]
fn me_object_field_read_stays_fail_closed_without_declared_field() {
    // `me.missing` names no declared field — declaration authority
    // declines and no read row is staged, so the statement keeps the
    // uncovered boundary the lane always had.
    let package = issue(
        "box Pool {
            size: i64 = 0
            birth() { }
            probe(): i64 {
                local l = me.missing
                return 0
            }
        }
        static box Main { main() {
            local pool = new Pool()
            local r = pool.probe()
            return r
        } }",
    )
    .expect("undeclared `me.` field package");
    assert!(
        package
            .ordinary_new_claim_ledger
            .staged_local_field_read(&field_access_site(&package, "missing"))
            .is_none(),
        "an undeclared `me.` field stages no read row"
    );
}

#[test]
fn local_field_read_rejects_parameter_receiver() {
    // A parameter-rooted read — `p.size` where `p` is an opaque formal —
    // is outside the entry loan's `me` provenance for this slice and
    // stages no row.
    let package = issue(
        "box Pool {
            size: i64 = 0
            birth() { }
        }
        static box Helpers {
            probe(p): i64 {
                local n = p.size
                return n
            }
        }
        static box Main { main() {
            local pool = new Pool()
            return Helpers.probe(pool)
        } }",
    )
    .expect("parameter-receiver package");
    let site = field_access_site(&package, "size");
    assert!(
        package
            .ordinary_new_claim_ledger
            .staged_local_field_read(&site)
            .is_none(),
        "a parameter receiver is not the entry loan's `me` root"
    );
}
