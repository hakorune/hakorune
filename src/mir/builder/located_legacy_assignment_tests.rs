use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
use crate::mir::callable_result_representation::VerifiedCallableResultLegacySourceViewV1;
use crate::mir::resolved_semantics::{BodyChildRoleV1, SourcePathSegmentV1};
use crate::mir::MirInstruction;

use super::local_tests::{
    builder_for, caller, instructions, lower_root_statements, seal_plan, site, CallSiteSpecV1,
};
use super::{LocatedLegacyLoweringErrorV1, LocatedLegacyLoweringSessionV1};

fn call_targets(builder: &crate::mir::MirBuilder) -> Vec<String> {
    instructions(builder)
        .into_iter()
        .filter_map(|instruction| match instruction {
            MirInstruction::Call { callee, .. } => Some(format!("{callee:?}")),
            _ => None,
        })
        .collect()
}

fn spec(segments: Vec<SourcePathSegmentV1>) -> CallSiteSpecV1 {
    CallSiteSpecV1 {
        site: site(segments),
    }
}

#[test]
fn located_assignment_claims_outer_rhs_before_nested_argument_and_completes_once() {
    const SOURCE: &str = r#"
        box ParserBox {
            parse(text, pos) {
                local selected = 0
                selected = Helpers.outer(1, Helpers.inner(2))
                return selected
            }
        }
        static box Helpers {
            outer(left, right) { return right }
            inner(value) { return value }
        }
    "#;
    let plan = seal_plan(
        SOURCE,
        vec![
            spec(vec![
                SourcePathSegmentV1::Body(1),
                SourcePathSegmentV1::Value,
            ]),
            spec(vec![
                SourcePathSegmentV1::Body(1),
                SourcePathSegmentV1::Value,
                SourcePathSegmentV1::Argument(1),
            ]),
        ],
    );
    let caller = caller(plan.declaration_catalog());
    let mut session = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
    let mut builder = builder_for(SOURCE, "located_assignment_nested/0");
    let _scope = LexicalScopeGuard::new(&mut builder);

    lower_root_statements(&mut session, &plan, &caller, &mut builder, &[0, 1]).unwrap();
    session.finish().unwrap();

    let targets = call_targets(&builder);
    assert_eq!(targets.len(), 2, "{targets:?}");
    assert!(targets[0].contains("Helpers.inner"), "{targets:?}");
    assert!(targets[1].contains("Helpers.outer"), "{targets:?}");
    assert!(builder
        .function_state
        .variable_ctx
        .variable_map
        .contains_key("selected"));
    assert_eq!(builder.recursion_depth, 0);
}

#[test]
fn located_assignment_reuses_binary_and_deferred_short_circuit_children() {
    const SOURCE: &str = r#"
        box ParserBox {
            parse(text, pos) {
                local selected = 0
                selected = Helpers.left(1) + Helpers.right(2)
                selected = Helpers.left(1) && Helpers.right(3)
                return selected
            }
        }
        static box Helpers {
            left(value) { return value }
            right(value) { return value }
        }
    "#;
    let plan = seal_plan(
        SOURCE,
        vec![
            spec(vec![
                SourcePathSegmentV1::Body(1),
                SourcePathSegmentV1::Value,
                SourcePathSegmentV1::Lhs,
            ]),
            spec(vec![
                SourcePathSegmentV1::Body(1),
                SourcePathSegmentV1::Value,
                SourcePathSegmentV1::Rhs,
            ]),
            spec(vec![
                SourcePathSegmentV1::Body(2),
                SourcePathSegmentV1::Value,
                SourcePathSegmentV1::Lhs,
            ]),
            spec(vec![
                SourcePathSegmentV1::Body(2),
                SourcePathSegmentV1::Value,
                SourcePathSegmentV1::Rhs,
            ]),
        ],
    );
    let caller = caller(plan.declaration_catalog());
    let mut session = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
    let mut builder = builder_for(SOURCE, "located_assignment_expression_spine/0");
    let _scope = LexicalScopeGuard::new(&mut builder);

    lower_root_statements(&mut session, &plan, &caller, &mut builder, &[0, 1, 2]).unwrap();
    session.finish().unwrap();

    let targets = call_targets(&builder);
    assert_eq!(targets.len(), 4, "{targets:?}");
    assert_eq!(
        targets
            .iter()
            .filter(|target| target.contains("Helpers.left"))
            .count(),
        2,
        "{targets:?}"
    );
    assert_eq!(
        targets
            .iter()
            .filter(|target| target.contains("Helpers.right"))
            .count(),
        2,
        "{targets:?}"
    );
    assert!(instructions(&builder)
        .iter()
        .any(|instruction| matches!(instruction, MirInstruction::Phi { .. })));
    assert_eq!(builder.recursion_depth, 0);
}

