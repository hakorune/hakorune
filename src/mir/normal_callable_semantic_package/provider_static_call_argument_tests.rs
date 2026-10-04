//! Focused seal tests for `MIRBUILDER-APP-MIMALLOC-LITE-HEAP-PROVIDER-CALL-ARG-S0`
//! — the provider-`new` argument arm that admits a proven `Alias.m(..)`
//! qualified static call inside a Birth construction plan.
//!
//! The seal is fail-closed at `FieldContractUnsupported`: the arm requires
//! the resolver's `QualifiedUnbound` receiver shape, an exact `(caller,
//! site)` claim-index row naming the `StaticBoxMethod` target, matching
//! arity, and Integer/Bool literal actuals with i64 evidence at every
//! required-i64 ordinal.  Any deviation leaves the plan ineligible — the
//! unsupported argument is never silently sealed or skipped.

use super::brand_catalog_tests::issue_with_brand_catalog as issue;
use super::{
    ConstructionEligibilityV1, ConstructionStoreRhsV1, ConstructionUnavailableV1,
    OrdinaryNewTrivialArgumentKindV1, QualifiedStaticCallArgumentKindV1,
};

fn parent_construction(source: &str) -> ConstructionEligibilityV1 {
    let package = issue(source).expect("package issues with the ineligible plan");
    package
        .instance_constructors()
        .rows()
        .iter()
        .find(|row| row.box_name() == "Parent")
        .expect("parent birth row")
        .construction()
        .clone()
}

fn expect_field_contract_unsupported(label: &str, provider_args: &str) {
    let source = format!(
        "static box LayoutBox {{
            class_size(unused) {{ return 8 }}
            class_id(base) {{ return base }}
            make(unused) {{ return new Page(0, 0) }}
        }}
        box Page {{
            s: i64
            birth(a, b) {{ me.s = a }}
        }}
        box Parent {{
            child: Page = new Page({provider_args})
            birth() {{ }}
        }}
        static box Main {{ main() {{ return 0 }} }}"
    );
    let construction = parent_construction(&source);
    assert!(
        matches!(
            construction,
            Err(ConstructionUnavailableV1::FieldContractUnsupported)
        ),
        "{label}: unproven provider argument keeps FieldContractUnsupported, got {construction:?}"
    );
}

#[test]
fn provider_static_call_argument_seals_target_and_literal_actuals() {
    let construction = parent_construction(
        "static box LayoutBox { class_size(unused) { return 8 } }
        box Page {
            s: i64
            birth(a, b) { me.s = a }
        }
        box Parent {
            child: Page = new Page(0, LayoutBox.class_size(0))
            birth() { }
        }
        static box Main { main() { return 0 } }",
    );
    let plan = construction.expect("sealed provider argument stays eligible");
    let rhs = plan
        .stores()
        .iter()
        .map(|store| store.rhs())
        .find(|rhs| matches!(rhs, ConstructionStoreRhsV1::ProviderConstruction { .. }))
        .expect("one provider construction store");
    let ConstructionStoreRhsV1::ProviderConstruction {
        caller, arguments, ..
    } = rhs
    else {
        unreachable!()
    };
    assert_eq!(
        caller,
        &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::birth_constructor("Parent", 0),
        "the caller half of the publication row is this birth"
    );
    assert_eq!(arguments.len(), 2);
    assert_eq!(
        arguments[0].kind(),
        &OrdinaryNewTrivialArgumentKindV1::Integer(0)
    );
    let OrdinaryNewTrivialArgumentKindV1::QualifiedStaticCall {
        target,
        arguments: actuals,
    } = arguments[1].kind()
    else {
        panic!(
            "second argument seals the qualified static call, got {:?}",
            arguments[1].kind()
        )
    };
    assert_eq!(
        target,
        &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::static_box_method(
            "LayoutBox",
            "class_size",
            1
        )
    );
    assert_eq!(
        actuals.as_ref(),
        &[QualifiedStaticCallArgumentKindV1::Integer(0)]
    );
}

#[test]
fn provider_static_call_arguments_seal_in_source_order() {
    let construction = parent_construction(
        "static box LayoutBox {
            first(unused) { return 1 }
            second(unused) { return 2 }
        }
        box Page {
            s: i64
            birth(a, b) { me.s = a }
        }
        box Parent {
            child: Page = new Page(LayoutBox.first(1), LayoutBox.second(2))
            birth() { }
        }
        static box Main { main() { return 0 } }",
    );
    let plan = construction.expect("two qualified static arguments stay eligible");
    let rhs = plan
        .stores()
        .iter()
        .map(|store| store.rhs())
        .find(|rhs| matches!(rhs, ConstructionStoreRhsV1::ProviderConstruction { .. }))
        .expect("one provider construction store");
    let ConstructionStoreRhsV1::ProviderConstruction { arguments, .. } = rhs else {
        unreachable!()
    };
    let names: Vec<&str> = arguments
        .iter()
        .map(|argument| match argument.kind() {
            OrdinaryNewTrivialArgumentKindV1::QualifiedStaticCall { target, .. } => target.name(),
            other => panic!("both arguments seal qualified static calls, got {other:?}"),
        })
        .collect();
    assert_eq!(
        names,
        ["first", "second"],
        "source order is the sealed order"
    );
}

#[test]
fn provider_static_call_argument_stays_fail_closed() {
    for (label, args) in [
        // No `StaticBoxMethod` declaration exists for the call — the claim
        // index keeps no row for the site.
        ("unresolved-method", "0, LayoutBox.missing(0)"),
        // `Page` is an instance box: no static target inventory row.
        ("non-static-receiver", "0, Page.class_size(0)"),
        // `make` returns an object — `ExactI64` membership never seals.
        ("non-i64-result", "0, LayoutBox.make(0)"),
        // A non-literal inner actual is outside the literal seal.
        ("compound-inner-arg", "0, LayoutBox.class_size(0 + 1)"),
        // `class_id` returns its parameter, so ordinal 0 is a required-i64
        // position — Bool evidence refuses the seal.
        ("bool-at-required-i64", "0, LayoutBox.class_id(true)"),
        // A bare call is not the qualified-static receiver shape.
        ("unqualified-call", "0, class_size(0)"),
    ] {
        expect_field_contract_unsupported(label, args);
    }
}
