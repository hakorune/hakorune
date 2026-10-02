// Focused tests for the `NullableObject` result-class claim arm and the
// explicit `NullLiteral` terminal-relation retention.
//
// A callee whose sealed exits are exactly `return new C(...)` plus
// `return null` claims `NullableObject(C)` — proven class, nullable
// result. The claim never authorizes Handle/lifecycle behavior: the
// `callable_result_class` accessor only exposes definite `Object` claims,
// so every owned-result consumer fails closed on a nullable callee.

/// The one result-class claim row for `owner.name`, or `None` when the
/// callable stayed unclaimed.
fn result_class_claim_row<'a>(
    package: &'a super::VerifiedNormalCallableSemanticPackageV1,
    owner: &str,
    name: &str,
) -> Option<(
    &'a hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
    &'a super::OrdinaryNewResultClassV1,
)> {
    package
        .ordinary_new_claim_ledger
        .callable_result_class_claims_for_test()
        .iter()
        .find(|(key, _)| key.owner() == owner && key.name() == name)
}

/// Every `local x = me.m(..)` observation bound to `owner.name` —
/// the claim-first in-walk pass's site-keyed evidence.
fn receiver_observations_for<'a>(
    package: &'a super::VerifiedNormalCallableSemanticPackageV1,
    owner: &str,
    name: &str,
) -> Vec<&'a super::ReceiverCallClassObservationV1> {
    package
        .ordinary_new_claim_ledger
        .receiver_call_observations_for_test()
        .values()
        .filter(|row| row.callee().owner() == owner && row.callee().name() == name)
        .collect()
}

/// The local binding declared by the exact initializer at `site`, or
/// `None` when no declaration owns that initializer — the caller and
/// destination the row must carry verbatim.
fn initializer_destination(
    package: &super::VerifiedNormalCallableSemanticPackageV1,
    site: &crate::mir::resolved_semantics::OwnedExprSiteV1,
) -> Option<crate::mir::resolved_semantics::BindingRefV1> {
    package.batch().declarations().find_map(|declaration| {
        (declaration.owner() == site.owner())
            .then(|| {
                package
                    .batch()
                    .with_lowering_input(declaration.batch_slot(), |input| {
                        input
                            .function()
                            .expression_source()
                            .initializers()
                            .find(|row| row.initializer_site() == Some(site.site()))
                            .map(|row| row.binding())
                    })
            })?
            .expect("batch loan")
    })
}

/// Owner of the one callable whose body shape returns the exact `null`
/// literal — found through the same sealed facts the claim issuer walks.
fn null_return_owner(
    package: &super::VerifiedNormalCallableSemanticPackageV1,
) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
    use crate::mir::resolved_semantics::{BodyStatementShapeV1, ResolvedLiteralSourceV1};
    package
        .batch()
        .declarations()
        .find_map(|declaration| {
            package
                .batch()
                .with_lowering_input(declaration.batch_slot(), |input| {
                    let function = input.function();
                    declaration
                        .body_shape()
                        .statements()
                        .iter()
                        .any(|statement| {
                            matches!(
                                statement,
                                BodyStatementShapeV1::Return { value: Some(site), .. }
                                    if matches!(
                                        function.expression_source().literal(site),
                                        Some(ResolvedLiteralSourceV1::Null)
                                    )
                            )
                        })
                        .then(|| declaration.owner())
                })
                .expect("batch loan")
        })
        .expect("a callable with a `return null` exit")
}

#[test]
fn mixed_new_and_null_exits_issue_nullable_object_claim() {
    let package = issue_with_brand_catalog(
        r#"
box Page {
    init { v }
    birth(v) { me.v = v }
    fetch(flag) {
        if flag == 0 {
            return null
        }
        return new Page(7)
    }
}
static box Main {
    main() {
        local p = new Page(0)
        return 0
    }
}
"#,
    )
    .expect("mixed new/null callee source package");
    let ledger = &package.ordinary_new_claim_ledger;
    let (key, claim) = result_class_claim_row(&package, "Page", "fetch")
        .expect("Page.fetch claims a result class");
    assert!(matches!(
        claim,
        super::OrdinaryNewResultClassV1::NullableObject(class) if class.as_ref() == "Page"
    ));
    // The nullable claim never authorizes owned-result behavior: the
    // Handle-facing accessor stays empty for this callee.
    assert_eq!(ledger.callable_result_class(key), None);
    assert_eq!(ledger.nullable_callable_result_class(key), Some("Page"));
}

