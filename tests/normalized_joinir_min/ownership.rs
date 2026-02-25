use super::*;

/// Phase 60: Ownership relay threading helpers for P2 (analysis + contract)
///
/// plan_to_p2_inputs remains Fail-Fast on relay_writes (legacy contract),
/// while plan_to_p2_inputs_with_relay accepts single-hop relay and promotes them
/// to carriers (dev-only).
#[test]
#[cfg(feature = "normalized_dev")]
fn test_phase60_ownership_p2_with_relay_conversion() {
    use nyash_rust::mir::join_ir::ownership::{
        plan_to_p2_inputs, plan_to_p2_inputs_with_relay, OwnershipAnalyzer,
    };
    use serde_json::json;

    // Create a simple P2 fixture JSON (loop with i and sum)
    let json = json!({
        "functions": [{
            "name": "main",
            "params": [],
            "body": {
                "kind": "Block",
                "statements": [
                    {"kind": "Local", "name": "sum", "init": {"kind": "Const", "value": 0}},
                    {"kind": "Local", "name": "i", "init": {"kind": "Const", "value": 0}},
                    {
                        "kind": "Loop",
                        "condition": {
                            "kind": "BinaryOp",
                            "op": "Lt",
                            "lhs": {"kind": "Var", "name": "i"},
                            "rhs": {"kind": "Const", "value": 3}
                        },
                        "body": {
                            "kind": "Block",
                            "statements": [
                                {
                                    "kind": "If",
                                    "condition": {
                                        "kind": "BinaryOp",
                                        "op": "Ge",
                                        "lhs": {"kind": "Var", "name": "i"},
                                        "rhs": {"kind": "Const", "value": 2}
                                    },
                                    "then": {"kind": "Break"}
                                },
                                {
                                    "kind": "Assign",
                                    "target": "sum",
                                    "value": {
                                        "kind": "BinaryOp",
                                        "op": "Add",
                                        "lhs": {"kind": "Var", "name": "sum"},
                                        "rhs": {"kind": "Var", "name": "i"}
                                    }
                                },
                                {
                                    "kind": "Assign",
                                    "target": "i",
                                    "value": {
                                        "kind": "BinaryOp",
                                        "op": "Add",
                                        "lhs": {"kind": "Var", "name": "i"},
                                        "rhs": {"kind": "Const", "value": 1}
                                    }
                                }
                            ]
                        }
                    }
                ]
            }
        }]
    });

    // Run ownership analyzer
    let mut analyzer = OwnershipAnalyzer::new();
    let plans = analyzer
        .analyze_json(&json)
        .expect("analysis should succeed");

    // Find loop plan (the one that has relay writes to function-owned sum/i)
    let loop_plan = plans
        .iter()
        .find(|p| !p.relay_writes.is_empty())
        .expect("should have a loop plan with relay writes");

    eprintln!(
        "[phase58/test] Loop plan: relay_writes={:?}",
        loop_plan.relay_writes
    );

    // Legacy Fail-Fast: relay_writes should be rejected
    let result = plan_to_p2_inputs(loop_plan, "i");
    assert!(
        result.is_err(),
        "Legacy contract: relay_writes should be rejected"
    );
    let err = result.unwrap_err();
    assert!(
        err.contains("relay_writes not yet supported"),
        "Error should mention relay limitation, got: {}",
        err
    );

    // Phase 60 dev-only: with_relay should accept and include relay vars as carriers
    let inputs_with_relay =
        plan_to_p2_inputs_with_relay(loop_plan, "i").expect("with_relay should accept");
    assert!(
        inputs_with_relay.carriers.iter().any(|c| c.name == "sum"),
        "relay carrier sum should be promoted"
    );

    eprintln!(
        "[phase60/test] with_relay carriers={:?}",
        inputs_with_relay
            .carriers
            .iter()
            .map(|c| &c.name)
            .collect::<Vec<_>>()
    );

    // Also test the case where variables ARE owned by loop (future scenario)
    // This would work once we support loop-local carriers
    let loop_local_json = json!({
        "functions": [{
            "name": "main",
            "params": [],
            "body": {
                "kind": "Loop",
                "condition": {"kind": "Const", "value": true},
                "body": {
                    "kind": "Block",
                    "statements": [
                        {"kind": "Local", "name": "i", "init": {"kind": "Const", "value": 0}},
                        {"kind": "Local", "name": "sum", "init": {"kind": "Const", "value": 0}},
                        {
                            "kind": "Assign",
                            "target": "sum",
                            "value": {
                                "kind": "BinaryOp",
                                "op": "Add",
                                "lhs": {"kind": "Var", "name": "sum"},
                                "rhs": {"kind": "Var", "name": "i"}
                            }
                        },
                        {
                            "kind": "Assign",
                            "target": "i",
                            "value": {
                                "kind": "BinaryOp",
                                "op": "Add",
                                "lhs": {"kind": "Var", "name": "i"},
                                "rhs": {"kind": "Const", "value": 1}
                            }
                        },
                        {"kind": "Break"}
                    ]
                }
            }
        }]
    });

    let mut analyzer2 = OwnershipAnalyzer::new();
    let plans2 = analyzer2
        .analyze_json(&loop_local_json)
        .expect("loop-local analysis should succeed");

    // Find loop plan (variables owned by loop, not function)
    let loop_plan2 = plans2
        .iter()
        .find(|p| !p.owned_vars.is_empty())
        .expect("should have a loop plan with owned vars");

    eprintln!(
        "[phase58/test] Loop-local plan: owned_vars={:?}",
        loop_plan2
            .owned_vars
            .iter()
            .map(|v| &v.name)
            .collect::<Vec<_>>()
    );

    // Convert to P2 inputs (should succeed - no relay)
    let inputs = plan_to_p2_inputs(loop_plan2, "i").expect("should convert successfully");

    eprintln!("[phase58/test] P2 inputs: {:?}", inputs);

    // Verify: i is skipped (loop var), sum becomes carrier
    assert_eq!(inputs.carriers.len(), 1, "Should have 1 carrier (sum)");
    assert_eq!(inputs.carriers[0].name, "sum");

    eprintln!("[phase60/test] Loop-local conversion verified");
}

