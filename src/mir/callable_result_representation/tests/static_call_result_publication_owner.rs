use crate::mir::resolved_semantics::SourcePathSegmentV1;

use super::super::{
    CallableResultUnavailableReasonV1, StaticCallResultPublicationOwnerFinishErrorV1,
    StaticCallResultPublicationOwnerTakeErrorV1, StaticCallResultPublicationTakeV1,
    VerifiedCallableResultEvidenceV1, VerifiedCallableResultRepresentationV1,
    VerifiedStaticCallResultPublicationOwnerV1,
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
    assert_eq!(owner.pending_len(), 1);
    assert_eq!(owner.pending_selected_len(), 1);
    assert_eq!(owner.pending_target_only_len(), 0);

    let caller = key(&declarations, "StringHelpers", "int_to_str", 1);
    let target = key(&declarations, "StringHelpers", "to_i64", 1);
    let decision = owner
        .take_for_source(&declarations, &caller, &call_site())
        .expect("branded take must verify");
    let StaticCallResultPublicationTakeV1::Selected(handoff) = decision else {
        panic!("exact row must be selected")
    };
    assert_eq!(handoff.caller(), &caller);
    assert_eq!(handoff.target(), &target);
    assert_eq!(
        owner.take_for_source(&declarations, &caller, &call_site()),
        Err(
            StaticCallResultPublicationOwnerTakeErrorV1::RowAlreadyConsumed {
                caller,
                site: call_site(),
                target,
            }
        )
    );
    assert!(owner.finish_empty().is_ok());
}

#[test]
fn general_owner_handoff_preserves_callee_formal_for_literal_call_site() {
    let source = r#"
        static box ProviderV1 {
            second(left, right) { return right }
        }
        static box ConsumerV1 {
            fixed() { return ProviderV1.second("ignored", 41) }
        }
    "#;
    let declarations = declarations(source);
    let targets = qualified_targets(
        &declarations,
        &[],
        &[CallSiteSpecV1 {
            caller_owner: "ConsumerV1",
            caller_name: "fixed",
            caller_arity: 0,
            site: return_call_site(),
        }],
    );
    let results = seal_with_targets(&declarations, &targets);
    let caller = key(&declarations, "ConsumerV1", "fixed", 0);
    let row = results
        .call_result(&caller, &return_call_site())
        .expect("literal static call must retain its source row");
    assert_eq!(row.required_i64_arguments(), &[] as &[u32]);
    match row.evidence() {
        VerifiedCallableResultEvidenceV1::SameModuleStatic {
            callee_required_i64_arguments,
            ..
        } => assert_eq!(callee_required_i64_arguments.as_ref(), &[1]),
        VerifiedCallableResultEvidenceV1::CoreStringMethod { .. } => {
            panic!("expected same-module static evidence")
        }
    }

    let mut owner =
        VerifiedStaticCallResultPublicationOwnerV1::issue(&declarations, &targets, &results)
            .expect("publication owner must issue the general row");
    let handoff = owner
        .selected_handoff_for_source(&caller, &return_call_site())
        .expect("selected row must expose a handoff");
    assert_eq!(handoff.caller(), &caller);
    assert_eq!(
        handoff.target(),
        &key(&declarations, "ProviderV1", "second", 2)
    );
    assert_eq!(handoff.required_callee_i64_arguments(), &[1]);
    assert!(matches!(
        owner
            .take_for_source(&declarations, &caller, &return_call_site())
            .expect("selected row must be consumable"),
        StaticCallResultPublicationTakeV1::Selected(_)
    ));
    assert!(owner.finish_empty().is_ok());
}

#[test]
fn selected_handoff_peek_reads_requirement_without_consuming() {
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
    let caller = key(&declarations, "StringHelpers", "int_to_str", 1);
    let target = key(&declarations, "StringHelpers", "to_i64", 1);

    let handoff = owner
        .selected_handoff_for_source(&caller, &call_site())
        .expect("selected row must be peekable");
    assert_eq!(handoff.caller(), &caller);
    assert_eq!(handoff.site(), &call_site());
    assert_eq!(handoff.target(), &target);
    assert_eq!(
        handoff.representation(),
        &VerifiedCallableResultRepresentationV1::ExactI64
    );
    assert_eq!(handoff.required_callee_i64_arguments(), &[0]);

    assert!(matches!(
        owner
            .take_for_source(&declarations, &caller, &call_site())
            .expect("peek must not consume the selected row"),
        StaticCallResultPublicationTakeV1::Selected(_)
    ));
    assert!(owner.finish_empty().is_ok());
}

