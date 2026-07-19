use crate::mir::resolved_semantics::SourcePathSegmentV1;

use super::super::{
    classify_activation_source_site_v1, CallableResultActivationDispositionV1,
    CallableResultActivationSourceDecisionV1, CallableResultActivationUnselectedReasonV1,
    VerifiedCallableResultActivationPlanV1, VerifiedCallableResultActivationRowsV1,
};
use super::actual_parser_add_fixture;
use super::support::{
    declarations, instance_key, key, qualified_targets, seal_with_targets, site, CallSiteSpecV1,
};

const SOURCE: &str = r#"
    box ParserBox {
        static_const_parse_add(text, pos) {
            local next = ParserStringUtilsBox.skip_ws(text, pos)
            local width = text.length()
            return next
        }
    }
    static box ParserStringUtilsBox {
        skip_ws(text, pos) { return pos }
    }
"#;

const NESTED_INSTANCE_ARGUMENT_SOURCE: &str = r#"
    box ParserBox {
        static_const_parse_add(text, pos) {
            local next = ParserStringUtilsBox.skip_ws(text, me.dynamic_pos(text))
            return next
        }

        dynamic_pos(text) { return 0 }
    }
    static box ParserStringUtilsBox {
        skip_ws(text, pos) { return pos }
    }
"#;

fn selected_site() -> crate::mir::resolved_semantics::SourceExprSiteV1 {
    site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Initializer(0),
    ])
}

fn unselected_site() -> crate::mir::resolved_semantics::SourceExprSiteV1 {
    site(vec![
        SourcePathSegmentV1::Body(1),
        SourcePathSegmentV1::Initializer(0),
    ])
}

fn value_site() -> crate::mir::resolved_semantics::SourceExprSiteV1 {
    site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
    ])
}

fn seal_plan(source: &str) -> VerifiedCallableResultActivationPlanV1 {
    let declarations = Box::new(declarations(source));
    let targets = qualified_targets(
        declarations.as_ref(),
        &[],
        &[CallSiteSpecV1 {
            caller_owner: "ParserBox",
            caller_name: "static_const_parse_add",
            caller_arity: 2,
            site: selected_site(),
        }],
    );
    let results = seal_with_targets(declarations.as_ref(), &targets);
    let rows =
        VerifiedCallableResultActivationRowsV1::verify(declarations.as_ref(), &targets, &results)
            .expect("owned activation rows");
    drop(results);
    drop(targets);
    VerifiedCallableResultActivationPlanV1::seal(declarations, rows).expect("owned activation plan")
}

#[test]
fn instance_caller_with_unproven_required_formal_keeps_all_rows_unselected() {
    let plan = seal_plan(SOURCE);
    let caller = instance_key(
        plan.declaration_catalog(),
        "ParserBox",
        "static_const_parse_add",
        2,
    );
    let rows = plan.rows_for(&caller).expect("instance caller rows");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].site(), &selected_site());
    assert_eq!(
        rows[0].disposition(),
        &CallableResultActivationDispositionV1::Unselected
    );
    assert_eq!(rows[1].site(), &unselected_site());
    assert_eq!(
        rows[1].disposition(),
        &CallableResultActivationDispositionV1::Unselected
    );
}

#[test]
fn declaration_reorder_preserves_owned_activation_rows() {
    let forward = seal_plan(SOURCE);
    let reversed = seal_plan(
        r#"
        static box ParserStringUtilsBox {
            skip_ws(text, pos) { return pos }
        }
        box ParserBox {
            static_const_parse_add(text, pos) {
                local next = ParserStringUtilsBox.skip_ws(text, pos)
                local width = text.length()
                return next
            }
        }
        "#,
    );
    let forward_caller = instance_key(
        forward.declaration_catalog(),
        "ParserBox",
        "static_const_parse_add",
        2,
    );
    let reversed_caller = instance_key(
        reversed.declaration_catalog(),
        "ParserBox",
        "static_const_parse_add",
        2,
    );
    assert_eq!(
        forward.rows_for(&forward_caller),
        reversed.rows_for(&reversed_caller)
    );
}

