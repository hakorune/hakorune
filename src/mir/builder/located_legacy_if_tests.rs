use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
use crate::mir::callable_result_representation::{
    CallableResultCallerLedgerErrorV1, VerifiedCallableResultLegacySourceViewV1,
};
use crate::mir::resolved_semantics::{SourceNodeSiteV1, SourcePathSegmentV1};
use crate::mir::{BasicBlockId, ConstValue, MirInstruction, ValueId};

use super::if_adapter::select_exact_statement_if_v1;
use super::local_tests::{
    builder_for, caller, instructions, lower_root_statements, seal_plan, site, CallSiteSpecV1,
};
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

fn all_void_const_count(builder: &crate::mir::MirBuilder) -> usize {
    instructions(builder)
        .into_iter()
        .filter(|instruction| {
            matches!(
                instruction,
                MirInstruction::Const {
                    value: ConstValue::Void,
                    ..
                }
            )
        })
        .count()
}

#[derive(Debug, PartialEq)]
struct LocatedIfBoundarySnapshotV1 {
    current_block: Option<BasicBlockId>,
    function_next_value: u32,
    core_next_value: ValueId,
    core_next_block: BasicBlockId,
    variables: Vec<(String, ValueId)>,
    blocks: Vec<(
        BasicBlockId,
        Vec<MirInstruction>,
        Option<MirInstruction>,
        Vec<BasicBlockId>,
        Vec<BasicBlockId>,
        bool,
        bool,
    )>,
}

fn boundary_snapshot(builder: &crate::mir::MirBuilder) -> LocatedIfBoundarySnapshotV1 {
    let function = builder.function_state.current_function.as_ref().unwrap();
    let mut variables = builder
        .function_state
        .variable_ctx
        .variable_map
        .iter()
        .map(|(name, value)| (name.clone(), *value))
        .collect::<Vec<_>>();
    variables.sort_by(|left, right| left.0.cmp(&right.0));
    let mut blocks = function
        .blocks
        .values()
        .map(|block| {
            (
                block.id,
                block.instructions.clone(),
                block.terminator.clone(),
                block.predecessors.iter().copied().collect(),
                block.successors.iter().copied().collect(),
                block.reachable,
                block.sealed,
            )
        })
        .collect::<Vec<_>>();
    blocks.sort_by_key(|row| row.0);
    LocatedIfBoundarySnapshotV1 {
        current_block: builder.function_state.current_block,
        function_next_value: function.next_value_id,
        core_next_value: builder.core_ctx.peek_next_value(),
        core_next_block: builder.core_ctx.peek_next_block(),
        variables,
        blocks,
    }
}

#[test]
fn located_statement_if_claims_actual_top_level_condition_lhs() {
    const SOURCE: &str = r#"
        box ParserBox {
            parse(text, pos) {
                local ret = Helpers.parse_mul(1)
                if Helpers.is_error(ret) == 1 { return ret }
                return 0
            }
        }
        static box Helpers {
            parse_mul(value) { return 0 }
            is_error(value) { return 0 }
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
                SourcePathSegmentV1::IfCondition,
                SourcePathSegmentV1::Lhs,
            ]),
        ],
    );
    let caller = caller(plan.declaration_catalog());
    let mut session = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
    let mut builder = builder_for(SOURCE, "located_statement_if_actual/0");
    let _scope = LexicalScopeGuard::new(&mut builder);

    lower_root_statements(&mut session, &plan, &caller, &mut builder, &[0, 1]).unwrap();
    session.finish().unwrap();

    let targets = call_targets(&builder);
    assert_eq!(targets.len(), 2, "{targets:?}");
    assert!(targets[0].contains("Helpers.parse_mul"), "{targets:?}");
    assert!(targets[1].contains("Helpers.is_error"), "{targets:?}");
    assert!(instructions(&builder)
        .iter()
        .any(|instruction| matches!(instruction, MirInstruction::Compare { .. })));
    assert_eq!(all_void_const_count(&builder), 2);
    assert_eq!(builder.recursion_depth, 0);
}

#[test]
fn located_statement_if_reuses_short_circuit_condition_descent() {
    const SOURCE: &str = r#"
        box ParserBox {
            parse(text, pos) {
                if Helpers.left(1) && Helpers.right(2) { return 1 }
                return 0
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
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::IfCondition,
                SourcePathSegmentV1::Lhs,
            ]),
            spec(vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::IfCondition,
                SourcePathSegmentV1::Rhs,
            ]),
        ],
    );
    let caller = caller(plan.declaration_catalog());
    let mut session = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
    let mut builder = builder_for(SOURCE, "located_statement_if_short_circuit/0");
    let _scope = LexicalScopeGuard::new(&mut builder);

    lower_root_statements(&mut session, &plan, &caller, &mut builder, &[0]).unwrap();
    session.finish().unwrap();

    let targets = call_targets(&builder);
    assert_eq!(targets.len(), 2, "{targets:?}");
    // Block storage order is not execution order. The successful exact-ledger
    // finish above proves left-before-right claims; this inventory proves both
    // physical calls remain present in the short-circuit CFG.
    assert!(
        targets.iter().any(|target| target.contains("Helpers.left")),
        "{targets:?}"
    );
    assert!(
        targets
            .iter()
            .any(|target| target.contains("Helpers.right")),
        "{targets:?}"
    );
    let all_instructions = instructions(&builder);
    assert_eq!(
        all_instructions
            .iter()
            .filter(|instruction| matches!(instruction, MirInstruction::Phi { .. }))
            .count(),
        1
    );
    assert!(!all_instructions
        .iter()
        .any(|instruction| matches!(instruction, MirInstruction::BinOp { .. })));
    assert_eq!(
        builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .blocks
            .values()
            .filter(|block| matches!(block.terminator, Some(MirInstruction::Branch { .. })))
            .count(),
        3
    );
    assert_eq!(all_void_const_count(&builder), 2);
}