#[test]
fn wrong_assignment_order_has_no_rhs_effect_and_fresh_session_succeeds() {
    const SOURCE: &str = r#"
        box ParserBox {
            parse(text, pos) {
                local selected = 0
                selected = Helpers.first(1)
                selected = Helpers.second(2)
                return selected
            }
        }
        static box Helpers {
            first(value) { return value }
            second(value) { return value }
        }
    "#;
    let plan = seal_plan(
        SOURCE,
        vec![
            spec(vec![
                SourcePathSegmentV1::Body(1),
                SourcePathSegmentV1::Value,
            ]),
            spec(vec![
                SourcePathSegmentV1::Body(2),
                SourcePathSegmentV1::Value,
            ]),
        ],
    );
    let caller_key = caller(plan.declaration_catalog());
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller_key).unwrap();
    let body = view.root_body();
    let mut session = LocatedLegacyLoweringSessionV1::verify(&plan, &caller_key).unwrap();
    let mut builder = builder_for(SOURCE, "located_assignment_wrong_order/0");
    let _scope = LexicalScopeGuard::new(&mut builder);
    session
        .lower_statement(&mut builder, view.body_stmt(&body, 0).unwrap())
        .unwrap();
    let before_calls = call_targets(&builder);
    let before_value = builder.function_state.variable_ctx.variable_map["selected"];

    let error = session
        .lower_statement(&mut builder, view.body_stmt(&body, 2).unwrap())
        .unwrap_err();

    assert!(format!("{error:?}").contains("WrongOrder"));
    assert_eq!(call_targets(&builder), before_calls);
    assert_eq!(
        builder.function_state.variable_ctx.variable_map["selected"],
        before_value
    );
    assert_eq!(builder.recursion_depth, 0);
    assert!(matches!(
        session.finish(),
        Err(LocatedLegacyLoweringErrorV1::Poisoned)
    ));

    let mut fresh = LocatedLegacyLoweringSessionV1::verify(&plan, &caller_key).unwrap();
    let mut fresh_builder = builder_for(SOURCE, "located_assignment_fresh/0");
    let _fresh_scope = LexicalScopeGuard::new(&mut fresh_builder);
    lower_root_statements(
        &mut fresh,
        &plan,
        &caller_key,
        &mut fresh_builder,
        &[0, 1, 2],
    )
    .unwrap();
    fresh.finish().unwrap();
    assert_eq!(call_targets(&fresh_builder).len(), 2);
}

#[test]
fn undeclared_target_and_rhs_failure_publish_no_assignment() {
    const UNDECLARED: &str = r#"
        box ParserBox {
            parse(text, pos) {
                local missing = 0
                missing = Helpers.step(1)
                return 0
            }
        }
        static box Helpers { step(value) { return value } }
    "#;
    let plan = seal_plan(
        UNDECLARED,
        vec![spec(vec![
            SourcePathSegmentV1::Body(1),
            SourcePathSegmentV1::Value,
        ])],
    );
    let undeclared_caller = caller(plan.declaration_catalog());
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &undeclared_caller).unwrap();
    let body = view.root_body();
    let mut session = LocatedLegacyLoweringSessionV1::verify(&plan, &undeclared_caller).unwrap();
    let mut builder = builder_for(UNDECLARED, "located_assignment_undeclared/0");
    let _scope = LexicalScopeGuard::new(&mut builder);

    let error = session
        .lower_statement(&mut builder, view.body_stmt(&body, 1).unwrap())
        .unwrap_err();
    assert!(format!("{error:?}").contains("Undefined variable: missing"));
    assert!(call_targets(&builder).is_empty());
    assert!(instructions(&builder).is_empty());
    assert_eq!(builder.recursion_depth, 0);

    const RHS_FAILURE: &str = r#"
        box ParserBox {
            parse(text, pos) {
                local selected = 0
                selected = Helpers.step(text)
                return selected
            }
        }
        static box Helpers { step(value) { return value } }
    "#;
    let plan = seal_plan(
        RHS_FAILURE,
        vec![spec(vec![
            SourcePathSegmentV1::Body(1),
            SourcePathSegmentV1::Value,
        ])],
    );
    let rhs_caller = caller(plan.declaration_catalog());
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &rhs_caller).unwrap();
    let body = view.root_body();
    let mut session = LocatedLegacyLoweringSessionV1::verify(&plan, &rhs_caller).unwrap();
    let mut builder = builder_for(RHS_FAILURE, "located_assignment_rhs_failure/0");
    let _scope = LexicalScopeGuard::new(&mut builder);
    session
        .lower_statement(&mut builder, view.body_stmt(&body, 0).unwrap())
        .unwrap();
    let old = builder.function_state.variable_ctx.variable_map["selected"];
    let before = instructions(&builder);

    let error = session
        .lower_statement(&mut builder, view.body_stmt(&body, 1).unwrap())
        .unwrap_err();
    assert!(format!("{error:?}").contains("Undefined variable: text"));
    assert_eq!(
        builder.function_state.variable_ctx.variable_map["selected"],
        old
    );
    assert_eq!(instructions(&builder), before);
    assert_eq!(builder.recursion_depth, 0);
}

