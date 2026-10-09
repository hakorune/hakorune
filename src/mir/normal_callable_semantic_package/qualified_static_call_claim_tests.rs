//! Focused boundary tests for `STATIC-CALL-CLAIM-S0` — the
//! package-injected qualified static-call claim index.
//!
//! Positive membership requires the sealed chain: `QualifiedUnbound`
//! receiver → alias view resolution → `StaticBoxMethod` target →
//! `ExactI64` result disposition, plus argument sealing to Integer/Bool
//! literals or trivial-scalar bindings (callee-required i64 ordinals
//! must carry i64-class evidence).  Every rejected row keeps the
//! ordinary `PrefixNotCovered` unavailability — the site is never
//! silently claimed.

use super::brand_catalog_tests::issue_with_brand_catalog as issue;
use crate::mir::builder::NormalRootExecutionConsumerV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    LocalCallArgumentV1, LocalCallObservationV1, LocalCallResultClassV1,
};
use crate::mir::resolved_semantics::{FunctionSemanticResolverSessionV1, SourceBindingSiteV1};
use crate::parser::{NyashParser, ParserBuildConfig};

/// Issue the semantic package with injected `using` alias rows — the
/// same channel `using_import_boxes` feeds in the production lane.
fn issue_with_import_rows(
    source: &str,
    imports: &[(String, String)],
) -> Result<
    super::VerifiedNormalCallableSemanticPackageV1,
    super::NormalCallableSemanticPackageIssueV1,
> {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("normal callable source");
    let transformed = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        crate::r#macro::transform_normal_callable_program_v1(parsed)
            .expect("exact callable transform")
    });
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed else {
        panic!("fixture must remain source-backed")
    };
    let catalog =
        crate::analysis::brand_program_declaration_catalog::issue_brand_program_declaration_catalog_v1(
            source.ast(),
        )
        .expect("brand catalog");
    let source = NormalRootExecutionConsumerV1::consume_once(source)
        .expect("root execution")
        .into_consumed_source();
    let mut resolver = FunctionSemanticResolverSessionV1::new(93).unwrap();
    super::issue_normal_callable_semantic_package_with_brand_catalog_and_loop_policy_v1(
        &mut resolver,
        source,
        Some(&catalog),
        crate::mir::builder::LoopFactsPolicyFrameV1::from_environment(),
        imports,
    )
}

/// All local-call claim rows across every declaration owner — App
/// Main's completion lives on the root slot, selected callables' in the
/// per-owner completion index.
fn local_calls(
    package: &super::VerifiedNormalCallableSemanticPackageV1,
) -> Vec<LocalCallObservationV1> {
    let mut calls = Vec::new();
    for declaration in package.batch().declarations() {
        if let Some(flow) = package
            .ordinary_new_claim_ledger
            .completion_for_owner(declaration.owner())
            .and_then(|completion| completion.cleanup().root_flow())
        {
            calls.extend(flow.local_calls().iter().cloned());
        }
    }
    calls
}

/// The `new Page()` claim's prefix record — `Err(PrefixNotCovered(_))`
/// is the named unavailability an unclaimed call leaves behind.
fn new_claim_prefix_covered(package: &super::VerifiedNormalCallableSemanticPackageV1) -> bool {
    package
        .ordinary_new_claim_ledger
        .pending_claims_for_test()
        .values()
        .all(|claim| claim.home_prefix().is_ok())
}

#[test]
fn qualified_static_call_claims_i64_literal_argument() {
    let package = issue(
        "box Page { birth() { } }
        static box LayoutBox { class_id(size) { return size } }
        static box Main { main() {
            local class_id = LayoutBox.class_id(7)
            local page = new Page()
            return class_id
        } }",
    )
    .expect("qualified static call package");
    let calls = local_calls(&package);
    let [call] = calls.as_slice() else {
        panic!("one qualified static-call claim, got {calls:?}")
    };
    assert_eq!(call.result(), LocalCallResultClassV1::I64);
    assert_eq!(call.arguments(), &[LocalCallArgumentV1::Integer(7)]);
    assert!(new_claim_prefix_covered(&package));
}

