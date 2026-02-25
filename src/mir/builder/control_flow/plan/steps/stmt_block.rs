//! Step: Lower a block of statements with a provided lowering function.
//! (plan::steps SSOT)

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::LoweredRecipe;

/// Lower each stmt in `block` and append resulting plans.
pub fn lower_stmt_block<F>(
    block: &[ASTNode],
    mut lower_stmt: F,
) -> Result<Vec<LoweredRecipe>, String>
where
    F: FnMut(&ASTNode) -> Result<Vec<LoweredRecipe>, String>,
{
    let mut plans = Vec::new();
    for stmt in block {
        plans.extend(lower_stmt(stmt)?);
    }
    Ok(plans)
}
