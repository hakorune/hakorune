//! Utility helpers for Nyash AST nodes extracted from `ast.rs`.

use super::{ASTNode, Span};
use std::fmt;

mod classify;
mod node_type;

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
            } => {
                for statement in statements {
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
            ASTNode::New { arguments, .. }
            | ASTNode::FromCall { arguments, .. }
            | ASTNode::FunctionCall { arguments, .. } => {
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

    /// AST nodeの詳細情報を取得 (デバッグ用)
    pub fn info(&self) -> String {
        match self {
            ASTNode::Program { statements, .. } => {
                format!("Program({} statements)", statements.len())
            }
            ASTNode::Assignment { target, .. } => {
                format!("Assignment(target: {})", target.info())
            }
            ASTNode::Print { .. } => "Print".to_string(),
            ASTNode::If { .. } => "If".to_string(),
            ASTNode::Loop {
                condition: _, body, ..
            } => {
                format!("Loop({} statements)", body.len())
            }
            ASTNode::LoopRange {
                var_name,
                start: _,
                end: _,
                body,
                ..
            } => {
                format!("LoopRange(var={}, {} statements)", var_name, body.len())
            }
            ASTNode::Return { value, .. } => {
                if value.is_some() {
                    "Return(with value)".to_string()
                } else {
                    "Return(void)".to_string()
                }
            }
            ASTNode::Break { .. } => "Break".to_string(),
            ASTNode::Continue { .. } => "Continue".to_string(),
            ASTNode::UsingStatement { namespace_name, .. } => {
                format!("UsingStatement({})", namespace_name)
            }
            ASTNode::ImportStatement { path, alias, .. } => {
                if let Some(a) = alias {
                    format!("ImportStatement({}, as {})", path, a)
                } else {
                    format!("ImportStatement({})", path)
                }
            }
            ASTNode::BoxDeclaration {
                name,
                fields,
                methods,
                constructors,
                is_interface,
                is_record,
                extends,
                implements,
                ..
            } => {
                let mut desc = if *is_record {
                    format!("RecordDeclaration({}, {} fields", name, fields.len())
                } else if *is_interface {
                    format!("InterfaceBox({}, {} methods", name, methods.len())
                } else {
                    format!(
                        "BoxDeclaration({}, {} fields, {} methods, {} constructors",
                        name,
                        fields.len(),
                        methods.len(),
                        constructors.len()
                    )
                };

                if !extends.is_empty() {
                    desc.push_str(&format!(", extends [{}]", extends.join(", ")));
                }

                if !implements.is_empty() {
                    desc.push_str(&format!(", implements [{}]", implements.join(", ")));
                }

                desc.push(')');
                desc
            }
            ASTNode::EnumDeclaration {
                name,
                variants,
                type_parameters,
                ..
            } => {
                if type_parameters.is_empty() {
                    format!("EnumDeclaration({}, {} variants)", name, variants.len())
                } else {
                    format!(
                        "EnumDeclaration({}<{}>, {} variants)",
                        name,
                        type_parameters.join(", "),
                        variants.len()
                    )
                }
            }
            ASTNode::BrandDeclaration {
                name,
                underlying_type_name,
                ..
            } => format!("BrandDeclaration({}: {})", name, underlying_type_name),
            ASTNode::TypeAliasDeclaration {
                name,
                target_type_name,
                ..
            } => format!("TypeAliasDeclaration({} = {})", name, target_type_name),
            ASTNode::FunctionDeclaration {
                name,
                params,
                body,
                is_static,
                is_override,
                ..
            } => {
                let static_str = if *is_static { "static " } else { "" };
                let override_str = if *is_override { "override " } else { "" };
                format!(
                    "FunctionDeclaration({}{}{}({}), {} statements)",
                    override_str,
                    static_str,
                    name,
                    params.join(", "),
                    body.len()
                )
            }
            ASTNode::GlobalVar { name, .. } => {
                format!("GlobalVar({})", name)
            }
            ASTNode::StaticConstTable {
                name,
                element_type,
                values,
                ..
            } => {
                format!(
                    "StaticConstTable({}: {}[{}])",
                    name,
                    element_type,
                    values.len()
                )
            }
            ASTNode::Literal { .. } => "Literal".to_string(),
            ASTNode::Variable { name, .. } => {
                format!("Variable({})", name)
            }
            ASTNode::UnaryOp { operator, .. } => {
                format!("UnaryOp({})", operator)
            }
            ASTNode::BinaryOp { operator, .. } => {
                format!("BinaryOp({})", operator)
            }
            ASTNode::CheckExpr { name, items, .. } => {
                let name = name.as_deref().unwrap_or("<anonymous>");
                format!("CheckExpr({}, {} items)", name, items.len())
            }
            ASTNode::MethodCall {
                method, arguments, ..
            } => {
                format!("MethodCall({}, {} args)", method, arguments.len())
            }
            ASTNode::FieldAccess { field, .. } => {
                format!("FieldAccess({})", field)
            }
            ASTNode::New {
                class,
                arguments,
                type_arguments,
                ..
            } => {
                if type_arguments.is_empty() {
                    format!("New({}, {} args)", class, arguments.len())
                } else {
                    format!(
                        "New({}<{}>, {} args)",
                        class,
                        type_arguments.join(", "),
                        arguments.len()
                    )
                }
            }
            ASTNode::This { .. } => "This".to_string(),
            ASTNode::Me { .. } => "Me".to_string(),
            ASTNode::FromCall {
                parent,
                method,
                arguments,
                ..
            } => {
                format!("FromCall({}.{}, {} args)", parent, method, arguments.len())
            }
            ASTNode::ThisField { field, .. } => {
                format!("ThisField({})", field)
            }
            ASTNode::MeField { field, .. } => {
                format!("MeField({})", field)
            }

            ASTNode::Local { variables, .. } => {
                format!("Local({})", variables.join(", "))
            }
            ASTNode::Outbox { variables, .. } => {
                format!("Outbox({})", variables.join(", "))
            }
            ASTNode::FunctionCall {
                name, arguments, ..
            } => {
                format!("FunctionCall({}, {} args)", name, arguments.len())
            }
            ASTNode::Call { .. } => "Call".to_string(),
            ASTNode::Nowait { variable, .. } => {
                format!("Nowait({})", variable)
            }
            ASTNode::Arrow { .. } => "Arrow(>>)".to_string(),
            ASTNode::TryCatch {
                try_body,
                catch_clauses,
                finally_body,
                ..
            } => {
                let mut desc = format!(
                    "TryCatch({} try statements, {} catch clauses",
                    try_body.len(),
                    catch_clauses.len()
                );
                if finally_body.is_some() {
                    desc.push_str(", has finally");
                }
                desc.push(')');
                desc
            }
            ASTNode::Throw { .. } => "Throw".to_string(),
            ASTNode::AwaitExpression { expression, .. } => {
                format!("Await({:?})", expression)
            }
            ASTNode::MatchExpr { .. } => "MatchExpr".to_string(),
            ASTNode::EnumMatchExpr { .. } => "EnumMatchExpr".to_string(),
            ASTNode::QMarkPropagate { .. } => "QMarkPropagate".to_string(),
            ASTNode::Lambda { params, body, .. } => {
                format!("Lambda({} params, {} statements)", params.len(), body.len())
            }
            ASTNode::ArrayLiteral { elements, .. } => {
                format!("ArrayLiteral({} elements)", elements.len())
            }
            ASTNode::MapLiteral { entries, .. } => {
                format!("MapLiteral({} entries)", entries.len())
            }
            ASTNode::RecordLiteral {
                record_type_name,
                fields,
                ..
            } => {
                format!(
                    "RecordLiteral({}, {} fields)",
                    record_type_name,
                    fields.len()
                )
            }
            ASTNode::RecordUpdate { updates, .. } => {
                format!("RecordUpdate({} fields)", updates.len())
            }
            ASTNode::BlockExpr {
                prelude_stmts,
                tail_expr,
                ..
            } => {
                format!(
                    "BlockExpr({} prelude stmts, tail={})",
                    prelude_stmts.len(),
                    tail_expr.node_type()
                )
            }
            ASTNode::Index { target, index, .. } => {
                format!("Index(target={:?}, index={:?})", target, index)
            }
            ASTNode::ScopeBox { .. } => "ScopeBox".to_string(),
            // Phase 152-A: Grouped assignment expression
            ASTNode::GroupedAssignmentExpr { lhs, .. } => {
                format!("GroupedAssignmentExpr(lhs={})", lhs)
            }
        }
    }

    /// ASTノードからSpan情報を取得
    pub fn span(&self) -> Span {
        match self {
            ASTNode::Program { span, .. } => *span,
            ASTNode::Assignment { span, .. } => *span,
            ASTNode::Print { span, .. } => *span,
            ASTNode::If { span, .. } => *span,
            ASTNode::Loop { span, .. } => *span,
            ASTNode::LoopRange { span, .. } => *span,
            ASTNode::Return { span, .. } => *span,
            ASTNode::Break { span, .. } => *span,
            ASTNode::Continue { span, .. } => *span,
            ASTNode::UsingStatement { span, .. } => *span,
            ASTNode::ImportStatement { span, .. } => *span,
            ASTNode::Nowait { span, .. } => *span,
            ASTNode::Arrow { span, .. } => *span,
            ASTNode::TryCatch { span, .. } => *span,
            ASTNode::Throw { span, .. } => *span,
            ASTNode::BoxDeclaration { span, .. } => *span,
            ASTNode::EnumDeclaration { span, .. } => *span,
            ASTNode::BrandDeclaration { span, .. } => *span,
            ASTNode::TypeAliasDeclaration { span, .. } => *span,
            ASTNode::FunctionDeclaration { span, .. } => *span,
            ASTNode::GlobalVar { span, .. } => *span,
            ASTNode::StaticConstTable { span, .. } => *span,
            ASTNode::Literal { span, .. } => *span,
            ASTNode::Variable { span, .. } => *span,
            ASTNode::UnaryOp { span, .. } => *span,
            ASTNode::BinaryOp { span, .. } => *span,
            ASTNode::CheckExpr { span, .. } => *span,
            ASTNode::MethodCall { span, .. } => *span,
            ASTNode::FieldAccess { span, .. } => *span,
            ASTNode::Index { span, .. } => *span,
            ASTNode::New { span, .. } => *span,
            ASTNode::This { span, .. } => *span,
            ASTNode::Me { span, .. } => *span,
            ASTNode::FromCall { span, .. } => *span,
            ASTNode::ThisField { span, .. } => *span,
            ASTNode::MeField { span, .. } => *span,

            ASTNode::Local { span, .. } => *span,
            ASTNode::Outbox { span, .. } => *span,
            ASTNode::FunctionCall { span, .. } => *span,
            ASTNode::Call { span, .. } => *span,
            ASTNode::AwaitExpression { span, .. } => *span,
            ASTNode::MatchExpr { span, .. } => *span,
            ASTNode::EnumMatchExpr { span, .. } => *span,
            ASTNode::QMarkPropagate { span, .. } => *span,
            ASTNode::Lambda { span, .. } => *span,
            ASTNode::ArrayLiteral { span, .. } => *span,
            ASTNode::MapLiteral { span, .. } => *span,
            ASTNode::RecordLiteral { span, .. } => *span,
            ASTNode::RecordUpdate { span, .. } => *span,
            ASTNode::BlockExpr { span, .. } => *span,
            ASTNode::ScopeBox { span, .. } => *span,
            // Phase 152-A: Grouped assignment expression
            ASTNode::GroupedAssignmentExpr { span, .. } => *span,
        }
    }
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.info())
    }
}

