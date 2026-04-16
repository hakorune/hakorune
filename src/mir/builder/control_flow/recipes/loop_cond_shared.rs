//! Shared recipe vocabulary for loop_cond families.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::recipes::RecipeBody;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopCondRecipe<T> {
    pub body: RecipeBody,
    pub items: Vec<T>,
}

impl<T> LoopCondRecipe<T> {
    pub fn new(body: Vec<ASTNode>, items: Vec<T>) -> Self {
        Self {
            body: RecipeBody::new(body),
            items,
        }
    }
}
