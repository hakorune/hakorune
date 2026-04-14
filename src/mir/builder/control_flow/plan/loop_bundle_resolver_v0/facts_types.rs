use crate::ast::ASTNode;

use super::recipe::LoopBundleResolverV0Recipe;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopBundleResolverV0Facts {
    pub loop_var: String,
    pub limit_var: String,
    pub condition: ASTNode,
    pub recipe: LoopBundleResolverV0Recipe,
}