#[test]
fn uniform_new_exits_keep_definite_object_claim() {
    let package = issue_with_brand_catalog(
        r#"
box Page {
    init { v }
    birth(v) { me.v = v }
    make(flag) {
        if flag == 0 {
            return new Page(1)
        }
        return new Page(2)
    }
}
static box Main {
    main() {
        local p = new Page(0)
        return 0
    }
}
"#,
    )
    .expect("uniform new callee source package");
    let ledger = &package.ordinary_new_claim_ledger;
    let (key, claim) =
        result_class_claim_row(&package, "Page", "make").expect("Page.make claims a result class");
    assert!(matches!(
        claim,
        super::OrdinaryNewResultClassV1::Object(class) if class.as_ref() == "Page"
    ));
    assert_eq!(ledger.callable_result_class(key), Some("Page"));
    assert_eq!(ledger.nullable_callable_result_class(key), None);
}

#[test]
fn non_literal_or_forwarded_exits_issue_no_result_class_claim() {
    let package = issue_with_brand_catalog(
        r#"
box Page {
    init { v }
    birth(v) { me.v = v }
    via_local() {
        local page = new Page(3)
        return page
    }
    null_only() {
        return null
    }
    mixed_local(flag) {
        if flag == 0 {
            return new Page(4)
        }
        local page = new Page(5)
        return page
    }
}
static box Main {
    main() {
        local p = new Page(0)
        return 0
    }
}
"#,
    )
    .expect("non-uniform callee source package");
    // Forwarded/local returns stay unclaimed — `NullableObject` is only for
    // the exact `new`/`null` grammar.
    for name in ["via_local", "null_only", "mixed_local"] {
        assert!(
            result_class_claim_row(&package, "Page", name).is_none(),
            "Page.{name} must carry no result-class claim"
        );
    }
}

/// A callee that forwards to a claimed callee inherits that class — F3
/// field-receiver instance calls and F5 sole-initializer local forwards
/// compose through the same claim product.
#[test]
fn forwarded_call_exits_compose_the_callee_result_class() {
    let package = issue_with_brand_catalog(
        r#"
box Page {
    init { v }
    birth(v) { me.v = v }
    make(flag) {
        if flag == 0 {
            return new Page(1)
        }
        return new Page(2)
    }
}
box Work {
    page: Page
    birth() { me.page = new Page(0) }
    field_receiver(flag) {
        return me.page.make(flag)
    }
    via_local(flag) {
        local made = me.page.make(flag)
        return made
    }
}
static box Main {
    main() { return 0 }
}
"#,
    )
    .expect("forwarded-call callee source package");
    let ledger = &package.ordinary_new_claim_ledger;
    for name in ["field_receiver", "via_local"] {
        let (key, claim) = result_class_claim_row(&package, "Work", name)
            .unwrap_or_else(|| panic!("Work.{name} composes its callee's result class"));
        assert!(
            matches!(
                claim,
                super::OrdinaryNewResultClassV1::Object(class) if class.as_ref() == "Page"
            ),
            "Work.{name} claims the definite callee class"
        );
        assert_eq!(ledger.callable_result_class(key), Some("Page"));
    }
    // F1 bare direct calls stay in the grammar, but the package's
    // direct-call co-seal gate only issues observations whose callee
    // carries a result signature — object classes have none today, so
    // no in-lane fixture can exercise a composed F1 claim.
}

