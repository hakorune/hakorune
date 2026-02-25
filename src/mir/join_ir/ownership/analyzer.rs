//! Ownership Analyzer - Produces OwnershipPlan from AST/ProgramJSON
//!
//! Phase 57: dev-only analysis, not connected to lowering yet.

use super::*;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

/// Scope kind for ownership analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeKind {
    Function,
    Loop,
    Block,
    If,
}

/// Internal scope representation during analysis
#[derive(Debug)]
struct ScopeInfo {
    id: ScopeId,
    kind: ScopeKind,
    parent: Option<ScopeId>,
    /// Variables defined in this scope
    defined: BTreeSet<String>,
    /// Variables read in this scope (including nested)
    reads: BTreeSet<String>,
    /// Variables written in this scope (including nested)
    writes: BTreeSet<String>,
    /// Variables read in condition expressions
    condition_reads: BTreeSet<String>,
}

/// Analyzes AST/ProgramJSON to produce OwnershipPlan.
pub struct OwnershipAnalyzer {
    scopes: BTreeMap<ScopeId, ScopeInfo>,
    next_scope_id: u32,
}

impl OwnershipAnalyzer {
    pub fn new() -> Self {
        Self {
            scopes: BTreeMap::new(),
            next_scope_id: 0,
        }
    }

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

    fn alloc_scope(&mut self, kind: ScopeKind, parent: Option<ScopeId>) -> ScopeId {
        let id = ScopeId(self.next_scope_id);
        self.next_scope_id += 1;
        self.scopes.insert(
            id,
            ScopeInfo {
                id,
                kind,
                parent,
                defined: BTreeSet::new(),
                reads: BTreeSet::new(),
                writes: BTreeSet::new(),
                condition_reads: BTreeSet::new(),
            },
        );
        id
    }

    fn analyze_function(
        &mut self,
        func: &Value,
        parent: Option<ScopeId>,
    ) -> Result<ScopeId, String> {
        let scope_id = self.alloc_scope(ScopeKind::Function, parent);

        // Collect function parameters as defined
        if let Some(params) = func.get("params").and_then(|p| p.as_array()) {
            for param in params {
                if let Some(name) = param.as_str() {
                    self.scopes
                        .get_mut(&scope_id)
                        .unwrap()
                        .defined
                        .insert(name.to_string());
                }
            }
        }

        // Analyze body
        if let Some(body) = func.get("body") {
            self.analyze_statement(body, scope_id)?;
        }

        Ok(scope_id)
    }

    fn analyze_statement(&mut self, stmt: &Value, current_scope: ScopeId) -> Result<(), String> {
        let kind = stmt
            .get("kind")
            .or_else(|| stmt.get("type"))
            .and_then(|k| k.as_str())
            .unwrap_or("");

        match kind {
            "Local" => {
                // NOTE: Program(JSON v0) historically uses Local for both "new binding"
                // and "rebind/update". This analyzer treats Local as:
                // - definition when the name is not yet defined in scope chain
                // - write when the name is already defined (rebind)
                if let Some(name) = stmt.get("name").and_then(|n| n.as_str()) {
                    if self.is_defined_in_scope_chain(current_scope, name) {
                        self.scopes
                            .get_mut(&current_scope)
                            .unwrap()
                            .writes
                            .insert(name.to_string());
                    } else {
                        // Find enclosing loop (or function) for ownership
                        let owner_scope = self.find_enclosing_loop_or_function(current_scope);
                        self.scopes
                            .get_mut(&owner_scope)
                            .unwrap()
                            .defined
                            .insert(name.to_string());
                    }
                }
                // Analyze initializer if present
                if let Some(init) = stmt.get("init").or_else(|| stmt.get("expr")) {
                    self.analyze_expression(init, current_scope, false)?;
                }
            }
            "Assign" | "Assignment" => {
                if let Some(target) = stmt.get("target").and_then(|t| t.as_str()) {
                    self.scopes
                        .get_mut(&current_scope)
                        .unwrap()
                        .writes
                        .insert(target.to_string());
                }
                if let Some(value) = stmt.get("value").or_else(|| stmt.get("expr")) {
                    self.analyze_expression(value, current_scope, false)?;
                }
            }
            "Loop" => {
                let loop_scope = self.alloc_scope(ScopeKind::Loop, Some(current_scope));

                // Analyze condition (mark as condition_reads)
                if let Some(cond) = stmt.get("condition").or_else(|| stmt.get("cond")) {
                    self.analyze_expression(cond, loop_scope, true)?;
                }

                // Analyze body
                if let Some(body) = stmt.get("body") {
                    self.analyze_statement(body, loop_scope)?;
                }

                // Propagate reads/writes up to parent
                self.propagate_to_parent(loop_scope);
            }
            "If" => {
                let if_scope = self.alloc_scope(ScopeKind::If, Some(current_scope));

                // Analyze condition
                if let Some(cond) = stmt.get("condition").or_else(|| stmt.get("cond")) {
                    self.analyze_expression(cond, if_scope, true)?;
                }

                // Analyze then/else branches
                if let Some(then_branch) = stmt.get("then") {
                    self.analyze_statement(then_branch, if_scope)?;
                }
                if let Some(else_branch) = stmt.get("else") {
                    self.analyze_statement(else_branch, if_scope)?;
                }

                self.propagate_to_parent(if_scope);
            }
            "Block" => {
                let block_scope = self.alloc_scope(ScopeKind::Block, Some(current_scope));

                let stmts = stmt
                    .get("statements")
                    .or_else(|| stmt.get("body"))
                    .and_then(|s| s.as_array());
                if let Some(stmts) = stmts {
                    for s in stmts {
                        self.analyze_statement(s, block_scope)?;
                    }
                }

                self.propagate_to_parent(block_scope);
            }
            "Return" | "Break" | "Continue" => {
                if let Some(value) = stmt.get("value").or_else(|| stmt.get("expr")) {
                    self.analyze_expression(value, current_scope, false)?;
                }
            }
            "ExprStmt" => {
                if let Some(expr) = stmt.get("expr") {
                    self.analyze_expression(expr, current_scope, false)?;
                }
            }
            _ => {
                // Handle array of statements
                if let Some(stmts) = stmt.as_array() {
                    for s in stmts {
                        self.analyze_statement(s, current_scope)?;
                    }
                }
            }
        }

        Ok(())
    }

