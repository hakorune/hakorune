use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::facts::no_exit_block::try_build_no_exit_block_recipe;
use crate::mir::builder::control_flow::facts::stmt_view::try_build_stmt_only_block_recipe;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::try_build_exit_allowed_block_recipe;
use crate::mir::builder::control_flow::recipes::loop_scan_methods_block_v0::LoopScanMethodsBlockV0Recipe;
use crate::mir::builder::control_flow::recipes::loop_scan_methods_block_v0::{
    LinearBlockRecipe, NestedLoopRecipe, ScanSegment,
};
use crate::mir::builder::control_flow::recipes::RecipeBody;
use crate::mir::policies::BodyLoweringPolicy;

pub(in crate::mir::builder) struct LoopScanMethodsBlockRecipeBuild {
    pub body_lowering_policy: BodyLoweringPolicy,
    pub recipe: LoopScanMethodsBlockV0Recipe,
}

pub(in crate::mir::builder) fn try_build_loop_scan_methods_block_recipe(
    body: &[ASTNode],
) -> Option<LoopScanMethodsBlockRecipeBuild> {
    const ALLOW_EXTENDED: bool = true;

    let mut flat = Vec::new();
    flatten_stmt_list(body, &mut flat);
    let segments = try_segmentize_stmt_list(&flat, ALLOW_EXTENDED)?;

    Some(LoopScanMethodsBlockRecipeBuild {
        body_lowering_policy: BodyLoweringPolicy::ExitAllowed {
            allow_join_if: false,
        },
        recipe: LoopScanMethodsBlockV0Recipe { segments },
    })
}

fn flatten_stmt_list(stmts: &[ASTNode], out: &mut Vec<ASTNode>) {
    for stmt in stmts {
        match stmt {
            ASTNode::Program { statements, .. } => flatten_stmt_list(statements, out),
            ASTNode::ScopeBox { body, .. } => flatten_stmt_list(body, out),
            _ => out.push(stmt.clone()),
        }
    }
}

fn build_linear_block_recipe(stmts: &[ASTNode], allow_extended: bool) -> Option<LinearBlockRecipe> {
    if let Some(recipe) = try_build_no_exit_block_recipe(stmts, allow_extended) {
        return Some(LinearBlockRecipe::NoExit(recipe));
    }
    if let Some(recipe) = try_build_exit_allowed_block_recipe(stmts, allow_extended) {
        return Some(LinearBlockRecipe::ExitAllowed(recipe));
    }
    None
}

fn try_segmentize_stmt_list(stmts: &[ASTNode], allow_extended: bool) -> Option<Vec<ScanSegment>> {
    if stmts.is_empty() {
        return None;
    }

    let mut segments = Vec::new();
    let mut cur_stmts: Vec<ASTNode> = Vec::new();
    let mut cur_recipe: Option<LinearBlockRecipe> = None;

    let flush_linear = |segments: &mut Vec<ScanSegment>,
                        cur_stmts: &mut Vec<ASTNode>,
                        cur_recipe: &mut Option<LinearBlockRecipe>| {
        if let Some(recipe) = cur_recipe.take() {
            segments.push(ScanSegment::Linear(recipe));
            cur_stmts.clear();
        }
    };

    for stmt in stmts {
        match stmt {
            ASTNode::Loop {
                condition, body, ..
            }
            | ASTNode::While {
                condition, body, ..
            } => {
                flush_linear(&mut segments, &mut cur_stmts, &mut cur_recipe);
                segments.push(ScanSegment::NestedLoop(NestedLoopRecipe {
                    cond_view: CondBlockView::from_expr(condition),
                    loop_stmt: stmt.clone(),
                    body: RecipeBody::new(body.to_vec()),
                    body_stmt_only: try_build_stmt_only_block_recipe(body),
                }));
            }
            _ => {
                let mut candidate = cur_stmts.clone();
                candidate.push(stmt.clone());

                if let Some(recipe) = build_linear_block_recipe(&candidate, allow_extended) {
                    cur_stmts = candidate;
                    cur_recipe = Some(recipe);
                    continue;
                }

                flush_linear(&mut segments, &mut cur_stmts, &mut cur_recipe);

                cur_stmts.push(stmt.clone());
                cur_recipe = build_linear_block_recipe(&cur_stmts, allow_extended);
                cur_recipe.as_ref()?;
            }
        }
    }

    flush_linear(&mut segments, &mut cur_stmts, &mut cur_recipe);
    if segments.is_empty() {
        return None;
    }
    Some(segments)
}