/// The real `page_heap_box.hako` fixture family: `HakoAllocPage.allocate`
/// is the leaf mixed null/`new` claim, `HakoAllocHeap.allocate` composes
/// it through two `me.<field>.allocate` exits plus `return null`, and
/// `HakoAllocHeap.realloc` stays unclaimed because it forwards to the
/// parameter-returning `resizeInPlace`.
#[test]
fn page_heap_fixture_composes_allocate_and_keeps_realloc_unclaimed() {
    let package = issue_with_brand_catalog(include_str!(
        "../../../lang/src/hako_alloc/memory/page_heap_box.hako"
    ))
    .expect("page_heap fixture source package");
    let ledger = &package.ordinary_new_claim_ledger;
    let (key, claim) = result_class_claim_row(&package, "HakoAllocHeap", "allocate")
        .expect("HakoAllocHeap.allocate composes the page claim");
    assert!(matches!(
        claim,
        super::OrdinaryNewResultClassV1::NullableObject(class)
            if class.as_ref() == "HakoAllocHandle"
    ));
    // Nullable composed claims never authorize Handle behavior.
    assert_eq!(ledger.callable_result_class(key), None);
    assert_eq!(
        ledger.nullable_callable_result_class(key),
        Some("HakoAllocHandle")
    );
    assert!(
        result_class_claim_row(&package, "HakoAllocHeap", "realloc").is_none(),
        "realloc forwards to a parameter-returning callee — no claim"
    );
}

/// A `return` nested inside a trailing `if` (or any non-completing
/// terminal) can never mint a claim. The claim issuer now derives exits
/// from the verified Completion — the same evidence that already makes
/// package issuance reject a reachable fallthrough (`NonTerminalReturn`),
/// so the claim layer no longer re-derives "last statement" by itself.
#[test]
fn nested_return_with_fallthrough_is_rejected_before_any_claim() {
    for source in [
        // Nested return as the flat list's last row — the shape the old
        // tail check misread as an all-exit body.
        r#"
box Page {
    init { v }
    birth(v) { me.v = v }
    no_else(flag) {
        local x = flag
        if flag == 0 {
            return new Page(1)
        }
    }
}
static box Main {
    main() { return 0 }
}
"#,
        // An else branch that does not return keeps a fallthrough path.
        r#"
box Page {
    init { v }
    birth(v) { me.v = v }
    else_fallthrough(flag) {
        if flag == 0 {
            return new Page(1)
        } else {
            me.v = flag
        }
        me.v = flag + 1
    }
}
static box Main {
    main() { return 0 }
}
"#,
    ] {
        assert!(
            issue_with_brand_catalog(source).is_err(),
            "a reachable fallthrough past a value return must be rejected"
        );
    }
    // Positive control: nested `if` returns plus a terminal `return`
    // still claim — every path verified by Completion exits with a value.
    let package = issue_with_brand_catalog(
        r#"
box Page {
    init { v }
    birth(v) { me.v = v }
    nested_positive(flag) {
        if flag == 0 {
            return new Page(1)
        }
        return new Page(2)
    }
}
static box Main {
    main() { return 0 }
}
"#,
    )
    .expect("all-exit value-return callee source package");
    let (_, claim) = result_class_claim_row(&package, "Page", "nested_positive")
        .expect("nested_positive keeps its claim");
    assert!(matches!(
        claim,
        super::OrdinaryNewResultClassV1::Object(class) if class.as_ref() == "Page"
    ));
}

/// Fail-closed forwarded grammar: a parameter return, a rebound local,
/// class disagreement, and a recursive/mutually recursive forwarded
/// cycle all leave the caller unclaimed — and the fixpoint terminates.
#[test]
fn forwarded_grammar_negative_exits_issue_no_result_class_claim() {
    let package = issue_with_brand_catalog(
        r#"
box Page {
    init { v }
    birth(v) { me.v = v }
    make(flag) { return new Page(flag) }
}
box Other {
    init { v }
    birth(v) { me.v = v }
}
box Work {
    page: Page
    birth() { me.page = new Page(0) }
    param_return(p) {
        return p
    }
    rebound(flag) {
        local made = me.page.make(flag)
        made = me.page.make(flag + 1)
        return made
    }
    disagree(flag) {
        if flag == 0 {
            return me.page.make(flag)
        }
        return new Other(0)
    }
    rec_a(flag) {
        return me.rec_b(flag)
    }
    rec_b(flag) {
        return me.rec_a(flag)
    }
}
static box Main {
    main() { return 0 }
}
"#,
    )
    .expect("negative forwarded callee source package");
    for name in ["param_return", "rebound", "disagree", "rec_a", "rec_b"] {
        assert!(
            result_class_claim_row(&package, "Work", name).is_none(),
            "Work.{name} must carry no result-class claim"
        );
    }
    // The leaf callee still claims — negative callers do not veto it.
    assert!(matches!(
        result_class_claim_row(&package, "Page", "make"),
        Some((_, super::OrdinaryNewResultClassV1::Object(class)))
            if class.as_ref() == "Page"
    ));
}

