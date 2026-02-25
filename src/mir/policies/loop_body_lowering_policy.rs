//! Body lowering policy for loop recipes (SSOT).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyLoweringPolicy {
    RecipeOnly,
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
