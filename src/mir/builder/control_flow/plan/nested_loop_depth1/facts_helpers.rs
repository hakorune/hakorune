use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::stmt_view::try_build_stmt_only_block_recipe;
use crate::mir::builder::control_flow::plan::extractors::common_helpers::strip_trailing_continue_view;
use crate::mir::builder::control_flow::plan::facts::nested_loop_profile::scan_nested_loop_body;
use crate::mir::builder::control_flow::recipes::RecipeBody;

use super::facts_types::{NestedLoopDepth1Facts, NestedLoopDepth1Kind};

pub(super) fn try_extract_break_continue_pure(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Option<NestedLoopDepth1Facts> {
    let kind = NestedLoopDepth1Kind::BreakContinuePure;
    if scan_nested_loop_body(body, kind.profile(), true).is_none() {
        return None;
    }

    let (trimmed_body, _has_trailing_continue) = strip_trailing_continue_view(body);
    Some(build_facts(kind, condition, trimmed_body))
}

pub(super) fn try_extract_no_break_or_continue_pure(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Option<NestedLoopDepth1Facts> {
    let kind = NestedLoopDepth1Kind::NoBreakOrContinuePure;
    if scan_nested_loop_body(body, kind.profile(), true).is_none() {
        return None;
    }

    Some(build_facts(kind, condition, body))
}

pub(super) fn try_extract_methodcall(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Option<NestedLoopDepth1Facts> {
    let kind = NestedLoopDepth1Kind::MethodCall;
    if scan_nested_loop_body(body, kind.profile(), true).is_none() {
        return None;
    }

    Some(build_facts(kind, condition, body))
}

pub(super) fn try_extract_no_break_or_continue(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Option<NestedLoopDepth1Facts> {
    let kind = NestedLoopDepth1Kind::NoBreakOrContinue;
    if scan_nested_loop_body(body, kind.profile(), true).is_none() {
        return None;
    }

    Some(build_facts(kind, condition, body))
}

fn build_facts(
    kind: NestedLoopDepth1Kind,
    condition: &ASTNode,
    body: &[ASTNode],
) -> NestedLoopDepth1Facts {
    NestedLoopDepth1Facts {
        kind,
        condition: condition.clone(),
        body: RecipeBody::new(body.to_vec()),
        body_stmt_only: try_build_stmt_only_block_recipe(body),
    }
}