/// `local x = me.m(..)` initializers bind to the callee's composed claim:
/// an `Object` callee yields definite evidence and a `NullableObject`
/// callee yields nullable evidence — site-keyed, claim-faithful rows that
/// never authorize Handle behavior by themselves.
#[test]
fn receiver_call_initializers_observe_the_composed_result_class() {
    let package = issue_with_brand_catalog(
        r#"
box Page {
    init { v }
    birth(v) { me.v = v }
    make(flag) { return new Page(flag) }
    fetch(flag) {
        if flag == 0 {
            return null
        }
        return new Page(7)
    }
    observe(flag) {
        local made = me.make(flag)
        local fetched = me.fetch(flag)
        local literal = me.make(7)
        return 0
    }
}
static box Main {
    main() { return 0 }
}
"#,
    )
    .expect("receiver-call observation source package");
    let made = receiver_observations_for(&package, "Page", "make");
    assert_eq!(made.len(), 2, "the `flag` and `7` `me.make` sites");
    assert!(made.iter().all(|row| matches!(
        row.class(),
        super::OrdinaryNewResultClassV1::Object(class) if class.as_ref() == "Page"
    )));
    // Typed arguments: the parameter arg is an exact `Local` binding and
    // the literal arg carries its sealed value — both fail-closed kinds.
    use crate::mir::resolved_semantics::home_new_prefix::SelectedNewArgumentKindV1;
    assert!(made.iter().any(|row| matches!(
        row.arguments(),
        [arg] if matches!(arg.kind(), SelectedNewArgumentKindV1::Local { .. })
            && arg.ordinal() == 0
    )));
    assert!(made.iter().any(|row| matches!(
        row.arguments(),
        [arg] if matches!(arg.kind(), SelectedNewArgumentKindV1::Integer(7))
            && arg.ordinal() == 0
    )));
    let fetched = receiver_observations_for(&package, "Page", "fetch");
    assert_eq!(fetched.len(), 1, "exactly one `me.fetch` initializer site");
    assert!(matches!(
        fetched[0].class(),
        super::OrdinaryNewResultClassV1::NullableObject(class) if class.as_ref() == "Page"
    ));
    // Exact caller/site/destination: every row key resolves to the
    // initializer's own declaration owner, and the row's destination is
    // the binding that initializer declares — nothing recomputed.
    for (site, row) in package
        .ordinary_new_claim_ledger
        .receiver_call_observations_for_test()
    {
        assert_eq!(site.owner(), row.destination().owner());
        assert_eq!(
            initializer_destination(&package, site),
            Some(row.destination()),
            "destination is the exact `local` the site declares"
        );
    }
}

