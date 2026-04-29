//! Recipe Contract types (Recipe-first migration Phase A).
//!
//! These types define the "contract" between Facts and Lower.
//! `RecipeMatcher` builds this contract; block verification remains the
//! release-mode acceptance boundary.

/// Recipe contract kind (structure-only, no pattern names).
#[derive(Debug, Clone)]
pub(in crate::mir::builder) enum RecipeContractKind {
    /// Loop with optional exit (break/continue/return).
    LoopWithExit {
        has_break: bool,
        has_continue: bool,
        has_return: bool,
    },
}

/// Recipe contract (structure-only).
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct RecipeContract {
    pub kind: RecipeContractKind,
}
