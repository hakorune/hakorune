use crate::mir::resolved_semantics::{BodyChildRoleV1, ExprChildRoleV1};

use super::super::{
    CallableResultActivationDispositionV1, CallableResultCallerLedgerErrorV1,
    CallableResultLegacyLocationErrorV1, VerifiedCallableResultActivationPlanV1,
    VerifiedCallableResultActivationRowsV1, VerifiedCallableResultCallerLedgerV1,
    VerifiedCallableResultLegacySourceViewV1,
};
use super::support::{
    declarations, instance_key, qualified_targets, seal_with_targets, site, CallSiteSpecV1,
};
use crate::mir::resolved_semantics::SourcePathSegmentV1;

const SOURCE: &str = r#"
    box ParserBox {
        parse(text, pos) {
            local next = Helpers.step(text, pos)
            local width = text.length()
            return next
        }
    }
    static box Helpers {
        step(text, pos) { return pos }
    }
"#;

fn selected_site() -> crate::mir::resolved_semantics::SourceExprSiteV1 {
    site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Initializer(0),
    ])
}

fn seal_plan() -> VerifiedCallableResultActivationPlanV1 {
    let declarations = Box::new(declarations(SOURCE));
    let targets = qualified_targets(
        declarations.as_ref(),
        &[],
        &[CallSiteSpecV1 {
            caller_owner: "ParserBox",
            caller_name: "parse",
            caller_arity: 2,
            site: selected_site(),
        }],
    );
    let results = seal_with_targets(declarations.as_ref(), &targets);
    let rows =
        VerifiedCallableResultActivationRowsV1::verify(declarations.as_ref(), &targets, &results)
            .expect("caller ledger rows");
    drop(results);
    drop(targets);
    VerifiedCallableResultActivationPlanV1::seal(declarations, rows).expect("caller ledger plan")
}

fn caller(
    plan: &VerifiedCallableResultActivationPlanV1,
) -> crate::mir::builder::CanonicalSameModuleCallableKeyV1 {
    instance_key(plan.declaration_catalog(), "ParserBox", "parse", 2)
}

fn seal_body_domain_plan(
    selected: Vec<Vec<SourcePathSegmentV1>>,
) -> VerifiedCallableResultActivationPlanV1 {
    const BODY_SOURCE: &str = r#"
        box ParserBox {
            parse(text, pos) {
                if pos {
                    local then_value = Helpers.step(1)
                } else {
                    local else_value = Helpers.step(2)
                }
                loop(pos < 1) {
                    local loop_value = Helpers.step(3)
                    break
                }
                return 0
            }
        }
        static box Helpers { step(value) { return value } }
    "#;
    seal_body_domain_plan_for_source(BODY_SOURCE, selected)
}

fn seal_body_domain_plan_for_source(
    source: &str,
    selected: Vec<Vec<SourcePathSegmentV1>>,
) -> VerifiedCallableResultActivationPlanV1 {
    let declarations = Box::new(declarations(source));
    let targets = qualified_targets(
        declarations.as_ref(),
        &[],
        &selected
            .into_iter()
            .map(|segments| CallSiteSpecV1 {
                caller_owner: "ParserBox",
                caller_name: "parse",
                caller_arity: 2,
                site: site(segments),
            })
            .collect::<Vec<_>>(),
    );
    let results = seal_with_targets(declarations.as_ref(), &targets);
    let rows =
        VerifiedCallableResultActivationRowsV1::verify(declarations.as_ref(), &targets, &results)
            .expect("body-domain rows");
    drop(results);
    drop(targets);
    VerifiedCallableResultActivationPlanV1::seal(declarations, rows).expect("body-domain plan")
}

fn calls<'plan>(
    view: &VerifiedCallableResultLegacySourceViewV1<'plan>,
) -> (
    super::super::LegacyBodyInputV1<'plan>,
    super::super::LegacyExprInputV1<'plan>,
    super::super::LegacyExprInputV1<'plan>,
) {
    let body = view.root_body();
    let first = view.body_stmt(&body, 0).unwrap();
    let selected = view
        .child_expr_from_stmt(&first, ExprChildRoleV1::LocalInitializer(0))
        .unwrap();
    let second = view.body_stmt(&body, 1).unwrap();
    let unselected = view
        .child_expr_from_stmt(&second, ExprChildRoleV1::LocalInitializer(0))
        .unwrap();
    (body, selected, unselected)
}