/// Fail-closed receiver-call grammar: an unclaimed callee, a `me.f.m`
/// receiver, a parameter receiver, a same-box local receiver (the entry
/// loan proves `me` only — no other binding counts), and a rebound
/// destination all stay unobserved — the existing `BoundValue` +
/// `PrefixNotCovered` floor.
#[test]
fn receiver_call_observation_negative_edges_stay_unobserved() {
    let package = issue_with_brand_catalog(
        r#"
box Page {
    init { v }
    birth(v) { me.v = v }
    make(flag) { return new Page(flag) }
    param_return(p) { return p }
    observe(flag) {
        local made = me.make(flag)
        return 0
    }
    unclaimed(flag) {
        local same = me.param_return(flag)
        return 0
    }
    param_receiver(p, flag) {
        local made = p.make(flag)
        return 0
    }
    local_receiver(flag) {
        local p = new Page(0)
        local made = p.make(flag)
        return 0
    }
    complex_argument(flag) {
        local made = me.make(flag + 1)
        return 0
    }
    rebound(flag) {
        local made = me.make(flag)
        made = me.make(flag + 1)
        return 0
    }
}
box Work {
    page: Page
    birth() { me.page = new Page(0) }
    field_receiver(flag) {
        local made = me.page.make(flag)
        return 0
    }
}
static box Main {
    main() { return 0 }
}
"#,
    )
    .expect("negative receiver-call source package");
    let made = receiver_observations_for(&package, "Page", "make");
    // Only `observe`'s sole-initialized `me.make` site is evidence — the
    // field receiver, parameter receiver, same-box local receiver,
    // rebound destination, and the `flag + 1` argument site all keep the
    // non-coverage floor.
    assert_eq!(made.len(), 1);
    assert!(
        receiver_observations_for(&package, "Page", "param_return").is_empty(),
        "an unclaimed callee never mints an observation"
    );
}

/// `me.m(..)` inside `birth` carries no InstanceBoxMethod entry loan —
/// and the receiver-non-escape gate rejects the use before issuance, so
/// the unloaned `me` receiver can never reach the observer at all.
#[test]
fn receiver_call_in_birth_is_rejected_before_observation() {
    let error = issue_with_brand_catalog(
        r#"
box Page {
    init { v }
    birth(v) {
        local made = me.make(v)
        me.v = v
    }
    make(flag) { return new Page(flag) }
}
static box Main {
    main() { return 0 }
}
"#,
    )
    .expect_err("an unloaned `me` receiver is rejected upstream");
    let report = format!("{error:?}");
    assert!(
        report.contains("ReceiverNonEscape"),
        "birth-body `me.m` fails the receiver-non-escape proof: {report}"
    );
}

/// The real `page_heap_box.hako` sites: `local handle = me.allocate(size)`
/// and `local replacement = me.allocate(requested_size)` observe the
/// composed `NullableObject(HakoAllocHandle)` claim, while `me.realloc`
/// and the `me.<field>.m` sites stay unobserved.
#[test]
fn page_heap_fixture_observes_me_allocate_and_floors_realloc() {
    let package = issue_with_brand_catalog(include_str!(
        "../../../lang/src/hako_alloc/memory/page_heap_box.hako"
    ))
    .expect("page_heap fixture source package");
    let ledger = &package.ordinary_new_claim_ledger;
    let allocate = receiver_observations_for(&package, "HakoAllocHeap", "allocate");
    assert_eq!(
        allocate.len(),
        2,
        "allocateResult:219 and realloc:287 are the two `me.allocate` sites"
    );
    for row in &allocate {
        assert!(matches!(
            row.class(),
            super::OrdinaryNewResultClassV1::NullableObject(class)
                if class.as_ref() == "HakoAllocHandle"
        ));
    }
    assert!(
        receiver_observations_for(&package, "HakoAllocHeap", "realloc").is_empty(),
        "realloc stays unclaimed — its `me.realloc` site keeps the floor"
    );
    // `me.<field>.m(..)` receivers are `Other` — never observed here even
    // if the callee were claimed.
    assert!(ledger
        .receiver_call_observations_for_test()
        .values()
        .all(|row| row.callee().owner() == "HakoAllocHeap" && row.callee().name() == "allocate"));
}

#[test]
fn mixed_callee_null_literal_terminal_relation_is_retained() {
    use crate::mir::resolved_semantics::home_new_prefix::{
        TerminalRelationV1, TerminalReturnedSourceV1,
    };
    let package = issue_with_brand_catalog(
        r#"
box Page {
    init { v }
    birth(v) { me.v = v }
    fetch(flag) {
        if flag == 0 {
            return null
        }
        return new Page(7)
    }
}
static box Main {
    main() {
        local p = new Page(0)
        return 0
    }
}
"#,
    )
    .expect("mixed new/null callee source package");
    let owner = null_return_owner(&package);
    let relations = package
        .ordinary_new_claim_ledger
        .terminal_relations_for_owner(owner);
    // The `return new` exit keeps its construction evidence and the
    // `return null` exit now lands its NullLiteral row in the child
    // relation index — dropping it would let the mixed callee masquerade
    // as a uniform-object callee downstream.
    assert!(relations.iter().any(|relation| matches!(
        relation,
        TerminalRelationV1::Value(row)
            if matches!(row.returned(), TerminalReturnedSourceV1::NullLiteral)
    )));
    assert!(relations.iter().any(|relation| matches!(
        relation,
        TerminalRelationV1::Value(row)
            if matches!(row.returned(), TerminalReturnedSourceV1::Construction(_))
    )));
}

