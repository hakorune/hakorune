use crate::mir::builder::{LocatedLegacyLoweringErrorV1, LocatedLegacyLoweringSessionV1};
use crate::mir::resolved_semantics::{ExprChildRoleV1, SourcePathSegmentV1};
use crate::mir::{MirBuilder, MirInstruction};

use super::super::{
    CallableResultCallerLedgerErrorV1, VerifiedCallableResultActivationPlanV1,
    VerifiedCallableResultActivationRowsV1, VerifiedCallableResultLegacySourceViewV1,
};
use super::support::{
    declarations, instance_key, qualified_targets, seal_with_targets, site, CallSiteSpecV1,
};

const NESTED_SOURCE: &str = r#"
    box ParserBox {
        parse(text, pos) {
            local first = Helpers.step(Helpers.step(1))
            local width = "abc".length()
            local inactive = 7
            return first
        }
    }
    static box Helpers {
        step(value) { return value }
    }
"#;

fn selected_outer_site() -> crate::mir::resolved_semantics::SourceExprSiteV1 {
    site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Initializer(0),
    ])
}

fn selected_inner_site() -> crate::mir::resolved_semantics::SourceExprSiteV1 {
    site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Initializer(0),
        SourcePathSegmentV1::Argument(0),
    ])
}

fn seal_plan(
    source: &str,
    selected: &[crate::mir::resolved_semantics::SourceExprSiteV1],
) -> VerifiedCallableResultActivationPlanV1 {
    let declarations = Box::new(declarations(source));
    let specs = selected
        .iter()
        .cloned()
        .map(|site| CallSiteSpecV1 {
            caller_owner: "ParserBox",
            caller_name: "parse",
            caller_arity: 2,
            site,
        })
        .collect::<Vec<_>>();
    let targets = qualified_targets(declarations.as_ref(), &[], &specs);
    let results = seal_with_targets(declarations.as_ref(), &targets);
    let rows =
        VerifiedCallableResultActivationRowsV1::verify(declarations.as_ref(), &targets, &results)
            .expect("located lowering activation rows");
    drop(results);
    drop(targets);
    VerifiedCallableResultActivationPlanV1::seal(declarations, rows).expect("located lowering plan")
}

fn caller(
    plan: &VerifiedCallableResultActivationPlanV1,
) -> crate::mir::builder::CanonicalSameModuleCallableKeyV1 {
    instance_key(plan.declaration_catalog(), "ParserBox", "parse", 2)
}

fn expression_at<'plan>(
    view: &VerifiedCallableResultLegacySourceViewV1<'plan>,
    statement_index: usize,
) -> super::super::LegacyExprInputV1<'plan> {
    let body = view.root_body();
    let statement = view.body_stmt(&body, statement_index).unwrap();
    view.child_expr_from_stmt(&statement, ExprChildRoleV1::LocalInitializer(0))
        .unwrap()
}

fn builder_for(source: &str, name: &str) -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder
        .comp_ctx
        .install_callable_declaration_catalog(declarations(source))
        .unwrap();
    builder.enter_function_for_test(name.to_string());
    builder
}

fn instructions(builder: &MirBuilder) -> Vec<MirInstruction> {
    builder
        .scope_ctx
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter().cloned())
        .collect()
}

#[test]
fn selected_nested_and_unselected_method_rows_are_claimed_before_descent() {
    let plan = seal_plan(
        NESTED_SOURCE,
        &[selected_outer_site(), selected_inner_site()],
    );
    let caller = caller(&plan);
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    let outer = expression_at(&view, 0);
    let unselected = expression_at(&view, 1);
    let mut session = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
    let mut builder = builder_for(NESTED_SOURCE, "located_nested/0");

    session.lower_expression(&mut builder, outer).unwrap();
    session.lower_expression(&mut builder, unselected).unwrap();
    session.finish().unwrap();

    let calls = instructions(&builder)
        .into_iter()
        .filter(|instruction| matches!(instruction, MirInstruction::Call { .. }))
        .count();
    assert_eq!(calls, 3, "inner, outer, and unselected terminals");
}

