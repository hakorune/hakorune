use crate::mir::resolved_semantics::{ExprChildRoleV1, SourcePathSegmentV1};
use crate::mir::source_call_target::{
    VerifiedSourceStaticCallTargetCatalogV1, VerifiedStaticImportAliasViewV1,
};
use crate::parser::NyashParser;

use super::super::{
    CallableResultActivationDispositionV1, CallableResultCallerLedgerErrorV1,
    CallableResultLoopClaimBatchErrorV1, VerifiedCallableResultActivationPlanV1,
    VerifiedCallableResultActivationRowsV1, VerifiedCallableResultCallerLedgerV1,
    VerifiedCallableResultLegacySourceViewV1, VerifiedCallableResultLoopClaimScheduleV1,
    VerifiedSameModuleCallableResultCatalogV1,
};
use super::actual_parser_add_fixture;
use super::support::{instance_key, site};

const SOURCE: &str = r#"
    box ParserBox {
        parse(text, pos) {
            local before = Helpers.before(pos)
            loop(Helpers.condition(pos)) {
                local inside = Helpers.outer(Helpers.inner(pos))
            }
            return Helpers.after(pos)
        }
    }
    box OtherBox {
        parse(text, pos) {
            loop(Helpers.condition(pos)) {
                local inside = Helpers.outer(pos)
            }
            return pos
        }
    }
    static box Helpers {
        before(value) { return value }
        condition(value) { return value }
        outer(value) { return value }
        inner(value) { return value }
        after(value) { return value }
    }
"#;

fn seal_unselected_plan(source: &str) -> VerifiedCallableResultActivationPlanV1 {
    let root = NyashParser::parse_from_string(source).expect("Loop claim fixture must parse");
    let declarations = Box::new(
        crate::mir::builder::VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root)
            .expect("Loop claim declarations"),
    );
    let imports = VerifiedStaticImportAliasViewV1::seal(&declarations, Vec::new()).unwrap();
    let targets =
        VerifiedSourceStaticCallTargetCatalogV1::seal_qualified(&imports, std::iter::empty())
            .unwrap();
    let results =
        VerifiedSameModuleCallableResultCatalogV1::verify(&declarations, &targets).unwrap();
    let rows =
        VerifiedCallableResultActivationRowsV1::verify(&declarations, &targets, &results).unwrap();
    drop(results);
    drop(targets);
    drop(imports);
    VerifiedCallableResultActivationPlanV1::seal(declarations, rows).unwrap()
}

fn caller(
    plan: &VerifiedCallableResultActivationPlanV1,
    owner: &str,
) -> crate::mir::builder::CanonicalSameModuleCallableKeyV1 {
    instance_key(plan.declaration_catalog(), owner, "parse", 2)
}

fn loop_schedule<'plan>(
    plan: &'plan VerifiedCallableResultActivationPlanV1,
    caller: &crate::mir::builder::CanonicalSameModuleCallableKeyV1,
    statement_index: usize,
) -> VerifiedCallableResultLoopClaimScheduleV1<'plan> {
    let view = VerifiedCallableResultLegacySourceViewV1::verify(plan, caller).unwrap();
    let statement = view.body_stmt(&view.root_body(), statement_index).unwrap();
    VerifiedCallableResultLoopClaimScheduleV1::verify(plan, caller, statement).unwrap()
}

#[test]
fn source_order_claims_allow_plan_order_consumption() {
    let plan = seal_unselected_plan(SOURCE);
    let parser = caller(&plan, "ParserBox");
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &parser).unwrap();
    let body = view.root_body();
    let before_stmt = view.body_stmt(&body, 0).unwrap();
    let before = view
        .child_expr_from_stmt(&before_stmt, ExprChildRoleV1::LocalInitializer(0))
        .unwrap();
    let after_stmt = view.body_stmt(&body, 2).unwrap();
    let after = view
        .child_expr_from_stmt(&after_stmt, ExprChildRoleV1::ReturnValue)
        .unwrap();
    let schedule = loop_schedule(&plan, &parser, 1);
    let sites = schedule
        .sites_in_source_order()
        .cloned()
        .collect::<Vec<_>>();
    assert_eq!(sites.len(), 3);

    let mut ledger = VerifiedCallableResultCallerLedgerV1::verify(&plan, &parser).unwrap();
    ledger.claim(&before).unwrap();
    let mut batch = ledger.claim_loop_batch(schedule).unwrap();
    assert!(batch.is_branded_by(&plan));
    assert_eq!(batch.caller(), &parser);
    assert_eq!(batch.take_claim(&sites[2]).unwrap().site(), &sites[2]);
    assert_eq!(batch.take_claim(&sites[1]).unwrap().site(), &sites[1]);
    assert_eq!(batch.take_claim(&sites[0]).unwrap().site(), &sites[0]);
    batch.finish().unwrap();
    ledger.claim(&after).unwrap();
    ledger.finish().unwrap();
}

