//! Parser-private semantic outcome for one evaluated BuildGate.
//!
//! This is separate from `SourceBuildGateBranchV1`: that type represents an
//! actual child path segment and therefore has no `NoElse` variant. A receipt
//! may represent a no-else outcome even when no child path exists.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BuildGateSelectionOutcomeV1 {
    Then,
    Else,
    NoElse,
}
