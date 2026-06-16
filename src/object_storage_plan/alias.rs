use super::ids::{AliasClassId, ObjectValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LocalAliasSourceKind {
    LocalAssignment,
    SsaCopy,
    Phi,
    Select,
    SimpleReceiverAlias,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalAliasClassObservation {
    pub value_id: ObjectValueId,
    pub alias_class: AliasClassId,
    pub source_kind: LocalAliasSourceKind,
}