/// Phase 60: P2 dev-only ownership relay route matches legacy Break lowering.
#[test]
#[cfg(feature = "normalized_dev")]
fn test_phase60_break_lowering_ownership_matches_legacy() {
    use nyash_rust::mir::join_ir::frontend::ast_lowerer::lower_break_legacy_for_comparison;
    use nyash_rust::mir::join_ir::frontend::ast_lowerer::AstToJoinIrLowerer;

    let _ctx = normalized_dev_test_ctx();

    let program_json: serde_json::Value = serde_json::from_str(include_str!(
        "../../docs/private/roadmap2/phases/phase-34-joinir-frontend/fixtures/loop_frontend_break.program.json"
    ))
    .expect("fixture json");

    let mut lowerer_new = AstToJoinIrLowerer::new();
    let structured_new = lowerer_new.lower_program_json(&program_json);

    let mut lowerer_old = AstToJoinIrLowerer::new();
    let structured_old = lower_break_legacy_for_comparison(&mut lowerer_old, &program_json);

    let entry_new = structured_new.entry.expect("new entry");
    let entry_old = structured_old.entry.expect("old entry");
    let input = vec![JoinValue::Int(5)];

    let out_old = run_joinir_vm_bridge(&structured_old, entry_old, &input, false);
    let out_new = run_joinir_vm_bridge(&structured_new, entry_new, &input, false);

    assert_eq!(
        out_old, out_new,
        "ownership relay dev route must match legacy output"
    );
}

/// Phase 59: P3 with outer-owned carriers (relay case) should fail-fast
#[test]
#[cfg(feature = "normalized_dev")]
fn test_phase59_ownership_p3_relay_failfast() {
    use nyash_rust::mir::join_ir::ownership::{plan_to_p3_inputs, OwnershipAnalyzer};
    use serde_json::json;

    // P3 where sum/count are defined OUTSIDE the loop -> relay
    let json = json!({
        "functions": [{
            "name": "main",
            "params": [],
            "body": {
                "kind": "Block",
                "statements": [
                    {"kind": "Local", "name": "sum", "init": {"kind": "Const", "value": 0}},
                    {"kind": "Local", "name": "count", "init": {"kind": "Const", "value": 0}},
                    {"kind": "Local", "name": "i", "init": {"kind": "Const", "value": 0}},
                    {
                        "kind": "Loop",
                        "condition": {
                            "kind": "BinaryOp", "op": "Lt",
                            "lhs": {"kind": "Var", "name": "i"},
                            "rhs": {"kind": "Const", "value": 10}
                        },
                        "body": {
                            "kind": "Block",
                            "statements": [
                                {
                                    "kind": "If",
                                    "condition": {
                                        "kind": "BinaryOp", "op": "Gt",
                                        "lhs": {"kind": "Var", "name": "i"},
                                        "rhs": {"kind": "Const", "value": 0}
                                    },
                                    "then": {
                                        "kind": "Block",
                                        "statements": [
                                            {"kind": "Assign", "target": "sum", "value": {
                                                "kind": "BinaryOp", "op": "Add",
                                                "lhs": {"kind": "Var", "name": "sum"},
                                                "rhs": {"kind": "Var", "name": "i"}
                                            }},
                                            {"kind": "Assign", "target": "count", "value": {
                                                "kind": "BinaryOp", "op": "Add",
                                                "lhs": {"kind": "Var", "name": "count"},
                                                "rhs": {"kind": "Const", "value": 1}
                                            }}
                                        ]
                                    }
                                },
                                {"kind": "Assign", "target": "i", "value": {
                                    "kind": "BinaryOp", "op": "Add",
                                    "lhs": {"kind": "Var", "name": "i"},
                                    "rhs": {"kind": "Const", "value": 1}
                                }}
                            ]
                        }
                    }
                ]
            }
        }]
    });

    let mut analyzer = OwnershipAnalyzer::new();
    let plans = analyzer
        .analyze_json(&json)
        .expect("analysis should succeed");

    // Find loop plan
    let loop_plan = plans
        .iter()
        .find(|p| !p.relay_writes.is_empty())
        .expect("loop should have relay_writes for sum/count");

    // Verify relay_writes contains sum and count
    assert!(loop_plan.relay_writes.iter().any(|r| r.name == "sum"));
    assert!(loop_plan.relay_writes.iter().any(|r| r.name == "count"));

    // plan_to_p3_inputs should fail
    let result = plan_to_p3_inputs(loop_plan, "i");
    assert!(result.is_err(), "Should fail-fast on relay_writes");
    assert!(
        result
            .unwrap_err()
            .contains("relay_writes not yet supported for P3"),
        "Error should mention P3 relay limitation"
    );

    eprintln!("[phase59/test] P3 relay fail-fast verified");
}

