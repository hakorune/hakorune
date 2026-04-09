use crate::ast::ASTNode;
use std::collections::HashSet;

#[allow(dead_code)]
pub(in crate::mir::builder) fn collect_free_vars(
    node: &ASTNode,
    used: &mut HashSet<String>,
    locals: &mut HashSet<String>,
) {
    let mut collector = FreeVarCollector::new(used, locals);
    collector.walk(node, true);
    *locals = collector.take_root_locals();
}

struct FreeVarCollector<'a> {
    used: &'a mut HashSet<String>,
    root_locals: HashSet<String>,
    scope_stack: Vec<HashSet<String>>,
}

impl<'a> FreeVarCollector<'a> {
    fn new(used: &'a mut HashSet<String>, root_locals: &HashSet<String>) -> Self {
        Self {
            used,
            root_locals: root_locals.clone(),
            scope_stack: vec![root_locals.clone()],
        }
    }

    fn take_root_locals(self) -> HashSet<String> {
        self.root_locals
    }

    fn is_declared(&self, name: &str) -> bool {
        self.scope_stack.iter().rev().any(|s| s.contains(name))
    }

    fn declare_in_current_scope(&mut self, name: &str) {
        if let Some(scope) = self.scope_stack.last_mut() {
            scope.insert(name.to_string());
        }
        if self.scope_stack.len() == 1 {
            self.root_locals.insert(name.to_string());
        }
    }

    fn with_child_scope(&mut self, f: impl FnOnce(&mut Self)) {
        self.scope_stack.push(HashSet::new());
        f(self);
        self.scope_stack.pop();
    }

    fn walk_block(&mut self, statements: &[ASTNode]) {
        for st in statements {
            self.walk(st, false);
        }
    }

