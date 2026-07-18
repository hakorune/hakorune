use crate::mir::builder::control_flow::plan::{planner::Freeze, LoopPlanExpressionPortErrorV1};

#[derive(Debug)]
pub(in crate::mir::builder) enum LocatedGenericLoopRepresentationErrorV1 {
    Port(LoopPlanExpressionPortErrorV1),
    Extraction(Freeze),
    NotLoopRoot,
    NoGenericLoopV1Extraction,
    UnsupportedCarrierRole,
    UnsupportedStepPlacement,
    EmptyBody,
    BodyLengthOverflow,
    CanonicalBodyLengthMismatch { exact: usize, canonical: usize },
    UnsupportedNestedStatement,
    MissingExitAllowedRecipe,
    MissingRecipeBody,
    RecipeBodyLengthMismatch { exact: usize, recipe: usize },
    RecipeItemCountMismatch { exact: usize, recipe: usize },
    RecipeOrdinalMismatch { expected: usize, actual: usize },
    RecipeSourceKindMismatch,
    RecipeLoopUnsupported,
    RecipeContractMismatch,
    ExitKindMismatch,
    IfElsePresenceMismatch,
    WrappedJoinIfRecipeRejected,
    WrappedJoinIfRootCardinality,
    WrappedJoinIfRootNotJoin,
}

impl From<LoopPlanExpressionPortErrorV1> for LocatedGenericLoopRepresentationErrorV1 {
    fn from(value: LoopPlanExpressionPortErrorV1) -> Self {
        Self::Port(value)
    }
}

impl From<Freeze> for LocatedGenericLoopRepresentationErrorV1 {
    fn from(value: Freeze) -> Self {
        Self::Extraction(value)
    }
}
