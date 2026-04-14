use crate::ast::ASTNode;
use crate::mir::policies::BodyLoweringPolicy;

use super::recipe::{LoopScanPhiSegment, LoopScanPhiVarsV0Recipe};

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopScanPhiVarsV0Facts {
    pub loop_var: String,
    pub limit_var: String,
    pub condition: ASTNode,
    pub body_lowering_policy: BodyLoweringPolicy,
    pub recipe: LoopScanPhiVarsV0Recipe,
    pub segments: Vec<LoopScanPhiSegment>,
}