#[test]
fn located_statement_if_orders_condition_rows_and_lowers_inactive_branches() {
    const SOURCE: &str = r#"
        box ParserBox {
            parse(text, pos) {
                local result = 0
                if Helpers.left(1) + Helpers.right(2) {
                    result = 1
                } else {
                    result = 2
                }
                return result
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
                SourcePathSegmentV1::IfCondition,
                SourcePathSegmentV1::Lhs,
            ]),
            spec(vec![
                SourcePathSegmentV1::Body(1),
                SourcePathSegmentV1::IfCondition,
                SourcePathSegmentV1::Rhs,
            ]),
        ],
    );
    let caller = caller(plan.declaration_catalog());
    let mut session = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
    let mut builder = builder_for(SOURCE, "located_statement_if_explicit/0");
    let _scope = LexicalScopeGuard::new(&mut builder);

    lower_root_statements(&mut session, &plan, &caller, &mut builder, &[0]).unwrap();
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    let root = view.root_body();
    let output = session
        .lower_statement(&mut builder, view.body_stmt(&root, 1).unwrap())
        .unwrap();
    session.finish().unwrap();

    let targets = call_targets(&builder);
    assert_eq!(targets.len(), 2, "{targets:?}");
    assert!(targets[0].contains("Helpers.left"), "{targets:?}");
    assert!(targets[1].contains("Helpers.right"), "{targets:?}");
    assert!(instructions(&builder)
        .iter()
        .any(|instruction| matches!(instruction, MirInstruction::BinOp { .. })));
    assert!(instructions(&builder)
        .iter()
        .any(|instruction| matches!(instruction, MirInstruction::Phi { .. })));
    assert!(matches!(
        builder.function_state.current_function.as_ref().unwrap().blocks
            [&builder.function_state.current_block.unwrap()]
            .instructions
            .last(),
        Some(MirInstruction::Const {
            dst,
            value: ConstValue::Void,
        }) if *dst == output
    ));
    assert_eq!(all_void_const_count(&builder), 1);
}

#[test]
fn active_then_row_fails_before_raw_branch_effects_and_poisons_session() {
    assert_active_branch_fails_closed(
        r#"
            box ParserBox {
                parse(text, pos) {
                    if 1 {
                        local before_selected = 701
                        local selected = Helpers.then_call(1)
                    }
                    return 0
                }
            }
            static box Helpers { then_call(value) { return value } }
        "#,
        vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::IfThen(1),
            SourcePathSegmentV1::Initializer(0),
        ],
        "located_statement_if_then_reject/0",
        "IfThenBody",
        None,
        701,
    );
}

#[test]
fn active_else_row_fails_without_else_call_effects_and_poisons_session() {
    assert_active_branch_fails_closed(
        r#"
            box ParserBox {
                parse(text, pos) {
                    if 0 { local completed_then = 601 }
                    else {
                        local before_selected = 701
                        local selected = Helpers.else_call(1)
                    }
                    return 0
                }
            }
            static box Helpers { else_call(value) { return value } }
        "#,
        vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::IfElse(1),
            SourcePathSegmentV1::Initializer(0),
        ],
        "located_statement_if_else_reject/0",
        "IfElseBody",
        Some(601),
        701,
    );
}

