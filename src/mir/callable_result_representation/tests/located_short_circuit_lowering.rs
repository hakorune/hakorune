use crate::mir::builder::{LocatedLegacyLoweringErrorV1, LocatedLegacyLoweringSessionV1};
use crate::mir::resolved_semantics::{ExprChildRoleV1, SourcePathSegmentV1};
use crate::mir::{BasicBlockId, MirBuilder, MirInstruction};

use super::super::{
    VerifiedCallableResultActivationPlanV1, VerifiedCallableResultActivationRowsV1,
    VerifiedCallableResultLegacySourceViewV1,
};
use super::support::{
    declarations, instance_key, qualified_targets, seal_with_targets, site, CallSiteSpecV1,
};

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
            .expect("located short-circuit activation rows");
    drop(results);
    drop(targets);
    VerifiedCallableResultActivationPlanV1::seal(declarations, rows)
        .expect("located short-circuit plan")
}

fn caller(
    plan: &VerifiedCallableResultActivationPlanV1,
) -> crate::mir::builder::CanonicalSameModuleCallableKeyV1 {
    instance_key(plan.declaration_catalog(), "ParserBox", "parse", 2)
}

fn expression_at<'plan>(
    view: &VerifiedCallableResultLegacySourceViewV1<'plan>,
    statement_index: usize,
    role: ExprChildRoleV1,
) -> super::super::LegacyExprInputV1<'plan> {
    let body = view.root_body();
    let statement = view.body_stmt(&body, statement_index).unwrap();
    view.child_expr_from_stmt(&statement, role).unwrap()
}

fn builder_for(source: &str, name: &str, install_catalog: bool) -> MirBuilder {
    let mut builder = MirBuilder::new();
    if install_catalog {
        builder
            .comp_ctx
            .install_callable_declaration_catalog(declarations(source))
            .unwrap();
    }
    builder.enter_function_for_test(name.to_string());
    builder
}

fn call_rows(builder: &MirBuilder) -> Vec<(BasicBlockId, String)> {
    builder
        .scope_ctx
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .iter()
        .flat_map(|(block, data)| {
            data.instructions
                .iter()
                .filter_map(|instruction| match instruction {
                    MirInstruction::Call { callee, .. } => Some((*block, format!("{callee:?}"))),
                    _ => None,
                })
        })
        .collect()
}

#[test]
fn located_short_circuit_claims_left_before_deferred_right_inside_eval_block() {
    const SOURCE: &str = r#"
        box ParserBox {
            parse(text, pos) {
                local value = Helpers.left(1) && Helpers.right(2)
                return value
            }
        }
        static box Helpers {
            left(value) { return value }
            right(value) { return value }
        }
    "#;
    let left = site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Initializer(0),
        SourcePathSegmentV1::Lhs,
    ]);
    let right = site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Initializer(0),
        SourcePathSegmentV1::Rhs,
    ]);
    let plan = seal_plan(SOURCE, &[left, right]);
    let caller = caller(&plan);
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    let expression = expression_at(&view, 0, ExprChildRoleV1::LocalInitializer(0));
    let mut session = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
    let mut builder = builder_for(SOURCE, "located_sc_order/0", true);

    session.lower_expression(&mut builder, expression).unwrap();
    session.finish().unwrap();

    let rows = call_rows(&builder);
    assert_eq!(rows.len(), 2);
    let left_row = rows
        .iter()
        .find(|(_, target)| target.contains("Helpers.left"))
        .expect("left call row");
    let right_row = rows
        .iter()
        .find(|(_, target)| target.contains("Helpers.right"))
        .expect("right call row");
    assert_ne!(
        left_row.0, right_row.0,
        "RHS must lower in eval-RHS: {rows:?}"
    );
    assert_eq!(builder.recursion_depth, 0);
}

