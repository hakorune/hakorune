//! Immutable source result outcomes and their legacy safe projection.
//! Only the enclosing source solver constructs rows; lookup is passive.
use super::OrdinaryNewResultClassV1;
use crate::mir::resolved_semantics::OwnedExprSiteV1;
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ResultValueOriginV1 {
    Null,
    Fresh(Box<str>),
    ForwardFormal { ordinal: u32 },
}

#[derive(Debug)]
pub(crate) struct ResultExitOriginV1 {
    pub(super) site: OwnedExprSiteV1,
    pub(super) alternatives: BTreeSet<ResultValueOriginV1>,
}
impl ResultExitOriginV1 {
    pub(crate) fn site(&self) -> &OwnedExprSiteV1 {
        &self.site
    }
    pub(crate) fn alternatives(&self) -> &BTreeSet<ResultValueOriginV1> {
        &self.alternatives
    }
}

#[derive(Debug)]
pub(super) struct SourceResultRowV1 {
    pub(super) exits: Box<[ResultExitOriginV1]>,
    pub(super) projection: Option<OrdinaryNewResultClassV1>,
}

/// One solver product. Legacy APIs cannot expose mixed/null-only rows.
#[derive(Debug, Default)]
pub(crate) struct VerifiedSourceCallableResultFactsV1 {
    rows: BTreeMap<CanonicalSameModuleCallableKeyV1, SourceResultRowV1>,
}
impl VerifiedSourceCallableResultFactsV1 {
    pub(crate) fn new() -> Self {
        Self::default()
    }
    pub(crate) fn get(
        &self,
        key: &CanonicalSameModuleCallableKeyV1,
    ) -> Option<&OrdinaryNewResultClassV1> {
        self.rows.get(key)?.projection.as_ref()
    }
    pub(crate) fn contains_key(&self, key: &CanonicalSameModuleCallableKeyV1) -> bool {
        self.get(key).is_some()
    }
    pub(crate) fn iter(
        &self,
    ) -> impl Iterator<Item = (&CanonicalSameModuleCallableKeyV1, &OrdinaryNewResultClassV1)> {
        self.rows
            .iter()
            .filter_map(|(key, row)| row.projection.as_ref().map(|value| (key, value)))
    }
    pub(crate) fn keys(&self) -> impl Iterator<Item = &CanonicalSameModuleCallableKeyV1> {
        self.iter().map(|(key, _)| key)
    }
    pub(crate) fn outcomes(
        &self,
        key: &CanonicalSameModuleCallableKeyV1,
    ) -> Option<&[ResultExitOriginV1]> {
        Some(&self.rows.get(key)?.exits)
    }
    pub(super) fn insert(&mut self, key: CanonicalSameModuleCallableKeyV1, row: SourceResultRowV1) {
        self.rows.insert(key, row);
    }
}