#[test]
fn exact_source_order_claims_selected_and_unselected_rows_once() {
    let plan = seal_plan();
    let caller = caller(&plan);
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    let (_, selected, unselected) = calls(&view);
    let mut ledger = VerifiedCallableResultCallerLedgerV1::verify(&plan, &caller).unwrap();

    let selected_claim = ledger.claim(&selected).unwrap();
    assert_eq!(selected_claim.site(), selected.activation_site().unwrap().1);
    assert!(matches!(
        selected_claim.disposition(),
        CallableResultActivationDispositionV1::SelectedExactI64 { .. }
    ));
    assert_eq!(
        ledger.claim(&unselected).unwrap().disposition(),
        &CallableResultActivationDispositionV1::Unselected
    );
    assert_eq!(
        ledger.claim(&selected).unwrap_err(),
        CallableResultCallerLedgerErrorV1::Duplicate {
            site: selected.activation_site().unwrap().1.clone(),
        }
    );
    ledger.finish().unwrap();
}

#[test]
fn wrong_order_duplicate_unexpected_and_missing_are_distinct() {
    let plan = seal_plan();
    let caller = caller(&plan);
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    let (_, selected, unselected) = calls(&view);

    let mut wrong = VerifiedCallableResultCallerLedgerV1::verify(&plan, &caller).unwrap();
    assert!(matches!(
        wrong.claim(&unselected),
        Err(CallableResultCallerLedgerErrorV1::WrongOrder { .. })
    ));

    let mut duplicate = VerifiedCallableResultCallerLedgerV1::verify(&plan, &caller).unwrap();
    duplicate.claim(&selected).unwrap();
    assert!(matches!(
        duplicate.claim(&selected),
        Err(CallableResultCallerLedgerErrorV1::Duplicate { .. })
    ));

    let receiver = view
        .child_expr(&selected, ExprChildRoleV1::Receiver)
        .unwrap();
    let mut unexpected = VerifiedCallableResultCallerLedgerV1::verify(&plan, &caller).unwrap();
    assert!(matches!(
        unexpected.claim(&receiver),
        Err(CallableResultCallerLedgerErrorV1::ClaimRequiresMethodCall { .. })
    ));

    let missing = VerifiedCallableResultCallerLedgerV1::verify(&plan, &caller).unwrap();
    assert!(matches!(
        missing.finish(),
        Err(CallableResultCallerLedgerErrorV1::Missing { remaining: 2, .. })
    ));
}

#[test]
fn prefix_proof_is_exact_and_unlocated_inputs_never_prove_inactive() {
    let plan = seal_plan();
    let caller = caller(&plan);
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    let (body, selected, _) = calls(&view);
    let ledger = VerifiedCallableResultCallerLedgerV1::verify(&plan, &caller).unwrap();

    assert!(matches!(
        ledger.prove_body_inactive(&body),
        Err(CallableResultCallerLedgerErrorV1::RowsUnderPrefix { .. })
    ));
    let argument = view
        .child_expr(&selected, ExprChildRoleV1::CallArgument(0))
        .unwrap();
    let inactive = ledger.prove_expr_inactive(&argument).unwrap();
    assert_eq!(inactive.caller(), &caller);
    assert_eq!(
        inactive.prefix(),
        Some(argument.activation_site().unwrap().1.node())
    );

    let return_stmt = view.body_stmt(&body, 2).unwrap();
    let inactive_stmt = ledger.prove_stmt_inactive(&return_stmt).unwrap();
    assert_eq!(inactive_stmt.caller(), &caller);

    let unlocated = view.unlocated_expr(argument.node());
    assert_eq!(
        ledger.prove_expr_inactive(&unlocated).unwrap_err(),
        CallableResultCallerLedgerErrorV1::LegacyLocation(
            CallableResultLegacyLocationErrorV1::UnlocatedCannotProveInactive,
        )
    );
}