impl ASTNode {
    /// FunctionDeclarationのパラメータ数を取得
    pub fn get_param_count(&self) -> usize {
        match self {
            ASTNode::FunctionDeclaration { params, .. } => params.len(),
            _ => 0,
        }
    }

    /// Returns true if this node contains a `return` statement (recursively).
    ///
    /// Scope boundaries (`lambda` / `function` / `box`) stop the search.
    pub fn contains_return_stmt(&self) -> bool {
        fn contains(node: &ASTNode) -> bool {
            match node {
                ASTNode::Return { .. } => true,

                ASTNode::Lambda { .. }
                | ASTNode::FunctionDeclaration { .. }
                | ASTNode::EnumDeclaration { .. }
                | ASTNode::BrandDeclaration { .. }
                | ASTNode::TypeAliasDeclaration { .. }
                | ASTNode::BoxDeclaration { .. }
                | ASTNode::StaticConstTable { .. } => false,

                _ => node.any_child(contains),
            }
        }

        contains(self)
    }

    /// Returns true if this node contains `break` or `continue` (recursively).
    ///
    /// Scope boundaries (`lambda` / `function` / `box`) stop the search.
    pub fn contains_break_continue(&self) -> bool {
        fn contains(node: &ASTNode) -> bool {
            match node {
                ASTNode::Break { .. } | ASTNode::Continue { .. } => true,

                ASTNode::Lambda { .. }
                | ASTNode::FunctionDeclaration { .. }
                | ASTNode::EnumDeclaration { .. }
                | ASTNode::BrandDeclaration { .. }
                | ASTNode::TypeAliasDeclaration { .. }
                | ASTNode::BoxDeclaration { .. }
                | ASTNode::StaticConstTable { .. } => false,

                _ => node.any_child(contains),
            }
        }

        contains(self)
    }

