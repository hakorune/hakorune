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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalAliasLink {
    pub from: ObjectValueId,
    pub to: ObjectValueId,
    pub source_kind: LocalAliasSourceKind,
}

impl LocalAliasLink {
    pub const fn new(
        from: ObjectValueId,
        to: ObjectValueId,
        source_kind: LocalAliasSourceKind,
    ) -> Self {
        Self {
            from,
            to,
            source_kind,
        }
    }
}

pub fn linear_alias_chain_observations(
    root: ObjectValueId,
    alias_class: AliasClassId,
    links: &[LocalAliasLink],
) -> Vec<LocalAliasClassObservation> {
    let mut observations = Vec::with_capacity(links.len() + 1);
    observations.push(LocalAliasClassObservation {
        value_id: root,
        alias_class,
        source_kind: LocalAliasSourceKind::LocalAssignment,
    });
    observations.extend(links.iter().map(|link| LocalAliasClassObservation {
        value_id: link.to,
        alias_class,
        source_kind: link.source_kind,
    }));
    observations
}
