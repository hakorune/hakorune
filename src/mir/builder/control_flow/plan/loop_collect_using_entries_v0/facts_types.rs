use crate::ast::ASTNode;

use super::recipe::LoopCollectUsingEntriesV0Recipe;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopCollectUsingEntriesV0Facts {
    pub loop_var: String,
    pub limit_var: String,
    pub condition: ASTNode,
    pub recipe: LoopCollectUsingEntriesV0Recipe,
}
