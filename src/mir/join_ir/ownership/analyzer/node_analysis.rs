use super::{OwnershipAnalyzer, ScopeKind};
use crate::mir::join_ir::ownership::ScopeId;
use serde_json::Value;

impl OwnershipAnalyzer {
    pub(super) fn analyze_statement(
        &mut self,
        stmt: &Value,
        current_scope: ScopeId,
    ) -> Result<(), String> {
        let kind = stmt
            .get("kind")
            .or_else(|| stmt.get("type"))
            .and_then(|k| k.as_str())
            .unwrap_or("");

        match kind {
            "Local" => {
                if let Some(name) = stmt.get("name").and_then(|n| n.as_str()) {
                    if self.is_defined_in_scope_chain(current_scope, name) {
                        self.scopes
                            .get_mut(&current_scope)
                            .unwrap()
                            .writes
                            .insert(name.to_string());
                    } else {
                        let owner_scope = self.find_enclosing_loop_or_function(current_scope);
                        self.scopes
                            .get_mut(&owner_scope)
                            .unwrap()
                            .defined
                            .insert(name.to_string());
                    }
                }
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

                if let Some(cond) = stmt.get("condition").or_else(|| stmt.get("cond")) {
                    self.analyze_expression(cond, loop_scope, true)?;
                }

                if let Some(body) = stmt.get("body") {
                    self.analyze_statement(body, loop_scope)?;
                }

                self.propagate_to_parent(loop_scope);
            }
            "If" => {
                let if_scope = self.alloc_scope(ScopeKind::If, Some(current_scope));

                if let Some(cond) = stmt.get("condition").or_else(|| stmt.get("cond")) {
                    self.analyze_expression(cond, if_scope, true)?;
                }

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
                    for stmt in stmts {
                        self.analyze_statement(stmt, block_scope)?;
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
                if let Some(stmts) = stmt.as_array() {
                    for stmt in stmts {
                        self.analyze_statement(stmt, current_scope)?;
                    }
                }
            }
        }

        Ok(())
    }

    pub(super) fn analyze_expression(
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
                if let Some(obj) = expr.as_object() {
                    for (_, value) in obj {
                        if value.is_object() || value.is_array() {
                            self.analyze_expression(value, current_scope, is_condition)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
