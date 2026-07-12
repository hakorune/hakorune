use super::{ChildRoleV0, PathV0, WireNodeKindV0};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AtomValueV0 {
    I64(i64),
    Bool(bool),
    Text(String),
    Null,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotNodeV0 {
    pub path: PathV0,
    pub kind: WireNodeKindV0,
    pub atoms: Vec<(&'static str, AtomValueV0)>,
    pub children: Vec<(ChildRoleV0, usize)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedBodyAnalysisSnapshotV0 {
    pub schema_version: u32,
    pub source_program_version: i32,
    pub nodes: Vec<SnapshotNodeV0>,
    pub node_count: usize,
    pub max_depth_observed: usize,
}

impl BoundedBodyAnalysisSnapshotV0 {
    pub const SCHEMA_VERSION: u32 = 0;

    pub fn new(
        source_program_version: i32,
        nodes: Vec<SnapshotNodeV0>,
        max_depth_observed: usize,
    ) -> Self {
        Self {
            schema_version: Self::SCHEMA_VERSION,
            source_program_version,
            node_count: nodes.len(),
            nodes,
            max_depth_observed,
        }
    }
}