#[test]
fn selected_handoff_peek_stays_empty_for_target_only_and_foreign_sites() {
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
            site: return_call_site(),
        }],
    );
    let results = seal_with_targets(&declarations, &targets);
    let mut owner =
        VerifiedStaticCallResultPublicationOwnerV1::issue(&declarations, &targets, &results)
            .expect("target-only row must issue");
    let caller = key(&declarations, "TextOwner", "caller", 0);

    assert!(
        owner
            .selected_handoff_for_source(&caller, &return_call_site())
            .is_none(),
        "target-only rows have no selected handoff to peek"
    );
    assert!(
        owner
            .selected_handoff_for_source(&caller, &digit_call_site())
            .is_none(),
        "foreign sites have no selected handoff to peek"
    );

    let target = key(&declarations, "TextOwner", "text", 0);
    let StaticCallResultPublicationTakeV1::TargetOnly(target_only) = owner
        .take_for_source(&declarations, &caller, &return_call_site())
        .expect("target-only lookup remains well-formed")
    else {
        panic!("target-only row must remain target-only")
    };
    assert_eq!(target_only.target(), &target);
    assert_eq!(
        target_only.reason(),
        &CallableResultUnavailableReasonV1::KnownNonI64Return
    );
    assert!(owner.finish_empty().is_ok());
}

#[test]
fn source_keyed_take_rejects_wrong_site_and_foreign_catalog() {
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
    let caller = key(&declarations, "StringHelpers", "int_to_str", 1);

    assert_eq!(
        owner
            .take_for_source(&declarations, &caller, &digit_call_site())
            .expect("wrong-site lookup remains well-formed"),
        StaticCallResultPublicationTakeV1::NoExactStaticTarget
    );

    let foreign_declarations = super::support::declarations(SOURCE);
    assert_eq!(
        owner.take_for_source(&foreign_declarations, &caller, &call_site()),
        Err(StaticCallResultPublicationOwnerTakeErrorV1::CatalogBrandMismatch)
    );

    assert!(matches!(
        owner
            .take_for_source(&declarations, &caller, &call_site())
            .expect("original branded row must remain available"),
        StaticCallResultPublicationTakeV1::Selected(_)
    ));
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
        .take_for_source(&declarations, &caller, &call_site())
        .expect("general row take must verify");
    let StaticCallResultPublicationTakeV1::Selected(handoff) = decision else {
        panic!("general exact row must be selected")
    };
    assert_eq!(handoff.caller(), &caller);
    assert_eq!(handoff.target(), &target);
    assert!(handoff.required_callee_i64_arguments().is_empty());
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
    let mut owner =
        VerifiedStaticCallResultPublicationOwnerV1::issue(&declarations, &targets, &results)
            .expect("exact Box row must issue through the existing owner");

    let StaticCallResultPublicationTakeV1::Selected(handoff) = owner
        .take_for_source(&declarations, &caller, &return_call_site())
        .expect("exact Box row must select")
    else {
        panic!("exact Box row must not become unselected")
    };
    let (demand, required_callee_i64_arguments) = handoff.consume();
    assert_eq!(
        demand.representation(),
        &VerifiedCallableResultRepresentationV1::ExactNominalBox {
            box_name: "ProductV1".to_owned(),
        }
    );
    assert!(required_callee_i64_arguments.is_empty());
}

