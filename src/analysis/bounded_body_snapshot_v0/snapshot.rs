use super::{AtomKeyV0, ChildRoleV0, PathV0, WireNodeKindV0};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AtomValueV0 {
    I64(i64),
    Bool(bool),
    Text(String),
    Null,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotNodeV0 {
    path: PathV0,
    kind: WireNodeKindV0,
    atoms: Vec<(AtomKeyV0, AtomValueV0)>,
    children: Vec<(ChildRoleV0, usize)>,
}

impl SnapshotNodeV0 {
    pub(super) fn from_verified_parts(
        path: PathV0,
        kind: WireNodeKindV0,
        atoms: Vec<(AtomKeyV0, AtomValueV0)>,
        children: Vec<(ChildRoleV0, usize)>,
    ) -> Self {
        Self {
            path,
            kind,
            atoms,
            children,
        }
    }

    pub fn path(&self) -> &PathV0 {
        &self.path
    }

    pub fn kind(&self) -> WireNodeKindV0 {
        self.kind
    }

    pub fn atoms(&self) -> &[(AtomKeyV0, AtomValueV0)] {
        &self.atoms
    }

    pub fn children(&self) -> &[(ChildRoleV0, usize)] {
        &self.children
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedBodyAnalysisSnapshotV0 {
    schema_version: u32,
    source_program_version: i32,
    nodes: Vec<SnapshotNodeV0>,
    node_count: usize,
    max_depth_observed: usize,
}

impl BoundedBodyAnalysisSnapshotV0 {
    pub const SCHEMA_VERSION: u32 = 0;

    pub(super) fn from_verified_parts(
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

    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }
    pub fn source_program_version(&self) -> i32 {
        self.source_program_version
    }
    pub fn nodes(&self) -> &[SnapshotNodeV0] {
        &self.nodes
    }
    pub fn node_count(&self) -> usize {
        self.node_count
    }
    pub fn max_depth_observed(&self) -> usize {
        self.max_depth_observed
    }
}