/// A sealed `Nullable` receiver-call observation also mints the caller's
/// `Nullable` local-call flow row — the emission-side membership check
/// (`nullable_call_source`) reads exactly this row, so the scan and the
/// ledger lookup must agree on site, owner, and class.
#[test]
fn nullable_receiver_call_mints_the_caller_local_call_flow_row() {
    let package = issue_with_brand_catalog(
        r#"
box Page {
    init { v }
    birth(v) { me.v = v }
    make(flag) { return new Page(flag) }
    fetch(flag) {
        if flag == 0 {
            return null
        }
        return new Page(7)
    }
    observe(flag) {
        local made = me.make(flag)
        local fetched = me.fetch(flag)
        return 0
    }
}
static box Main {
    main() { return 0 }
}
"#,
    )
    .expect("nullable receiver-call flow source package");
    let ledger = &package.ordinary_new_claim_ledger;
    for (site, row) in ledger.receiver_call_observations_for_test() {
        let expected_nullable = matches!(
            row.class(),
            super::OrdinaryNewResultClassV1::NullableObject(_)
        );
        let flow = ledger
            .completion_for_owner(site.owner())
            .and_then(|completion| completion.cleanup().root_flow())
            .unwrap_or_else(|| panic!("{site:?} caller has a root flow"));
        let local = flow.local_calls().iter().find(|call| call.site() == site);
        if expected_nullable {
            let local = local.unwrap_or_else(|| panic!("{site:?} mints a local-call flow row"));
            assert_eq!(
                local.result(),
                crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Nullable,
                "{site:?} flow row class mirrors the sealed observation class"
            );
            assert_eq!(
                local.local_binding().expect("local destination").1,
                row.destination(),
                "the flow row keeps the exact declared destination"
            );
        } else {
            // `Object` receiver observations keep the generic call floor —
            // no lifecycle flow row is minted for the Handle-absent `me`
            // receiver lane.
            assert!(
                local.is_none(),
                "{site:?} definite receiver calls mint no lifecycle flow row"
            );
        }
    }
}

#[test]
fn nullable_callee_result_claim_seals_the_returned_class() {
    let package = issue_with_brand_catalog(
        r#"
box Probe {
    v: i64
    birth(v) { me.v = v }
    fetch(flag) {
        if flag == 0 {
            return null
        }
        return new Probe(7)
    }
    run(flag: i64) {
        local h = me.fetch(flag)
        return 0
    }
}
static box Main {
    main() {
        local p = new Probe(1)
        return p.run(0)
    }
}
"#,
    )
    .expect("probe fixture package");
    let ledger = &package.ordinary_new_claim_ledger;
    let claims = ledger.pending_result_claims_for_test();
    let classes: Vec<&str> = claims.values().map(|c| c.class()).collect();
    assert_eq!(
        classes,
        ["Probe"],
        "the nullable callee's `return new Probe(7)` seals exactly one result claim"
    );
    for claim in claims.values() {
        assert!(claim.home_prefix().is_ok(), "result home prefix sealed");
        assert!(claim.construction().is_ok(), "result construction sealed");
        assert!(claim.argument_rows().is_ok(), "result arguments sealed");
    }
}