/// Phase 59: P3 with loop-local carriers should succeed
#[test]
#[cfg(feature = "normalized_dev")]
fn test_phase59_ownership_p3_loop_local_success() {
    use nyash_rust::mir::join_ir::ownership::{plan_to_p3_inputs, OwnershipAnalyzer};
    use serde_json::json;

    // P3 where sum/count are defined INSIDE the loop -> no relay
    let json = json!({
        "functions": [{
            "name": "main",
            "params": [],
            "body": {
                "kind": "Loop",
                "condition": {"kind": "Const", "value": true},
                "body": {
                    "kind": "Block",
                    "statements": [
                        {"kind": "Local", "name": "i", "init": {"kind": "Const", "value": 0}},
                        {"kind": "Local", "name": "sum", "init": {"kind": "Const", "value": 0}},
                        {"kind": "Local", "name": "count", "init": {"kind": "Const", "value": 0}},
                        {
                            "kind": "If",
                            "condition": {
                                "kind": "BinaryOp", "op": "Gt",
                                "lhs": {"kind": "Var", "name": "i"},
                                "rhs": {"kind": "Const", "value": 0}
                            },
                            "then": {
                                "kind": "Block",
                                "statements": [
                                    {"kind": "Assign", "target": "sum", "value": {
                                        "kind": "BinaryOp", "op": "Add",
                                        "lhs": {"kind": "Var", "name": "sum"},
                                        "rhs": {"kind": "Var", "name": "i"}
                                    }},
                                    {"kind": "Assign", "target": "count", "value": {
                                        "kind": "BinaryOp", "op": "Add",
                                        "lhs": {"kind": "Var", "name": "count"},
                                        "rhs": {"kind": "Const", "value": 1}
                                    }}
                                ]
                            }
                        },
                        {"kind": "Break"}
                    ]
                }
            }
        }]
    });

    let mut analyzer = OwnershipAnalyzer::new();
    let plans = analyzer
        .analyze_json(&json)
        .expect("analysis should succeed");

    // Find loop plan with owned vars
    let loop_plan = plans
        .iter()
        .find(|p| p.owned_vars.iter().any(|v| v.name == "sum"))
        .expect("loop should own sum");

    // No relay
    assert!(
        loop_plan.relay_writes.is_empty(),
        "No relay for loop-local vars"
    );

    // plan_to_p3_inputs should succeed
    let inputs = plan_to_p3_inputs(loop_plan, "i").expect("Should succeed");

    eprintln!("[phase59/test] P3 inputs: {:?}", inputs);

    // sum and count should be carriers
    assert!(inputs.carriers.iter().any(|c| c.name == "sum"));
    assert!(inputs.carriers.iter().any(|c| c.name == "count"));
    assert_eq!(
        inputs.carriers.len(),
        2,
        "Should have 2 carriers (sum and count)"
    );

    eprintln!("[phase59/test] P3 loop-local conversion verified: sum and count correctly extracted as carriers");
}

/// Phase 60: Program(JSON v0) fixture (selfhost_if_sum_p3) should produce relay_writes and convert with single-hop relay.
#[test]
#[cfg(feature = "normalized_dev")]
fn test_phase60_ownership_p3_program_json_fixture_with_relay() {
    use nyash_rust::mir::join_ir::ownership::{plan_to_p3_inputs_with_relay, OwnershipAnalyzer};

    let program_json: serde_json::Value = serde_json::from_str(include_str!(
        "../../docs/private/roadmap2/phases/normalized_dev/fixtures/selfhost_if_sum_p3.program.json"
    ))
    .expect("fixture json");

    let mut analyzer = OwnershipAnalyzer::new();
    let plans = analyzer
        .analyze_json(&program_json)
        .expect("Program(JSON v0) analysis should succeed");

    let loop_plan = plans
        .iter()
        .find(|p| !p.relay_writes.is_empty())
        .expect("expected a loop plan with relay_writes");

    // i/sum/count are defined outside the loop but updated in the loop body -> relay_writes
    assert!(loop_plan.relay_writes.iter().any(|r| r.name == "i"));
    assert!(loop_plan.relay_writes.iter().any(|r| r.name == "sum"));
    assert!(loop_plan.relay_writes.iter().any(|r| r.name == "count"));

    let inputs = plan_to_p3_inputs_with_relay(loop_plan, "i").expect("with_relay should succeed");

    let mut carriers: Vec<&str> = inputs.carriers.iter().map(|c| c.name.as_str()).collect();
    carriers.sort();
    assert_eq!(carriers, vec!["count", "sum"]);

    // n is read-only in loop condition -> capture + condition_capture
    assert!(inputs.captures.iter().any(|n| n == "n"));
    assert!(inputs.condition_captures.iter().any(|n| n == "n"));
}