    pub fn contains_non_local_exit(&self) -> bool {
        match self {
            ASTNode::Return { .. }
            | ASTNode::Break { .. }
            | ASTNode::Continue { .. }
            | ASTNode::Throw { .. } => true,

            // Scope boundary: exits inside nested function/box/lambda do not escape.
            ASTNode::Lambda { .. }
            | ASTNode::FunctionDeclaration { .. }
            | ASTNode::EnumDeclaration { .. }
            | ASTNode::BrandDeclaration { .. }
            | ASTNode::TypeAliasDeclaration { .. }
            | ASTNode::BoxDeclaration { .. }
            | ASTNode::StaticConstTable { .. } => false,

            _ => self.any_child(ASTNode::contains_non_local_exit),
        }
    }

    /// Returns true if this node contains a non-local exit *outside of nested loops*.
    ///
    /// Contract:
    /// - `return` / `throw` are always treated as non-local exits (unless inside a scope boundary).
    /// - `break` / `continue` are treated as non-local exits only when they occur outside any
    ///   `loop` / `while` / `for` (loop_depth == 0).
    /// - Scope boundaries (`lambda` / `function` / `box`) stop the search.
    ///
    /// This is an observation helper used by Facts-level recipe builders where nested loops are
    /// permitted, but exits that would escape the surrounding block must be rejected.
    pub fn contains_non_local_exit_outside_loops(&self) -> bool {
        fn contains(node: &ASTNode, loop_depth: usize) -> bool {
            match node {
                ASTNode::Return { .. } | ASTNode::Throw { .. } => true,
                ASTNode::Break { .. } | ASTNode::Continue { .. } => loop_depth == 0,

                ASTNode::Lambda { .. }
                | ASTNode::FunctionDeclaration { .. }
                | ASTNode::EnumDeclaration { .. }
                | ASTNode::BrandDeclaration { .. }
                | ASTNode::TypeAliasDeclaration { .. }
                | ASTNode::BoxDeclaration { .. }
                | ASTNode::StaticConstTable { .. } => false,

                ASTNode::Program { statements, .. } => {
                    statements.iter().any(|s| contains(s, loop_depth))
                }
                ASTNode::Assignment { target, value, .. } => {
                    contains(target, loop_depth) || contains(value, loop_depth)
                }
                ASTNode::Print { expression, .. } => contains(expression, loop_depth),
                ASTNode::If {
                    condition,
                    then_body,
                    else_body,
                    ..
                } => {
                    contains(condition, loop_depth)
                        || then_body.iter().any(|s| contains(s, loop_depth))
                        || else_body
                            .as_ref()
                            .is_some_and(|b| b.iter().any(|s| contains(s, loop_depth)))
                }
                ASTNode::Loop {
                    condition, body, ..
                } => {
                    contains(condition, loop_depth)
                        || body
                            .iter()
                            .any(|s| contains(s, loop_depth.saturating_add(1)))
                }
                ASTNode::LoopRange {
                    start, end, body, ..
                } => {
                    contains(start, loop_depth)
                        || contains(end, loop_depth)
                        || body
                            .iter()
                            .any(|s| contains(s, loop_depth.saturating_add(1)))
                }
                ASTNode::UsingStatement { .. } | ASTNode::ImportStatement { .. } => false,
                ASTNode::FromCall { .. } => false,
                _ => node.any_child(|child| contains(child, loop_depth)),
            }
        }

        contains(self, 0)
    }
}