#[test]
fn qualified_static_call_claims_scalar_parameter_argument() {
    // The mimalloc-lite shape: `local class_id = LayoutBox.class_id(size)`
    // inside a non-birth caller — the argument is a parameter binding,
    // not a literal.
    let package = issue(
        "box Page { birth() { } }
        static box LayoutBox { class_id(size) { return size } }
        static box Helpers {
            lookup(size: i64): i64 {
                local class_id = LayoutBox.class_id(size)
                local page = new Page()
                return class_id
            }
        }
        static box Main { main() { return Helpers.lookup(9) } }",
    )
    .expect("parameter-argument package");
    let calls = local_calls(&package);
    let [call] = calls.as_slice() else {
        panic!("one qualified static-call claim, got {calls:?}")
    };
    assert_eq!(call.result(), LocalCallResultClassV1::I64);
    let [LocalCallArgumentV1::Scalar(binding)] = call.arguments() else {
        panic!(
            "parameter argument must seal as Scalar, got {:?}",
            call.arguments()
        )
    };
    let declaration = package
        .batch()
        .declarations()
        .find(|row| row.owner() == call.owner())
        .expect("exact owner");
    let parameter = package
        .batch()
        .with_lowering_input(declaration.batch_slot(), |input| {
            input
                .function()
                .declaration_binding(&SourceBindingSiteV1::Parameter { index: 0 })
        })
        .expect("lowering input");
    assert_eq!(Some(*binding), parameter);
    assert!(new_claim_prefix_covered(&package));
}

#[test]
fn qualified_static_call_claim_resolves_import_alias() {
    let package = issue_with_import_rows(
        "box Page { birth() { } }
        static box LayoutBox { class_id(size) { return size } }
        static box Main { main() {
            local class_id = LB.class_id(7)
            local page = new Page()
            return class_id
        } }",
        &[("LB".to_string(), "LayoutBox".to_string())],
    )
    .expect("alias-resolved package");
    let calls = local_calls(&package);
    let [call] = calls.as_slice() else {
        panic!("one alias-resolved claim, got {calls:?}")
    };
    assert_eq!(call.result(), LocalCallResultClassV1::I64);
    assert_eq!(call.arguments(), &[LocalCallArgumentV1::Integer(7)]);
    assert!(new_claim_prefix_covered(&package));
}

#[test]
fn qualified_static_call_claim_stays_fail_closed() {
    for (label, call) in [
        // No `StaticBoxMethod` declaration exists for the receiver.
        ("unresolved-alias", "MissingBox.class_id(7)"),
        // `Page` is an instance box — a `StaticBoxMethod` for owner
        // `Page` does not exist.
        ("non-static-target", "Page.class_id(7)"),
        // `Any.make` has an `ExactNominalBox` disposition, not `ExactI64`.
        ("unproven-result", "Any.make()"),
        // A compound argument is outside the scalar seal.
        ("non-trivial-arg", "LayoutBox.class_id(nine + 1)"),
        // A Home argument is not scalar evidence.
        ("handle-arg", "LayoutBox.class_id(page)"),
    ] {
        let source = format!(
            "box Page {{ birth() {{ }} }}
            static box LayoutBox {{ class_id(size) {{ return size }} }}
            static box Any {{ make() {{ return new Page() }} }}
            static box Main {{ main() {{
                local nine = 9
                local page = new Page()
                local x = {call}
                local page2 = new Page()
                return 0
            }} }}"
        );
        let package = issue(&source).unwrap_or_else(|issue| panic!("{label} package: {issue:?}"));
        assert!(
            local_calls(&package).is_empty(),
            "{label}: unclaimed call issues no local-call row"
        );
        assert!(
            !new_claim_prefix_covered(&package),
            "{label}: a `new` past the unclaimed call keeps PrefixNotCovered"
        );
    }
}

#[test]
fn qualified_static_call_claim_rejects_non_i64_evidence_at_required_ordinal() {
    // `class_id` returns its `size` parameter, so ordinal 0 is a required
    // i64 argument — a Bool literal there must refuse the claim.
    let package = issue(
        "box Page { birth() { } }
        static box LayoutBox { class_id(size) { return size } }
        static box Main { main() {
            local class_id = LayoutBox.class_id(true)
            local page = new Page()
            return 0
        } }",
    )
    .expect("bool-argument package");
    assert!(local_calls(&package).is_empty());
    assert!(!new_claim_prefix_covered(&package));
}

