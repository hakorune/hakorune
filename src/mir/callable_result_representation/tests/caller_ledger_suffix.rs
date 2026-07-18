use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::resolved_semantics::{
    BodyChildRoleV1, ExprChildRoleV1, SourceExprSiteV1, SourcePathSegmentV1,
};

use super::super::{
    CallableResultBodySuffixDecisionV1, CallableResultCallerLedgerErrorV1,
    CallableResultLegacyLocationErrorV1, VerifiedCallableResultActivationPlanV1,
    VerifiedCallableResultActivationRowsV1, VerifiedCallableResultCallerLedgerV1,
    VerifiedCallableResultLegacySourceViewV1,
};
use super::support::{
    declarations, instance_key, qualified_targets, seal_with_targets, site, CallSiteSpecV1,
};

const ROOT_SOURCE: &str = r#"
    box ParserBox {
        parse(text, pos) {
            local first = Helpers.step(1)
            local plain = 0
            local later = Helpers.step(2)
            return plain
        }
    }
    static box Helpers { step(value) { return value } }
"#;

fn seal_plan(
    source: &str,
    specs: &[(&'static str, Vec<SourcePathSegmentV1>)],
) -> VerifiedCallableResultActivationPlanV1 {
    let declarations = Box::new(declarations(source));
    let targets = qualified_targets(
        declarations.as_ref(),
        &[],
        &specs
            .iter()
            .map(|(caller_name, segments)| CallSiteSpecV1 {
                caller_owner: "ParserBox",
                caller_name,
                caller_arity: 2,
                site: site(segments.clone()),
            })
            .collect::<Vec<_>>(),
    );
    let results = seal_with_targets(declarations.as_ref(), &targets);
    let rows =
        VerifiedCallableResultActivationRowsV1::verify(declarations.as_ref(), &targets, &results)
            .expect("suffix rows");
    drop(results);
    drop(targets);
    VerifiedCallableResultActivationPlanV1::seal(declarations, rows).expect("suffix plan")
}

fn root_plan() -> VerifiedCallableResultActivationPlanV1 {
    seal_plan(
        ROOT_SOURCE,
        &[
            (
                "parse",
                vec![
                    SourcePathSegmentV1::Body(0),
                    SourcePathSegmentV1::Initializer(0),
                ],
            ),
            (
                "parse",
                vec![
                    SourcePathSegmentV1::Body(2),
                    SourcePathSegmentV1::Initializer(0),
                ],
            ),
        ],
    )
}

fn caller(plan: &VerifiedCallableResultActivationPlanV1) -> CanonicalSameModuleCallableKeyV1 {
    instance_key(plan.declaration_catalog(), "ParserBox", "parse", 2)
}

fn call_at<'plan>(
    view: &VerifiedCallableResultLegacySourceViewV1<'plan>,
    body: &super::super::LegacyBodyInputV1<'plan>,
    index: usize,
) -> super::super::LegacyExprInputV1<'plan> {
    let statement = view.body_stmt(body, index).expect("call statement");
    view.child_expr_from_stmt(&statement, ExprChildRoleV1::LocalInitializer(0))
        .expect("call initializer")
}

fn active_site(decision: CallableResultBodySuffixDecisionV1<'_>) -> SourceExprSiteV1 {
    match decision {
        CallableResultBodySuffixDecisionV1::Active { first } => first.clone(),
        CallableResultBodySuffixDecisionV1::Inactive(_) => panic!("expected active suffix"),
    }
}

#[test]
fn inactive_root_start_zero_borrows_the_complete_body() {
    const SOURCE: &str = r#"
        box ParserBox {
            parse(text, pos) {
                local value = pos
                return value
            }
        }
    "#;
    let plan = seal_plan(SOURCE, &[]);
    let caller = caller(&plan);
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    let body = view.root_body();
    let ledger = VerifiedCallableResultCallerLedgerV1::verify(&plan, &caller).unwrap();
    let inactive = match ledger
        .classify_body_suffix(view.body_suffix(&body, 0).unwrap())
        .unwrap()
    {
        CallableResultBodySuffixDecisionV1::Inactive(proof) => proof,
        CallableResultBodySuffixDecisionV1::Active { .. } => panic!("root must be inactive"),
    };
    assert_eq!(inactive.as_ref().len(), body.statements().len());
    assert_eq!(inactive.as_ref().as_ptr(), body.statements().as_ptr());
    ledger.finish().unwrap();
}

