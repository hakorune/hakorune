//! Immutable schema for the bounded ProgramV0 wire observation quotient.

mod budget;
mod outcome;
mod path;
mod program_v0_body_view;
mod program_v0_snapshot_witness;
mod schema;
mod snapshot;
mod snapshot_builder;
mod strict_json;
mod validated_view;

pub use budget::{BoundedBodyBudgetV0, BudgetLimitV0};
pub use outcome::{AnalysisIssueV0, BoundedBodyAnalysisOutcomeV0};
pub use path::{PathFieldV0, PathSegmentV0, PathV0};
pub use program_v0_body_view::{
    read_program_v0_body, ProgramV0BodyViewError, ValidatedProgramV0BodyView,
};
pub(crate) use program_v0_snapshot_witness::build_snapshot_from_validated_view_v0;
pub use schema::{
    AtomKeyV0, AtomSpecV0, AtomValueKindV0, BinaryOperatorV0, ChildCardinalityV0, ChildRoleV0,
    ChildSpecV0, CompareOperatorV0, DepthConventionV0, LogicalOperatorV0, SnapshotLimitsV0,
    TextClassV0, WireClassificationV0, WireExprKindV0, WireNodeKindV0, WireStmtKindV0,
};
pub use snapshot::{AtomValueV0, BoundedBodyAnalysisSnapshotV0, SnapshotNodeV0};
pub use snapshot_builder::{SnapshotBuildErrorV0, SnapshotBuilderV0, SnapshotNodeIndexV0};
pub use validated_view::{ValidatedAtomValueV0, ValidatedNodeV0, ValidatedTextV0};

#[cfg(test)]
mod tests;