/// Real `page_heap_box.hako` claim census: the nine retained `return new`
/// sites split by evidence state. `HakoAllocPage.allocate`'s sole site
/// carries a sealed construction and `me.page_id` argument evidence; its
/// prefix covers the `me.<field> = ..` receiver writes and the
/// manifest-proven `me.<ArrayBox>.get/.set` cluster, stopping at
/// `me.requested_sizes.set(block_id, requested_size)` (Body(8)) — the
/// `requested_size` parameter is an `OpaqueHandle` and stays outside
/// Home-neutral argument admission by design.
/// Every `HakoAllocHandleResult` site is blocked on the result class's
/// Birth plan — `me.handle = handle` stores an object-typed parameter —
/// and `reallocResult`'s two trailing sites additionally stop their
/// prefixes at the unclaimed `me.realloc` forward (Body(3)).
#[test]
fn page_heap_fixture_result_claim_census() {
    let package = issue_with_brand_catalog(include_str!(
        "../../../lang/src/hako_alloc/memory/page_heap_box.hako"
    ))
    .expect("page_heap fixture source package");
    let ledger = &package.ordinary_new_claim_ledger;
    let claims = ledger.pending_result_claims_for_test();
    assert_eq!(claims.len(), 9, "nine retained result-new claims");
    let mut handle_sites = 0;
    let mut result_ok_prefix = 0;
    let mut result_bad_prefix = 0;
    for (key, claim) in claims.iter() {
        match claim.class() {
            "HakoAllocHandle" => {
                handle_sites += 1;
                assert!(claim.construction().is_ok(), "{key:?} construction");
                match claim.home_prefix() {
                    Err(crate::mir::resolved_semantics::home_new_prefix::HomePrefixUnavailableV1::PrefixNotCovered(site)) => {
                        assert_eq!(
                            site.node().segments().last(),
                            Some(&crate::mir::resolved_semantics::SourcePathSegmentV1::Body(8)),
                            "{key:?} prefix covers the field-call cluster and stops at `requested_size`"
                        );
                    }
                    other => panic!("{key:?} expected PrefixNotCovered, got {other:?}"),
                }
            }
            "HakoAllocHandleResult" => {
                assert!(claim.construction().is_err(), "{key:?} construction");
                if claim.home_prefix().is_ok() {
                    result_ok_prefix += 1;
                } else {
                    result_bad_prefix += 1;
                }
            }
            other => panic!("unexpected retained class {other}"),
        }
        assert!(claim.argument_rows().is_ok(), "{key:?} argument rows");
    }
    assert_eq!(handle_sites, 1, "one HakoAllocHandle site");
    assert_eq!(
        (result_ok_prefix, result_bad_prefix),
        (6, 2),
        "six prefix-covered plus two realloc-forward-blocked sites"
    );
}

/// A `v: i64` declared-field box seals a complete result claim for a
/// return-position `new` — the declared field list is the construction
/// authority, and every evidence row arrives `Ok`.
#[test]
fn declared_fields_result_claim_carries_complete_evidence() {
    let package = issue_with_brand_catalog(
        r#"
box Probe {
    v: i64
    birth(v) { me.v = v }
    fetch(flag) {
        if flag == 0 {
            return null
        }
        return new Probe(7)
    }
    run() {
        local h = me.fetch(0)
        return 0
    }
}
static box Main {
    main() {
        local p = new Probe(1)
        return p.run()
    }
}
"#,
    )
    .expect("probe fixture package");
    let ledger = &package.ordinary_new_claim_ledger;
    let local_claims = ledger.pending_claims_for_test();
    let claims: Vec<_> = local_claims.values().collect();
    assert_eq!(claims.len(), 1, "one `local p = new Probe(1)` claim");
    for claim in claims {
        assert!(claim.home_prefix().is_ok(), "local home prefix sealed");
        assert!(claim.construction().is_ok(), "local construction sealed");
        assert!(claim.argument_rows().is_ok(), "local arguments sealed");
    }
    let result_rows = ledger.pending_result_claims_for_test();
    let result_claims: Vec<_> = result_rows.values().collect();
    assert_eq!(
        result_claims.len(),
        1,
        "one `return new Probe(7)` result claim"
    );
    for claim in result_claims {
        assert!(claim.home_prefix().is_ok(), "result home prefix sealed");
        assert!(claim.construction().is_ok(), "result construction sealed");
        assert!(claim.argument_rows().is_ok(), "result arguments sealed");
    }
}
