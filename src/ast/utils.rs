//! Utility helpers for Nyash AST nodes extracted from `ast.rs`.

use super::{ASTNode, Span};
use std::fmt;

mod node_type;
mod classify;

impl ASTNode {
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
            ASTNode::While {
                condition: _, body, ..
            } => {
                format!("While({} statements)", body.len())
            }
            ASTNode::ForRange {
                var_name,
                start: _,
                end: _,
                body,
                ..
            } => {
                format!("ForRange(var={}, {} statements)", var_name, body.len())
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
                extends,
                implements,
                ..
            } => {
                let mut desc = if *is_interface {
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
            ASTNode::While { span, .. } => *span,
            ASTNode::ForRange { span, .. } => *span,
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
            ASTNode::FunctionDeclaration { span, .. } => *span,
            ASTNode::GlobalVar { span, .. } => *span,
            ASTNode::Literal { span, .. } => *span,
            ASTNode::Variable { span, .. } => *span,
            ASTNode::UnaryOp { span, .. } => *span,
            ASTNode::BinaryOp { span, .. } => *span,
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
            ASTNode::QMarkPropagate { span, .. } => *span,
            ASTNode::Lambda { span, .. } => *span,
            ASTNode::ArrayLiteral { span, .. } => *span,
            ASTNode::MapLiteral { span, .. } => *span,
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
                | ASTNode::BoxDeclaration { .. } => false,

                ASTNode::Program { statements, .. } => statements.iter().any(contains),
                ASTNode::ScopeBox { body, .. } => body.iter().any(contains),

                ASTNode::Assignment { target, value, .. } => contains(target) || contains(value),
                ASTNode::Print { expression, .. } => contains(expression),
                ASTNode::If {
                    condition,
                    then_body,
                    else_body,
                    ..
                } => {
                    contains(condition)
                        || then_body.iter().any(contains)
                        || else_body.as_ref().is_some_and(|b| b.iter().any(contains))
                }
                ASTNode::Loop { condition, body, .. }
                | ASTNode::While { condition, body, .. } => {
                    contains(condition) || body.iter().any(contains)
                }
                ASTNode::ForRange {
                    start, end, body, ..
                } => contains(start) || contains(end) || body.iter().any(contains),
                ASTNode::Nowait { expression, .. } => contains(expression),
                ASTNode::AwaitExpression { expression, .. } => contains(expression),
                ASTNode::QMarkPropagate { expression, .. } => contains(expression),
                ASTNode::Throw { expression, .. } => contains(expression),
                ASTNode::GlobalVar { value, .. } => contains(value),
                ASTNode::UsingStatement { .. } | ASTNode::ImportStatement { .. } => false,

                ASTNode::MatchExpr {
                    scrutinee,
                    arms,
                    else_expr,
                    ..
                } => {
                    contains(scrutinee)
                        || arms.iter().any(|(_, arm_expr)| contains(arm_expr))
                        || contains(else_expr)
                }
                ASTNode::ArrayLiteral { elements, .. } => elements.iter().any(contains),
                ASTNode::MapLiteral { entries, .. } => entries.iter().any(|(_, expr)| contains(expr)),
                ASTNode::BlockExpr {
                    prelude_stmts,
                    tail_expr,
                    ..
                } => prelude_stmts.iter().any(contains) || contains(tail_expr),
                ASTNode::Arrow { sender, receiver, .. } => contains(sender) || contains(receiver),
                ASTNode::TryCatch {
                    try_body,
                    catch_clauses,
                    finally_body,
                    ..
                } => {
                    try_body.iter().any(contains)
                        || catch_clauses.iter().any(|c| c.body.iter().any(contains))
                        || finally_body.as_ref().is_some_and(|b| b.iter().any(contains))
                }

                ASTNode::UnaryOp { operand, .. } => contains(operand),
                ASTNode::BinaryOp { left, right, .. } => contains(left) || contains(right),
                ASTNode::GroupedAssignmentExpr { rhs, .. } => contains(rhs),
                ASTNode::MethodCall {
                    object, arguments, ..
                } => contains(object) || arguments.iter().any(contains),
                ASTNode::FieldAccess { object, .. } => contains(object),
                ASTNode::Index { target, index, .. } => contains(target) || contains(index),
                ASTNode::New { arguments, .. }
                | ASTNode::FromCall { arguments, .. }
                | ASTNode::FunctionCall { arguments, .. } => arguments.iter().any(contains),
                ASTNode::Call { callee, arguments, .. } => contains(callee) || arguments.iter().any(contains),
                ASTNode::Local { initial_values, .. }
                | ASTNode::Outbox { initial_values, .. } => initial_values
                    .iter()
                    .filter_map(|v| v.as_deref())
                    .any(contains),

                _ => false,
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
                | ASTNode::BoxDeclaration { .. } => false,

                ASTNode::Program { statements, .. } => statements.iter().any(contains),
                ASTNode::ScopeBox { body, .. } => body.iter().any(contains),

                ASTNode::Assignment { target, value, .. } => contains(target) || contains(value),
                ASTNode::Print { expression, .. } => contains(expression),
                ASTNode::If {
                    condition,
                    then_body,
                    else_body,
                    ..
                } => {
                    contains(condition)
                        || then_body.iter().any(contains)
                        || else_body.as_ref().is_some_and(|b| b.iter().any(contains))
                }
                ASTNode::Loop { condition, body, .. }
                | ASTNode::While { condition, body, .. } => {
                    contains(condition) || body.iter().any(contains)
                }
                ASTNode::ForRange {
                    start, end, body, ..
                } => contains(start) || contains(end) || body.iter().any(contains),
                ASTNode::Nowait { expression, .. } => contains(expression),
                ASTNode::AwaitExpression { expression, .. } => contains(expression),
                ASTNode::QMarkPropagate { expression, .. } => contains(expression),
                ASTNode::Throw { expression, .. } => contains(expression),
                ASTNode::GlobalVar { value, .. } => contains(value),
                ASTNode::UsingStatement { .. } | ASTNode::ImportStatement { .. } => false,

                ASTNode::MatchExpr {
                    scrutinee,
                    arms,
                    else_expr,
                    ..
                } => {
                    contains(scrutinee)
                        || arms.iter().any(|(_, arm_expr)| contains(arm_expr))
                        || contains(else_expr)
                }
                ASTNode::ArrayLiteral { elements, .. } => elements.iter().any(contains),
                ASTNode::MapLiteral { entries, .. } => entries.iter().any(|(_, expr)| contains(expr)),
                ASTNode::BlockExpr {
                    prelude_stmts,
                    tail_expr,
                    ..
                } => prelude_stmts.iter().any(contains) || contains(tail_expr),
                ASTNode::Arrow { sender, receiver, .. } => contains(sender) || contains(receiver),
                ASTNode::TryCatch {
                    try_body,
                    catch_clauses,
                    finally_body,
                    ..
                } => {
                    try_body.iter().any(contains)
                        || catch_clauses.iter().any(|c| c.body.iter().any(contains))
                        || finally_body.as_ref().is_some_and(|b| b.iter().any(contains))
                }

                ASTNode::UnaryOp { operand, .. } => contains(operand),
                ASTNode::BinaryOp { left, right, .. } => contains(left) || contains(right),
                ASTNode::GroupedAssignmentExpr { rhs, .. } => contains(rhs),
                ASTNode::MethodCall {
                    object, arguments, ..
                } => contains(object) || arguments.iter().any(contains),
                ASTNode::FieldAccess { object, .. } => contains(object),
                ASTNode::Index { target, index, .. } => contains(target) || contains(index),
                ASTNode::New { arguments, .. }
                | ASTNode::FromCall { arguments, .. }
                | ASTNode::FunctionCall { arguments, .. } => arguments.iter().any(contains),
                ASTNode::Call { callee, arguments, .. } => contains(callee) || arguments.iter().any(contains),
                ASTNode::Local { initial_values, .. }
                | ASTNode::Outbox { initial_values, .. } => initial_values
                    .iter()
                    .filter_map(|v| v.as_deref())
                    .any(contains),

                _ => false,
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
            | ASTNode::BoxDeclaration { .. } => false,

            ASTNode::Program { statements, .. } => {
                statements.iter().any(ASTNode::contains_non_local_exit)
            }
            ASTNode::Assignment { target, value, .. } => {
                target.contains_non_local_exit() || value.contains_non_local_exit()
            }
            ASTNode::Print { expression, .. } => expression.contains_non_local_exit(),
            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                condition.contains_non_local_exit()
                    || then_body.iter().any(ASTNode::contains_non_local_exit)
                    || else_body
                        .as_ref()
                        .is_some_and(|b| b.iter().any(ASTNode::contains_non_local_exit))
            }
            ASTNode::Loop { condition, body, .. } | ASTNode::While { condition, body, .. } => {
                condition.contains_non_local_exit() || body.iter().any(ASTNode::contains_non_local_exit)
            }
            ASTNode::ForRange {
                start, end, body, ..
            } => {
                start.contains_non_local_exit()
                    || end.contains_non_local_exit()
                    || body.iter().any(ASTNode::contains_non_local_exit)
            }
            ASTNode::UsingStatement { .. } | ASTNode::ImportStatement { .. } => false,
            ASTNode::Nowait { expression, .. } => expression.contains_non_local_exit(),
            ASTNode::AwaitExpression { expression, .. } => expression.contains_non_local_exit(),
            ASTNode::QMarkPropagate { expression, .. } => expression.contains_non_local_exit(),
            ASTNode::MatchExpr {
                scrutinee,
                arms,
                else_expr,
                ..
            } => {
                scrutinee.contains_non_local_exit()
                    || arms
                        .iter()
                        .any(|(_, arm_expr)| arm_expr.contains_non_local_exit())
                    || else_expr.contains_non_local_exit()
            }
            ASTNode::ArrayLiteral { elements, .. } => {
                elements.iter().any(ASTNode::contains_non_local_exit)
            }
            ASTNode::MapLiteral { entries, .. } => entries
                .iter()
                .any(|(_, expr)| expr.contains_non_local_exit()),
            ASTNode::BlockExpr {
                prelude_stmts,
                tail_expr,
                ..
            } => {
                prelude_stmts.iter().any(ASTNode::contains_non_local_exit)
                    || tail_expr.contains_non_local_exit()
            }
            ASTNode::Arrow { sender, receiver, .. } => {
                sender.contains_non_local_exit() || receiver.contains_non_local_exit()
            }
            ASTNode::TryCatch {
                try_body,
                catch_clauses,
                finally_body,
                ..
            } => {
                try_body.iter().any(ASTNode::contains_non_local_exit)
                    || catch_clauses
                        .iter()
                        .any(|c| c.body.iter().any(ASTNode::contains_non_local_exit))
                    || finally_body
                        .as_ref()
                        .is_some_and(|b| b.iter().any(ASTNode::contains_non_local_exit))
            }
            ASTNode::GlobalVar { value, .. } => value.contains_non_local_exit(),
            ASTNode::Literal { .. }
            | ASTNode::Variable { .. }
            | ASTNode::This { .. }
            | ASTNode::Me { .. }
            | ASTNode::ThisField { .. }
            | ASTNode::MeField { .. } => false,
            ASTNode::UnaryOp { operand, .. } => operand.contains_non_local_exit(),
            ASTNode::BinaryOp { left, right, .. } => {
                left.contains_non_local_exit() || right.contains_non_local_exit()
            }
            ASTNode::GroupedAssignmentExpr { rhs, .. } => rhs.contains_non_local_exit(),
            ASTNode::MethodCall {
                object, arguments, ..
            } => {
                object.contains_non_local_exit()
                    || arguments.iter().any(ASTNode::contains_non_local_exit)
            }
            ASTNode::FieldAccess { object, .. } => object.contains_non_local_exit(),
            ASTNode::Index { target, index, .. } => {
                target.contains_non_local_exit() || index.contains_non_local_exit()
            }
            ASTNode::New { arguments, .. } | ASTNode::FromCall { arguments, .. } => {
                arguments.iter().any(ASTNode::contains_non_local_exit)
            }
            ASTNode::Local {
                initial_values, ..
            }
            | ASTNode::Outbox {
                initial_values, ..
            } => initial_values
                .iter()
                .filter_map(|v| v.as_deref())
                .any(ASTNode::contains_non_local_exit),
            ASTNode::ScopeBox { body, .. } => body.iter().any(ASTNode::contains_non_local_exit),
            ASTNode::FunctionCall { arguments, .. } => {
                arguments.iter().any(ASTNode::contains_non_local_exit)
            }
            ASTNode::Call {
                callee,
                arguments,
                ..
            } => {
                callee.contains_non_local_exit()
                    || arguments.iter().any(ASTNode::contains_non_local_exit)
            }
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
                | ASTNode::BoxDeclaration { .. } => false,

                ASTNode::Program { statements, .. } => statements.iter().any(|s| contains(s, loop_depth)),
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
                ASTNode::Loop { condition, body, .. } | ASTNode::While { condition, body, .. } => {
                    contains(condition, loop_depth)
                        || body.iter().any(|s| contains(s, loop_depth.saturating_add(1)))
                }
                ASTNode::ForRange {
                    start, end, body, ..
                } => {
                    contains(start, loop_depth)
                        || contains(end, loop_depth)
                        || body.iter().any(|s| contains(s, loop_depth.saturating_add(1)))
                }
                ASTNode::UsingStatement { .. } | ASTNode::ImportStatement { .. } => false,
                ASTNode::Nowait { expression, .. } => contains(expression, loop_depth),
                ASTNode::AwaitExpression { expression, .. } => contains(expression, loop_depth),
                ASTNode::QMarkPropagate { expression, .. } => contains(expression, loop_depth),
                ASTNode::MatchExpr {
                    scrutinee,
                    arms,
                    else_expr,
                    ..
                } => {
                    contains(scrutinee, loop_depth)
                        || arms
                            .iter()
                            .any(|(_, arm_expr)| contains(arm_expr, loop_depth))
                        || contains(else_expr, loop_depth)
                }
                ASTNode::ArrayLiteral { elements, .. } => {
                    elements.iter().any(|e| contains(e, loop_depth))
                }
                ASTNode::MapLiteral { entries, .. } => entries
                    .iter()
                    .any(|(_, expr)| contains(expr, loop_depth)),
                ASTNode::BlockExpr {
                    prelude_stmts,
                    tail_expr,
                    ..
                } => {
                    prelude_stmts.iter().any(|s| contains(s, loop_depth))
                        || contains(tail_expr, loop_depth)
                }
                ASTNode::Arrow { sender, receiver, .. } => {
                    contains(sender, loop_depth) || contains(receiver, loop_depth)
                }
                ASTNode::TryCatch {
                    try_body,
                    catch_clauses,
                    finally_body,
                    ..
                } => {
                    try_body.iter().any(|s| contains(s, loop_depth))
                        || catch_clauses
                            .iter()
                            .any(|c| c.body.iter().any(|s| contains(s, loop_depth)))
                        || finally_body
                            .as_ref()
                            .is_some_and(|b| b.iter().any(|s| contains(s, loop_depth)))
                }
                ASTNode::GlobalVar { value, .. } => contains(value, loop_depth),

                ASTNode::Literal { .. }
                | ASTNode::Variable { .. }
                | ASTNode::This { .. }
                | ASTNode::Me { .. }
                | ASTNode::ThisField { .. }
                | ASTNode::MeField { .. } => false,

                ASTNode::UnaryOp { operand, .. } => contains(operand, loop_depth),
                ASTNode::BinaryOp { left, right, .. } => contains(left, loop_depth) || contains(right, loop_depth),
                ASTNode::GroupedAssignmentExpr { rhs, .. } => contains(rhs, loop_depth),

                ASTNode::MethodCall {
                    object, arguments, ..
                } => contains(object, loop_depth) || arguments.iter().any(|a| contains(a, loop_depth)),
                ASTNode::FieldAccess { object, .. } => contains(object, loop_depth),
                ASTNode::Index { target, index, .. } => contains(target, loop_depth) || contains(index, loop_depth),
                ASTNode::New { arguments, .. } => arguments.iter().any(|a| contains(a, loop_depth)),
                ASTNode::FunctionCall { arguments, .. } => arguments.iter().any(|a| contains(a, loop_depth)),
                ASTNode::Call {
                    callee, arguments, ..
                } => contains(callee, loop_depth) || arguments.iter().any(|a| contains(a, loop_depth)),
                ASTNode::FromCall { .. } => false,
                ASTNode::ScopeBox { body, .. } => body.iter().any(|s| contains(s, loop_depth)),
                ASTNode::Local {
                    initial_values, ..
                }
                | ASTNode::Outbox {
                    initial_values, ..
                } => initial_values
                    .iter()
                    .filter_map(|v| v.as_deref())
                    .any(|v| contains(v, loop_depth)),
            }
        }

        contains(self, 0)
    }
}