#[test]
fn activation_rows_cannot_pair_with_an_equal_foreign_catalog() {
    let primary = Box::new(declarations(SOURCE));
    let targets = qualified_targets(
        primary.as_ref(),
        &[],
        &[CallSiteSpecV1 {
            caller_owner: "ParserBox",
            caller_name: "static_const_parse_add",
            caller_arity: 2,
            site: selected_site(),
        }],
    );
    let results = seal_with_targets(primary.as_ref(), &targets);
    let rows = VerifiedCallableResultActivationRowsV1::verify(primary.as_ref(), &targets, &results)
        .expect("primary rows");
    drop(results);
    drop(targets);

    let foreign = Box::new(declarations(SOURCE));
    assert!(VerifiedCallableResultActivationPlanV1::seal(foreign, rows).is_err());
}

#[test]
fn plan_is_owned_and_single_use() {
    let plan = seal_plan(SOURCE);
    let (catalog, rows) = plan.into_parts();
    assert_eq!(catalog.len(), 2);
    assert_eq!(rows.len(), 2);
}

#[test]
fn source_gate_selects_only_when_the_exact_call_result_row_exists() {
    super::generic_selected_activation_fixture::with_source_gate_inputs(
        |declarations, caller, call_site, targets, results| {
            let CallableResultActivationSourceDecisionV1::Selected(selected) =
                classify_activation_source_site_v1(
                    declarations,
                    caller,
                    call_site,
                    targets,
                    results,
                )
                .expect("source gate")
            else {
                panic!("literal required argument must have source proof");
            };
            assert_eq!(selected.target().owner(), "Provider");
            assert_eq!(selected.target().name(), "step");
            assert_eq!(selected.required_i64_arguments(), &[0]);
        },
    );
}

#[test]
fn activation_rows_preserve_the_generic_literal_selected_disposition() {
    let plan = super::generic_selected_activation_fixture::plan();
    let caller = super::generic_selected_activation_fixture::caller(&plan);
    let rows = plan.rows_for(&caller).expect("literal caller rows");

    assert_eq!(rows.len(), 1);
    assert_eq!(
        rows[0].site(),
        &super::generic_selected_activation_fixture::call_site()
    );
    assert_eq!(
        rows[0].disposition(),
        &CallableResultActivationDispositionV1::SelectedExactI64 {
            target: key(plan.declaration_catalog(), "Provider", "step", 1),
            required_i64_arguments: Box::new([0]),
        }
    );
}

#[test]
fn source_gate_selects_direct_formal_required_argument() {
    let source = r#"
        static box ProviderV1 {
            second(left, right) { return right }
        }
        static box ConsumerV1 {
            forward(value) { return ProviderV1.second("ignored", value) }
        }
    "#;
    let declarations = declarations(source);
    let call_site = value_site();
    let targets = qualified_targets(
        &declarations,
        &[],
        &[CallSiteSpecV1 {
            caller_owner: "ConsumerV1",
            caller_name: "forward",
            caller_arity: 1,
            site: call_site.clone(),
        }],
    );
    let results = seal_with_targets(&declarations, &targets);
    let caller = key(&declarations, "ConsumerV1", "forward", 1);

    let CallableResultActivationSourceDecisionV1::Selected(selected) =
        classify_activation_source_site_v1(&declarations, &caller, &call_site, &targets, &results)
            .expect("direct formal source gate")
    else {
        panic!("direct required formal must have source proof");
    };
    assert_eq!(selected.target().owner(), "ProviderV1");
    assert_eq!(selected.target().name(), "second");
    assert_eq!(selected.required_i64_arguments(), &[1]);
}

