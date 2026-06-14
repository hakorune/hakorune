//! Body lowering policy for loop recipes (SSOT).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyLoweringPolicy {
    /// Lower recipe items exactly once, in recipe order.
    ///
    /// This mode forbids route-level whole-body `ExitAllowed` fallback. Item
    /// lowering may use item-local fallback only when it preserves the selected
    /// item position and binding state.
    RecipeOnly,
    /// Lower the verified whole body as an exit-allowed block.
    ExitAllowed { allow_join_if: bool },
}

impl BodyLoweringPolicy {
    pub fn expect_recipe_only(self, box_tag: &str, ctx: &str) -> Result<(), String> {
        match self {
            BodyLoweringPolicy::RecipeOnly => Ok(()),
            BodyLoweringPolicy::ExitAllowed { .. } => Err(format!(
                "[freeze:contract]{} body_lowering_policy=ExitAllowed: ctx={}",
                box_tag, ctx
            )),
        }
    }

    pub fn expect_exit_allowed(self, box_tag: &str, ctx: &str) -> Result<(), String> {
        match self {
            BodyLoweringPolicy::ExitAllowed { .. } => Ok(()),
            BodyLoweringPolicy::RecipeOnly => Err(format!(
                "[freeze:contract]{} body_lowering_policy=RecipeOnly: ctx={}",
                box_tag, ctx
            )),
        }
    }
}