#[test]
fn root_suffix_scans_all_rows_and_inactive_proof_borrows_exact_slice() {
    let plan = root_plan();
    let root_caller = caller(&plan);
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &root_caller).unwrap();
    let body = view.root_body();
    let mut ledger = VerifiedCallableResultCallerLedgerV1::verify(&plan, &root_caller).unwrap();

    assert_eq!(
        active_site(
            ledger
                .classify_body_suffix(view.body_suffix(&body, 0).unwrap())
                .unwrap()
        )
        .node()
        .segments(),
        &[
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::Initializer(0),
        ]
    );
    assert_eq!(
        active_site(
            ledger
                .classify_body_suffix(view.body_suffix(&body, 1).unwrap())
                .unwrap()
        )
        .node()
        .segments(),
        &[
            SourcePathSegmentV1::Body(2),
            SourcePathSegmentV1::Initializer(0),
        ]
    );

    let expected = &body.statements()[3..];
    let inactive = match ledger
        .classify_body_suffix(view.body_suffix(&body, 3).unwrap())
        .unwrap()
    {
        CallableResultBodySuffixDecisionV1::Inactive(proof) => proof,
        CallableResultBodySuffixDecisionV1::Active { .. } => panic!("suffix must be inactive"),
    };
    assert_eq!(inactive.as_ref().len(), expected.len());
    assert_eq!(inactive.as_ref().as_ptr(), expected.as_ptr());

    let end = match ledger
        .classify_body_suffix(view.body_suffix(&body, body.statements().len()).unwrap())
        .unwrap()
    {
        CallableResultBodySuffixDecisionV1::Inactive(proof) => proof,
        CallableResultBodySuffixDecisionV1::Active { .. } => panic!("end suffix must be empty"),
    };
    assert!(end.as_ref().is_empty());

    // Classification is observation-only: exact claims remain available.
    ledger.claim(&call_at(&view, &body, 0)).unwrap();
    ledger.claim(&call_at(&view, &body, 2)).unwrap();
    ledger.finish().unwrap();
}

#[test]
fn condition_only_row_stays_outside_actual_empty_branch_body() {
    const SOURCE: &str = r#"
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
    let plan = seal_plan(
        SOURCE,
        &[(
            "parse",
            vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::IfCondition,
            ],
        )],
    );
    let caller = caller(&plan);
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    let root = view.root_body();
    let if_statement = view.body_stmt(&root, 0).unwrap();
    let then_body = view
        .child_body_from_stmt(&if_statement, BodyChildRoleV1::IfThen)
        .unwrap();
    let ledger = VerifiedCallableResultCallerLedgerV1::verify(&plan, &caller).unwrap();
    let inactive = match ledger
        .classify_body_suffix(view.body_suffix(&then_body, 0).unwrap())
        .unwrap()
    {
        CallableResultBodySuffixDecisionV1::Inactive(proof) => proof,
        CallableResultBodySuffixDecisionV1::Active { .. } => {
            panic!("condition row must stay outside then body")
        }
    };
    assert!(then_body.statements().is_empty());
    assert!(inactive.as_ref().is_empty());
    assert_eq!(inactive.as_ref().as_ptr(), then_body.statements().as_ptr());
}