#[test]
fn typed_body_domains_cover_canonical_items_without_crossing_siblings() {
    let plan = seal_body_domain_plan(vec![
        vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::IfThen(0),
            SourcePathSegmentV1::Initializer(0),
        ],
        vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::IfElse(0),
            SourcePathSegmentV1::Initializer(0),
        ],
        vec![
            SourcePathSegmentV1::Body(1),
            SourcePathSegmentV1::LoopBody(0),
            SourcePathSegmentV1::Initializer(0),
        ],
    ]);
    let caller = caller(&plan);
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    let root = view.root_body();
    let if_statement = view.body_stmt(&root, 0).unwrap();
    let then_body = view
        .child_body_from_stmt(&if_statement, BodyChildRoleV1::IfThen)
        .unwrap();
    let else_body = view
        .child_body_from_stmt(&if_statement, BodyChildRoleV1::IfElse)
        .unwrap();
    let loop_statement = view.body_stmt(&root, 1).unwrap();
    let loop_body = view
        .child_body_from_stmt(&loop_statement, BodyChildRoleV1::LoopBody)
        .unwrap();
    let ledger = VerifiedCallableResultCallerLedgerV1::verify(&plan, &caller).unwrap();

    let then_error = ledger.prove_body_inactive(&then_body).unwrap_err();
    assert!(matches!(
        then_error,
        CallableResultCallerLedgerErrorV1::RowsUnderPrefix { ref prefix, ref first }
            if prefix.as_ref().unwrap().segments() == &[
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::IfThenBody,
            ] && first.node().segments() == &[
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::IfThen(0),
                SourcePathSegmentV1::Initializer(0),
            ]
    ));
    let else_error = ledger.prove_body_inactive(&else_body).unwrap_err();
    assert!(matches!(
        else_error,
        CallableResultCallerLedgerErrorV1::RowsUnderPrefix { ref prefix, ref first }
            if prefix.as_ref().unwrap().segments() == &[
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::IfElseBody,
            ] && first.node().segments() == &[
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::IfElse(0),
                SourcePathSegmentV1::Initializer(0),
            ]
    ));
    let loop_error = ledger.prove_body_inactive(&loop_body).unwrap_err();
    assert!(matches!(
        loop_error,
        CallableResultCallerLedgerErrorV1::RowsUnderPrefix { ref prefix, ref first }
            if prefix.as_ref().unwrap().segments() == &[
                SourcePathSegmentV1::Body(1),
                SourcePathSegmentV1::LoopBodyRoot,
            ] && first.node().segments() == &[
                SourcePathSegmentV1::Body(1),
                SourcePathSegmentV1::LoopBody(0),
                SourcePathSegmentV1::Initializer(0),
            ]
    ));

    let root_error = ledger.prove_body_inactive(&root).unwrap_err();
    assert!(matches!(
        root_error,
        CallableResultCallerLedgerErrorV1::RowsUnderPrefix { prefix: None, .. }
    ));

    let then_statement = view.body_stmt(&then_body, 0).unwrap();
    let then_call = view
        .child_expr_from_stmt(&then_statement, ExprChildRoleV1::LocalInitializer(0))
        .unwrap();
    assert_eq!(
        then_call.activation_site().unwrap().1.node().segments(),
        &[
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::IfThenBody,
            SourcePathSegmentV1::IfThen(0),
            SourcePathSegmentV1::Initializer(0),
        ]
    );
    let body_error = VerifiedCallableResultCallerLedgerV1::verify(&plan, &caller)
        .unwrap()
        .prove_body_inactive(
            &view
                .child_body_from_stmt(&if_statement, BodyChildRoleV1::IfThen)
                .unwrap(),
        )
        .unwrap_err();
    assert!(format!("{body_error:?}").contains("IfThenBody"));
}

#[test]
fn body_domains_do_not_cross_siblings_condition_or_other_statements() {
    let cases = [
        (
            r#"
                box ParserBox {
                    parse(text, pos) {
                        if pos { local value = Helpers.step(1) } else { local value = 0 }
                        loop(pos < 1) { local value = 0 break }
                        return 0
                    }
                }
                static box Helpers { step(value) { return value } }
            "#,
            vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::IfThen(0),
                SourcePathSegmentV1::Initializer(0),
            ],
            true,
            false,
            false,
        ),
        (
            r#"
                box ParserBox {
                    parse(text, pos) {
                        if pos { local value = 0 } else { local value = Helpers.step(2) }
                        loop(pos < 1) { local value = 0 break }
                        return 0
                    }
                }
                static box Helpers { step(value) { return value } }
            "#,
            vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::IfElse(0),
                SourcePathSegmentV1::Initializer(0),
            ],
            false,
            true,
            false,
        ),
        (
            r#"
                box ParserBox {
                    parse(text, pos) {
                        if pos { local value = 0 } else { local value = 0 }
                        loop(pos < 1) { local value = Helpers.step(3) break }
                        return 0
                    }
                }
                static box Helpers { step(value) { return value } }
            "#,
            vec![
                SourcePathSegmentV1::Body(1),
                SourcePathSegmentV1::LoopBody(0),
                SourcePathSegmentV1::Initializer(0),
            ],
            false,
            false,
            true,
        ),
    ];

    for (source, segments, then_active, else_active, loop_active) in cases {
        let plan = seal_body_domain_plan_for_source(source, vec![segments]);
        let caller = caller(&plan);
        let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
        let root = view.root_body();
        let if_statement = view.body_stmt(&root, 0).unwrap();
        let loop_statement = view.body_stmt(&root, 1).unwrap();
        let then_body = view
            .child_body_from_stmt(&if_statement, BodyChildRoleV1::IfThen)
            .unwrap();
        let else_body = view
            .child_body_from_stmt(&if_statement, BodyChildRoleV1::IfElse)
            .unwrap();
        let loop_body = view
            .child_body_from_stmt(&loop_statement, BodyChildRoleV1::LoopBody)
            .unwrap();
        let ledger = VerifiedCallableResultCallerLedgerV1::verify(&plan, &caller).unwrap();

        assert_eq!(ledger.prove_body_inactive(&then_body).is_err(), then_active);
        assert_eq!(ledger.prove_body_inactive(&else_body).is_err(), else_active);
        assert_eq!(ledger.prove_body_inactive(&loop_body).is_err(), loop_active);
    }
}

