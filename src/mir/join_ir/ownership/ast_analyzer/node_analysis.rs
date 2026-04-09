use crate::ast::ASTNode;
use crate::mir::join_ir::ownership::ScopeKind;

use super::AstOwnershipAnalyzer;

impl AstOwnershipAnalyzer {
    pub(super) fn analyze_node(
        &mut self,
        node: &ASTNode,
        current_scope: crate::mir::join_ir::ownership::ScopeId,
        is_condition: bool,
    ) -> Result<(), String> {
        match node {
            ASTNode::Program { statements, .. } => {
                let block_scope = self.alloc_scope(ScopeKind::Block, Some(current_scope));
                self.push_env();
                let result: Result<(), String> = statements
                    .iter()
                    .try_for_each(|s| self.analyze_node(s, block_scope, false));
                self.pop_env();
                result?;
                self.propagate_to_parent(block_scope);
            }

            ASTNode::BlockExpr {
                prelude_stmts,
                tail_expr,
                ..
            } => {
                let block_scope = self.alloc_scope(ScopeKind::Block, Some(current_scope));
                self.push_env();
                let result: Result<(), String> = (|| {
                    for stmt in prelude_stmts {
                        self.analyze_node(stmt, block_scope, false)?;
                    }
                    self.analyze_node(tail_expr, block_scope, is_condition)?;
                    Ok(())
                })();
                self.pop_env();
                result?;
                self.propagate_to_parent(block_scope);
            }

            ASTNode::FunctionDeclaration { .. } => {
                self.analyze_function_decl(node, Some(current_scope))?;
            }

            ASTNode::EnumDeclaration { .. } => {}

            ASTNode::BoxDeclaration {
                methods,
                constructors,
                ..
            } => {
                for (_, f) in methods {
                    self.analyze_function_decl(f, Some(current_scope))?;
                }
                for (_, f) in constructors {
                    self.analyze_function_decl(f, Some(current_scope))?;
                }
            }

            ASTNode::Local {
                variables,
                initial_values,
                ..
            } => {
                for (name, init) in variables.iter().zip(initial_values.iter()) {
                    if let Some(expr) = init.as_ref() {
                        self.analyze_node(expr, current_scope, false)?;
                    }
                    self.declare_binding(current_scope, name)?;
                }
            }

            ASTNode::Outbox {
                variables,
                initial_values,
                ..
            } => {
                for (name, init) in variables.iter().zip(initial_values.iter()) {
                    if let Some(expr) = init.as_ref() {
                        self.analyze_node(expr, current_scope, false)?;
                    }
                    self.declare_binding(current_scope, name)?;
                }
            }

            ASTNode::Assignment { target, value, .. } => {
                self.record_assignment_target(target, current_scope)?;
                self.analyze_node(value, current_scope, false)?;
            }

            ASTNode::GroupedAssignmentExpr { lhs, rhs, .. } => {
                let binding = self.resolve_binding(lhs).ok_or_else(|| {
                    format!("AstOwnershipAnalyzer: write to undefined var '{}'", lhs)
                })?;
                self.record_write(binding, current_scope);
                self.analyze_node(rhs, current_scope, is_condition)?;
            }

            ASTNode::Nowait {
                variable,
                expression,
                ..
            } => {
                let binding = self.resolve_binding(variable).ok_or_else(|| {
                    format!(
                        "AstOwnershipAnalyzer: write to undefined var '{}'",
                        variable
                    )
                })?;
                self.record_write(binding, current_scope);
                self.analyze_node(expression, current_scope, false)?;
            }

            ASTNode::Print { expression, .. } => {
                self.analyze_node(expression, current_scope, false)?;
            }

            ASTNode::Return { value, .. } => {
                if let Some(v) = value.as_ref() {
                    self.analyze_node(v, current_scope, false)?;
                }
            }

            ASTNode::Break { .. } | ASTNode::Continue { .. } => {}

            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                let if_scope = self.alloc_scope(ScopeKind::If, Some(current_scope));
                self.analyze_node(condition, if_scope, true)?;

                let then_scope = self.alloc_scope(ScopeKind::Block, Some(if_scope));
                self.push_env();
                let result: Result<(), String> = then_body
                    .iter()
                    .try_for_each(|s| self.analyze_node(s, then_scope, false));
                self.pop_env();
                result?;
                self.propagate_to_parent(then_scope);

                if let Some(else_body) = else_body {
                    let else_scope = self.alloc_scope(ScopeKind::Block, Some(if_scope));
                    self.push_env();
                    let result: Result<(), String> = else_body
                        .iter()
                        .try_for_each(|s| self.analyze_node(s, else_scope, false));
                    self.pop_env();
                    result?;
                    self.propagate_to_parent(else_scope);
                }

                self.propagate_to_parent(if_scope);
            }

            ASTNode::Loop {
                condition, body, ..
            }
            | ASTNode::While {
                condition, body, ..
            } => {
                let loop_scope = self.alloc_scope(ScopeKind::Loop, Some(current_scope));
                self.push_env();
                let result: Result<(), String> = (|| {
                    for s in body {
                        self.analyze_node(s, loop_scope, false)?;
                    }
                    self.analyze_node(condition, loop_scope, true)?;
                    Ok(())
                })();
                self.pop_env();
                result?;
                self.propagate_to_parent(loop_scope);
            }

            ASTNode::ForRange {
                var_name,
                start,
                end,
                body,
                ..
            } => {
                let loop_scope = self.alloc_scope(ScopeKind::Loop, Some(current_scope));
                self.push_env();
                let result: Result<(), String> = (|| {
                    self.declare_binding(loop_scope, var_name)?;
                    self.analyze_node(start, loop_scope, true)?;
                    self.analyze_node(end, loop_scope, true)?;
                    for s in body {
                        self.analyze_node(s, loop_scope, false)?;
                    }
                    Ok(())
                })();
                self.pop_env();
                result?;
                self.propagate_to_parent(loop_scope);
            }

            ASTNode::ScopeBox { body, .. } => {
                let block_scope = self.alloc_scope(ScopeKind::Block, Some(current_scope));
                self.push_env();
                let result: Result<(), String> = body
                    .iter()
                    .try_for_each(|s| self.analyze_node(s, block_scope, false));
                self.pop_env();
                result?;
                self.propagate_to_parent(block_scope);
            }

            ASTNode::TryCatch {
                try_body,
                catch_clauses,
                finally_body,
                ..
            } => {
                let try_scope = self.alloc_scope(ScopeKind::Block, Some(current_scope));
                self.push_env();
                let result: Result<(), String> = try_body
                    .iter()
                    .try_for_each(|s| self.analyze_node(s, try_scope, false));
                self.pop_env();
                result?;
                self.propagate_to_parent(try_scope);

                for clause in catch_clauses {
                    let catch_scope = self.alloc_scope(ScopeKind::Block, Some(current_scope));
                    self.push_env();
                    if let Some(var) = clause.variable_name.as_ref() {
                        self.declare_binding(catch_scope, var)?;
                    }
                    let result: Result<(), String> = clause
                        .body
                        .iter()
                        .try_for_each(|s| self.analyze_node(s, catch_scope, false));
                    self.pop_env();
                    result?;
                    self.propagate_to_parent(catch_scope);
                }

                if let Some(finally_body) = finally_body {
                    let finally_scope = self.alloc_scope(ScopeKind::Block, Some(current_scope));
                    self.push_env();
                    let result: Result<(), String> = finally_body
                        .iter()
                        .try_for_each(|s| self.analyze_node(s, finally_scope, false));
                    self.pop_env();
                    result?;
                    self.propagate_to_parent(finally_scope);
                }
            }

            ASTNode::Throw { expression, .. } => {
                self.analyze_node(expression, current_scope, false)?;
            }

            ASTNode::UsingStatement { .. } | ASTNode::ImportStatement { .. } => {}

            ASTNode::GlobalVar { value, .. } => {
                self.analyze_node(value, current_scope, false)?;
            }

            ASTNode::Literal { .. }
            | ASTNode::This { .. }
            | ASTNode::Me { .. }
            | ASTNode::ThisField { .. }
            | ASTNode::MeField { .. } => {}

            ASTNode::Variable { name, .. } => {
                if let Some(binding) = self.resolve_binding(name) {
                    self.record_read(binding, current_scope, is_condition);
                }
            }

            ASTNode::UnaryOp { operand, .. } => {
                self.analyze_node(operand, current_scope, is_condition)?;
            }

            ASTNode::BinaryOp { left, right, .. } => {
                self.analyze_node(left, current_scope, is_condition)?;
                self.analyze_node(right, current_scope, is_condition)?;
            }

            ASTNode::MethodCall {
                object, arguments, ..
            } => {
                self.analyze_node(object, current_scope, is_condition)?;
                for a in arguments {
                    self.analyze_node(a, current_scope, is_condition)?;
                }
            }

            ASTNode::FieldAccess { object, .. } => {
                self.analyze_node(object, current_scope, is_condition)?;
            }

            ASTNode::Index { target, index, .. } => {
                self.analyze_node(target, current_scope, is_condition)?;
                self.analyze_node(index, current_scope, is_condition)?;
            }

            ASTNode::New { arguments, .. } => {
                for a in arguments {
                    self.analyze_node(a, current_scope, is_condition)?;
                }
            }

            ASTNode::FromCall { arguments, .. } => {
                for a in arguments {
                    self.analyze_node(a, current_scope, is_condition)?;
                }
            }

            ASTNode::FunctionCall { arguments, .. } => {
                for a in arguments {
                    self.analyze_node(a, current_scope, is_condition)?;
                }
            }

            ASTNode::Call {
                callee, arguments, ..
            } => {
                self.analyze_node(callee, current_scope, is_condition)?;
                for a in arguments {
                    self.analyze_node(a, current_scope, is_condition)?;
                }
            }

            ASTNode::ArrayLiteral { elements, .. } => {
                for e in elements {
                    self.analyze_node(e, current_scope, is_condition)?;
                }
            }

            ASTNode::MapLiteral { entries, .. } => {
                for (_, v) in entries {
                    self.analyze_node(v, current_scope, is_condition)?;
                }
            }

            ASTNode::MatchExpr {
                scrutinee,
                arms,
                else_expr,
                ..
            } => {
                self.analyze_node(scrutinee, current_scope, is_condition)?;
                for (_, e) in arms {
                    self.analyze_node(e, current_scope, is_condition)?;
                }
                self.analyze_node(else_expr, current_scope, is_condition)?;
            }

            ASTNode::EnumMatchExpr {
                scrutinee,
                arms,
                else_expr,
                ..
            } => {
                self.analyze_node(scrutinee, current_scope, is_condition)?;
                for arm in arms {
                    self.analyze_node(&arm.body, current_scope, is_condition)?;
                }
                if let Some(else_expr) = else_expr {
                    self.analyze_node(else_expr, current_scope, is_condition)?;
                }
            }

            ASTNode::Lambda { .. } => {}

            ASTNode::Arrow {
                sender, receiver, ..
            } => {
                self.analyze_node(sender, current_scope, is_condition)?;
                self.analyze_node(receiver, current_scope, is_condition)?;
            }

            ASTNode::AwaitExpression { expression, .. }
            | ASTNode::QMarkPropagate { expression, .. } => {
                self.analyze_node(expression, current_scope, is_condition)?;
            }
        }

        Ok(())
    }

    fn record_assignment_target(
        &mut self,
        target: &ASTNode,
        current_scope: crate::mir::join_ir::ownership::ScopeId,
    ) -> Result<(), String> {
        match target {
            ASTNode::Variable { name, .. } => {
                let binding = self.resolve_binding(name).ok_or_else(|| {
                    format!("AstOwnershipAnalyzer: write to undefined var '{}'", name)
                })?;
                self.record_write(binding, current_scope);
            }
            _ => {
                // For complex lvalues (field/index), conservatively treat subexpressions as reads.
                self.analyze_node(target, current_scope, false)?;
            }
        }
        Ok(())
    }
}
