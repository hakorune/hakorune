use crate::mir::resolved_semantics::ExprChildRoleV1;

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
