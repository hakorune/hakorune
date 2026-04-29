use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::loop_update_analyzer::UpdateExpr;
use crate::mir::policies::post_loop_early_return_plan::PostLoopEarlyReturnPlan;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct BalancedDepthScanRecipe {
    pub depth_var: String,
    pub ch_var: String,
    pub open: String,
    pub close: String,
    pub depth_delta_name: String,
    pub depth_next_name: String,
}

#[derive(Debug, Clone)]
pub struct BalancedDepthScanPolicyResult {
    pub break_condition_node: ASTNode,
    pub allowed_body_locals_for_conditions: Vec<String>,
    pub carrier_updates_override: BTreeMap<String, UpdateExpr>,
    pub derived_recipe: BalancedDepthScanRecipe,
    pub post_loop_early_return: PostLoopEarlyReturnPlan,
}