#[test]
fn located_short_circuit_accepts_nested_and_or_comparison_tree() {
    const SOURCE: &str = r#"
        box ParserBox {
            parse(text, pos) {
                local value = Helpers.left(1) == 1 &&
                    (Helpers.middle(2) == 2 || Helpers.right(3) == 3)
                return value
            }
        }
        static box Helpers {
            left(value) { return value }
            middle(value) { return value }
            right(value) { return value }
        }
    "#;
    let selected = [
        site(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::Initializer(0),
            SourcePathSegmentV1::Lhs,
            SourcePathSegmentV1::Lhs,
        ]),
        site(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::Initializer(0),
            SourcePathSegmentV1::Rhs,
            SourcePathSegmentV1::Lhs,
            SourcePathSegmentV1::Lhs,
        ]),
        site(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::Initializer(0),
            SourcePathSegmentV1::Rhs,
            SourcePathSegmentV1::Rhs,
            SourcePathSegmentV1::Lhs,
        ]),
    ];
    let plan = seal_plan(SOURCE, &selected);
    let caller = caller(&plan);
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    let expression = expression_at(&view, 0, ExprChildRoleV1::LocalInitializer(0));
    let mut session = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
    let mut builder = builder_for(SOURCE, "located_sc_nested/0", true);

    session.lower_expression(&mut builder, expression).unwrap();
    session.finish().unwrap();

    let mut targets = call_rows(&builder)
        .into_iter()
        .map(|(_, target)| target)
        .collect::<Vec<_>>();
    targets.sort();
    assert_eq!(targets.len(), 3);
    assert!(targets.iter().any(|target| target.contains("Helpers.left")));
    assert!(targets
        .iter()
        .any(|target| target.contains("Helpers.middle")));
    assert!(targets
        .iter()
        .any(|target| target.contains("Helpers.right")));
}

#[test]
fn located_short_circuit_accepts_actual_loop_condition_shape() {
    const SOURCE: &str = r#"
        box ParserBox {
            parse(text, pos) {
                loop(Helpers.left(1) && Helpers.right(2)) { break }
                return 0
            }
        }
        static box Helpers {
            left(value) { return value }
            right(value) { return value }
        }
    "#;
    let selected = [
        site(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::LoopCondition,
            SourcePathSegmentV1::Lhs,
        ]),
        site(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::LoopCondition,
            SourcePathSegmentV1::Rhs,
        ]),
    ];
    let plan = seal_plan(SOURCE, &selected);
    let caller = caller(&plan);
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    let condition = expression_at(&view, 0, ExprChildRoleV1::LoopCondition);
    let mut session = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
    let mut builder = builder_for(SOURCE, "located_sc_loop_condition/0", true);

    session.lower_expression(&mut builder, condition).unwrap();
    session.finish().unwrap();

    assert_eq!(call_rows(&builder).len(), 2);
}

#[test]
fn located_short_circuit_route_failure_poisons_only_that_session() {
    const SOURCE: &str = r#"
        box ParserBox {
            parse(text, pos) {
                local value = Helpers.left(1) && Helpers.right(2)
                return value
            }
        }
        static box Helpers {
            left(value) { return value }
            right(value) { return value }
        }
    "#;
    let left = site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Initializer(0),
        SourcePathSegmentV1::Lhs,
    ]);
    let right = site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Initializer(0),
        SourcePathSegmentV1::Rhs,
    ]);
    let plan = seal_plan(SOURCE, &[left, right]);
    let caller = caller(&plan);
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    let expression = expression_at(&view, 0, ExprChildRoleV1::LocalInitializer(0));
    let mut failed = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
    let mut failed_builder = builder_for(SOURCE, "located_sc_failure/0", false);

    assert!(failed
        .lower_expression(&mut failed_builder, expression)
        .is_err());
    assert_eq!(failed.finish(), Err(LocatedLegacyLoweringErrorV1::Poisoned));
    assert_eq!(failed_builder.recursion_depth, 0);

    let mut fresh = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
    let mut fresh_builder = builder_for(SOURCE, "located_sc_fresh/0", true);
    fresh
        .lower_expression(
            &mut fresh_builder,
            expression_at(&view, 0, ExprChildRoleV1::LocalInitializer(0)),
        )
        .unwrap();
    fresh.finish().unwrap();
}
