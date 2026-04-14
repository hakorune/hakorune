//! NestedLoopFeature (depth<=1) for loop(true) normalization.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::features::nested_loop_depth1_preheader::apply_nested_loop_preheader_freshness;
use crate::mir::builder::control_flow::plan::features::nested_loop_depth1_route::dispatch_nested_loop_depth1_any;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;

pub(in crate::mir::builder) fn lower_nested_loop_depth1_any(
    builder: &mut MirBuilder,
    condition: &ASTNode,
    body: &[ASTNode],
    error_prefix: &str,
) -> Result<LoweredRecipe, String> {
    let plan = dispatch_nested_loop_depth1_any(builder, condition, body, error_prefix)?;
    Ok(apply_nested_loop_preheader_freshness(builder, plan))
}
