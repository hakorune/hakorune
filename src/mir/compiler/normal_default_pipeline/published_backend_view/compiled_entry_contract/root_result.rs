//! Process-entry result category from the retained source projection.
use super::*;

/// Backend-facing result category. Source terminal provenance stops here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CompiledEntryRootResultV1 {
    I64,
    Unit,
}

pub(in crate::mir::compiler::normal_default_pipeline::published_backend_view) fn root_result_category(
    result: FinalizedRootResultAbiV1,
) -> CompiledEntryRootResultV1 {
    match result {
        FinalizedRootResultAbiV1::CallReturn { .. }
        | FinalizedRootResultAbiV1::I64AddReturn { .. }
        | FinalizedRootResultAbiV1::IntegerLiteralReturn { .. }
        | FinalizedRootResultAbiV1::I64FieldReturn { .. }
        | FinalizedRootResultAbiV1::I64ScalarReturn { .. }
        | FinalizedRootResultAbiV1::MapGetReturn { .. } => CompiledEntryRootResultV1::I64,
        FinalizedRootResultAbiV1::UnitReturn { .. } => CompiledEntryRootResultV1::Unit,
    }
}
