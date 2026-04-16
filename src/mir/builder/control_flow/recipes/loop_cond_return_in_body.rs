//! Recipe surface for loop_cond_return_in_body (recipes-owned surface).

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::loop_cond_shared::LoopCondRecipe;
use crate::mir::builder::control_flow::recipes::refs::StmtRef;
use crate::mir::builder::control_flow::recipes::RecipeBody;

pub(in crate::mir::builder) type LoopCondReturnInBodyRecipe =
    LoopCondRecipe<LoopCondReturnInBodyItem>;

pub(in crate::mir::builder) fn build_loop_cond_return_in_body_recipe(
    body: Vec<ASTNode>,
) -> LoopCondReturnInBodyRecipe {
    let recipe_body = RecipeBody::new(body);
    let items = (0..recipe_body.len())
        .map(|idx| LoopCondReturnInBodyItem::Stmt(StmtRef::new(idx)))
        .collect();
    LoopCondRecipe {
        body: recipe_body,
        items,
    }
}

#[derive(Debug, Clone)]
pub(in crate::mir::builder) enum LoopCondReturnInBodyItem {
    Stmt(StmtRef),
}
