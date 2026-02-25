//! Phase 12: Unified facts for nested loop depth1.
//!
//! This module consolidates 4 separate nested_loop_depth1 variants into a single
//! unified structure with a kind enum. Each kind corresponds to a different
//! NestedLoopBodyProfile configuration.
//!
//! Priority order for matching (most specific first):
//! 1. BreakContinuePure - break/continue allowed, no calls
//! 2. NoBreakOrContinuePure - no break/continue, no calls
//! 3. MethodCall - break/continue allowed, requires calls
//! 4. NoBreakOrContinue - no break/continue, allows calls

use crate::ast::{ASTNode, BinaryOperator};
use crate::mir::builder::control_flow::plan::extractors::common_helpers::strip_trailing_continue_view;
use crate::mir::builder::control_flow::plan::facts::nested_loop_profile::{
    scan_nested_loop_body, NestedLoopBodyProfile,
};
use crate::mir::builder::control_flow::plan::facts::stmt_view::{
    try_build_stmt_only_block_recipe, StmtOnlyBlockRecipe,
};
use crate::mir::builder::control_flow::plan::recipes::RecipeBody;

/// Kind of nested loop depth1 pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum NestedLoopDepth1Kind {
    /// break/continue allowed, no calls (most restrictive for calls)
    BreakContinuePure,
    /// no break/continue, no calls
    NoBreakOrContinuePure,
    /// break/continue allowed, requires calls
    MethodCall,
    /// no break/continue, allows calls (least restrictive)
    NoBreakOrContinue,
}

impl NestedLoopDepth1Kind {
    /// Get the NestedLoopBodyProfile for this kind.
    pub fn profile(self) -> NestedLoopBodyProfile {
        match self {
            NestedLoopDepth1Kind::BreakContinuePure => NestedLoopBodyProfile {
                allow_calls: false,
                require_call: false,
                allow_break_in_if: true,
                allow_continue_in_if: true,
                allow_trailing_continue: true,
            },
            NestedLoopDepth1Kind::NoBreakOrContinuePure => NestedLoopBodyProfile {
                allow_calls: false,
                require_call: false,
                allow_break_in_if: false,
                allow_continue_in_if: false,
                allow_trailing_continue: false,
            },
            NestedLoopDepth1Kind::MethodCall => NestedLoopBodyProfile {
                allow_calls: true,
                require_call: true,
                allow_break_in_if: true,
                allow_continue_in_if: true,
                allow_trailing_continue: false,
            },
            NestedLoopDepth1Kind::NoBreakOrContinue => NestedLoopBodyProfile {
                allow_calls: true,
                require_call: true,
                allow_break_in_if: false,
                allow_continue_in_if: false,
                allow_trailing_continue: false,
            },
        }
    }

    /// Get the context name for error messages.
    pub fn context_name(self) -> &'static str {
        match self {
            NestedLoopDepth1Kind::BreakContinuePure => "<nested-break-continue-pure>",
            NestedLoopDepth1Kind::NoBreakOrContinuePure => "<nested-no-bc-pure>",
            NestedLoopDepth1Kind::MethodCall => "<nested-methodcall>",
            NestedLoopDepth1Kind::NoBreakOrContinue => "<nested>",
        }
    }
}

/// Unified facts for nested loop depth1 patterns.
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct NestedLoopDepth1Facts {
    /// The kind of nested loop pattern.
    pub kind: NestedLoopDepth1Kind,
    /// Loop condition AST.
    pub condition: ASTNode,
    /// Loop body (possibly trimmed for BreakContinuePure).
    pub body: RecipeBody,
    /// Optional stmt-only payload (no acceptance reduction; used as a fast-path when available).
    pub body_stmt_only: Option<StmtOnlyBlockRecipe>,
}

/// Try to extract unified nested loop depth1 facts.
///
/// Tries all kinds in priority order and returns the first match.
/// Priority: BreakContinuePure > NoBreakOrContinuePure > MethodCall > NoBreakOrContinue
pub(in crate::mir::builder) fn try_extract_nested_loop_depth1_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Option<NestedLoopDepth1Facts> {
    // Common condition check: must be Less or LessEqual comparison
    if !matches!(
        condition,
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less | BinaryOperator::LessEqual,
            ..
        }
    ) {
        return None;
    }

    // Try BreakContinuePure first (most specific for pure code with control flow)
    if let Some(facts) = try_extract_break_continue_pure(condition, body) {
        return Some(facts);
    }

    // Try NoBreakOrContinuePure (pure code without control flow)
    if let Some(facts) = try_extract_no_break_or_continue_pure(condition, body) {
        return Some(facts);
    }

    // Try MethodCall (requires calls, allows control flow)
    if let Some(facts) = try_extract_methodcall(condition, body) {
        return Some(facts);
    }

    // Try NoBreakOrContinue (requires calls, no control flow)
    if let Some(facts) = try_extract_no_break_or_continue(condition, body) {
        return Some(facts);
    }

    None
}

fn try_extract_break_continue_pure(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Option<NestedLoopDepth1Facts> {
    let kind = NestedLoopDepth1Kind::BreakContinuePure;
    let profile = kind.profile();

    if scan_nested_loop_body(body, profile, true).is_none() {
        return None;
    }

    // BreakContinuePure needs trailing continue stripped
    let (trimmed_body, _has_trailing_continue) = strip_trailing_continue_view(body);
    let body_recipe = RecipeBody::new(trimmed_body.to_vec());
    let body_stmt_only = try_build_stmt_only_block_recipe(trimmed_body);

    Some(NestedLoopDepth1Facts {
        kind,
        condition: condition.clone(),
        body: body_recipe,
        body_stmt_only,
    })
}

fn try_extract_no_break_or_continue_pure(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Option<NestedLoopDepth1Facts> {
    let kind = NestedLoopDepth1Kind::NoBreakOrContinuePure;
    let profile = kind.profile();

    if scan_nested_loop_body(body, profile, true).is_none() {
        return None;
    }

    let body_recipe = RecipeBody::new(body.to_vec());
    let body_stmt_only = try_build_stmt_only_block_recipe(body);

    Some(NestedLoopDepth1Facts {
        kind,
        condition: condition.clone(),
        body: body_recipe,
        body_stmt_only,
    })
}

fn try_extract_methodcall(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Option<NestedLoopDepth1Facts> {
    let kind = NestedLoopDepth1Kind::MethodCall;
    let profile = kind.profile();

    if scan_nested_loop_body(body, profile, true).is_none() {
        return None;
    }

    let body_recipe = RecipeBody::new(body.to_vec());
    let body_stmt_only = try_build_stmt_only_block_recipe(body);

    Some(NestedLoopDepth1Facts {
        kind,
        condition: condition.clone(),
        body: body_recipe,
        body_stmt_only,
    })
}

fn try_extract_no_break_or_continue(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Option<NestedLoopDepth1Facts> {
    let kind = NestedLoopDepth1Kind::NoBreakOrContinue;
    let profile = kind.profile();

    if scan_nested_loop_body(body, profile, true).is_none() {
        return None;
    }

    let body_recipe = RecipeBody::new(body.to_vec());
    let body_stmt_only = try_build_stmt_only_block_recipe(body);

    Some(NestedLoopDepth1Facts {
        kind,
        condition: condition.clone(),
        body: body_recipe,
        body_stmt_only,
    })
}