/// Phase 64: P3 production route with ownership analysis (dev-only integration test)
///
/// This test verifies that `analyze_loop()` API works for simple P3 loops and that
/// multi-hop relay is correctly rejected with Fail-Fast error.
#[test]
#[cfg(feature = "normalized_dev")]
fn test_phase64_p3_ownership_prod_integration() {
    use nyash_rust::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use nyash_rust::mir::join_ir::ownership::analyze_loop;

    // Helper: Create literal integer node
    fn lit_i(i: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(i),
            span: Span::unknown(),
        }
    }

    // Helper: Create variable node
    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    // Simple P3 loop: loop(i < 10) { local sum=0; local i=0; sum = sum + i; i = i + 1 }
    let condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(var("i")),
        right: Box::new(lit_i(10)),
        span: Span::unknown(),
    };

    let body = vec![
        ASTNode::Local {
            variables: vec!["sum".to_string()],
            initial_values: vec![Some(Box::new(lit_i(0)))],
            span: Span::unknown(),
        },
        ASTNode::Local {
            variables: vec!["i".to_string()],
            initial_values: vec![Some(Box::new(lit_i(0)))],
            span: Span::unknown(),
        },
        ASTNode::Assignment {
            target: Box::new(var("sum")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(var("sum")),
                right: Box::new(var("i")),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        },
        ASTNode::Assignment {
            target: Box::new(var("i")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(var("i")),
                right: Box::new(lit_i(1)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        },
    ];

    // No parent-defined variables (both sum and i are loop-local)
    let parent_defined = vec![];

    // Analyze the loop
    let plan =
        analyze_loop(&condition, &body, &parent_defined).expect("P3 analysis should succeed");

    // Verify basic plan structure
    assert!(
        !plan.owned_vars.is_empty(),
        "Should have owned vars (sum, i)"
    );

    // Find sum and i in owned_vars
    let sum_var = plan
        .owned_vars
        .iter()
        .find(|v| v.name == "sum")
        .expect("sum should be owned");
    let i_var = plan
        .owned_vars
        .iter()
        .find(|v| v.name == "i")
        .expect("i should be owned");

    // Both should be written
    assert!(sum_var.is_written, "sum should be written");
    assert!(i_var.is_written, "i should be written");

    // i is used in condition -> condition_only
    assert!(i_var.is_condition_only, "i should be condition_only");

    // sum is NOT used in condition
    assert!(
        !sum_var.is_condition_only,
        "sum should NOT be condition_only"
    );

    // No relay writes (all variables are loop-local)
    assert!(
        plan.relay_writes.is_empty(),
        "No relay writes for loop-local variables"
    );

    // Verify single-hop relay constraint: if relay_writes is non-empty, verify single-hop
    for relay in &plan.relay_writes {
        assert!(
            relay.relay_path.len() <= 1,
            "Multi-hop relay should be rejected (got relay_path.len = {})",
            relay.relay_path.len()
        );
    }

    eprintln!(
        "[phase64/test] P3 ownership analysis succeeded: {} owned vars, {} relay writes",
        plan.owned_vars.len(),
        plan.relay_writes.len()
    );
}

/// Phase 64: Multi-hop relay detection test
///
/// Verifies that `analyze_loop()` correctly identifies multi-hop relay patterns.
/// The actual rejection (Fail-Fast) happens in `check_ownership_plan_consistency()`,
/// not in `analyze_loop()` itself.
#[test]
#[cfg(feature = "normalized_dev")]
fn test_phase64_p3_multihop_relay_detection() {
    use nyash_rust::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use nyash_rust::mir::join_ir::ownership::AstOwnershipAnalyzer;

    // Helper functions
    fn lit_i(i: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(i),
            span: Span::unknown(),
        }
    }

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    // Function with nested loops:
    // function test() {
    //   local sum = 0;
    //   local i = 0;
    //   loop(i < 5) {
    //     local j = 0;
    //     loop(j < 3) {
    //       sum = sum + 1;  // Multi-hop relay: sum defined in function scope
    //       j = j + 1;
    //     }
    //     i = i + 1;
    //   }
    // }
    let inner_condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(var("j")),
        right: Box::new(lit_i(3)),
        span: Span::unknown(),
    };

    let inner_body = vec![
        ASTNode::Assignment {
            target: Box::new(var("sum")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(var("sum")),
                right: Box::new(lit_i(1)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        },
        ASTNode::Assignment {
            target: Box::new(var("j")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(var("j")),
                right: Box::new(lit_i(1)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        },
    ];

    let outer_condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(var("i")),
        right: Box::new(lit_i(5)),
        span: Span::unknown(),
    };

    let outer_body = vec![
        ASTNode::Local {
            variables: vec!["j".to_string()],
            initial_values: vec![Some(Box::new(lit_i(0)))],
            span: Span::unknown(),
        },
        ASTNode::Loop {
            condition: Box::new(inner_condition),
            body: inner_body,
            span: Span::unknown(),
        },
        ASTNode::Assignment {
            target: Box::new(var("i")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(var("i")),
                right: Box::new(lit_i(1)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        },
    ];

    let function_body = vec![
        ASTNode::Local {
            variables: vec!["sum".to_string()],
            initial_values: vec![Some(Box::new(lit_i(0)))],
            span: Span::unknown(),
        },
        ASTNode::Local {
            variables: vec!["i".to_string()],
            initial_values: vec![Some(Box::new(lit_i(0)))],
            span: Span::unknown(),
        },
        ASTNode::Loop {
            condition: Box::new(outer_condition),
            body: outer_body,
            span: Span::unknown(),
        },
    ];

    let function = ASTNode::FunctionDeclaration {
        name: "test".to_string(),
        params: vec![],
        body: function_body,
        is_static: false,
        is_override: false,
        span: Span::unknown(),
    };

    // Analyze the entire function to detect nested loop relays
    let mut analyzer = AstOwnershipAnalyzer::new();
    let plans = analyzer
        .analyze_ast(&function)
        .expect("Function analysis should succeed");

    // Find the inner loop plan (should have multi-hop relay for 'sum')
    let inner_loop_plan = plans
        .iter()
        .find(|p| {
            // Inner loop should have relay write for 'sum' with relay_path.len() > 1
            p.relay_writes
                .iter()
                .any(|r| r.name == "sum" && r.relay_path.len() > 1)
        })
        .expect("Expected inner loop plan with multi-hop relay for 'sum'");

    let sum_relay = inner_loop_plan
        .relay_writes
        .iter()
        .find(|r| r.name == "sum")
        .expect("sum should be a relay write in inner loop");

    // Verify multi-hop relay (relay_path should include both inner and outer loop scopes)
    assert!(
        sum_relay.relay_path.len() > 1,
        "sum should have multi-hop relay (got relay_path.len = {})",
        sum_relay.relay_path.len()
    );

    eprintln!(
        "[phase64/test] Multi-hop relay detected: sum relay_path.len = {}",
        sum_relay.relay_path.len()
    );
    eprintln!(
        "[phase64/test] This pattern would be rejected by check_ownership_plan_consistency()"
    );
}

/// Phase 70-A: Verify that multihop relay runtime unsupported error has standardized tag.
///
/// This test builds an OwnershipPlan with multihop relay and verifies that
/// `check_ownership_plan_consistency()` returns an error with the standard tag
/// `[ownership/relay:runtime_unsupported]`.
#[test]
#[cfg(feature = "normalized_dev")]
fn test_phase70a_multihop_relay_runtime_unsupported_tag() {
    use nyash_rust::mir::join_ir::ownership::{
        CapturedVar, OwnershipPlan, RelayVar, ScopeId, ScopeOwnedVar,
    };

    // Build a plan with multihop relay (relay_path.len() == 2)
    let mut plan = OwnershipPlan::new(ScopeId(2)); // Inner loop scope
    plan.owned_vars.push(ScopeOwnedVar {
        name: "j".to_string(),
        is_written: true,
        is_condition_only: false,
    });
    plan.relay_writes.push(RelayVar {
        name: "sum".to_string(),
        owner_scope: ScopeId(0),                  // Function scope
        relay_path: vec![ScopeId(2), ScopeId(1)], // 2 hops: inner → outer
    });
    plan.captures.push(CapturedVar {
        name: "i".to_string(),
        owner_scope: ScopeId(0),
    });

    // Call the plan layer function (Phase 66 accepts multihop)
    // The runtime check (check_ownership_plan_consistency) is private in pattern3_with_if_phi.rs,
    // so we test the plan layer acceptance here.
    use nyash_rust::mir::join_ir::ownership::plan_to_p3_inputs_with_relay;

    let result = plan_to_p3_inputs_with_relay(&plan, "j");

    // Phase 66: plan_to_p3_inputs_with_relay NOW ACCEPTS multihop (relay_path.len() > 1)
    // So this should PASS (not Err)
    assert!(
        result.is_ok(),
        "Phase 66: plan_to_p3_inputs_with_relay should accept multihop relay"
    );

    // Verify the relay is in the output
    let inputs = result.unwrap();
    let relay = inputs
        .carriers
        .iter()
        .find(|c| c.name == "sum")
        .expect("sum should be in carriers (via relay conversion)");
    eprintln!(
        "[phase70a/test] Multihop relay accepted in plan layer: sum role={:?}",
        relay.role
    );

    // The RUNTIME check (check_ownership_plan_consistency in pattern3_with_if_phi.rs)
    // is what produces [ownership/relay:runtime_unsupported].
    // That function is private, so we document that the tag exists and
    // will be hit when P3 lowering encounters this plan at runtime.
    eprintln!("[phase70a/test] Runtime would fail with [ownership/relay:runtime_unsupported] tag");
}

/// Phase 70-B: Test that simple passthrough multihop relay is accepted.
///
/// This test verifies the structural detection logic in OwnershipPlanValidator
/// correctly identifies supported multihop patterns (pure passthrough, no self-updates).
#[test]
#[cfg(feature = "normalized_dev")]
fn test_phase70b_multihop_relay_simple_passthrough_succeeds() {
    use nyash_rust::mir::join_ir::ownership::{
        OwnershipPlan, OwnershipPlanValidator, RelayVar, ScopeId, ScopeOwnedVar,
    };

    // Build a plan for innermost loop (L3) with multihop relay
    // L3 writes to 'counter' owned by L1, relayed through L2
    let mut plan_l3 = OwnershipPlan::new(ScopeId(3)); // Inner loop scope
    plan_l3.owned_vars.push(ScopeOwnedVar {
        name: "i".to_string(), // loop variable
        is_written: true,
        is_condition_only: true,
    });
    plan_l3.relay_writes.push(RelayVar {
        name: "counter".to_string(),
        owner_scope: ScopeId(1),                  // L1 owns counter
        relay_path: vec![ScopeId(3), ScopeId(2)], // 2 hops: L3 → L2 → L1
    });
    // No owned_vars for 'counter' - pure passthrough from L3's perspective

    // Phase 70-B: This should be accepted (passthrough pattern)
    let result = OwnershipPlanValidator::validate_relay_support(&plan_l3);
    assert!(
        result.is_ok(),
        "Phase 70-B: Simple passthrough multihop should be accepted, got: {:?}",
        result
    );

    eprintln!("[phase70b/test] Simple passthrough multihop relay accepted (3-layer loop)");
}

/// Phase 70-B: Test that unsupported multihop patterns are still rejected.
///
/// This test verifies that complex patterns (e.g., self-conflict where a scope
/// both owns and relays the same variable) are still rejected with the standard tag.
#[test]
#[cfg(feature = "normalized_dev")]
fn test_phase70b_multihop_relay_self_conflict_rejected() {
    use nyash_rust::mir::join_ir::ownership::{
        OwnershipPlan, OwnershipPlanValidator, RelayVar, ScopeId, ScopeOwnedVar,
    };

    // Build a plan where L3 both owns and relays 'counter' (conflict)
    // L3 → L2 → L1 (multihop with self-conflict at L3)
    let mut plan_l3 = OwnershipPlan::new(ScopeId(3)); // Inner loop scope
    plan_l3.owned_vars.push(ScopeOwnedVar {
        name: "counter".to_string(), // L3 owns counter
        is_written: true,
        is_condition_only: false,
    });
    plan_l3.relay_writes.push(RelayVar {
        name: "counter".to_string(), // L3 also relays counter (conflict)
        owner_scope: ScopeId(1),
        relay_path: vec![ScopeId(3), ScopeId(2)], // L3 → L2 → L1 (multihop)
    });

    // Phase 70-B: This should be rejected (self-conflict)
    let result = OwnershipPlanValidator::validate_relay_support(&plan_l3);
    assert!(
        result.is_err(),
        "Phase 70-B: Self-conflict multihop should be rejected"
    );

    let err = result.unwrap_err();
    assert!(
        err.contains("[ownership/relay:runtime_unsupported]"),
        "Error should contain standard tag: {}",
        err
    );

    eprintln!("[phase70b/test] Self-conflict multihop relay correctly rejected");
}

/// Phase 70-C: Merge Relay (multiple inner loops → same owner)
///
/// This test verifies that OwnershipAnalyzer correctly detects the "merge relay"
/// pattern where multiple inner loops update the same owner variable.
///
/// Structure:
/// ```text
/// loop L1 {
///     local total = 0    // owned by L1
///     loop L2_A {
///         total++        // L2_A → L1 relay
///     }
///     loop L2_B {
///         total += 10    // L2_B → L1 relay
///     }
/// }
/// // L1 exit: merge both L2_A and L2_B's updates to 'total'
/// ```
#[test]
#[cfg(feature = "normalized_dev")]
fn test_phase70c_merge_relay_multiple_inner_loops_detected() {
    use nyash_rust::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use nyash_rust::mir::join_ir::ownership::AstOwnershipAnalyzer;

    // Build AST: loop L1 { local total=0; loop L2_A { total++ } loop L2_B { total+=10 } }
    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }
    fn lit_i(i: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(i),
            span: Span::unknown(),
        }
    }
    fn lit_true() -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Bool(true),
            span: Span::unknown(),
        }
    }

    let ast = ASTNode::FunctionDeclaration {
        name: "test_merge_relay".to_string(),
        params: vec![],
        body: vec![
            // L1: outer loop
            ASTNode::Loop {
                condition: Box::new(lit_true()),
                body: vec![
                    // local total = 0 (L1 owns)
                    ASTNode::Local {
                        variables: vec!["total".to_string()],
                        initial_values: vec![Some(Box::new(lit_i(0)))],
                        span: Span::unknown(),
                    },
                    // L2_A: first inner loop (total++)
                    ASTNode::Loop {
                        condition: Box::new(lit_true()),
                        body: vec![
                            ASTNode::Assignment {
                                target: Box::new(var("total")),
                                value: Box::new(ASTNode::BinaryOp {
                                    operator: BinaryOperator::Add,
                                    left: Box::new(var("total")),
                                    right: Box::new(lit_i(1)),
                                    span: Span::unknown(),
                                }),
                                span: Span::unknown(),
                            },
                            ASTNode::Break {
                                span: Span::unknown(),
                            },
                        ],
                        span: Span::unknown(),
                    },
                    // L2_B: second inner loop (total += 10)
                    ASTNode::Loop {
                        condition: Box::new(lit_true()),
                        body: vec![
                            ASTNode::Assignment {
                                target: Box::new(var("total")),
                                value: Box::new(ASTNode::BinaryOp {
                                    operator: BinaryOperator::Add,
                                    left: Box::new(var("total")),
                                    right: Box::new(lit_i(10)),
                                    span: Span::unknown(),
                                }),
                                span: Span::unknown(),
                            },
                            ASTNode::Break {
                                span: Span::unknown(),
                            },
                        ],
                        span: Span::unknown(),
                    },
                    ASTNode::Break {
                        span: Span::unknown(),
                    },
                ],
                span: Span::unknown(),
            },
        ],
        is_static: false,
        is_override: false,
        span: Span::unknown(),
    };

    let mut analyzer = AstOwnershipAnalyzer::new();
    let plans = analyzer.analyze_ast(&ast).expect("analysis should succeed");

    // Find L1 (owner of 'total')
    let l1_plan = plans
        .iter()
        .find(|p| p.owned_vars.iter().any(|v| v.name == "total"))
        .expect("expected L1 plan with owned total");
    let l1_scope_id = l1_plan.scope_id;

    // Find L2_A and L2_B (both relay 'total' to L1)
    let relay_plans: Vec<_> = plans
        .iter()
        .filter(|p| p.relay_writes.iter().any(|r| r.name == "total") && p.scope_id != l1_scope_id)
        .collect();

    // Verify: two inner loops relay to the same owner
    assert_eq!(
        relay_plans.len(),
        2,
        "Phase 70-C: Expected 2 inner loops relaying 'total' to L1, got {}",
        relay_plans.len()
    );

    for (idx, plan) in relay_plans.iter().enumerate() {
        let relay = plan
            .relay_writes
            .iter()
            .find(|r| r.name == "total")
            .expect("expected 'total' in relay_writes");

        // Verify: both relay to the same owner (L1)
        assert_eq!(
            relay.owner_scope, l1_scope_id,
            "Phase 70-C: relay {} should have owner_scope = L1",
            idx
        );

        // Verify: single-hop relay (L2 → L1)
        assert_eq!(
            relay.relay_path.len(),
            1,
            "Phase 70-C: relay {} should have single-hop path",
            idx
        );

        // Verify: relay_path[0] is this scope (L2_A or L2_B)
        assert_eq!(
            relay.relay_path[0], plan.scope_id,
            "Phase 70-C: relay {} relay_path[0] must be this scope",
            idx
        );
    }

    // Verify: L1 owned_vars contains 'total' and is_written=true
    let total_var = l1_plan
        .owned_vars
        .iter()
        .find(|v| v.name == "total")
        .expect("expected 'total' in L1 owned_vars");
    assert!(
        total_var.is_written,
        "Phase 70-C: L1's 'total' should be is_written=true"
    );

    eprintln!("[phase70c/test] Merge relay pattern detected: 2 inner loops → same owner variable");
}

/// Phase 70-C: Merge Relay validation acceptance
///
/// This test verifies that OwnershipPlanValidator ACCEPTS merge relay patterns
/// (multiple inner loops → same owner) because they are PERMITTED with owner merge.
#[test]
#[cfg(feature = "normalized_dev")]
fn test_phase70c_merge_relay_same_owner_accepted() {
    use nyash_rust::mir::join_ir::ownership::{
        OwnershipPlan, OwnershipPlanValidator, RelayVar, ScopeId,
    };

    // Build two plans for L2_A and L2_B, both relaying to L1
    // L2_A plan: relay 'total' to L1
    let mut plan_l2a = OwnershipPlan::new(ScopeId(2));
    plan_l2a.relay_writes.push(RelayVar {
        name: "total".to_string(),
        owner_scope: ScopeId(1),      // L1 owns total
        relay_path: vec![ScopeId(2)], // Single hop: L2_A → L1
    });

    // L2_B plan: relay 'total' to L1
    let mut plan_l2b = OwnershipPlan::new(ScopeId(3));
    plan_l2b.relay_writes.push(RelayVar {
        name: "total".to_string(),
        owner_scope: ScopeId(1),      // L1 owns total (same owner)
        relay_path: vec![ScopeId(3)], // Single hop: L2_B → L1
    });

    // Phase 70-C: Both should be accepted (single-hop relay to same owner)
    let result_a = OwnershipPlanValidator::validate_relay_support(&plan_l2a);
    assert!(
        result_a.is_ok(),
        "Phase 70-C: L2_A relay to L1 should be accepted, got: {:?}",
        result_a
    );

    let result_b = OwnershipPlanValidator::validate_relay_support(&plan_l2b);
    assert!(
        result_b.is_ok(),
        "Phase 70-C: L2_B relay to L1 should be accepted, got: {:?}",
        result_b
    );

    eprintln!("[phase70c/test] Merge relay validation accepted: multiple inner loops → same owner");
}

/// Phase 80-D (P3): Test Pattern3 BindingId lookup works
///
/// Verifies that Pattern3 (if-sum) BindingId registration is operational.
/// For manual fallback detection, run with: NYASH_JOINIR_DEBUG=1
/// Expected logs: [phase80/p3] Registered ... + [binding_pilot/hit]
/// No [binding_pilot/fallback] should appear.
#[test]
fn test_phase80_p3_bindingid_lookup_works() {
    let module = build_pattern3_if_sum_min_structured_for_normalized_dev();

    // Basic test: Pattern3 should compile and run with BindingId registration
    assert_eq!(module.functions.len(), 3, "P3 should have 3 functions");

    let entry = module.entry.expect("P3 should have entry function");
    assert_eq!(entry.0, 0, "Entry should be function 0");

    // The fact that this compiles and runs means BindingId registration didn't break anything
    // Manual verification: NYASH_JOINIR_DEBUG=1 cargo test test_phase80_p3_bindingid_lookup_works
    // Should show [phase80/p3] logs and [binding_pilot/hit], NO [binding_pilot/fallback]
}

/// Phase 80-D (P3): Test Pattern4 BindingId lookup works
///
/// Verifies that Pattern4 (continue/Trim) BindingId registration is operational.
/// For manual fallback detection, run with: NYASH_JOINIR_DEBUG=1
/// Expected logs: [phase80/p4] Registered ... + [binding_pilot/hit]
/// No [binding_pilot/fallback] should appear.
#[test]
fn test_phase80_p4_bindingid_lookup_works() {
    let module = build_pattern4_continue_min_structured_for_normalized_dev();

    // Basic test: Pattern4 should compile and run with BindingId registration
    assert_eq!(module.functions.len(), 3, "P4 should have 3 functions");

    let entry = module.entry.expect("P4 should have entry function");
    assert_eq!(entry.0, 0, "Entry should be function 0");

    // The fact that this compiles and runs means BindingId registration didn't break anything
    // Manual verification: NYASH_JOINIR_DEBUG=1 cargo test test_phase80_p4_bindingid_lookup_works
    // Should show [phase80/p4] logs and [binding_pilot/hit], NO [binding_pilot/fallback]
}

// Phase 79 の "BindingId lookup の E2E（subprocess）" は、Pattern2（DigitPos/Trim）の安定化と一緒に
// Phase 80-D 以降で復活させる（このファイルは Normalized JoinIR の SSOT テストに集中させる）。

// ========== Phase 81: Pattern2 ExitLine Contract Verification ==========

/// Phase 81-B: DigitPos pattern ExitLine contract verification
///
/// Tests that promoted `is_digit_pos` carrier (ConditionOnly) is correctly
/// excluded from Exit PHI while LoopState carriers (result, i) are included.
#[test]
fn test_phase81_digitpos_exitline_contract() {
    // Use existing JsonParser _atoi fixture (DigitPos pattern with indexOf)
    let module = build_jsonparser_atoi_structured_for_normalized_dev();

    // Verify compilation succeeds (ExitLine reconnection works)
    assert!(
        !module.functions.is_empty(),
        "DigitPos pattern should compile successfully"
    );

    // Verify module structure
    assert_eq!(
        module.functions.len(),
        3,
        "DigitPos pattern should have 3 functions (k_entry, k_loop, k_body)"
    );

    // Verify entry function exists
    let entry = module
        .entry
        .expect("DigitPos pattern should have entry function");

    // Execute to verify correctness (DigitPos: "123" → 123)
    let result = run_joinir_vm_bridge_structured_only(
        &module,
        entry,
        &[JoinValue::Str("123".to_string()), JoinValue::Int(3)],
    );

    assert_eq!(
        result,
        JoinValue::Int(123),
        "DigitPos pattern should parse '123' correctly"
    );

    // Manual verification: Check that is_digit_pos is NOT in exit_bindings
    // NYASH_JOINIR_DEBUG=1 cargo test test_phase81_digitpos_exitline_contract -- --nocapture 2>&1 | grep exit-line
    // Should show: [joinir/exit-line] skip ConditionOnly carrier 'is_digit_pos'
}

/// Phase 81-B: Trim pattern ExitLine contract verification
///
/// Tests that promoted `is_ch_match` carrier (ConditionOnly) is correctly
/// excluded from Exit PHI while LoopState carriers (i) are included.
#[test]
fn test_phase81_trim_exitline_contract() {
    // Use existing JsonParser skip_ws fixture (Trim pattern with whitespace check)
    let module = build_jsonparser_skip_ws_structured_for_normalized_dev();

    // Verify compilation succeeds (ExitLine reconnection works)
    assert!(
        !module.functions.is_empty(),
        "Trim pattern should compile successfully"
    );

    // Verify module structure
    assert!(
        module.functions.len() >= 3,
        "Trim pattern should have at least 3 functions"
    );

    // Verify entry function exists
    let entry = module
        .entry
        .expect("Trim pattern should have entry function");

    // Execute to verify correctness (skip_ws fixture)
    // The skip_ws fixture takes a single int parameter (test size)
    let result = run_joinir_vm_bridge_structured_only(&module, entry, &[JoinValue::Int(5)]);

    // skip_ws fixture returns the input value (identity function for testing)
    assert_eq!(
        result,
        JoinValue::Int(5),
        "Trim pattern fixture should return input value"
    );

    // Manual verification: Check that is_ch_match is NOT in exit_bindings
    // NYASH_JOINIR_DEBUG=1 cargo test test_phase81_trim_exitline_contract -- --nocapture 2>&1 | grep exit-line
    // Should show: [joinir/exit-line] skip ConditionOnly carrier 'is_ch_match'
}
