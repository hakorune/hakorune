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