#[test]
fn static_i64_call_claim_preserves_current_owner_boundary() {
    for (label, callee, expression, accepted) in [
        ("zeroarg-i64", "word() { return 7 }", "me.word()", true),
        (
            "nonzeroarg-i64",
            "word(size: i64): i64 { return size }",
            "me.word(7)",
            true,
        ),
        ("zeroarg-bool", "word() { return true }", "me.word()", false),
        (
            "zeroarg-text",
            "word() { return \"text\" }",
            "me.word()",
            false,
        ),
    ] {
        let package = issue(&format!(
            "box Page {{ birth() {{ }} }}
            static box LayoutBox {{
                {callee}
                wrap(): i64 {{
                    local inner = {expression}
                    local page = new Page()
                    return 0
                }}
            }}
            static box Main {{ main() {{ return LayoutBox.wrap() }} }}"
        ))
        .unwrap_or_else(|issue| panic!("{label}: {issue:?}"));
        let calls = local_calls(&package);
        assert_eq!(
            calls.len(),
            usize::from(accepted),
            "{label}: original route membership"
        );
        if accepted {
            assert_eq!(calls[0].result(), LocalCallResultClassV1::I64);
            if label == "nonzeroarg-i64" {
                assert!(matches!(
                    calls[0].arguments(),
                    [LocalCallArgumentV1::Integer(7)]
                ));
            } else {
                assert!(calls[0].arguments().is_empty());
            }
            assert!(calls[0].local_binding().is_some());
        }
        assert_eq!(
            new_claim_prefix_covered(&package),
            accepted,
            "{label}: exact prefix coverage"
        );
    }
    // Reuse the same source owner for each value context, including a live
    // Home at the call. No artificial local/discard destination is issued.
    for (body, count, binding, homes) in [
        ("local inner = me.word() return inner", 1, true, 0),
        ("local inner = 1 + me.word() return inner", 1, false, 0),
        (
            "local inner = me.word() + me.word() return inner",
            2,
            false,
            0,
        ),
        ("if me.word() > 0 { return 1 } return 0", 1, false, 0),
        ("return me.word()", 1, false, 0),
        ("return me.word() + 1", 1, false, 0),
        (
            "local page = new Page() local inner = me.word() + 1 return inner",
            1,
            false,
            1,
        ),
        (
            "local marker = me.word() if me.flag() { return 1 } return 0",
            1,
            true,
            0,
        ),
    ] {
        let package = issue(&format!(
            "box Page {{ birth() {{ }} }} static box LayoutBox {{
            word() {{ return 7 }} flag() {{ return true }} wrap(): i64 {{ {body} }}
            }} static box Main {{ main() {{ return LayoutBox.wrap() }} }}"
        ))
        .unwrap_or_else(|issue| panic!("{body}: {issue:?}"));
        let calls = local_calls(&package);
        assert_eq!(calls.len(), count, "original value sites: {body}");
        for call in &calls {
            assert_eq!(
                call.local_binding().is_some(),
                binding,
                "real destination: {body}"
            );
            assert_eq!(
                call.prior_homes().len(),
                homes,
                "original Home snapshot: {body}"
            );
            assert!(call.arguments().is_empty());
        }
        let wrap = package.batch().declarations().find(|declaration|
            package.selected.key_for_batch_slot(declaration.batch_slot())
                .is_some_and(|key| matches!(key, crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key) if key.name() == "wrap"))
        ).expect("original wrap declaration");
        let completion = package
            .ordinary_new_claim_ledger
            .completion_for_owner(wrap.owner())
            .expect("original completed owner");
        let flow = completion
            .cleanup()
            .root_flow()
            .expect("original homes-aware flow");
        for exit in completion.explicit_sites() {
            assert!(
                flow.exit_row(exit).is_some_and(|row| row.is_ok()),
                "whole exit coverage: {body}"
            );
        }
        assert!(new_claim_prefix_covered(&package), "covered prefix: {body}");
    }
    for body in [
        "local inner = me.word() + true local later = new Page() return 0",
        "local inner = me.word() + me.flag() local later = new Page() return 0",
        "if (me.word() > 0) && true { return 1 } local later = new Page() return 0",
        "local inner = LayoutBox.echo(me.word()) local later = new Page() return 0",
    ] {
        let package = issue(&format!(
            "box Page {{ birth() {{ }} }} static box LayoutBox {{ word() {{ return 7 }}
            flag() {{ return true }} echo(x: i64): i64 {{ return x }} wrap(): i64 {{ {body} }}
            }} static box Main {{ main() {{ return LayoutBox.wrap() }} }}"
        ))
        .unwrap_or_else(|issue| panic!("{body}: {issue:?}"));
        assert!(
            local_calls(&package).is_empty(),
            "no partial observation: {body}"
        );
        assert!(
            !new_claim_prefix_covered(&package),
            "unavailable responsibility retained: {body}"
        );
    }
}

