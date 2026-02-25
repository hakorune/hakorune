//! Phase 29bq P2: LoopTrueBreakContinueFacts (Facts SSOT)
//!
//! Recipe-first: Facts は recipe を作るだけ（意味論条件の羅列禁止）
//! Lower/pipeline は recipe を順に下ろすだけ（再判定禁止）

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::extractors::common_helpers::is_true_literal;
use crate::mir::builder::control_flow::plan::loop_cond_shared::LoopCondRecipe;
use crate::mir::builder::control_flow::plan::loop_true_break_continue::recipe::{
    ElseExitMixedRecipe, ElseItem, LoopTrueBreakContinueRecipe, LoopTrueItem,
};
use crate::mir::builder::control_flow::plan::facts::exit_only_block::{
    ExitAllowedBlockRecipe, try_build_exit_allowed_block_recipe,
};
use crate::mir::builder::control_flow::plan::recipes::refs::{StmtPair, StmtRef};
use crate::mir::builder::control_flow::plan::recipes::RecipeBody;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::policies::BodyLoweringPolicy;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopTrueBreakContinueFacts {
    pub recipe: LoopTrueBreakContinueRecipe,
    pub body_lowering_policy: BodyLoweringPolicy,
    pub body_exit_allowed: Option<ExitAllowedBlockRecipe>,
}

pub(in crate::mir::builder) fn try_extract_loop_true_break_continue_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<LoopTrueBreakContinueFacts>, Freeze> {
    if !is_true_literal(condition) {
        return Ok(None);
    }

    let counts = super::loop_cond_unified_helpers::count_control_flow_with_returns(body);
    // return は ExitIf で許可する（exit_if_map 制約: prelude+return+else=NG）
    if counts.break_count == 0 && counts.continue_count == 0 && counts.return_count == 0 {
        return Ok(None);
    }

    // Recipe-first: if the whole body can be represented as an ExitAllowed RecipeBlock,
    // accept it even if the legacy LoopTrueItem classification can't describe it.
    //
    // This is critical for Stage-B parser loops such as `parse_block` where the body may contain
    // nested loops (e.g. whitespace skipping) and join-bearing `if` statements.
    let allow_extended = crate::config::env::joinir_dev::planner_required_enabled();
    let body_exit_allowed = if allow_extended {
        try_build_exit_allowed_block_recipe(body, allow_extended)
    } else {
        None
    };
    if body_exit_allowed.is_some() {
        return Ok(Some(LoopTrueBreakContinueFacts {
            recipe: LoopCondRecipe::new(body.to_vec(), Vec::new()),
            body_lowering_policy: BodyLoweringPolicy::ExitAllowed {
                allow_join_if: false,
            },
            body_exit_allowed,
        }));
    }

    // Build recipe instead of just validating
    let mut items = Vec::new();
    let mut exit_if_seen = 0usize;
    let mut nested_seen = 0usize;
    if !build_recipe_items(body, &mut items, &mut exit_if_seen, &mut nested_seen, true) {
        return Ok(None);
    }

    if exit_if_seen == 0 {
        return Ok(None);
    }

    Ok(Some(LoopTrueBreakContinueFacts {
        recipe: LoopCondRecipe::new(body.to_vec(), items),
        body_lowering_policy: BodyLoweringPolicy::RecipeOnly,
        body_exit_allowed: None,
    }))
}

/// Build recipe items from body statements.
/// Returns true if all statements can be converted to recipe items.
fn build_recipe_items(
    body: &[ASTNode],
    items: &mut Vec<LoopTrueItem>,
    exit_if_seen: &mut usize,
    nested_seen: &mut usize,
    allow_nested: bool,
) -> bool {
    let mut idx = 0usize;
    while idx < body.len() {
        let stmt = &body[idx];
        let next = body.get(idx + 1);

        if idx + 1 == body.len() {
            if matches!(stmt, ASTNode::Return { value: Some(_), .. }) {
                items.push(LoopTrueItem::TailReturn(StmtRef::new(idx)));
                idx += 1;
                continue;
            }
        }

        // Check for if+tail-exit pair first
        if is_if_tail_exit_pair(stmt, next) {
            items.push(LoopTrueItem::IfTailExitPair(StmtPair::new(idx, idx + 1)));
            *exit_if_seen += 1;
            idx += 2;
            continue;
        }

        // Try to classify the statement
        if let Some(item) = classify_stmt(stmt, idx, exit_if_seen, nested_seen, allow_nested) {
            items.push(item);
            idx += 1;
            continue;
        }

        return false;
    }
    true
}

