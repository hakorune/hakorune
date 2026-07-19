//! Passive O0 located GenericLoopV1 representation seal.

mod direct_preflight;
mod error;
mod lowering_view;
mod product;
mod recipe_seal;

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::generic_loop_canon::StepPlacement;
use crate::mir::builder::control_flow::plan::generic_loop::facts::extract::try_extract_generic_loop_v1;
use crate::mir::builder::control_flow::plan::generic_loop::facts_types::{
    GenericLoopCarrierRoleV1, GenericLoopV1StepDispositionV1,
};
use crate::mir::builder::control_flow::plan::LocatedLoopPlanExpressionPortV1;
use crate::mir::callable_result_representation::LegacyStmtInputV1;
use crate::mir::policies::BodyLoweringPolicy;
use crate::mir::resolved_semantics::{BodyChildRoleV1, ExprChildRoleV1};

pub(in crate::mir::builder) use direct_preflight::{
    PreparedLocatedGenericLoopDirectExecutionV1, VerifiedLocatedGenericLoopDirectPreflightV1,
};
pub(in crate::mir::builder) use error::LocatedGenericLoopRepresentationErrorV1;
pub(in crate::mir::builder) use lowering_view::{
    VerifiedLocatedDirectBodyLoweringViewV1, VerifiedLocatedGenericLoopLoweringModeV1,
    VerifiedLocatedGenericLoopLoweringViewV1, VerifiedLocatedRecipeBlockLoweringViewV1,
    VerifiedLocatedRecipeItemLoweringViewV1, VerifiedStmtWrappedJoinIfLoweringViewV1,
};
use product::VerifiedLocatedGenericLoopBodyModeV1;
pub(in crate::mir::builder) use product::VerifiedLocatedGenericLoopBodyRepresentationV1;
use recipe_seal::{reject_unsupported_nested_statements, seal_recipe_block, RecipeSealDomainV1};

impl<'plan> VerifiedLocatedGenericLoopBodyRepresentationV1<'plan> {
    pub(in crate::mir::builder) fn verify_located_loop(
        port: &LocatedLoopPlanExpressionPortV1<'plan>,
        loop_root: LegacyStmtInputV1<'plan>,
    ) -> Result<Self, LocatedGenericLoopRepresentationErrorV1> {
        port.require_exact_stmt(&loop_root)?;
        if !matches!(loop_root.node(), ASTNode::Loop { .. }) {
            return Err(LocatedGenericLoopRepresentationErrorV1::NotLoopRoot);
        }
        let condition =
            port.exact_child_expr_from_stmt(&loop_root, ExprChildRoleV1::LoopCondition)?;
        let body = port.exact_child_body_from_stmt(&loop_root, BodyChildRoleV1::LoopBody)?;
        reject_unsupported_nested_statements(body.statements())?;

        let extraction = try_extract_generic_loop_v1(condition.node(), body.statements())?
            .ok_or(LocatedGenericLoopRepresentationErrorV1::NoGenericLoopV1Extraction)?;
        if extraction.facts().carrier_role != GenericLoopCarrierRoleV1::NumericProgression {
            return Err(LocatedGenericLoopRepresentationErrorV1::UnsupportedCarrierRole);
        }
        let canonical_body_len = match extraction.step() {
            GenericLoopV1StepDispositionV1::NumericProgression {
                placement: StepPlacement::Last,
                canonical_body_len,
            } => *canonical_body_len,
            GenericLoopV1StepDispositionV1::NumericProgression { .. } => {
                return Err(LocatedGenericLoopRepresentationErrorV1::UnsupportedStepPlacement)
            }
            GenericLoopV1StepDispositionV1::BodyManagedState => {
                return Err(LocatedGenericLoopRepresentationErrorV1::UnsupportedCarrierRole)
            }
        };
        let exact_len = body.statements().len();
        if exact_len == 0 {
            return Err(LocatedGenericLoopRepresentationErrorV1::EmptyBody);
        }
        let _exact_len_u32 = u32::try_from(exact_len)
            .map_err(|_| LocatedGenericLoopRepresentationErrorV1::BodyLengthOverflow)?;
        if canonical_body_len != exact_len {
            return Err(
                LocatedGenericLoopRepresentationErrorV1::CanonicalBodyLengthMismatch {
                    exact: exact_len,
                    canonical: canonical_body_len,
                },
            );
        }

        let cleanup = port.exact_body_stmt(&body, exact_len - 1)?;
        let mode = match extraction.facts().body_lowering_policy {
            BodyLoweringPolicy::RecipeOnly => {
                let mut prefix = Vec::with_capacity(exact_len - 1);
                for ordinal in 0..exact_len - 1 {
                    prefix.push(port.exact_body_stmt(&body, ordinal)?);
                }
                VerifiedLocatedGenericLoopBodyModeV1::DirectRecipeOnly {
                    prefix: prefix.into_boxed_slice(),
                    cleanup,
                }
            }
            BodyLoweringPolicy::ExitAllowed { .. } => {
                let recipe = extraction
                    .facts()
                    .body_exit_allowed
                    .as_ref()
                    .ok_or(LocatedGenericLoopRepresentationErrorV1::MissingExitAllowedRecipe)?;
                if recipe.block.items.len() != exact_len - 1 {
                    return Err(
                        LocatedGenericLoopRepresentationErrorV1::RecipeItemCountMismatch {
                            exact: exact_len - 1,
                            recipe: recipe.block.items.len(),
                        },
                    );
                }
                let root = seal_recipe_block(
                    port,
                    &recipe.arena,
                    &recipe.block,
                    &body,
                    exact_len - 1,
                    RecipeSealDomainV1::ExitAllowed,
                )?;
                VerifiedLocatedGenericLoopBodyModeV1::ExitAllowedRecipe { root, cleanup }
            }
        };

        Ok(Self {
            loop_root,
            condition,
            extraction,
            mode,
        })
    }
}

#[cfg(test)]
mod actual_parser_tests;
#[cfg(test)]
mod lowering_view_tests;
#[cfg(test)]
mod site_projection_tests;
#[cfg(test)]
mod tests;
