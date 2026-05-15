//! Sync box declaration checks.
//!
//! This module owns source-surface safety for `sync box` before serialized
//! runtime behavior exists. Parser acceptance carries the capsule; wait-like
//! constructs are rejected here so sync methods cannot silently become
//! ordinary box methods with hidden suspension points.

use crate::ast::ASTNode;
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use std::collections::HashMap;

pub(crate) fn validate_no_waits_in_sync_box(
    p: &NyashParser,
    box_name: &str,
    methods: &HashMap<String, ASTNode>,
    constructors: &HashMap<String, ASTNode>,
) -> Result<(), ParseError> {
    for (method_name, node) in methods.iter().chain(constructors.iter()) {
        if let Some(kind) = first_wait_like_in_callable(node) {
            return Err(ParseError::UnexpectedToken {
                found: p.current_token().token_type.clone(),
                expected: format!(
                    "[freeze:contract][sync_box/wait_forbidden] box={} method={} wait_kind={} sync box methods cannot contain await/nowait/channel wait until CONC-SYNCBOX runtime rows land",
                    box_name, method_name, kind
                ),
                line: p.current_token().line,
            });
        }
    }
    Ok(())
}

fn first_wait_like_in_callable(node: &ASTNode) -> Option<&'static str> {
    let ASTNode::FunctionDeclaration { body, .. } = node else {
        return None;
    };
    first_wait_like_in_body(body)
}

fn first_wait_like_in_body(body: &[ASTNode]) -> Option<&'static str> {
    body.iter().find_map(first_wait_like_in_node)
}

fn first_wait_like_in_optional_node(node: &Option<Box<ASTNode>>) -> Option<&'static str> {
    node.as_deref().and_then(first_wait_like_in_node)
}

fn first_wait_like_in_optional_body(body: &Option<Vec<ASTNode>>) -> Option<&'static str> {
    body.as_deref().and_then(first_wait_like_in_body)
}

