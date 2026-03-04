//! Recipe Contract types (Recipe-first migration Phase A).
//!
//! These types define the "contract" between Facts and Lower.
//! The Verifier is the only trusted boundary.
//!
//! NOTE: Not wired into routing yet. Phase B will connect.

/// Recipe contract kind (structure-only, no pattern names).
#[derive(Debug, Clone)]
pub(in crate::mir::builder) enum RecipeContractKind {
    /// Loop with optional exit (break/continue/return).
    LoopWithExit {
        has_break: bool,
        has_continue: bool,
        has_return: bool,
    },
    /// Loop with carrier merge (PHI).
    LoopWithCarrier {
        carrier_count: usize,
    },
    /// Simple statement sequence.
    StmtSeq,
}

/// Exit requirement for recipe contract.
#[derive(Debug, Clone)]
pub(in crate::mir::builder) enum ExitRequirement {
    BreakRequired,
    ContinueAllowed,
    ReturnAllowed,
}

/// Statement constraint for recipe contract.
#[derive(Debug, Clone)]
pub(in crate::mir::builder) enum StmtConstraint {
    Any,
    NoNestedLoop,
    StmtOnly,
}

/// Recipe contract (structure + requirements).
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct RecipeContract {
    pub kind: RecipeContractKind,
    pub required_exits: Vec<ExitRequirement>,
    pub allowed_stmts: StmtConstraint,
}