#[test]
fn inactive_expression_delegates_once_and_finish_reports_missing_rows() {
    let plan = seal_plan(
        NESTED_SOURCE,
        &[selected_outer_site(), selected_inner_site()],
    );
    let caller = caller(&plan);
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    let inactive = expression_at(&view, 2);
    let mut session = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
    let mut builder = builder_for(NESTED_SOURCE, "located_inactive/0");

    session.lower_expression(&mut builder, inactive).unwrap();
    assert_eq!(
        instructions(&builder)
            .iter()
            .filter(|instruction| matches!(instruction, MirInstruction::Const { .. }))
            .count(),
        1
    );
    assert!(matches!(
        session.finish(),
        Err(LocatedLegacyLoweringErrorV1::Ledger(
            CallableResultCallerLedgerErrorV1::Missing { remaining: 3, .. }
        ))
    ));
}

#[test]
fn active_row_under_non_method_prefix_never_reaches_raw_lowering() {
    const SOURCE: &str = r#"
        box ParserBox {
            parse(text, pos) {
                local value = 1 + Helpers.step(2)
                return value
            }
        }
        static box Helpers { step(value) { return value } }
    "#;
    let nested_site = site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Initializer(0),
        SourcePathSegmentV1::Rhs,
    ]);
    let plan = seal_plan(SOURCE, &[nested_site]);
    let caller = caller(&plan);
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    let binary = expression_at(&view, 0);
    let mut session = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
    let mut builder = builder_for(SOURCE, "located_active_prefix/0");

    assert!(matches!(
        session.lower_expression(&mut builder, binary),
        Err(LocatedLegacyLoweringErrorV1::Ledger(
            CallableResultCallerLedgerErrorV1::RowsUnderPrefix { .. }
        ))
    ));
    assert!(instructions(&builder).is_empty());
    assert_eq!(
        session.finish(),
        Err(LocatedLegacyLoweringErrorV1::Poisoned)
    );
}

#[test]
fn wrong_order_and_duplicate_claims_fail_before_new_child_effects() {
    let plan = seal_plan(
        NESTED_SOURCE,
        &[selected_outer_site(), selected_inner_site()],
    );
    let caller = caller(&plan);
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();

    let mut wrong = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
    let mut wrong_builder = builder_for(NESTED_SOURCE, "located_wrong_order/0");
    assert!(matches!(
        wrong.lower_expression(&mut wrong_builder, expression_at(&view, 1)),
        Err(LocatedLegacyLoweringErrorV1::Ledger(
            CallableResultCallerLedgerErrorV1::WrongOrder { .. }
        ))
    ));
    assert!(instructions(&wrong_builder).is_empty());

    let mut duplicate = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
    let mut duplicate_builder = builder_for(NESTED_SOURCE, "located_duplicate/0");
    duplicate
        .lower_expression(&mut duplicate_builder, expression_at(&view, 0))
        .unwrap();
    let before = instructions(&duplicate_builder).len();
    assert!(matches!(
        duplicate.lower_expression(&mut duplicate_builder, expression_at(&view, 0)),
        Err(LocatedLegacyLoweringErrorV1::Ledger(
            CallableResultCallerLedgerErrorV1::Duplicate { .. }
        ))
    ));
    assert_eq!(instructions(&duplicate_builder).len(), before);
}

#[test]
fn route_failure_after_claim_poisons_session_and_fresh_session_is_independent() {
    const SOURCE: &str = r#"
        box ParserBox {
            parse(text, pos) {
                local value = Helpers.step(1)
                return value
            }
        }
        static box Helpers { step(value) { return value } }
    "#;
    let call_site = selected_outer_site();
    let plan = seal_plan(SOURCE, &[call_site]);
    let caller = caller(&plan);
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    let call = expression_at(&view, 0);
    let mut session = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("located_failure/0".to_string());

    assert!(matches!(
        session.lower_expression(&mut builder, call),
        Err(LocatedLegacyLoweringErrorV1::Lowering(_))
    ));
    assert_eq!(builder.recursion_depth, 0);
    assert!(matches!(
        session.lower_expression(&mut builder, expression_at(&view, 0)),
        Err(LocatedLegacyLoweringErrorV1::Poisoned)
    ));
    assert_eq!(
        session.finish(),
        Err(LocatedLegacyLoweringErrorV1::Poisoned)
    );

    let fresh = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
    assert!(matches!(
        fresh.finish(),
        Err(LocatedLegacyLoweringErrorV1::Ledger(
            CallableResultCallerLedgerErrorV1::Missing { remaining: 1, .. }
        ))
    ));
}