/// Classify a single statement into a recipe item.
/// Returns None if the statement cannot be classified.
fn classify_stmt(
    stmt: &ASTNode,
    idx: usize,
    exit_if_seen: &mut usize,
    nested_seen: &mut usize,
    allow_nested: bool,
) -> Option<LoopTrueItem> {
    match stmt {
        // Simple statements (no control flow)
        ASTNode::Assignment { .. }
        | ASTNode::Local { .. }
        | ASTNode::MethodCall { .. }
        | ASTNode::FunctionCall { .. }
        | ASTNode::Print { .. } => Some(LoopTrueItem::Stmt(StmtRef::new(idx))),

        // Program stmt (block): allow only general-if body (no loops/exits).
        ASTNode::Program { statements, .. } => {
            if is_general_if_body(statements) {
                Some(LoopTrueItem::ProgramGeneralBlock(StmtRef::new(idx)))
            } else {
                None
            }
        }

        // Nested loop
        ASTNode::Loop { condition, body, .. } => {
            if !allow_nested || *nested_seen >= 1 || !super::loop_cond_unified_helpers::is_supported_nested_loop_condition(condition) {
                return None;
            }
            // Validate nested body can form a recipe
            let mut inner_items = Vec::new();
            let mut inner_exit_if_seen = 0usize;
            let mut inner_nested_seen = 0usize;
            if !build_recipe_items(body, &mut inner_items, &mut inner_exit_if_seen, &mut inner_nested_seen, false) {
                return None;
            }
            if inner_exit_if_seen == 0 {
                return None;
            }
            *nested_seen += 1;
            Some(LoopTrueItem::NestedLoopDepth1(StmtRef::new(idx)))
        }

        // If statement
        ASTNode::If { then_body, else_body, .. } => {
            if is_exit_block(then_body) {
                // ExitIf: both then+else are exit-blocks
                if let Some(else_body) = else_body {
                    if !is_exit_block(else_body) {
                        return None;
                    }
                }
                *exit_if_seen += 1;
                Some(LoopTrueItem::ExitIf(StmtRef::new(idx)))
            } else if let Some(else_body) = else_body {
                // NEW: GeneralIfElseExit pattern
                if is_general_if_body(then_body) {
                    // Build else recipe (classify else items)
                    if let Some(else_recipe) = build_else_exit_mixed_recipe(else_body) {
                        *exit_if_seen += 1;
                        return Some(LoopTrueItem::GeneralIfElseExit {
                            if_ref: StmtRef::new(idx),
                            else_recipe,
                        });
                    }
                }
                // GeneralIf: both then+else are general-if body
                if is_general_if_block(then_body, Some(else_body)) {
                    Some(LoopTrueItem::GeneralIf(StmtRef::new(idx)))
                } else {
                    None
                }
            } else {
                // No else branch
                if is_general_if_body(then_body) {
                    Some(LoopTrueItem::GeneralIf(StmtRef::new(idx)))
                } else {
                    None
                }
            }
        }

        _ => None,
    }
}

fn is_general_if_block(
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
) -> bool {
    if then_body.is_empty() {
        return false;
    }
    if !is_general_if_body(then_body) {
        return false;
    }
    if let Some(else_body) = else_body {
        if else_body.is_empty() || !is_general_if_body(else_body) {
            return false;
        }
    }
    true
}

fn is_general_if_body(body: &[ASTNode]) -> bool {
    for stmt in body {
        match stmt {
            ASTNode::Assignment { .. }
            | ASTNode::Local { .. }
            | ASTNode::MethodCall { .. }
            | ASTNode::FunctionCall { .. }
            | ASTNode::Print { .. } => {}
            ASTNode::If { then_body, else_body, .. } => {
                if !is_general_if_block(then_body, else_body.as_ref()) {
                    return false;
                }
            }
            _ => return false,
        }
    }
    true
}

fn is_if_tail_exit_pair(stmt: &ASTNode, next: Option<&ASTNode>) -> bool {
    let ASTNode::If { then_body, else_body: None, .. } = stmt else {
        return false;
    };
    let Some(next) = next else {
        return false;
    };
    let Some(last) = then_body.last() else {
        return false;
    };

    match (last, next) {
        (ASTNode::Continue { .. }, ASTNode::Break { .. }) => true,
        (ASTNode::Break { .. }, ASTNode::Continue { .. }) => true,
        _ => false,
    }
}

fn is_exit_block(body: &[ASTNode]) -> bool {
    if body.is_empty() {
        return false;
    }
    for (idx, stmt) in body.iter().enumerate() {
        let is_last = idx + 1 == body.len();
        match stmt {
            ASTNode::Assignment { .. }
            | ASTNode::Local { .. }
            | ASTNode::MethodCall { .. }
            | ASTNode::FunctionCall { .. }
            | ASTNode::Print { .. } => {
                if is_last {
                    // Exit must be explicit at the end.
                    return false;
                }
            }
            ASTNode::Break { .. } | ASTNode::Continue { .. } => {
                if !is_last {
                    return false;
                }
            }
            ASTNode::Return { value, .. } => {
                // BoxCount scope: return(value) only (return without value is out-of-scope).
                if !is_last || value.is_none() {
                    return false;
                }
            }
            _ => return false,
        }
    }
    true
}

