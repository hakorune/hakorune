use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
use crate::mir::callable_result_representation::VerifiedCallableResultLegacySourceViewV1;
use crate::mir::resolved_semantics::SourcePathSegmentV1;
use crate::mir::MirInstruction;

use super::local_tests::{
    builder_for, caller, instructions, lower_root_statements, seal_plan, site, CallSiteSpecV1,
};
use super::return_adapter::select_exact_value_return_v1;
use super::{LocatedLegacyLoweringErrorV1, LocatedLegacyLoweringSessionV1};

fn spec(segments: Vec<SourcePathSegmentV1>) -> CallSiteSpecV1 {
    CallSiteSpecV1 {
        site: site(segments),
    }
}

fn call_targets(builder: &crate::mir::MirBuilder) -> Vec<String> {
    instructions(builder)
        .into_iter()
        .filter_map(|instruction| match instruction {
            MirInstruction::Call { callee, .. } => Some(format!("{callee:?}")),
            _ => None,
        })
        .collect()
}

fn terminators(builder: &crate::mir::MirBuilder) -> Vec<MirInstruction> {
    builder
        .scope_ctx
        .current_function
        .as_ref()
        .expect("located Return function")
        .blocks
        .values()
        .filter_map(|block| block.terminator.clone())
        .collect()
}

fn return_count(builder: &crate::mir::MirBuilder) -> usize {
    terminators(builder)
        .iter()
        .filter(|instruction| matches!(instruction, MirInstruction::Return { .. }))
        .count()
}

#[test]
fn located_return_claims_actual_body_value_last_in_exact_order() {
    const SOURCE: &str = r#"
        box ParserBox {
            parse(text, pos) {
                local p0 = 0
                local p1 = 1
                local p2 = 2
                local p3 = 3
                local p4 = 4
                return Helpers.pair(1, 2)
            }
        }
        static box Helpers {
            pair(left, right) { return right }
        }
    "#;
    let plan = seal_plan(
        SOURCE,
        vec![spec(vec![
            SourcePathSegmentV1::Body(5),
            SourcePathSegmentV1::Value,
        ])],
    );
    let caller = caller(plan.declaration_catalog());
    let mut session = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
    let mut builder = builder_for(SOURCE, "located_return_actual_last/0");
    let _scope = LexicalScopeGuard::new(&mut builder);

    lower_root_statements(&mut session, &plan, &caller, &mut builder, &[5]).unwrap();
    session.finish().unwrap();

    let targets = call_targets(&builder);
    assert_eq!(targets.len(), 1, "{targets:?}");
    assert!(targets[0].contains("Helpers.pair"), "{targets:?}");
    assert_eq!(return_count(&builder), 1);
    assert_eq!(builder.recursion_depth, 0);
}

#[test]
fn located_return_claims_nested_argument_before_parent() {
    const SOURCE: &str = r#"
        box ParserBox {
            parse(text, pos) {
                return Helpers.outer(1, Helpers.inner(2))
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
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::Value,
            ]),
            spec(vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::Value,
                SourcePathSegmentV1::Argument(1),
            ]),
        ],
    );
    let caller = caller(plan.declaration_catalog());
    let mut session = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
    let mut builder = builder_for(SOURCE, "located_return_nested_argument/0");
    let _scope = LexicalScopeGuard::new(&mut builder);

    lower_root_statements(&mut session, &plan, &caller, &mut builder, &[0]).unwrap();
    session.finish().unwrap();

    let targets = call_targets(&builder);
    assert_eq!(targets.len(), 2, "{targets:?}");
    assert!(targets[0].contains("Helpers.inner"), "{targets:?}");
    assert!(targets[1].contains("Helpers.outer"), "{targets:?}");
    assert_eq!(return_count(&builder), 1);
    assert_eq!(builder.recursion_depth, 0);
}

