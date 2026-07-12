//! Immutable schema for the bounded ProgramV0 wire observation quotient.

mod budget;
mod outcome;
mod path;
mod program_v0_body_view;
mod schema;
mod snapshot;
mod strict_json;

pub use budget::{BoundedBodyBudgetV0, BudgetLimitV0};
pub use outcome::{AnalysisIssueV0, BoundedBodyAnalysisOutcomeV0};
pub use path::{PathFieldV0, PathSegmentV0, PathV0};
pub use program_v0_body_view::{
    read_program_v0_body, ProgramV0BodyViewError, ValidatedProgramV0BodyView,
};
pub use schema::{
    AtomKeyV0, AtomSpecV0, AtomValueKindV0, BinaryOperatorV0, ChildCardinalityV0, ChildRoleV0,
    ChildSpecV0, CompareOperatorV0, DepthConventionV0, LogicalOperatorV0, SnapshotLimitsV0,
    TextClassV0, WireClassificationV0, WireExprKindV0, WireNodeKindV0, WireStmtKindV0,
};
pub use snapshot::{AtomValueV0, BoundedBodyAnalysisSnapshotV0, SnapshotNodeV0};

#[cfg(test)]
mod tests;