#[test]
fn loop_body_assignment_path_seam_fails_closed_until_loop0() {
    const SOURCE: &str = r#"
        box ParserBox {
            parse(text, pos) {
                local selected = 0
                loop(pos < 1) {
                    local pad0 = 0
                    local pad1 = 0
                    local pad2 = 0
                    local pad3 = 0
                    local pad4 = 0
                    selected = Helpers.outer(1, Helpers.inner(2))
                }
                return selected
            }
        }
        static box Helpers {
            outer(left, right) { return right }
            inner(value) { return value }
        }
    "#;
    let plan = seal_plan(
        SOURCE,
        vec![
            spec(vec![
                SourcePathSegmentV1::Body(1),
                SourcePathSegmentV1::LoopBody(5),
                SourcePathSegmentV1::Value,
            ]),
            spec(vec![
                SourcePathSegmentV1::Body(1),
                SourcePathSegmentV1::LoopBody(5),
                SourcePathSegmentV1::Value,
                SourcePathSegmentV1::Argument(1),
            ]),
        ],
    );
    let caller = caller(plan.declaration_catalog());
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    let root = view.root_body();
    let loop_statement = view.body_stmt(&root, 1).unwrap();
    let loop_body = view
        .child_body_from_stmt(&loop_statement, BodyChildRoleV1::LoopBody)
        .unwrap();
    let assignment = view.body_stmt(&loop_body, 5).unwrap();
    let mut session = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
    let mut builder = builder_for(SOURCE, "located_assignment_loop_carrier/0");
    let _scope = LexicalScopeGuard::new(&mut builder);
    session
        .lower_statement(&mut builder, view.body_stmt(&root, 0).unwrap())
        .unwrap();

    let before = instructions(&builder);
    let old = builder.function_state.variable_ctx.variable_map["selected"];

    let error = session
        .lower_statement(&mut builder, assignment)
        .unwrap_err();

    assert!(format!("{error:?}").contains("Unexpected"));
    assert!(format!("{error:?}").contains("LoopBodyRoot"));
    assert_eq!(instructions(&builder), before);
    assert_eq!(
        builder.function_state.variable_ctx.variable_map["selected"],
        old
    );
    assert!(call_targets(&builder).is_empty());
    assert_eq!(builder.recursion_depth, 0);
    assert!(matches!(
        session.finish(),
        Err(LocatedLegacyLoweringErrorV1::Poisoned)
    ));
}

#[test]
fn non_variable_targets_and_active_loop_controls_fail_closed() {
    const CASES: [(&str, &[SourcePathSegmentV1]); 4] = [
        (
            r#"
                box ParserBox {
                    value: i64
                    parse(text, pos) {
                        me.value = Helpers.step(1)
                        return 0
                    }
                }
                static box Helpers { step(value) { return value } }
            "#,
            &[SourcePathSegmentV1::Body(0), SourcePathSegmentV1::Value],
        ),
        (
            r#"
                box ParserBox {
                    parse(text, pos) {
                        local items = [0]
                        items[0] = Helpers.step(1)
                        return 0
                    }
                }
                static box Helpers { step(value) { return value } }
            "#,
            &[SourcePathSegmentV1::Body(1), SourcePathSegmentV1::Value],
        ),
        (
            r#"
                box ParserBox {
                    parse(text, pos) {
                        loop(Helpers.step(1)) { return 1 }
                        return 0
                    }
                }
                static box Helpers { step(value) { return value } }
            "#,
            &[
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::LoopCondition,
            ],
        ),
        (
            r#"
                box ParserBox {
                    parse(text, pos) {
                        local selected = 0
                        selected += Helpers.step(1)
                        return selected
                    }
                }
                static box Helpers { step(value) { return value } }
            "#,
            &[SourcePathSegmentV1::Body(1), SourcePathSegmentV1::Value],
        ),
    ];

    for (index, (source, path)) in CASES.iter().enumerate() {
        let plan = seal_plan(source, vec![spec(path.to_vec())]);
        let caller = caller(plan.declaration_catalog());
        let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
        let body = view.root_body();
        let mut session = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
        let mut builder = builder_for(source, &format!("located_assignment_reject_{index}/0"));
        let _scope = LexicalScopeGuard::new(&mut builder);
        let before = instructions(&builder);

        let error = session
            .lower_statement(
                &mut builder,
                view.body_stmt(&body, path_body_index(path)).unwrap(),
            )
            .unwrap_err();

        assert!(format!("{error:?}").contains("RowsUnderPrefix"));
        assert_eq!(instructions(&builder), before);
        assert!(call_targets(&builder).is_empty());
        assert_eq!(builder.recursion_depth, 0);
    }
}

fn path_body_index(path: &[SourcePathSegmentV1]) -> usize {
    match path.first() {
        Some(SourcePathSegmentV1::Body(index)) => *index as usize,
        other => panic!("expected root Body path, got {other:?}"),
    }
}