/// Build ElseExitMixedRecipe from else_body statements.
/// Returns None if the pattern doesn't match.
///
/// Allowed pattern:
/// - Mix of ExitIf (if blocks ending with break/continue/return) and PreludeStmt
/// - At least one ExitIf required
/// - ExitIf can be else-less: `if { continue }` is OK
///
/// CRITICAL: StmtRef indices are RELATIVE to else_body (0..else_body.len())
fn build_else_exit_mixed_recipe(
    else_body: &[ASTNode],
) -> Option<ElseExitMixedRecipe> {
    if else_body.is_empty() {
        return None;
    }

    let mut items = Vec::new();
    let mut has_exit_if = false;

    for (offset, stmt) in else_body.iter().enumerate() {
        // FIXED: Use offset directly (else_body is 0-indexed)
        match stmt {
            // Simple statements → PreludeStmt
            ASTNode::Assignment { .. }
            | ASTNode::Local { .. }
            | ASTNode::MethodCall { .. }
            | ASTNode::FunctionCall { .. }
            | ASTNode::Print { .. } => {
                items.push(ElseItem::PreludeStmt(StmtRef::new(offset)));
            }

            // If statement → check for exit-if pattern
            ASTNode::If { then_body, else_body: inner_else, .. } => {
                // Accept exit-if: if { exit } or if { exit } else { exit }
                let then_is_exit = is_exit_block(then_body);
                let else_is_exit = inner_else.as_ref().map_or(false, |e| is_exit_block(e));

                if then_is_exit {
                    // Then has exit
                    if let Some(_inner_else) = inner_else {
                        // Else exists: must also be exit-block
                        if !else_is_exit {
                            return None; // Mixed exit/non-exit → reject
                        }
                    }
                    // OK: then has exit, else (if present) also has exit
                    items.push(ElseItem::ExitIf(StmtRef::new(offset)));
                    has_exit_if = true;
                } else if else_is_exit {
                    // Then doesn't have exit, but else does → reject
                    return None;
                } else {
                    // Neither then nor else has exit → not an exit-if
                    return None;
                }
            }

            // Direct exit statements are NOT allowed (must be inside if)
            ASTNode::Break { .. } | ASTNode::Continue { .. } | ASTNode::Return { .. } => {
                return None;
            }

            _ => {
                // Unsupported statement type
                return None;
            }
        }
    }

    if !has_exit_if {
        return None; // Must have at least one ExitIf
    }

    Some(ElseExitMixedRecipe {
        else_body: RecipeBody::new(else_body.to_vec()),
        items,
    })
}

#[cfg(test)]
mod tests {
    use super::try_extract_loop_true_break_continue_facts;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::policies::BodyLoweringPolicy;

    fn bool_lit(value: bool) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Bool(value),
            span: Span::unknown(),
        }
    }

    fn int(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn binop(operator: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator,
            left: Box::new(left),
            right: Box::new(right),
            span: Span::unknown(),
        }
    }

    #[test]
    fn policy_exit_allowed_when_extended() {
        std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", "1");

        let condition = bool_lit(true);
        let body = vec![ASTNode::Return {
            value: Some(Box::new(int(1))),
            span: Span::unknown(),
        }];

        let facts = try_extract_loop_true_break_continue_facts(&condition, &body)
            .expect("no freeze")
            .expect("facts");

        assert!(matches!(
            facts.body_lowering_policy,
            BodyLoweringPolicy::ExitAllowed { .. }
        ));
        assert!(facts.body_exit_allowed.is_some());
    }

    #[test]
    fn policy_recipe_only_without_extended() {
        std::env::remove_var("HAKO_JOINIR_PLANNER_REQUIRED");

        let condition = bool_lit(true);
        let body = vec![ASTNode::If {
            condition: Box::new(binop(
                BinaryOperator::Equal,
                bool_lit(true),
                bool_lit(true),
            )),
            then_body: vec![ASTNode::Break {
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        }];

        let facts = try_extract_loop_true_break_continue_facts(&condition, &body)
            .expect("no freeze")
            .expect("facts");

        assert!(matches!(
            facts.body_lowering_policy,
            BodyLoweringPolicy::RecipeOnly
        ));
        assert!(facts.body_exit_allowed.is_none());
    }
}