#[test]
fn batch_prevalidation_failure_has_zero_ledger_delta() {
    let plan = seal_unselected_plan(SOURCE);
    let parser = caller(&plan, "ParserBox");
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &parser).unwrap();
    let body = view.root_body();
    let before_stmt = view.body_stmt(&body, 0).unwrap();
    let before = view
        .child_expr_from_stmt(&before_stmt, ExprChildRoleV1::LocalInitializer(0))
        .unwrap();
    let mut ledger = VerifiedCallableResultCallerLedgerV1::verify(&plan, &parser).unwrap();

    assert!(matches!(
        ledger.claim_loop_batch(loop_schedule(&plan, &parser, 1)),
        Err(CallableResultCallerLedgerErrorV1::WrongOrder { .. })
    ));
    ledger.claim(&before).unwrap();
    let mut batch = ledger
        .claim_loop_batch(loop_schedule(&plan, &parser, 1))
        .unwrap();
    let sites = plan
        .rows_for(&parser)
        .unwrap()
        .iter()
        .skip(1)
        .take(3)
        .map(|row| row.site().clone())
        .collect::<Vec<_>>();
    for site in sites {
        batch.take_claim(&site).unwrap();
    }
    batch.finish().unwrap();
}

#[test]
fn malformed_batches_fail_after_staging_without_partial_commit() {
    let plan = seal_unselected_plan(SOURCE);
    let parser = caller(&plan, "ParserBox");
    let other = caller(&plan, "OtherBox");
    let canonical_parser = plan
        .declaration_catalog()
        .declaration(&parser)
        .unwrap()
        .key();
    let parser_rows = plan.rows_for(&parser).unwrap();
    let other_rows = plan.rows_for(&other).unwrap();
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &parser).unwrap();
    let before_stmt = view.body_stmt(&view.root_body(), 0).unwrap();
    let before = view
        .child_expr_from_stmt(&before_stmt, ExprChildRoleV1::LocalInitializer(0))
        .unwrap();

    let cases = [
        (
            vec![&parser_rows[0], &parser_rows[0]],
            CallableResultCallerLedgerErrorV1::Duplicate {
                site: parser_rows[0].site().clone(),
            },
            "duplicate after one staged row",
        ),
        (
            vec![&parser_rows[0], &parser_rows[2]],
            CallableResultCallerLedgerErrorV1::WrongOrder {
                expected: parser_rows[1].site().clone(),
                actual: parser_rows[2].site().clone(),
            },
            "wrong order after one staged row",
        ),
        (
            vec![&parser_rows[0], &other_rows[0]],
            CallableResultCallerLedgerErrorV1::Unexpected {
                site: other_rows[0].site().clone(),
            },
            "unknown caller row after one staged row",
        ),
        (
            parser_rows
                .iter()
                .chain(std::iter::once(&other_rows[0]))
                .collect::<Vec<_>>(),
            CallableResultCallerLedgerErrorV1::Unexpected {
                site: other_rows[0].site().clone(),
            },
            "extra row after the complete caller schedule",
        ),
    ];

    for (rows, expected, label) in cases {
        let mut ledger = VerifiedCallableResultCallerLedgerV1::verify(&plan, &parser).unwrap();
        assert_eq!(
            ledger
                .claim_rows_for_atomicity_test(&plan, canonical_parser, &rows)
                .unwrap_err(),
            expected,
            "{label}"
        );
        ledger
            .claim(&before)
            .unwrap_or_else(|error| panic!("{label} partially committed: {error:?}"));
    }
}

#[test]
fn removal_errors_and_unused_finish_are_typed() {
    let plan = seal_unselected_plan(SOURCE);
    let parser = caller(&plan, "ParserBox");
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &parser).unwrap();
    let before_stmt = view.body_stmt(&view.root_body(), 0).unwrap();
    let before = view
        .child_expr_from_stmt(&before_stmt, ExprChildRoleV1::LocalInitializer(0))
        .unwrap();
    let schedule = loop_schedule(&plan, &parser, 1);
    let sites = schedule
        .sites_in_source_order()
        .cloned()
        .collect::<Vec<_>>();
    let mut ledger = VerifiedCallableResultCallerLedgerV1::verify(&plan, &parser).unwrap();
    ledger.claim(&before).unwrap();
    let mut batch = ledger.claim_loop_batch(schedule).unwrap();

    batch.take_claim(&sites[0]).unwrap();
    assert_eq!(
        batch.take_claim(&sites[0]).unwrap_err(),
        CallableResultLoopClaimBatchErrorV1::AlreadyConsumed {
            site: sites[0].clone(),
        }
    );
    let unknown = site(vec![SourcePathSegmentV1::Body(99)]);
    assert_eq!(
        batch.take_claim(&unknown).unwrap_err(),
        CallableResultLoopClaimBatchErrorV1::UnexpectedSite { site: unknown }
    );
    assert_eq!(
        batch.finish().unwrap_err(),
        CallableResultLoopClaimBatchErrorV1::Unconsumed {
            first: sites[1].clone(),
            remaining: 2,
        }
    );
}

