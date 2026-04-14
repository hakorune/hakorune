use crate::ast::ASTNode;
use crate::mir::policies::BodyLoweringPolicy;

use super::recipe::LoopScanMethodsBlockV0Recipe;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopScanMethodsBlockV0Facts {
    pub loop_var: String,
    pub limit_var: String,
    pub condition: ASTNode,
    pub body_lowering_policy: BodyLoweringPolicy,
    pub recipe: LoopScanMethodsBlockV0Recipe,
}
