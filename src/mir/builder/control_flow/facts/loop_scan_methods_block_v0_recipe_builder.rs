use crate::ast::ASTNode;
use crate::mir::builder::control_flow::recipes::loop_scan_methods_block_v0::LoopScanMethodsBlockV0Recipe;
use crate::mir::builder::control_flow::recipes::RecipeBody;
use crate::mir::policies::BodyLoweringPolicy;

use crate::mir::builder::control_flow::facts::loop_scan_methods_block_v0_helpers::{
    flatten_stmt_list, try_segmentize_stmt_list,
};

pub(in crate::mir::builder) struct LoopScanMethodsBlockRecipeBuild {
    pub body_lowering_policy: BodyLoweringPolicy,
    pub recipe: LoopScanMethodsBlockV0Recipe,
}

pub(in crate::mir::builder) fn try_build_loop_scan_methods_block_recipe(
    body: &[ASTNode],
    next_i_var: String,
) -> Option<LoopScanMethodsBlockRecipeBuild> {
    const ALLOW_EXTENDED: bool = true;

    let mut flat = Vec::new();
    flatten_stmt_list(body, &mut flat);
    let segments = try_segmentize_stmt_list(&flat, ALLOW_EXTENDED)?;

    Some(LoopScanMethodsBlockRecipeBuild {
        body_lowering_policy: BodyLoweringPolicy::ExitAllowed {
            allow_join_if: false,
        },
        recipe: LoopScanMethodsBlockV0Recipe {
            next_i_var,
            body: RecipeBody::new(body.to_vec()),
            segments,
        },
    })
}