#[test]
fn located_return_reuses_binary_and_deferred_short_circuit_spines() {
    const BINARY_SOURCE: &str = r#"
        box ParserBox {
            parse(text, pos) {
                return Helpers.left(1) + Helpers.right(2)
            }
        }
        static box Helpers {
            left(value) { return value }
            right(value) { return value }
        }
    "#;
    let binary_plan = seal_plan(
        BINARY_SOURCE,
        vec![
            spec(vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::Value,
                SourcePathSegmentV1::Lhs,
            ]),
            spec(vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::Value,
                SourcePathSegmentV1::Rhs,
            ]),
        ],
    );
    let binary_caller = caller(binary_plan.declaration_catalog());
    let mut binary_session =
        LocatedLegacyLoweringSessionV1::verify(&binary_plan, &binary_caller).unwrap();
    let mut binary_builder = builder_for(BINARY_SOURCE, "located_return_binary/0");
    let binary_scope = LexicalScopeGuard::new(&mut binary_builder);
    lower_root_statements(
        &mut binary_session,
        &binary_plan,
        &binary_caller,
        &mut binary_builder,
        &[0],
    )
    .unwrap();
    binary_session.finish().unwrap();
    drop(binary_scope);

    let targets = call_targets(&binary_builder);
    assert_eq!(targets.len(), 2, "{targets:?}");
    assert!(targets[0].contains("Helpers.left"), "{targets:?}");
    assert!(targets[1].contains("Helpers.right"), "{targets:?}");
    assert_eq!(return_count(&binary_builder), 1);

    const SHORT_CIRCUIT_SOURCE: &str = r#"
        box ParserBox {
            parse(text, pos) {
                return Helpers.left(1) && Helpers.right(2)
            }
        }
        static box Helpers {
            left(value) { return value }
            right(value) { return value }
        }
    "#;
    let short_circuit_plan = seal_plan(
        SHORT_CIRCUIT_SOURCE,
        vec![
            spec(vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::Value,
                SourcePathSegmentV1::Lhs,
            ]),
            spec(vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::Value,
                SourcePathSegmentV1::Rhs,
            ]),
        ],
    );
    let short_circuit_caller = caller(short_circuit_plan.declaration_catalog());
    let mut short_circuit_session =
        LocatedLegacyLoweringSessionV1::verify(&short_circuit_plan, &short_circuit_caller).unwrap();
    let mut short_circuit_builder =
        builder_for(SHORT_CIRCUIT_SOURCE, "located_return_short_circuit/0");
    let short_circuit_scope = LexicalScopeGuard::new(&mut short_circuit_builder);
    lower_root_statements(
        &mut short_circuit_session,
        &short_circuit_plan,
        &short_circuit_caller,
        &mut short_circuit_builder,
        &[0],
    )
    .unwrap();
    short_circuit_session.finish().unwrap();
    drop(short_circuit_scope);

    assert_eq!(call_targets(&short_circuit_builder).len(), 2);
    assert!(instructions(&short_circuit_builder)
        .iter()
        .any(|instruction| matches!(instruction, MirInstruction::Phi { .. })));
    assert_eq!(return_count(&short_circuit_builder), 1);
    assert_eq!(short_circuit_builder.recursion_depth, 0);
}

#[test]
fn located_return_wrong_order_poisons_without_call_or_completion() {
    const SOURCE: &str = r#"
        box ParserBox {
            parse(text, pos) {
                local first = Helpers.first(1)
                return Helpers.second(2)
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
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::Initializer(0),
            ]),
            spec(vec![
                SourcePathSegmentV1::Body(1),
                SourcePathSegmentV1::Value,
            ]),
        ],
    );
    let caller = caller(plan.declaration_catalog());
    let mut rejected = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
    let mut rejected_builder = builder_for(SOURCE, "located_return_wrong_order/0");
    let rejected_scope = LexicalScopeGuard::new(&mut rejected_builder);
    let error = lower_root_statements(&mut rejected, &plan, &caller, &mut rejected_builder, &[1])
        .unwrap_err();
    drop(rejected_scope);
    assert!(format!("{error:?}").contains("WrongOrder"));
    assert!(call_targets(&rejected_builder).is_empty());
    assert_eq!(return_count(&rejected_builder), 0);
    assert_eq!(rejected_builder.recursion_depth, 0);
    assert_eq!(
        rejected.finish(),
        Err(LocatedLegacyLoweringErrorV1::Poisoned)
    );

    let mut accepted = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
    let mut accepted_builder = builder_for(SOURCE, "located_return_right_order/0");
    let accepted_scope = LexicalScopeGuard::new(&mut accepted_builder);
    lower_root_statements(
        &mut accepted,
        &plan,
        &caller,
        &mut accepted_builder,
        &[0, 1],
    )
    .unwrap();
    accepted.finish().unwrap();
    drop(accepted_scope);
    assert_eq!(call_targets(&accepted_builder).len(), 2);
    assert_eq!(return_count(&accepted_builder), 1);
}

