//! RecipeTree shared vocabulary (SSOT).
//!
//! This module intentionally holds only the *shared enums* used by RecipeBlock/Parts
//! without bringing in legacy `RecipeNode`.

/// IfMode: ExitIf vs ExitAll disambiguation.
#[derive(Debug, Clone, Copy)]
pub(in crate::mir::builder) enum IfMode {
    /// then is exit-only, else is optional (fallthrough allowed)
    ExitIf,
    /// then/else both exit-only, else required
    ExitAll,
    /// then=fallthrough (no exit), else=exit-only
    /// Used for patterns like: if cond { stmts } else { break }
    ElseOnlyExit,
}

/// Exit kind (Return/Break/Continue + depth).
#[derive(Debug, Clone, Copy)]
pub(in crate::mir::builder) enum ExitKind {
    Return,
    Break { depth: u32 },
    Continue { depth: u32 },
}