#[test]
fn current_owner_source_retains_original_route_and_result_without_qualified_claim() {
    use crate::mir::builder::{CanonicalSameModuleCallableKeyV1, SelectedNormalCallableKeyV1};
    use crate::mir::callable_result_representation::VerifiedCallableResultDispositionV1;
    use crate::mir::resolved_semantics::SourcePathV1;
    use crate::mir::source_call_target::CurrentOwnerStaticReceiverV1;
    let package = issue(
        "static box Layout {
        need(p) { return p }
        text(p) { return \"text\" }
        relay(p: i64) { local a = me.need(p) local b = me.text(p) return 0 }
    } static box Main { main() { return 0 } }",
    )
    .unwrap();
    let caller = CanonicalSameModuleCallableKeyV1::static_box_method("Layout", "relay", 1);
    let slot = package
        .selected
        .batch_slot(&SelectedNormalCallableKeyV1::Cataloged(caller.clone()))
        .unwrap();
    let claims = &package.source_static_claims_for_test;
    let mut checked = 0;
    package.batch().with_lowering_input(slot, |input| {
        for (site, call) in input.function().method_calls() {
            let row = claims.current_owner_source(&caller, site).expect("original current-owner source");
            assert_eq!(row.route().receiver(), CurrentOwnerStaticReceiverV1::CanonicalMe);
            assert_eq!(row.route().target(), &CanonicalSameModuleCallableKeyV1::static_box_method("Layout", call.selector(), 1));
            assert!(claims.claim_target(&caller, site).is_none());
            let owned = crate::mir::resolved_semantics::OwnedExprSiteV1::new(input.owner(), site.clone());
            let loan = claims.incoming_source(
                &caller, &owned, call, &package.selected, &package.parameter_contracts, None,
            ).unwrap().expect("explicit original CurrentOwner incoming loan");
            let retained = loan.retain();
            assert!(loan.corroborates_retained(&retained));
            assert_eq!(retained.current_owner_source(), Some(row));
            assert_eq!(retained.argument_sites(), &[call.arguments()[0].site().clone()]);
            assert_eq!(retained.parameters().len(), 1);
            assert!(retained.require_qualified().is_err());
            match call.selector() {
                "need" => assert!(matches!(row.result(), VerifiedCallableResultDispositionV1::ExactI64 { required_i64_arguments } if required_i64_arguments.as_ref() == [0])),
                "text" => assert_eq!(row.result(), &VerifiedCallableResultDispositionV1::ExactString),
                other => panic!("unexpected original call {other}"),
            }
            let foreign = CanonicalSameModuleCallableKeyV1::static_box_method("Foreign", "relay", 1);
            assert!(claims.current_owner_source(&foreign, site).is_none());
            assert!(claims.current_owner_source(&caller, &SourcePathV1::root_body(99).expr()).is_none());
            checked += 1;
        }
    }).unwrap();
    assert_eq!(checked, 2);
}

#[test]
fn current_owner_required_integer_without_actual_proof_has_no_call_observation() {
    let package = issue(
        "static box Layout {
            need(p) { return p }
            relay(p) { local a = me.need(p) return 0 }
        } static box Main { main() { return 0 } }",
    )
    .expect("unproved source remains passive");
    assert!(local_calls(&package).is_empty());
}

#[test]
fn current_owner_known_bool_actual_rejects_i64_formal() {
    let Err(issue) = issue(
        "static box Layout {
            need(p: i64): i64 { return p }
            relay() { local a = me.need(true) return 0 }
        } static box Main { main() { return 0 } }",
    ) else {
        panic!("known Bool cannot satisfy the I64 source contract");
    };
    assert!(format!("{issue:?}").contains("nonopaque-integer-unproved"));
}