    fn walk(&mut self, node: &ASTNode, is_root: bool) {
        match node {
            ASTNode::Variable { name, .. } => {
                if name != "me" && name != "this" && !self.is_declared(name) {
                    self.used.insert(name.clone());
                }
            }
            ASTNode::Local {
                variables,
                initial_values,
                ..
            } => {
                for init in initial_values {
                    if let Some(expr) = init {
                        self.walk(expr, false);
                    }
                }
                for v in variables {
                    self.declare_in_current_scope(v);
                }
            }
            ASTNode::Outbox {
                variables,
                initial_values,
                ..
            } => {
                for init in initial_values {
                    if let Some(expr) = init {
                        self.walk(expr, false);
                    }
                }
                for v in variables {
                    self.declare_in_current_scope(v);
                }
            }
            ASTNode::Assignment { target, value, .. } => {
                self.walk(target, false);
                self.walk(value, false);
            }
            ASTNode::GroupedAssignmentExpr { rhs, .. } => {
                self.walk(rhs, false);
            }
            ASTNode::BinaryOp { left, right, .. } => {
                self.walk(left, false);
                self.walk(right, false);
            }
            ASTNode::UnaryOp { operand, .. } => {
                self.walk(operand, false);
            }
            ASTNode::MethodCall {
                object, arguments, ..
            } => {
                self.walk(object, false);
                for a in arguments {
                    self.walk(a, false);
                }
            }
            ASTNode::FunctionCall { arguments, .. } => {
                for a in arguments {
                    self.walk(a, false);
                }
            }
            ASTNode::Call {
                callee, arguments, ..
            } => {
                self.walk(callee, false);
                for a in arguments {
                    self.walk(a, false);
                }
            }
            ASTNode::FieldAccess { object, .. } => {
                self.walk(object, false);
            }
            ASTNode::Index { target, index, .. } => {
                self.walk(target, false);
                self.walk(index, false);
            }
            ASTNode::New { arguments, .. } => {
                for a in arguments {
                    self.walk(a, false);
                }
            }
            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                self.walk(condition, false);
                self.with_child_scope(|this| this.walk_block(then_body));
                if let Some(eb) = else_body {
                    self.with_child_scope(|this| this.walk_block(eb));
                }
            }
            ASTNode::Loop {
                condition, body, ..
            } => {
                self.walk(condition, false);
                self.with_child_scope(|this| this.walk_block(body));
            }
            ASTNode::TryCatch {
                try_body,
                catch_clauses,
                finally_body,
                ..
            } => {
                self.with_child_scope(|this| this.walk_block(try_body));
                for c in catch_clauses {
                    self.with_child_scope(|this| this.walk_block(&c.body));
                }
                if let Some(fb) = finally_body {
                    self.with_child_scope(|this| this.walk_block(fb));
                }
            }
            ASTNode::Throw { expression, .. } => {
                self.walk(expression, false);
            }
            ASTNode::Print { expression, .. } => {
                self.walk(expression, false);
            }
            ASTNode::Return { value, .. } => {
                if let Some(v) = value {
                    self.walk(v, false);
                }
            }
            ASTNode::AwaitExpression { expression, .. } => {
                self.walk(expression, false);
            }
            ASTNode::MatchExpr {
                scrutinee,
                arms,
                else_expr,
                ..
            } => {
                self.walk(scrutinee, false);
                for (_, e) in arms {
                    self.walk(e, false);
                }
                self.walk(else_expr, false);
            }
            ASTNode::EnumMatchExpr {
                scrutinee,
                arms,
                else_expr,
                ..
            } => {
                self.walk(scrutinee, false);
                for arm in arms {
                    self.walk(&arm.body, false);
                }
                if let Some(else_expr) = else_expr {
                    self.walk(else_expr, false);
                }
            }
            ASTNode::Program { statements, .. } => {
                if is_root {
                    self.walk_block(statements);
                } else {
                    self.with_child_scope(|this| this.walk_block(statements));
                }
            }
            ASTNode::ScopeBox { body, .. } => {
                if is_root {
                    self.walk_block(body);
                } else {
                    self.with_child_scope(|this| this.walk_block(body));
                }
            }
            ASTNode::FunctionDeclaration { params, body, .. } => {
                self.with_child_scope(|this| {
                    for p in params {
                        this.declare_in_current_scope(p);
                    }
                    this.walk_block(body);
                });
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::collect_free_vars;
    use crate::ast::ASTNode;
    use crate::ast::Span;
    use std::collections::HashSet;

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn local(name: &str) -> ASTNode {
        ASTNode::Local {
            variables: vec![name.to_string()],
            initial_values: vec![None],
            span: Span::unknown(),
        }
    }

    fn block(statements: Vec<ASTNode>) -> ASTNode {
        ASTNode::Program {
            statements,
            span: Span::unknown(),
        }
    }

    fn scopebox(statements: Vec<ASTNode>) -> ASTNode {
        ASTNode::ScopeBox {
            body: statements,
            span: Span::unknown(),
        }
    }

    #[test]
    fn block_local_does_not_leak_to_outer_scope() {
        let node = block(vec![block(vec![local("y")]), var("y")]);
        let mut used = HashSet::new();
        let mut locals = HashSet::new();
        collect_free_vars(&node, &mut used, &mut locals);
        assert!(used.contains("y"));
    }

    #[test]
    fn scopebox_local_does_not_leak_to_outer_scope() {
        let node = block(vec![scopebox(vec![local("y")]), var("y")]);
        let mut used = HashSet::new();
        let mut locals = HashSet::new();
        collect_free_vars(&node, &mut used, &mut locals);
        assert!(used.contains("y"));
    }

    #[test]
    fn local_initializer_is_counted_as_read_before_declare() {
        let node = block(vec![ASTNode::Local {
            variables: vec!["x".to_string()],
            initial_values: vec![Some(Box::new(var("y")))],
            span: Span::unknown(),
        }]);
        let mut used = HashSet::new();
        let mut locals = HashSet::new();
        collect_free_vars(&node, &mut used, &mut locals);
        assert!(used.contains("y"));
        assert!(locals.contains("x"));
    }
}