#[test]
fn body_domain_covers_nested_descendants_of_its_direct_item() {
    const NESTED_SOURCE: &str = r#"
        box ParserBox {
            parse(text, pos) {
                if pos {
                    if text {
                        local nested = Helpers.step(1)
                    }
                }
                return 0
            }
        }
        static box Helpers { step(value) { return value } }
    "#;
    let nested_site = vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::IfThen(0),
        SourcePathSegmentV1::IfThen(0),
        SourcePathSegmentV1::Initializer(0),
    ];
    let plan = seal_body_domain_plan_for_source(NESTED_SOURCE, vec![nested_site.clone()]);
    let caller = caller(&plan);
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    let root = view.root_body();
    let outer_if = view.body_stmt(&root, 0).unwrap();
    let outer_then = view
        .child_body_from_stmt(&outer_if, BodyChildRoleV1::IfThen)
        .unwrap();
    let ledger = VerifiedCallableResultCallerLedgerV1::verify(&plan, &caller).unwrap();

    assert!(matches!(
        ledger.prove_body_inactive(&outer_then),
        Err(CallableResultCallerLedgerErrorV1::RowsUnderPrefix { ref prefix, ref first })
            if prefix.as_ref().unwrap().segments() == &[
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::IfThenBody,
            ] && first.node().segments() == nested_site.as_slice()
    ));
}

#[test]
fn empty_bodies_and_condition_rows_remain_outside_branch_domains() {
    const EMPTY_SOURCE: &str = r#"
        box ParserBox {
            parse(text, pos) {
                if Helpers.step(pos) {
                } else {
                }
                return 0
            }
        }
        static box Helpers { step(value) { return value } }
    "#;
    let plan = seal_body_domain_plan_for_source(
        EMPTY_SOURCE,
        vec![vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::IfCondition,
        ]],
    );
    let caller = caller(&plan);
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    let root = view.root_body();
    let if_statement = view.body_stmt(&root, 0).unwrap();
    let then_body = view
        .child_body_from_stmt(&if_statement, BodyChildRoleV1::IfThen)
        .unwrap();
    let else_body = view
        .child_body_from_stmt(&if_statement, BodyChildRoleV1::IfElse)
        .unwrap();
    let ledger = VerifiedCallableResultCallerLedgerV1::verify(&plan, &caller).unwrap();

    ledger.prove_body_inactive(&then_body).unwrap();
    ledger.prove_body_inactive(&else_body).unwrap();
}

#[test]
fn equal_foreign_plan_location_cannot_claim_or_prove_a_prefix() {
    let primary = seal_plan();
    let foreign = seal_plan();
    let primary_caller = caller(&primary);
    let foreign_caller = caller(&foreign);
    let foreign_view =
        VerifiedCallableResultLegacySourceViewV1::verify(&foreign, &foreign_caller).unwrap();
    let (foreign_body, foreign_selected, _) = calls(&foreign_view);
    let mut ledger =
        VerifiedCallableResultCallerLedgerV1::verify(&primary, &primary_caller).unwrap();

    assert_eq!(
        ledger.claim(&foreign_selected).unwrap_err(),
        CallableResultCallerLedgerErrorV1::ForeignPlan
    );
    assert_eq!(
        ledger.prove_body_inactive(&foreign_body).unwrap_err(),
        CallableResultCallerLedgerErrorV1::ForeignPlan
    );

    let unknown_catalog = declarations("box Missing { parse(text, pos) { return pos } }");
    let unknown = instance_key(&unknown_catalog, "Missing", "parse", 2);
    assert_eq!(
        VerifiedCallableResultCallerLedgerV1::verify(&primary, &unknown).unwrap_err(),
        CallableResultCallerLedgerErrorV1::UnknownCaller(unknown)
    );
}
