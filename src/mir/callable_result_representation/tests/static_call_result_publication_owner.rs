use crate::mir::resolved_semantics::SourcePathSegmentV1;

use super::super::{
    StaticCallResultPublicationOwnerTakeErrorV1, StaticCallResultPublicationTakeV1,
    VerifiedCallableResultRepresentationV1, VerifiedStaticCallResultPublicationOwnerV1,
};
use super::support::{
    declarations, extend_current_owner_targets, key, qualified_targets, seal_with_targets, site,
    CallSiteSpecV1,
};

const SOURCE: &str = r#"
static box StringHelpers {
  int_to_str(n) { local value = me.to_i64("x") return value }
  to_i64(x) { return x + 1 }
}
"#;

fn call_site() -> crate::mir::resolved_semantics::SourceExprSiteV1 {
    site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Initializer(0),
    ])
}

fn digit_call_site() -> crate::mir::resolved_semantics::SourceExprSiteV1 {
    site(vec![
        SourcePathSegmentV1::Body(12),
        SourcePathSegmentV1::LoopBody(2),
        SourcePathSegmentV1::Initializer(0),
    ])
}

fn return_call_site() -> crate::mir::resolved_semantics::SourceExprSiteV1 {
    site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
    ])
}

#[test]
fn issuer_keeps_exact_source_row_and_consumes_it_once() {
    let declarations = declarations(SOURCE);
    let targets = qualified_targets(&declarations, &[], &[]);
    let targets = extend_current_owner_targets(
        targets,
        &declarations,
        &[CallSiteSpecV1 {
            caller_owner: "StringHelpers",
            caller_name: "int_to_str",
            caller_arity: 1,
            site: call_site(),
        }],
    );
    let results = seal_with_targets(&declarations, &targets);
    let mut owner =
        VerifiedStaticCallResultPublicationOwnerV1::issue(&declarations, &targets, &results)
            .expect("source-bound owner must issue exact rows");
    assert_eq!(owner.len(), 1);

    let caller = key(&declarations, "StringHelpers", "int_to_str", 1);
    let target = key(&declarations, "StringHelpers", "to_i64", 1);
    assert_eq!(
        owner.take(&declarations, &caller, &call_site(), &caller),
        Err(
            StaticCallResultPublicationOwnerTakeErrorV1::TargetMismatch {
                caller: caller.clone(),
                site: call_site(),
                expected: target.clone(),
                actual: caller.clone(),
            }
        )
    );
    let decision = owner
        .take(&declarations, &caller, &call_site(), &target)
        .expect("branded take must verify");
    let StaticCallResultPublicationTakeV1::Selected(handoff) = decision else {
        panic!("exact row must be selected")
    };
    assert_eq!(handoff.caller(), &caller);
    assert_eq!(handoff.target(), &target);
    assert_eq!(
        owner.take(&declarations, &caller, &call_site(), &target),
        Err(
            StaticCallResultPublicationOwnerTakeErrorV1::SelectedRowAlreadyConsumed {
                caller,
                site: call_site(),
                target,
            }
        )
    );
}

#[test]
fn issuer_projects_actual_string_helpers_general_row_into_the_same_owner() {
    let source = include_str!(concat!(
        "../../../../lang/src/shared/common/",
        "string_helpers.hako"
    ));
    let declarations = declarations(source);
    let targets = qualified_targets(&declarations, &[], &[]);
    let targets = extend_current_owner_targets(
        targets,
        &declarations,
        &[
            CallSiteSpecV1 {
                caller_owner: "StringHelpers",
                caller_name: "int_to_str",
                caller_arity: 1,
                site: call_site(),
            },
            CallSiteSpecV1 {
                caller_owner: "StringHelpers",
                caller_name: "to_i64",
                caller_arity: 1,
                site: digit_call_site(),
            },
        ],
    );
    let results = seal_with_targets(&declarations, &targets);
    let caller = key(&declarations, "StringHelpers", "int_to_str", 1);
    let target = key(&declarations, "StringHelpers", "to_i64", 1);
    assert!(
        results.call_result(&caller, &call_site()).is_some(),
        "actual to_i64 call must already have the general exact result proof"
    );

    let mut owner =
        VerifiedStaticCallResultPublicationOwnerV1::issue(&declarations, &targets, &results)
            .expect("general and bounded rows must share one owner");
    let decision = owner
        .take(&declarations, &caller, &call_site(), &target)
        .expect("general row take must verify");
    let StaticCallResultPublicationTakeV1::Selected(handoff) = decision else {
        panic!("general exact row must be selected")
    };
    assert_eq!(handoff.caller(), &caller);
    assert_eq!(handoff.target(), &target);
    assert!(handoff.required_i64_arguments().is_empty());
}

#[test]
fn exact_nominal_box_row_reaches_the_owned_publication_handoff() {
    let source = r#"
        box ProductV1 { birth() {} }
        static box ProductFactoryV1 {
            make() { return new ProductV1() }
            forward() { return me.make() }
        }
    "#;
    let declarations = declarations(source);
    let targets = qualified_targets(&declarations, &[], &[]);
    let targets = extend_current_owner_targets(
        targets,
        &declarations,
        &[CallSiteSpecV1 {
            caller_owner: "ProductFactoryV1",
            caller_name: "forward",
            caller_arity: 0,
            site: return_call_site(),
        }],
    );
    let results = seal_with_targets(&declarations, &targets);
    let caller = key(&declarations, "ProductFactoryV1", "forward", 0);
    let target = key(&declarations, "ProductFactoryV1", "make", 0);
    let mut owner =
        VerifiedStaticCallResultPublicationOwnerV1::issue(&declarations, &targets, &results)
            .expect("exact Box row must issue through the existing owner");

    let StaticCallResultPublicationTakeV1::Selected(handoff) = owner
        .take(&declarations, &caller, &return_call_site(), &target)
        .expect("exact Box row must select")
    else {
        panic!("exact Box row must not become unselected")
    };
    let (demand, required_i64_arguments) = handoff.consume();
    assert_eq!(
        demand.representation(),
        &VerifiedCallableResultRepresentationV1::ExactNominalBox {
            box_name: "ProductV1".to_owned(),
        }
    );
    assert!(required_i64_arguments.is_empty());
}

#[test]
fn exact_source_target_without_an_i64_result_stays_unselected() {
    let source = r#"
        static box TextOwner {
            caller() { return me.text() }
            text() { return "text" }
        }
    "#;
    let declarations = declarations(source);
    let targets = qualified_targets(&declarations, &[], &[]);
    let targets = extend_current_owner_targets(
        targets,
        &declarations,
        &[CallSiteSpecV1 {
            caller_owner: "TextOwner",
            caller_name: "caller",
            caller_arity: 0,
            site: site(vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::Value,
            ]),
        }],
    );
    let results = seal_with_targets(&declarations, &targets);
    let caller = key(&declarations, "TextOwner", "caller", 0);
    let target = key(&declarations, "TextOwner", "text", 0);
    let call_site = site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
    ]);
    let mut owner =
        VerifiedStaticCallResultPublicationOwnerV1::issue(&declarations, &targets, &results)
            .expect("non-i64 row must remain a valid unselected source target");

    assert_eq!(
        owner
            .take(&declarations, &caller, &call_site, &target)
            .expect("unselected lookup remains well-formed"),
        StaticCallResultPublicationTakeV1::Unselected
    );
}