#[test]
fn source_gate_keeps_nested_instance_required_argument_unselected() {
    let declarations = declarations(NESTED_INSTANCE_ARGUMENT_SOURCE);
    let call_site = selected_site();
    let targets = qualified_targets(
        &declarations,
        &[],
        &[CallSiteSpecV1 {
            caller_owner: "ParserBox",
            caller_name: "static_const_parse_add",
            caller_arity: 2,
            site: call_site.clone(),
        }],
    );
    let results = seal_with_targets(&declarations, &targets);
    let caller = instance_key(&declarations, "ParserBox", "static_const_parse_add", 2);

    assert!(results.call_result(&caller, &call_site).is_none());
    assert!(matches!(
        classify_activation_source_site_v1(&declarations, &caller, &call_site, &targets, &results,)
            .expect("source proof absence is ordinary unselected"),
        CallableResultActivationSourceDecisionV1::Unselected(
            CallableResultActivationUnselectedReasonV1::RequiredArgumentSourceProofUnavailable,
        )
    ));
}

#[test]
fn source_gate_treats_missing_target_as_unselected() {
    let declarations = declarations(SOURCE);
    let targets = qualified_targets(&declarations, &[], &[]);
    let results = seal_with_targets(&declarations, &targets);
    let caller = instance_key(&declarations, "ParserBox", "static_const_parse_add", 2);

    assert!(matches!(
        classify_activation_source_site_v1(
            &declarations,
            &caller,
            &selected_site(),
            &targets,
            &results,
        )
        .expect("missing target is ordinary unselected"),
        CallableResultActivationSourceDecisionV1::Unselected(
            CallableResultActivationUnselectedReasonV1::NoStaticSourceTarget,
        )
    ));
}

#[test]
fn actual_parser_source_gate_is_all_unselected_without_activation_or_builder_state() {
    actual_parser_add_fixture::with_source_gate_inputs(
        |declarations, caller, sites, targets, results| {
            assert_eq!(sites.len(), 15);
            let decisions = sites
                .iter()
                .map(|site| {
                    classify_activation_source_site_v1(declarations, caller, site, targets, results)
                        .expect("actual source gate decision")
                })
                .collect::<Vec<_>>();

            assert_eq!(
                decisions
                    .iter()
                    .filter(|decision| matches!(
                        decision,
                        CallableResultActivationSourceDecisionV1::Selected(_)
                    ))
                    .count(),
                0
            );
            assert_eq!(
                decisions
                    .iter()
                    .filter(|decision| matches!(
                        decision,
                        CallableResultActivationSourceDecisionV1::Unselected(_)
                    ))
                    .count(),
                15
            );
            for candidate in actual_parser_add_fixture::static_target_candidate_sites() {
                let index = sites
                    .iter()
                    .position(|site| site == &candidate)
                    .expect("target candidate appears in actual source inventory");
                assert!(matches!(
                    decisions[index],
                    CallableResultActivationSourceDecisionV1::Unselected(
                        CallableResultActivationUnselectedReasonV1::RequiredArgumentSourceProofUnavailable,
                    )
                ));
            }
            assert!(matches!(
                decisions[13],
                CallableResultActivationSourceDecisionV1::Unselected(
                    CallableResultActivationUnselectedReasonV1::NoStaticSourceTarget,
                )
            ));
            assert_eq!(
                actual_parser_add_fixture::static_target_candidate_sites().len(),
                2
            );
        },
    );
}

#[test]
fn actual_parser_add_inventory_keeps_every_source_row_unselected() {
    let plan = actual_parser_add_fixture::plan();
    let caller = actual_parser_add_fixture::caller(&plan);
    let rows = plan.rows_for(&caller).expect("actual ParserBox rows");
    assert_eq!(rows.len(), 15);
    assert_eq!(
        rows.iter()
            .filter(|row| matches!(
                row.disposition(),
                CallableResultActivationDispositionV1::SelectedExactI64 { .. }
            ))
            .count(),
        0
    );
    assert_eq!(
        rows.iter()
            .filter(|row| row.disposition() == &CallableResultActivationDispositionV1::Unselected)
            .count(),
        15
    );
}