fn assert_active_branch_fails_closed(
    source: &'static str,
    selected: Vec<SourcePathSegmentV1>,
    function_name: &str,
    body_root: &str,
    expected_present_marker: Option<i64>,
    expected_absent_marker: i64,
) {
    let selected_site = selected.clone();
    let plan = seal_plan(source, vec![spec(selected)]);
    let caller = caller(plan.declaration_catalog());
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    let body = view.root_body();
    let mut session = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
    let mut builder = builder_for(source, function_name);
    let _scope = LexicalScopeGuard::new(&mut builder);

    let error = session
        .lower_statement(&mut builder, view.body_stmt(&body, 0).unwrap())
        .unwrap_err();

    let diagnostic = format!("{error:?}");
    assert!(diagnostic.contains("RowsUnderPrefix"), "{diagnostic}");
    assert!(diagnostic.contains(body_root), "{diagnostic}");
    assert!(
        diagnostic.contains(&format!("{:?}", selected_site)),
        "{diagnostic}"
    );
    assert!(call_targets(&builder).is_empty());
    let integer_constants = instructions(&builder)
        .into_iter()
        .filter_map(|instruction| match instruction {
            MirInstruction::Const {
                value: ConstValue::Integer(value),
                ..
            } => Some(value),
            _ => None,
        })
        .collect::<Vec<_>>();
    if let Some(expected) = expected_present_marker {
        assert!(
            integer_constants.contains(&expected),
            "{integer_constants:?}"
        );
    }
    assert!(
        !integer_constants.contains(&expected_absent_marker),
        "{integer_constants:?}"
    );
    assert_eq!(all_void_const_count(&builder), 0);
    assert_eq!(
        session.finish(),
        Err(LocatedLegacyLoweringErrorV1::Poisoned)
    );
}

#[test]
fn root_loop_keeps_nested_if_row_parked_for_loop0() {
    const SOURCE: &str = r#"
        box ParserBox {
            parse(text, pos) {
                loop(1) {
                    if Helpers.nested(1) { return 1 }
                    break
                }
                return 0
            }
        }
        static box Helpers { nested(value) { return value } }
    "#;
    let plan = seal_plan(
        SOURCE,
        vec![spec(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::LoopBody(0),
            SourcePathSegmentV1::IfCondition,
        ])],
    );
    let caller = caller(plan.declaration_catalog());
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    let body = view.root_body();
    let mut session = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
    let mut builder = builder_for(SOURCE, "located_statement_if_loop_boundary/0");
    let _scope = LexicalScopeGuard::new(&mut builder);
    let before = boundary_snapshot(&builder);

    let error = session
        .lower_statement(&mut builder, view.body_stmt(&body, 0).unwrap())
        .unwrap_err();

    assert!(matches!(
        error,
        LocatedLegacyLoweringErrorV1::Ledger(
            CallableResultCallerLedgerErrorV1::RowsUnderPrefix { ref prefix, ref first }
        ) if prefix.as_ref() == Some(&SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(0),
        ])) && first.node().segments() == &[
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::LoopBody(0),
            SourcePathSegmentV1::IfCondition,
        ]
    ));
    assert_eq!(boundary_snapshot(&builder), before);
    assert!(call_targets(&builder).is_empty());
    assert_eq!(
        session.finish(),
        Err(LocatedLegacyLoweringErrorV1::Poisoned)
    );
}

#[test]
fn condition_wrong_order_has_no_control_effects_then_fresh_session_succeeds() {
    const SOURCE: &str = r#"
        box ParserBox {
            parse(text, pos) {
                local first = Helpers.first(1)
                if Helpers.second(2) { return 1 }
                return 0
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
                SourcePathSegmentV1::IfCondition,
            ]),
        ],
    );
    let caller = caller(plan.declaration_catalog());
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    let root = view.root_body();
    let if_statement = view.body_stmt(&root, 1).unwrap();
    let mut session = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
    let mut builder = builder_for(SOURCE, "located_statement_if_wrong_order/0");
    let _scope = LexicalScopeGuard::new(&mut builder);
    builder
        .metadata_ctx
        .set_current_span(if_statement.node().span());
    let before = boundary_snapshot(&builder);

    let error = session
        .lower_statement(&mut builder, if_statement)
        .unwrap_err();
    assert!(format!("{error:?}").contains("WrongOrder"));
    assert_eq!(boundary_snapshot(&builder), before);
    assert_eq!(all_void_const_count(&builder), 0);
    let after = boundary_snapshot(&builder);
    assert_eq!(
        session
            .lower_statement(&mut builder, view.body_stmt(&root, 0).unwrap())
            .unwrap_err(),
        LocatedLegacyLoweringErrorV1::Poisoned
    );
    assert_eq!(boundary_snapshot(&builder), after);

    let mut fresh = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
    let mut fresh_builder = builder_for(SOURCE, "located_statement_if_fresh/0");
    let _fresh_scope = LexicalScopeGuard::new(&mut fresh_builder);
    lower_root_statements(&mut fresh, &plan, &caller, &mut fresh_builder, &[0, 1]).unwrap();
    fresh.finish().unwrap();
    assert_eq!(call_targets(&fresh_builder).len(), 2);
    assert_eq!(all_void_const_count(&fresh_builder), 2);
}

#[test]
fn located_statement_if_selector_rejects_non_if_without_rebuilding_carrier() {
    const SOURCE: &str = r#"
        box ParserBox {
            parse(text, pos) {
                local value = Helpers.step(1)
                return value
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

    let input = view.body_stmt(&body, 0).unwrap();
    let original_node = input.node();
    let rejected = match select_exact_statement_if_v1(input) {
        Ok(_) => panic!("non-If input unexpectedly selected"),
        Err(rejected) => rejected,
    };

    assert!(std::ptr::eq(rejected.node(), original_node));
}
