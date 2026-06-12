use crate::ast::ASTNode;

impl ASTNode {
    /// Visit direct AST children in source order.
    ///
    /// This is the local traversal SSOT for generic recursive predicates. Callers
    /// that need scope boundaries or loop-depth changes should handle those
    /// variants before delegating here.
    pub fn for_each_child<'a>(&'a self, visitor: &mut impl FnMut(&'a ASTNode)) {
        match self {
            ASTNode::Program { statements, .. }
            | ASTNode::ScopeBox {
                body: statements, ..
            }
            | ASTNode::TaskScope {
                body: statements, ..
            }
            | ASTNode::FastMemRegion {
                body: statements, ..
            } => {
                for statement in statements {
                    visitor(statement);
                }
            }
            ASTNode::ContextScope { value, body, .. } => {
                visitor(value);
                for statement in body {
                    visitor(statement);
                }
            }
            ASTNode::Assignment { target, value, .. } => {
                visitor(target);
                visitor(value);
            }
            ASTNode::Print { expression, .. }
            | ASTNode::Nowait { expression, .. }
            | ASTNode::AwaitExpression { expression, .. }
            | ASTNode::QMarkPropagate { expression, .. }
            | ASTNode::Throw { expression, .. } => visitor(expression),
            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                visitor(condition);
                for statement in then_body {
                    visitor(statement);
                }
                if let Some(else_body) = else_body {
                    for statement in else_body {
                        visitor(statement);
                    }
                }
            }
            ASTNode::Loop {
                condition, body, ..
            } => {
                visitor(condition);
                for statement in body {
                    visitor(statement);
                }
            }
            ASTNode::LoopRange {
                start, end, body, ..
            } => {
                visitor(start);
                visitor(end);
                for statement in body {
                    visitor(statement);
                }
            }
            ASTNode::Return { value, .. } => {
                if let Some(value) = value {
                    visitor(value);
                }
            }
            ASTNode::BuildGate {
                then_items,
                else_items,
                ..
            } => {
                for item in then_items {
                    visitor(item);
                }
                if let Some(else_items) = else_items {
                    for item in else_items {
                        visitor(item);
                    }
                }
            }
            ASTNode::BoxDeclaration {
                methods,
                constructors,
                static_init,
                invariants,
                ..
            } => {
                for invariant in invariants {
                    visitor(invariant);
                }
                if let Some(static_init) = static_init {
                    for statement in static_init {
                        visitor(statement);
                    }
                }
                for method in methods.values() {
                    visitor(method);
                }
                for constructor in constructors.values() {
                    visitor(constructor);
                }
            }
            ASTNode::FunctionDeclaration {
                body, contracts, ..
            } => {
                for contract in contracts {
                    visitor(&contract.condition);
                }
                for statement in body {
                    visitor(statement);
                }
            }
            ASTNode::Lambda { body, .. } => {
                for statement in body {
                    visitor(statement);
                }
            }
            ASTNode::GlobalVar { value, .. } => visitor(value),
            ASTNode::UnaryOp { operand, .. } => visitor(operand),
            ASTNode::BinaryOp { left, right, .. } => {
                visitor(left);
                visitor(right);
            }
            ASTNode::CheckExpr { items, .. } => {
                for item in items {
                    visitor(&item.expression);
                }
            }
            ASTNode::GroupedAssignmentExpr { rhs, .. } => visitor(rhs),
            ASTNode::MethodCall {
                object, arguments, ..
            } => {
                visitor(object);
                for argument in arguments {
                    visitor(argument);
                }
            }
            ASTNode::FieldAccess { object, .. } => visitor(object),
            ASTNode::Index { target, index, .. } => {
                visitor(target);
                visitor(index);
            }
            ASTNode::New {
                arguments,
                field_initializers,
                ..
            } => {
                for argument in arguments {
                    visitor(argument);
                }
                for (_, initializer) in field_initializers {
                    visitor(initializer);
                }
            }
            ASTNode::FromCall { arguments, .. } | ASTNode::FunctionCall { arguments, .. } => {
                for argument in arguments {
                    visitor(argument);
                }
            }
            ASTNode::Call {
                callee, arguments, ..
            } => {
                visitor(callee);
                for argument in arguments {
                    visitor(argument);
                }
            }
            ASTNode::MatchExpr {
                scrutinee,
                arms,
                else_expr,
                ..
            } => {
                visitor(scrutinee);
                for (_, arm_expr) in arms {
                    visitor(arm_expr);
                }
                visitor(else_expr);
            }
            ASTNode::EnumMatchExpr {
                scrutinee,
                arms,
                else_expr,
                ..
            } => {
                visitor(scrutinee);
                for arm in arms {
                    visitor(&arm.body);
                }
                if let Some(else_expr) = else_expr {
                    visitor(else_expr);
                }
            }
            ASTNode::ArrayLiteral { elements, .. } => {
                for element in elements {
                    visitor(element);
                }
            }
            ASTNode::MapLiteral { entries, .. } => {
                for (_, value) in entries {
                    visitor(value);
                }
            }
            ASTNode::RecordLiteral { fields, .. } => {
                for (_, value) in fields {
                    visitor(value);
                }
            }
            ASTNode::RecordUpdate { base, updates, .. } => {
                visitor(base);
                for (_, value) in updates {
                    visitor(value);
                }
            }
            ASTNode::BlockExpr {
                prelude_stmts,
                tail_expr,
                ..
            } => {
                for statement in prelude_stmts {
                    visitor(statement);
                }
                visitor(tail_expr);
            }
            ASTNode::Arrow {
                sender, receiver, ..
            } => {
                visitor(sender);
                visitor(receiver);
            }
            ASTNode::TryCatch {
                try_body,
                catch_clauses,
                finally_body,
                ..
            } => {
                for statement in try_body {
                    visitor(statement);
                }
                for clause in catch_clauses {
                    for statement in &clause.body {
                        visitor(statement);
                    }
                }
                if let Some(finally_body) = finally_body {
                    for statement in finally_body {
                        visitor(statement);
                    }
                }
            }
            ASTNode::Local { initial_values, .. } | ASTNode::Outbox { initial_values, .. } => {
                for value in initial_values.iter().filter_map(|value| value.as_deref()) {
                    visitor(value);
                }
            }
            ASTNode::Literal { .. }
            | ASTNode::Variable { .. }
            | ASTNode::Break { .. }
            | ASTNode::Continue { .. }
            | ASTNode::UsingStatement { .. }
            | ASTNode::ImportStatement { .. }
            | ASTNode::EnumDeclaration { .. }
            | ASTNode::BrandDeclaration { .. }
            | ASTNode::TypeAliasDeclaration { .. }
            | ASTNode::StaticConstTable { .. }
            | ASTNode::This { .. }
            | ASTNode::Me { .. }
            | ASTNode::ThisField { .. }
            | ASTNode::MeField { .. } => {}
        }
    }

    pub fn any_child(&self, mut predicate: impl FnMut(&ASTNode) -> bool) -> bool {
        let mut found = false;
        self.for_each_child(&mut |child| {
            if !found && predicate(child) {
                found = true;
            }
        });
        found
    }
}
