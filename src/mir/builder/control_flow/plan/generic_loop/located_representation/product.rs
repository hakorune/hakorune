use crate::mir::builder::control_flow::facts::no_exit_block::NoExitBlockRecipe;
use crate::mir::builder::control_flow::plan::generic_loop::facts_types::GenericLoopV1ExtractionV1;
use crate::mir::builder::control_flow::plan::recipe_tree::{ExitKind, IfContractKind};
use crate::mir::callable_result_representation::{
    LegacyBodyInputV1, LegacyExprInputV1, LegacyStmtInputV1,
};

#[derive(Debug)]
pub(in crate::mir::builder) struct VerifiedLocatedGenericLoopBodyRepresentationV1<'plan> {
    pub(super) loop_root: LegacyStmtInputV1<'plan>,
    pub(super) condition: LegacyExprInputV1<'plan>,
    pub(super) extraction: GenericLoopV1ExtractionV1,
    pub(super) mode: VerifiedLocatedGenericLoopBodyModeV1<'plan>,
}

#[derive(Debug)]
pub(super) enum VerifiedLocatedGenericLoopBodyModeV1<'plan> {
    DirectRecipeOnly {
        prefix: Box<[LegacyStmtInputV1<'plan>]>,
        cleanup: LegacyStmtInputV1<'plan>,
    },
    ExitAllowedRecipe {
        root: VerifiedLocatedRecipeBlockV1<'plan>,
        cleanup: LegacyStmtInputV1<'plan>,
    },
}

#[derive(Debug)]
pub(super) struct VerifiedLocatedRecipeBlockV1<'plan> {
    pub(super) items: Box<[VerifiedLocatedRecipeItemV1<'plan>]>,
}

#[derive(Debug)]
pub(super) enum VerifiedLocatedRecipeItemV1<'plan> {
    OpaqueStmt {
        source: LegacyStmtInputV1<'plan>,
    },
    OpaqueExit {
        source: LegacyStmtInputV1<'plan>,
        kind: ExitKind,
    },
    ExplicitIfV2 {
        source: LegacyStmtInputV1<'plan>,
        condition: LegacyExprInputV1<'plan>,
        then_body: LegacyBodyInputV1<'plan>,
        else_body: Option<LegacyBodyInputV1<'plan>>,
        contract: IfContractKind,
        then_block: Box<VerifiedLocatedRecipeBlockV1<'plan>>,
        else_block: Option<Box<VerifiedLocatedRecipeBlockV1<'plan>>>,
    },
    StmtWrappedJoinIf {
        bridge: VerifiedStmtWrappedJoinIfV1<'plan>,
    },
}

#[derive(Debug)]
pub(super) struct VerifiedStmtWrappedJoinIfV1<'plan> {
    pub(super) source_if: LegacyStmtInputV1<'plan>,
    pub(super) condition: LegacyExprInputV1<'plan>,
    pub(super) then_body: LegacyBodyInputV1<'plan>,
    pub(super) else_body: Option<LegacyBodyInputV1<'plan>>,
    pub(super) singleton_recipe: NoExitBlockRecipe,
    pub(super) singleton_root: VerifiedLocatedJoinIfRootV1<'plan>,
}

#[derive(Debug)]
pub(super) struct VerifiedLocatedJoinIfRootV1<'plan> {
    pub(super) then_block: Box<VerifiedLocatedRecipeBlockV1<'plan>>,
    pub(super) else_block: Option<Box<VerifiedLocatedRecipeBlockV1<'plan>>>,
}