#[test]
fn located_return_cleanup_and_child_failures_require_fresh_sessions() {
    const CLEANUP_SOURCE: &str = r#"
        box ParserBox { parse(text, pos) { return Helpers.step(1) } }
        static box Helpers { step(value) { return value } }
    "#;
    let cleanup_plan = seal_plan(
        CLEANUP_SOURCE,
        vec![spec(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::Value,
        ])],
    );
    let cleanup_caller = caller(cleanup_plan.declaration_catalog());
    let mut cleanup_session =
        LocatedLegacyLoweringSessionV1::verify(&cleanup_plan, &cleanup_caller).unwrap();
    let mut cleanup_builder = builder_for(CLEANUP_SOURCE, "located_return_cleanup/0");
    cleanup_builder.in_cleanup_block = true;
    cleanup_builder.cleanup_allow_return = false;
    let cleanup_scope = LexicalScopeGuard::new(&mut cleanup_builder);
    lower_root_statements(
        &mut cleanup_session,
        &cleanup_plan,
        &cleanup_caller,
        &mut cleanup_builder,
        &[0],
    )
    .unwrap_err();
    drop(cleanup_scope);
    assert!(call_targets(&cleanup_builder).is_empty());
    assert_eq!(return_count(&cleanup_builder), 0);
    assert_eq!(cleanup_builder.recursion_depth, 0);
    assert_eq!(
        cleanup_session.finish(),
        Err(LocatedLegacyLoweringErrorV1::Poisoned)
    );

    let mut recovered =
        LocatedLegacyLoweringSessionV1::verify(&cleanup_plan, &cleanup_caller).unwrap();
    let mut recovered_builder = builder_for(CLEANUP_SOURCE, "located_return_cleanup_reuse/0");
    let recovered_scope = LexicalScopeGuard::new(&mut recovered_builder);
    lower_root_statements(
        &mut recovered,
        &cleanup_plan,
        &cleanup_caller,
        &mut recovered_builder,
        &[0],
    )
    .unwrap();
    recovered.finish().unwrap();
    drop(recovered_scope);
    assert_eq!(call_targets(&recovered_builder).len(), 1);
    assert_eq!(return_count(&recovered_builder), 1);

    const CHILD_SOURCE: &str = r#"
        box ParserBox { parse(text, pos) { return Helpers.step(text) } }
        static box Helpers { step(value) { return value } }
    "#;
    let child_plan = seal_plan(
        CHILD_SOURCE,
        vec![spec(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::Value,
        ])],
    );
    let child_caller = caller(child_plan.declaration_catalog());
    let mut child_session =
        LocatedLegacyLoweringSessionV1::verify(&child_plan, &child_caller).unwrap();
    let mut child_builder = builder_for(CHILD_SOURCE, "located_return_child_failure/0");
    let child_scope = LexicalScopeGuard::new(&mut child_builder);
    lower_root_statements(
        &mut child_session,
        &child_plan,
        &child_caller,
        &mut child_builder,
        &[0],
    )
    .unwrap_err();
    drop(child_scope);
    assert!(call_targets(&child_builder).is_empty());
    assert_eq!(return_count(&child_builder), 0);
    assert_eq!(child_builder.recursion_depth, 0);
    assert_eq!(
        child_session.finish(),
        Err(LocatedLegacyLoweringErrorV1::Poisoned)
    );
}

#[test]
fn located_return_selector_excludes_void_and_non_return_statements() {
    const SOURCE: &str = r#"
        box ParserBox {
            parse(text, pos) {
                local seed = Helpers.step(1)
                return
            }
        }
        static box Helpers { step(value) { return value } }
    "#;
    let plan = seal_plan(
        SOURCE,
        vec![spec(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::Initializer(0),
        ])],
    );
    let caller = caller(plan.declaration_catalog());
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    let body = view.root_body();
    let local = view.body_stmt(&body, 0).unwrap();
    let void_return = view.body_stmt(&body, 1).unwrap();

    assert!(select_exact_value_return_v1(local).is_err());
    assert!(select_exact_value_return_v1(void_return).is_err());
}