#[test]
fn exact_source_target_without_an_i64_result_stays_target_only() {
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
    let call_site = site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
    ]);
    let mut owner =
        VerifiedStaticCallResultPublicationOwnerV1::issue(&declarations, &targets, &results)
            .expect("non-i64 row must remain a valid unselected source target");
    let target = key(&declarations, "TextOwner", "text", 0);

    assert_eq!(owner.pending_len(), 1);
    assert_eq!(owner.pending_selected_len(), 0);
    assert_eq!(owner.pending_target_only_len(), 1);
    assert_eq!(
        owner.finish_empty(),
        Err(
            StaticCallResultPublicationOwnerFinishErrorV1::UnconsumedTargetOnly {
                caller: caller.clone(),
                site: call_site.clone(),
                target: target.clone(),
                reason: CallableResultUnavailableReasonV1::KnownNonI64Return,
            }
        )
    );

    let StaticCallResultPublicationTakeV1::TargetOnly(target_only) = owner
        .take_for_source(&declarations, &caller, &call_site)
        .expect("target-only lookup remains well-formed")
    else {
        panic!("target-only row must remain target-only")
    };
    assert_eq!(target_only.target(), &target);
    assert_eq!(
        target_only.reason(),
        &CallableResultUnavailableReasonV1::KnownNonI64Return
    );
    assert!(matches!(
        owner.take_for_source(&declarations, &caller, &call_site),
        Err(StaticCallResultPublicationOwnerTakeErrorV1::RowAlreadyConsumed { .. })
    ));
    assert!(owner.finish_empty().is_ok());
}

#[test]
fn issuer_finish_empty_rejects_unconsumed_selected_and_accepts_after_take() {
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
            .expect("selected row must issue");
    let caller = key(&declarations, "StringHelpers", "int_to_str", 1);
    let target = key(&declarations, "StringHelpers", "to_i64", 1);

    assert_eq!(
        owner.finish_empty(),
        Err(
            StaticCallResultPublicationOwnerFinishErrorV1::UnconsumedSelected {
                caller: caller.clone(),
                site: call_site(),
                target,
            }
        )
    );
    assert!(matches!(
        owner
            .take_for_source(&declarations, &caller, &call_site())
            .expect("selected row must be consumed"),
        StaticCallResultPublicationTakeV1::Selected(_)
    ));
    assert!(owner.finish_empty().is_ok());
}

#[test]
fn issuer_finish_empty_rejects_mixed_selected_and_target_only_rows() {
    let source = r#"
        static box StringHelpers {
            int_to_str(n) { local value = me.to_i64("x") return value }
            to_i64(x) { return x + 1 }
        }
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
        &[
            CallSiteSpecV1 {
                caller_owner: "StringHelpers",
                caller_name: "int_to_str",
                caller_arity: 1,
                site: call_site(),
            },
            CallSiteSpecV1 {
                caller_owner: "TextOwner",
                caller_name: "caller",
                caller_arity: 0,
                site: return_call_site(),
            },
        ],
    );
    let results = seal_with_targets(&declarations, &targets);
    let mut owner =
        VerifiedStaticCallResultPublicationOwnerV1::issue(&declarations, &targets, &results)
            .expect("mixed selected and target-only rows must issue");
    let selected_caller = key(&declarations, "StringHelpers", "int_to_str", 1);
    let target_only_caller = key(&declarations, "TextOwner", "caller", 0);

    assert_eq!(owner.pending_selected_len(), 1);
    assert_eq!(owner.pending_target_only_len(), 1);
    assert!(matches!(
        owner.finish_empty(),
        Err(StaticCallResultPublicationOwnerFinishErrorV1::UnconsumedSelected { .. })
            | Err(StaticCallResultPublicationOwnerFinishErrorV1::UnconsumedTargetOnly { .. })
    ));
    assert!(matches!(
        owner
            .take_for_source(&declarations, &selected_caller, &call_site())
            .expect("selected mixed row must be consumed"),
        StaticCallResultPublicationTakeV1::Selected(_)
    ));
    assert!(matches!(
        owner.finish_empty(),
        Err(StaticCallResultPublicationOwnerFinishErrorV1::UnconsumedTargetOnly { .. })
    ));
    let target_only_target = key(&declarations, "TextOwner", "text", 0);
    let StaticCallResultPublicationTakeV1::TargetOnly(target_only) = owner
        .take_for_source(&declarations, &target_only_caller, &return_call_site())
        .expect("target-only mixed row must be consumed")
    else {
        panic!("target-only row must remain target-only")
    };
    assert_eq!(target_only.target(), &target_only_target);
    assert_eq!(
        target_only.reason(),
        &CallableResultUnavailableReasonV1::KnownNonI64Return
    );
    assert_eq!(owner.pending_len(), 0);
    assert!(owner.finish_empty().is_ok());
}
