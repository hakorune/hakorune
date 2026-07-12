//! Immutable schema for the bounded ProgramV0 wire observation quotient.

mod budget;
mod outcome;
mod path;
mod schema;
mod snapshot;

pub use budget::{BoundedBodyBudgetV0, BudgetLimitV0};
pub use outcome::{AnalysisIssueV0, BoundedBodyAnalysisOutcomeV0};
pub use path::{PathSegmentV0, PathV0};
pub use schema::{
    BinaryOperatorV0, ChildRoleV0, CompareOperatorV0, LogicalOperatorV0, SnapshotLimitsV0,
    WireClassificationV0, WireExprKindV0, WireNodeKindV0, WireStmtKindV0,
};
pub use snapshot::{AtomValueV0, BoundedBodyAnalysisSnapshotV0, SnapshotNodeV0};

#[cfg(test)]
mod tests;