    fn analyze_expression(
        &mut self,
        expr: &Value,
        current_scope: ScopeId,
        is_condition: bool,
    ) -> Result<(), String> {
        let kind = expr
            .get("kind")
            .or_else(|| expr.get("type"))
            .and_then(|k| k.as_str())
            .unwrap_or("");

        match kind {
            "Var" | "Variable" | "Identifier" => {
                if let Some(name) = expr.get("name").and_then(|n| n.as_str()) {
                    let scope = self.scopes.get_mut(&current_scope).unwrap();
                    scope.reads.insert(name.to_string());
                    if is_condition {
                        scope.condition_reads.insert(name.to_string());
                    }
                }
            }
            "BinaryOp" | "Binary" => {
                if let Some(lhs) = expr.get("lhs").or(expr.get("left")) {
                    self.analyze_expression(lhs, current_scope, is_condition)?;
                }
                if let Some(rhs) = expr.get("rhs").or(expr.get("right")) {
                    self.analyze_expression(rhs, current_scope, is_condition)?;
                }
            }
            "UnaryOp" | "Unary" => {
                if let Some(operand) = expr.get("operand").or(expr.get("expr")) {
                    self.analyze_expression(operand, current_scope, is_condition)?;
                }
            }
            "Call" | "MethodCall" => {
                if let Some(args) = expr.get("args").and_then(|a| a.as_array()) {
                    for arg in args {
                        self.analyze_expression(arg, current_scope, is_condition)?;
                    }
                }
                if let Some(receiver) = expr.get("receiver") {
                    self.analyze_expression(receiver, current_scope, is_condition)?;
                }
            }
            "Index" => {
                if let Some(base) = expr.get("base") {
                    self.analyze_expression(base, current_scope, is_condition)?;
                }
                if let Some(index) = expr.get("index") {
                    self.analyze_expression(index, current_scope, is_condition)?;
                }
            }
            _ => {
                // Recurse into any nested expressions
                if let Some(obj) = expr.as_object() {
                    for (_, v) in obj {
                        if v.is_object() || v.is_array() {
                            self.analyze_expression(v, current_scope, is_condition)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn is_defined_in_scope_chain(&self, from_scope: ScopeId, name: &str) -> bool {
        let mut current = Some(from_scope);
        while let Some(id) = current {
            let scope = match self.scopes.get(&id) {
                Some(scope) => scope,
                None => break,
            };
            if scope.defined.contains(name) {
                return true;
            }
            current = scope.parent;
        }
        false
    }

    /// Find enclosing Loop or Function scope (body-local ownership rule)
    fn find_enclosing_loop_or_function(&self, scope_id: ScopeId) -> ScopeId {
        let scope = &self.scopes[&scope_id];
        match scope.kind {
            ScopeKind::Loop | ScopeKind::Function => scope_id,
            _ => {
                if let Some(parent) = scope.parent {
                    self.find_enclosing_loop_or_function(parent)
                } else {
                    scope_id // Shouldn't happen
                }
            }
        }
    }

    /// Propagate reads/writes from child to parent
    fn propagate_to_parent(&mut self, child_id: ScopeId) {
        let child = &self.scopes[&child_id];
        let reads = child.reads.clone();
        let writes = child.writes.clone();
        let cond_reads = child.condition_reads.clone();
        let child_kind = child.kind;
        let child_defined = child.defined.clone();

        if let Some(parent_id) = child.parent {
            let parent = self.scopes.get_mut(&parent_id).unwrap();
            parent.reads.extend(reads);
            parent.condition_reads.extend(cond_reads);

            // Only propagate writes that are NOT locally owned by Loop/Function children
            // This prevents parent scopes from trying to relay variables owned by children
            if child_kind == ScopeKind::Loop || child_kind == ScopeKind::Function {
                // Don't propagate writes for variables defined in this Loop/Function
                for write in writes {
                    if !child_defined.contains(&write) {
                        parent.writes.insert(write);
                    }
                }
            } else {
                // For If/Block, propagate all writes
                parent.writes.extend(writes);
            }
        }
    }

    /// Build OwnershipPlan for each Loop/Function scope
    fn build_plans(&self) -> Result<Vec<OwnershipPlan>, String> {
        let mut plans = Vec::new();

        for (_, scope) in &self.scopes {
            // Only generate plans for Loop and Function scopes
            if scope.kind != ScopeKind::Loop && scope.kind != ScopeKind::Function {
                continue;
            }

            let mut plan = OwnershipPlan::new(scope.id);

            // owned_vars: defined in this scope
            for name in &scope.defined {
                let is_written = scope.writes.contains(name);
                let is_condition_only = scope.condition_reads.contains(name)
                    && !scope
                        .writes
                        .iter()
                        .any(|w| w == name && !scope.condition_reads.contains(w));

                plan.owned_vars.push(ScopeOwnedVar {
                    name: name.clone(),
                    is_written,
                    is_condition_only: is_condition_only && is_written,
                });
            }

            // relay_writes: written but not owned - find owner
            for name in &scope.writes {
                if scope.defined.contains(name) {
                    continue; // It's owned, not relay
                }

                // Find owner in ancestors
                if let Some((owner_scope, relay_path)) = self.find_owner(scope.id, name) {
                    plan.relay_writes.push(RelayVar {
                        name: name.clone(),
                        owner_scope,
                        relay_path,
                    });
                } else {
                    return Err(format!(
                        "Relay violation: variable '{}' written in scope {:?} has no owner",
                        name, scope.id
                    ));
                }
            }

            // captures: read but not owned (and not relay)
            for name in &scope.reads {
                if scope.defined.contains(name) || scope.writes.contains(name) {
                    continue;
                }

                if let Some((owner_scope, _)) = self.find_owner(scope.id, name) {
                    plan.captures.push(CapturedVar {
                        name: name.clone(),
                        owner_scope,
                    });
                }
                // If no owner found, might be global/builtin - skip
            }

            // condition_captures: captures used in conditions
            for cap in &plan.captures {
                if scope.condition_reads.contains(&cap.name) {
                    plan.condition_captures.push(cap.clone());
                }
            }

            // Verify invariants
            #[cfg(debug_assertions)]
            plan.verify_invariants()?;

            plans.push(plan);
        }

        Ok(plans)
    }

    /// Find owner scope for a variable, returning (owner_id, relay_path)
    fn find_owner(&self, from_scope: ScopeId, name: &str) -> Option<(ScopeId, Vec<ScopeId>)> {
        let mut current = from_scope;
        let mut path = Vec::new();

        loop {
            let scope = &self.scopes[&current];

            if scope.defined.contains(name) {
                return Some((current, path));
            }

            if let Some(parent) = scope.parent {
                // Only Loop scopes are in the relay path
                if scope.kind == ScopeKind::Loop {
                    path.push(current);
                }
                current = parent;
            } else {
                return None; // No owner found
            }
        }
    }
}

impl Default for OwnershipAnalyzer {
    fn default() -> Self {
        Self::new()
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
            "../../../../docs/private/roadmap2/phases/phase-34-joinir-frontend/fixtures/loop_frontend_break.program.json"
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
