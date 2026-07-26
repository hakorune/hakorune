//! Brand-free Script physical-exit kernel.
//!
//! `terminal` is the first cell: it preserves source-selected Script Unit
//! provenance while lowering physical operands. Completion, Return planning,
//! and commit are added by later cells in this module; Raw lifecycle remains
//! outside this boundary.

mod terminal;

pub(in crate::mir) use terminal::{
    LoweredScriptTerminalV1, LoweredScriptUnitPayloadV1, ScriptRecipeLoweringErrorV1,
    ScriptRecipeLoweringOperationV1,
};

#[cfg(test)]
mod tests;