#[test]
fn nested_rows_belong_to_their_item_without_crossing_sibling_bodies() {
    const SOURCE: &str = r#"
        box ParserBox {
            parse(text, pos) {
                if Helpers.step(pos) {
                    if text { local nested = Helpers.step(1) }
                } else {
                }
                loop(pos < 1) { local looped = Helpers.step(2) break }
                return 0
            }
        }
        static box Helpers { step(value) { return value } }
    "#;
    let plan = seal_plan(
        SOURCE,
        &[
            (
                "parse",
                vec![
                    SourcePathSegmentV1::Body(0),
                    SourcePathSegmentV1::IfCondition,
                ],
            ),
            (
                "parse",
                vec![
                    SourcePathSegmentV1::Body(0),
                    SourcePathSegmentV1::IfThen(0),
                    SourcePathSegmentV1::IfThen(0),
                    SourcePathSegmentV1::Initializer(0),
                ],
            ),
            (
                "parse",
                vec![
                    SourcePathSegmentV1::Body(1),
                    SourcePathSegmentV1::LoopBody(0),
                    SourcePathSegmentV1::Initializer(0),
                ],
            ),
        ],
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

    let then_first = active_site(
        ledger
            .classify_body_suffix(view.body_suffix(&then_body, 0).unwrap())
            .unwrap(),
    );
    assert_eq!(
        then_first.node().segments(),
        &[
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::IfThen(0),
            SourcePathSegmentV1::IfThen(0),
            SourcePathSegmentV1::Initializer(0),
        ]
    );
    let empty_else = match ledger
        .classify_body_suffix(view.body_suffix(&else_body, 0).unwrap())
        .unwrap()
    {
        CallableResultBodySuffixDecisionV1::Inactive(proof) => proof,
        CallableResultBodySuffixDecisionV1::Active { .. } => panic!("empty else must be inactive"),
    };
    assert!(empty_else.as_ref().is_empty());
    assert_eq!(
        active_site(
            ledger
                .classify_body_suffix(view.body_suffix(&root, 1).unwrap())
                .unwrap()
        )
        .node()
        .segments()[0],
        SourcePathSegmentV1::Body(1)
    );
    let loop_statement = view.body_stmt(&root, 1).unwrap();
    let loop_body = view
        .child_body_from_stmt(&loop_statement, BodyChildRoleV1::LoopBody)
        .unwrap();
    assert_eq!(
        active_site(
            ledger
                .classify_body_suffix(view.body_suffix(&loop_body, 0).unwrap())
                .unwrap()
        )
        .node()
        .segments(),
        &[
            SourcePathSegmentV1::Body(1),
            SourcePathSegmentV1::LoopBody(0),
            SourcePathSegmentV1::Initializer(0),
        ]
    );
}

#[test]
fn suffix_location_rejects_unlocated_overflow_and_out_of_bounds() {
    let plan = root_plan();
    let root_caller = caller(&plan);
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &root_caller).unwrap();
    let body = view.root_body();

    assert_eq!(
        view.body_suffix(&body, body.statements().len() + 1)
            .unwrap_err(),
        CallableResultLegacyLocationErrorV1::BodySuffixStartOutOfBounds {
            body: None,
            start: (body.statements().len() + 1) as u32,
            len: body.statements().len(),
        }
    );
    if usize::BITS > 32 {
        let index = u32::MAX as usize + 1;
        assert_eq!(
            view.body_suffix(&body, index).unwrap_err(),
            CallableResultLegacyLocationErrorV1::BodySuffixIndexOverflow { index }
        );
    }

    const IF_SOURCE: &str = r#"
        box ParserBox {
            parse(text, pos) {
                if pos { local value = Helpers.step(1) }
                return 0
            }
        }
        static box Helpers { step(value) { return value } }
    "#;
    let plan = seal_plan(
        IF_SOURCE,
        &[(
            "parse",
            vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::IfThen(0),
                SourcePathSegmentV1::Initializer(0),
            ],
        )],
    );
    let caller = caller(&plan);
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    let root = view.root_body();
    let if_statement = view.body_stmt(&root, 0).unwrap();
    let unlocated_if = view.unlocated_expr(if_statement.node());
    let unlocated_body = view
        .child_body(&unlocated_if, BodyChildRoleV1::IfThen)
        .unwrap();
    assert_eq!(
        view.body_suffix(&unlocated_body, 0).unwrap_err(),
        CallableResultLegacyLocationErrorV1::UnlocatedCannotProveInactive
    );
}

#[test]
fn suffix_carriers_reject_foreign_plan_and_foreign_caller() {
    let primary = root_plan();
    let foreign = root_plan();
    let primary_caller = caller(&primary);
    let foreign_caller = caller(&foreign);
    let foreign_view =
        VerifiedCallableResultLegacySourceViewV1::verify(&foreign, &foreign_caller).unwrap();
    let foreign_body = foreign_view.root_body();
    let primary_ledger =
        VerifiedCallableResultCallerLedgerV1::verify(&primary, &primary_caller).unwrap();
    assert_eq!(
        primary_ledger
            .classify_body_suffix(foreign_view.body_suffix(&foreign_body, 0).unwrap())
            .unwrap_err(),
        CallableResultCallerLedgerErrorV1::ForeignPlan
    );

    const TWO_CALLERS: &str = r#"
        box ParserBox {
            parse(text, pos) { local value = Helpers.step(1) return value }
            other(text, pos) { local value = Helpers.step(2) return value }
        }
        static box Helpers { step(value) { return value } }
    "#;
    let plan = seal_plan(
        TWO_CALLERS,
        &[
            (
                "parse",
                vec![
                    SourcePathSegmentV1::Body(0),
                    SourcePathSegmentV1::Initializer(0),
                ],
            ),
            (
                "other",
                vec![
                    SourcePathSegmentV1::Body(0),
                    SourcePathSegmentV1::Initializer(0),
                ],
            ),
        ],
    );
    let parse = instance_key(plan.declaration_catalog(), "ParserBox", "parse", 2);
    let other = instance_key(plan.declaration_catalog(), "ParserBox", "other", 2);
    let other_view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &other).unwrap();
    let other_body = other_view.root_body();
    let ledger = VerifiedCallableResultCallerLedgerV1::verify(&plan, &parse).unwrap();
    assert!(matches!(
        ledger.classify_body_suffix(other_view.body_suffix(&other_body, 0).unwrap()),
        Err(CallableResultCallerLedgerErrorV1::ForeignCaller { expected, actual })
            if expected == parse && actual == other
    ));
}
