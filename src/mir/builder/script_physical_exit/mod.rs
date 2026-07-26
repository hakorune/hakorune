//! Brand-free Script physical-exit kernel.
//!
//! `terminal` is the first cell: it preserves source-selected Script Unit
//! provenance while lowering physical operands. Completion, Return planning,
//! and commit are added by later cells in this module; Raw lifecycle remains
//! outside this boundary.

mod entry_session;
mod exit;
mod terminal;

pub(in crate::mir) use entry_session::{
    CompletedScriptPhysicalFunctionV1, OpenScriptPhysicalEntrySessionV1,
    ScriptPhysicalEntrySessionErrorV1,
};
pub(in crate::mir) use exit::{
    CompletedScriptBodyCompletionV1, CompletedScriptPhysicalExitCoreV1,
    PreparedScriptBodyCompletionV1, PreparedScriptPhysicalExitCoreV1,
    PreparedScriptPhysicalResultV1, ScriptPhysicalExitCommitV1, ScriptPhysicalExitErrorV1,
    ScriptPhysicalExitOpenContractV1, ScriptPhysicalResultV1, ScriptSourceCompletionV1,
};
pub(in crate::mir) use terminal::{
    LoweredScriptTerminalV1, LoweredScriptUnitPayloadV1, ScriptRecipeLoweringErrorV1,
    ScriptRecipeLoweringOperationV1,
};

#[cfg(test)]
mod tests;
