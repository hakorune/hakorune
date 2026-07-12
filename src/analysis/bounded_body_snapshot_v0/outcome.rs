use super::{BoundedBodyAnalysisSnapshotV0, PathV0};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisIssueV0 {
    pub path: PathV0,
    pub node_kind: Option<String>,
    pub reason: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoundedBodyAnalysisOutcomeV0 {
    Ready(BoundedBodyAnalysisSnapshotV0),
    Unsupported(AnalysisIssueV0),
    InvalidInput(AnalysisIssueV0),
}
