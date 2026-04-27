//! Type definitions for generic loop facts

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::no_exit_block::NoExitBlockRecipe;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::ExitAllowedBlockRecipe;
use crate::mir::builder::control_flow::recipes::RecipeBody;
use crate::mir::policies::{BodyLoweringPolicy, CondProfile, GenericLoopV1ShapeId};

/// Facts extracted for generic loop v0 (ExitIf-capable, no carriers)
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct GenericLoopV0Facts {
    pub loop_var: String,
    pub condition: ASTNode,
    pub loop_increment: ASTNode,
    pub body: RecipeBody,
    pub cond_profile: CondProfile,
}

/// Facts extracted for generic loop v1
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct GenericLoopV1Facts {
    pub loop_var: String,
    pub condition: ASTNode,
    pub loop_increment: ASTNode,
    pub body: RecipeBody,
    pub shape_id: Option<GenericLoopV1ShapeId>,
    pub body_lowering_policy: BodyLoweringPolicy,
    pub body_exit_allowed: Option<ExitAllowedBlockRecipe>,
    pub body_no_exit: Option<NoExitBlockRecipe>,
    pub cond_profile: CondProfile,
}
