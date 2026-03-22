//! Ownership Analyzer - Produces OwnershipPlan from AST/ProgramJSON
//!
//! Phase 57: dev-only analysis, not connected to lowering yet.

mod core;
mod node_analysis;

use super::OwnershipPlan;
pub use core::{OwnershipAnalyzer, ScopeKind};
use serde_json::Value;

impl OwnershipAnalyzer {
    /// Analyze ProgramJSON and return OwnershipPlan for each scope.
    pub fn analyze_json(&mut self, json: &Value) -> Result<Vec<OwnershipPlan>, String> {
        // Reset state
        self.scopes.clear();
        self.next_scope_id = 0;

        // Find functions and analyze each.
        //
        // Supported inputs:
        // - "functions": [...], with statement nodes using "kind" (test schema)
        // - "defs": [...], where FunctionDef nodes contain "params"/"body" (Program(JSON v0))
        if let Some(functions) = json.get("functions").and_then(|f| f.as_array()) {
            for func in functions {
                self.analyze_function(func, None)?;
            }
        } else if let Some(defs) = json.get("defs").and_then(|d| d.as_array()) {
            let mut found = false;
            for def in defs {
                let def_kind = def
                    .get("type")
                    .or_else(|| def.get("kind"))
                    .and_then(|k| k.as_str())
                    .unwrap_or("");
                if def_kind == "FunctionDef" {
                    found = true;
                    self.analyze_function(def, None)?;
                }
            }
            if !found {
                return Err("OwnershipAnalyzer: no FunctionDef found in 'defs'".to_string());
            }
        } else {
            return Err(
                "OwnershipAnalyzer: expected top-level 'functions' or 'defs' array".to_string(),
            );
        }

        // Convert ScopeInfo to OwnershipPlan
        self.build_plans()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_simple_loop_ownership() {
        // loop(i < 10) { sum = sum + i; i = i + 1 }
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
                                "rhs": {"kind": "Const", "value": 10}
                            },
                            "body": {
                                "kind": "Block",
                                "statements": [
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

        let mut analyzer = OwnershipAnalyzer::new();
        let plans = analyzer.analyze_json(&json).unwrap();

        // Should have 2 plans: function + loop
        assert!(plans.len() >= 1);

        // Find the loop plan
        let loop_plan = plans.iter().find(|p| {
            p.relay_writes
                .iter()
                .any(|r| r.name == "sum" || r.name == "i")
        });

        assert!(
            loop_plan.is_some(),
            "Should have a loop plan with relay writes"
        );
        let loop_plan = loop_plan.unwrap();

        // sum and i are written in loop but owned by function -> relay_writes
        assert!(loop_plan.relay_writes.iter().any(|r| r.name == "sum"));
        assert!(loop_plan.relay_writes.iter().any(|r| r.name == "i"));
    }

    #[test]
    fn test_loop_local_carrier() {
        // loop { local count = 0; count = count + 1; break }
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
                            {"kind": "Local", "name": "count", "init": {"kind": "Const", "value": 0}},
                            {
                                "kind": "Assign",
                                "target": "count",
                                "value": {
                                    "kind": "BinaryOp",
                                    "op": "Add",
                                    "lhs": {"kind": "Var", "name": "count"},
                                    "rhs": {"kind": "Const", "value": 1}
                                }
                            },
                            {"kind": "Break"}
                        ]
                    }
                }
            }]
        });

        let mut analyzer = OwnershipAnalyzer::new();
        let plans = analyzer.analyze_json(&json).unwrap();

        // Find loop plan
        let loop_plan = plans
            .iter()
            .find(|p| p.owned_vars.iter().any(|v| v.name == "count"));

        assert!(loop_plan.is_some(), "Loop should own 'count'");
        let loop_plan = loop_plan.unwrap();

        // count is owned AND written -> carrier
        let count_var = loop_plan
            .owned_vars
            .iter()
            .find(|v| v.name == "count")
            .unwrap();
        assert!(count_var.is_written, "count should be marked as written");

        // No relay for count (it's owned)
        assert!(!loop_plan.relay_writes.iter().any(|r| r.name == "count"));
    }

    #[test]
    fn test_capture_read_only() {
        // local limit = 10; loop(i < limit) { ... }
        let json = json!({
            "functions": [{
                "name": "main",
                "params": [],
                "body": {
                    "kind": "Block",
                    "statements": [
                        {"kind": "Local", "name": "limit", "init": {"kind": "Const", "value": 10}},
                        {"kind": "Local", "name": "i", "init": {"kind": "Const", "value": 0}},
                        {
                            "kind": "Loop",
                            "condition": {
                                "kind": "BinaryOp",
                                "op": "Lt",
                                "lhs": {"kind": "Var", "name": "i"},
                                "rhs": {"kind": "Var", "name": "limit"}
                            },
                            "body": {
                                "kind": "Assign",
                                "target": "i",
                                "value": {
                                    "kind": "BinaryOp",
                                    "op": "Add",
                                    "lhs": {"kind": "Var", "name": "i"},
                                    "rhs": {"kind": "Const", "value": 1}
                                }
                            }
                        }
                    ]
                }
            }]
        });

        let mut analyzer = OwnershipAnalyzer::new();
        let plans = analyzer.analyze_json(&json).unwrap();

        // Find loop plan
        let loop_plan = plans
            .iter()
            .find(|p| p.captures.iter().any(|c| c.name == "limit"));

        assert!(loop_plan.is_some(), "Loop should capture 'limit'");
        let loop_plan = loop_plan.unwrap();

        // limit is captured (read-only)
        assert!(loop_plan.captures.iter().any(|c| c.name == "limit"));

        // limit should also be in condition_captures
        assert!(loop_plan
            .condition_captures
            .iter()
            .any(|c| c.name == "limit"));

        // limit is NOT in relay_writes (not written)
        assert!(!loop_plan.relay_writes.iter().any(|r| r.name == "limit"));
    }

    #[test]
    fn test_nested_loop_relay() {
        // local total = 0
        // loop outer { loop inner { total = total + 1 } }
        let json = json!({
            "functions": [{
                "name": "main",
                "params": [],
                "body": {
                    "kind": "Block",
                    "statements": [
                        {"kind": "Local", "name": "total", "init": {"kind": "Const", "value": 0}},
                        {
                            "kind": "Loop",
                            "condition": {"kind": "Const", "value": true},
                            "body": {
                                "kind": "Loop",
                                "condition": {"kind": "Const", "value": true},
                                "body": {
                                    "kind": "Assign",
                                    "target": "total",
                                    "value": {
                                        "kind": "BinaryOp",
                                        "op": "Add",
                                        "lhs": {"kind": "Var", "name": "total"},
                                        "rhs": {"kind": "Const", "value": 1}
                                    }
                                }
                            }
                        }
                    ]
                }
            }]
        });

        let mut analyzer = OwnershipAnalyzer::new();
        let plans = analyzer.analyze_json(&json).unwrap();

        // At least one loop should relay total
        let any_relay = plans
            .iter()
            .any(|p| p.relay_writes.iter().any(|r| r.name == "total"));

        assert!(any_relay, "Some loop should relay 'total' to function");
    }

    #[test]
    fn test_program_json_v0_break_fixture_relay_and_capture() {
        let program_json: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../../docs/private/roadmap2/phases/phase-34-joinir-frontend/fixtures/loop_frontend_break.program.json"
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

        // i/acc are defined outside the loop but rebound inside loop body -> relay_writes
        assert!(loop_plan.relay_writes.iter().any(|r| r.name == "i"));
        assert!(loop_plan.relay_writes.iter().any(|r| r.name == "acc"));

        // n is read-only in loop condition -> capture + condition_capture
        assert!(loop_plan.captures.iter().any(|c| c.name == "n"));
        assert!(loop_plan.condition_captures.iter().any(|c| c.name == "n"));
    }
}