#[test]
fn foreign_plan_and_caller_fail_without_claiming() {
    let primary = seal_unselected_plan(SOURCE);
    let foreign = seal_unselected_plan(SOURCE);
    let parser = caller(&primary, "ParserBox");
    let foreign_parser = caller(&foreign, "ParserBox");
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&primary, &parser).unwrap();
    let before_stmt = view.body_stmt(&view.root_body(), 0).unwrap();
    let before = view
        .child_expr_from_stmt(&before_stmt, ExprChildRoleV1::LocalInitializer(0))
        .unwrap();
    let mut ledger = VerifiedCallableResultCallerLedgerV1::verify(&primary, &parser).unwrap();

    assert_eq!(
        ledger
            .claim_loop_batch(loop_schedule(&foreign, &foreign_parser, 1))
            .unwrap_err(),
        CallableResultCallerLedgerErrorV1::ForeignPlan
    );
    ledger.claim(&before).unwrap();

    let other = caller(&primary, "OtherBox");
    assert!(matches!(
        ledger.claim_loop_batch(loop_schedule(&primary, &other, 0)),
        Err(CallableResultCallerLedgerErrorV1::ForeignCaller { .. })
    ));
    let mut batch = ledger
        .claim_loop_batch(loop_schedule(&primary, &parser, 1))
        .unwrap();
    for site in batch_sites(&primary, &parser) {
        batch.take_claim(&site).unwrap();
    }
    batch.finish().unwrap();
}

fn batch_sites(
    plan: &VerifiedCallableResultActivationPlanV1,
    caller: &crate::mir::builder::CanonicalSameModuleCallableKeyV1,
) -> Vec<crate::mir::resolved_semantics::SourceExprSiteV1> {
    plan.rows_for(caller)
        .unwrap()
        .iter()
        .skip(1)
        .take(3)
        .map(|row| row.site().clone())
        .collect()
}

#[test]
fn actual_fifteen_rows_claim_loop_six_through_fourteen_atomically() {
    let plan = actual_parser_add_fixture::plan();
    let parser = actual_parser_add_fixture::caller(&plan);
    let rows = plan.rows_for(&parser).unwrap();
    assert_eq!(rows.len(), 15);
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &parser).unwrap();
    let body = view.root_body();
    let stmt0 = view.body_stmt(&body, 0).unwrap();
    let row0 = view
        .child_expr_from_stmt(&stmt0, ExprChildRoleV1::LocalInitializer(0))
        .unwrap();
    let stmt1 = view.body_stmt(&body, 1).unwrap();
    let if_condition = view
        .child_expr_from_stmt(&stmt1, ExprChildRoleV1::IfCondition)
        .unwrap();
    let row1 = view
        .child_expr(&if_condition, ExprChildRoleV1::BinaryLeft)
        .unwrap();
    let stmt2 = view.body_stmt(&body, 2).unwrap();
    let row2 = view
        .child_expr_from_stmt(&stmt2, ExprChildRoleV1::LocalInitializer(0))
        .unwrap();
    let stmt3 = view.body_stmt(&body, 3).unwrap();
    let row3 = view
        .child_expr_from_stmt(&stmt3, ExprChildRoleV1::AssignmentValue)
        .unwrap();
    let row4 = view
        .child_expr(&row3, ExprChildRoleV1::CallArgument(1))
        .unwrap();
    let stmt5 = view.body_stmt(&body, 5).unwrap();
    let row14 = view
        .child_expr_from_stmt(&stmt5, ExprChildRoleV1::ReturnValue)
        .unwrap();

    let mut ledger = VerifiedCallableResultCallerLedgerV1::verify(&plan, &parser).unwrap();
    for expression in [&row0, &row1, &row2, &row3, &row4] {
        ledger.claim(expression).unwrap();
    }
    let schedule = loop_schedule(&plan, &parser, 4);
    let loop_sites = schedule
        .sites_in_source_order()
        .cloned()
        .collect::<Vec<_>>();
    assert_eq!(
        loop_sites,
        rows[5..=13]
            .iter()
            .map(|row| row.site().clone())
            .collect::<Vec<_>>()
    );
    let mut batch = ledger.claim_loop_batch(schedule).unwrap();

    // Source claims outer human row 13 before nested row 14; plan emission may consume 14 first.
    assert!(matches!(
        batch.take_claim(rows[13].site()).unwrap().disposition(),
        CallableResultActivationDispositionV1::Unselected
    ));
    assert!(matches!(
        batch.take_claim(rows[12].site()).unwrap().disposition(),
        CallableResultActivationDispositionV1::SelectedExactI64 { .. }
    ));
    for row in &rows[5..=11] {
        batch.take_claim(row.site()).unwrap();
    }
    batch.finish().unwrap();
    ledger.claim(&row14).unwrap();
    ledger.finish().unwrap();
}
