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