fn first_wait_like_in_node(node: &ASTNode) -> Option<&'static str> {
    match node {
        ASTNode::AwaitExpression { .. } => Some("await"),
        ASTNode::Nowait { .. } => Some("nowait"),
        ASTNode::Program { statements, .. }
        | ASTNode::ScopeBox {
            body: statements, ..
        }
        | ASTNode::TaskScope {
            body: statements, ..
        } => first_wait_like_in_body(statements),
        ASTNode::Assignment { target, value, .. } => first_wait_like_in_node(target)
            .or_else(|| first_wait_like_in_node(value)),
        ASTNode::Print { expression, .. }
        | ASTNode::QMarkPropagate { expression, .. }
        | ASTNode::Throw { expression, .. } => first_wait_like_in_node(expression),
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => first_wait_like_in_node(condition)
            .or_else(|| first_wait_like_in_body(then_body))
            .or_else(|| first_wait_like_in_optional_body(else_body)),
        ASTNode::Loop {
            condition, body, ..
        } => first_wait_like_in_node(condition).or_else(|| first_wait_like_in_body(body)),
        ASTNode::LoopRange {
            start, end, body, ..
        } => first_wait_like_in_node(start)
            .or_else(|| first_wait_like_in_node(end))
            .or_else(|| first_wait_like_in_body(body)),
        ASTNode::Return { value, .. } => first_wait_like_in_optional_node(value),
        ASTNode::GlobalVar { value, .. } => first_wait_like_in_node(value),
        ASTNode::UnaryOp { operand, .. } => first_wait_like_in_node(operand),
        ASTNode::BinaryOp { left, right, .. } => first_wait_like_in_node(left)
            .or_else(|| first_wait_like_in_node(right)),
        ASTNode::CheckExpr { items, .. } => items
            .iter()
            .find_map(|item| first_wait_like_in_node(&item.expression)),
        ASTNode::GroupedAssignmentExpr { rhs, .. } => first_wait_like_in_node(rhs),
        ASTNode::MethodCall {
            object, arguments, ..
        } => first_wait_like_in_node(object)
            .or_else(|| arguments.iter().find_map(first_wait_like_in_node)),
        ASTNode::FieldAccess { object, .. } => first_wait_like_in_node(object),
        ASTNode::Index { target, index, .. } => first_wait_like_in_node(target)
            .or_else(|| first_wait_like_in_node(index)),
        ASTNode::New { arguments, .. }
        | ASTNode::FromCall { arguments, .. }
        | ASTNode::FunctionCall { arguments, .. } => {
            arguments.iter().find_map(first_wait_like_in_node)
        }
        ASTNode::Call {
            callee, arguments, ..
        } => first_wait_like_in_node(callee)
            .or_else(|| arguments.iter().find_map(first_wait_like_in_node)),
        ASTNode::MatchExpr {
            scrutinee,
            arms,
            else_expr,
            ..
        } => first_wait_like_in_node(scrutinee)
            .or_else(|| arms.iter().find_map(|(_, arm)| first_wait_like_in_node(arm)))
            .or_else(|| first_wait_like_in_node(else_expr)),
        ASTNode::EnumMatchExpr {
            scrutinee,
            arms,
            else_expr,
            ..
        } => first_wait_like_in_node(scrutinee)
            .or_else(|| {
                arms.iter()
                    .find_map(|arm| first_wait_like_in_node(&arm.body))
            })
            .or_else(|| first_wait_like_in_optional_node(else_expr)),
        ASTNode::ArrayLiteral { elements, .. } => elements.iter().find_map(first_wait_like_in_node),
        ASTNode::MapLiteral { entries, .. } => entries
            .iter()
            .find_map(|(_, value)| first_wait_like_in_node(value)),
        ASTNode::RecordLiteral { fields, .. } => fields
            .iter()
            .find_map(|(_, value)| first_wait_like_in_node(value)),
        ASTNode::RecordUpdate { base, updates, .. } => first_wait_like_in_node(base)
            .or_else(|| {
                updates
                    .iter()
                    .find_map(|(_, value)| first_wait_like_in_node(value))
            }),
        ASTNode::BlockExpr {
            prelude_stmts,
            tail_expr,
            ..
        } => first_wait_like_in_body(prelude_stmts)
            .or_else(|| first_wait_like_in_node(tail_expr)),
        ASTNode::Arrow {
            sender, receiver, ..
        } => first_wait_like_in_node(sender)
            .or_else(|| first_wait_like_in_node(receiver)),
        ASTNode::TryCatch {
            try_body,
            catch_clauses,
            finally_body,
            ..
        } => first_wait_like_in_body(try_body)
            .or_else(|| {
                catch_clauses
                    .iter()
                    .find_map(|clause| first_wait_like_in_body(&clause.body))
            })
            .or_else(|| first_wait_like_in_optional_body(finally_body)),
        ASTNode::Local { initial_values, .. } | ASTNode::Outbox { initial_values, .. } => {
            initial_values.iter().find_map(first_wait_like_in_optional_node)
        }
        ASTNode::Literal { .. }
        | ASTNode::Variable { .. }
        | ASTNode::This { .. }
        | ASTNode::Me { .. }
        | ASTNode::ThisField { .. }
        | ASTNode::MeField { .. }
        | ASTNode::Break { .. }
        | ASTNode::Continue { .. }
        | ASTNode::UsingStatement { .. }
        | ASTNode::ImportStatement { .. }
        | ASTNode::FunctionDeclaration { .. }
        | ASTNode::EnumDeclaration { .. }
        | ASTNode::BrandDeclaration { .. }
        | ASTNode::TypeAliasDeclaration { .. }
        | ASTNode::BoxDeclaration { .. }
        | ASTNode::StaticConstTable { .. }
        | ASTNode::Lambda { .. } => None,
    }
}
