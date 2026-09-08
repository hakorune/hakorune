//! Map responsibility successors from the existing caller ownership walk.
//! No physical ID, package descriptor, or runtime storage policy is issued here.
use super::*;
use crate::mir::resolved_semantics::{RegionId, ScopeId, SourcePathSegmentV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RootHomeFlow {
    pub(super) terminal: Result<Box<[BindingRefV1]>, HomePrefixUnavailableV1>,
    pub(super) maps: Vec<MapHomeObservation>,
}
impl RootHomeFlow {
    pub(crate) fn terminal_homes(&self) -> Result<&[BindingRefV1], &HomePrefixUnavailableV1> {
        self.terminal.as_deref()
    }
    pub(crate) fn maps(&self) -> &[MapHomeObservation] {
        &self.maps
    }
}

/// Every encountered Map requires the consumer, even if its ownership analysis
/// is unavailable. Unavailable is not an accepted partial transfer plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum MapHomeObservation {
    Complete(MapHomeFlow),
    Unavailable { site: OwnedExprSiteV1 },
}
impl MapHomeObservation {
    pub(crate) fn site(&self) -> &OwnedExprSiteV1 {
        match self {
            Self::Complete(row) => row.site(),
            Self::Unavailable { site } => site,
        }
    }
    pub(crate) fn complete(&self) -> Option<&MapHomeFlow> {
        match self {
            Self::Complete(row) => Some(row),
            Self::Unavailable { .. } => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MapHomeFlow {
    site: OwnedExprSiteV1,
    destination: BindingRefV1,
    source_scope: ScopeId,
    target_function: RegionId,
    allocation_fault: Box<[BindingRefV1]>,
    entries: Box<[MapHomeEntry]>,
}
impl MapHomeFlow {
    pub(crate) fn site(&self) -> &OwnedExprSiteV1 {
        &self.site
    }
    pub(crate) fn destination(&self) -> BindingRefV1 {
        self.destination
    }
    pub(crate) fn source_scope(&self) -> ScopeId {
        self.source_scope
    }
    pub(crate) fn target_function(&self) -> RegionId {
        self.target_function
    }
    /// No Map exists on allocation Fault. Normal acquires an empty construction Map.
    pub(crate) fn allocation_fault(&self) -> &[BindingRefV1] {
        &self.allocation_fault
    }
    pub(crate) fn entries(&self) -> &[MapHomeEntry] {
        &self.entries
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MapHomeEntry {
    site: SourceExprSiteV1,
    key: Box<str>,
    acquisition: OwnedExprSiteV1,
    binding: BindingRefV1,
    precommit_outer: Box<[BindingRefV1]>,
    committed_outer: Box<[BindingRefV1]>,
    live_before: Box<[SourceExprSiteV1]>,
    live_after: Box<[SourceExprSiteV1]>,
    displaced: Option<SourceExprSiteV1>,
}
impl MapHomeEntry {
    pub(crate) fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }
    pub(crate) fn key(&self) -> &str {
        &self.key
    }
    pub(crate) fn acquisition(&self) -> &OwnedExprSiteV1 {
        &self.acquisition
    }
    pub(crate) fn binding(&self) -> BindingRefV1 {
        self.binding
    }
    /// Key preparation / direct-local observation / install Fault: release any
    /// temporary native key, then end the construction Map's live_before,
    /// its native storage, then these outer Homes.
    /// The direct candidate has acquired no evaluation-owned Home.
    pub(crate) fn precommit_outer(&self) -> &[BindingRefV1] {
        &self.precommit_outer
    }
    /// Install Normal transfers once. Detached-old end Fault uses this SAME state;
    /// its end attempt is consumed on either outcome and must not be retried.
    pub(crate) fn committed_outer(&self) -> &[BindingRefV1] {
        &self.committed_outer
    }
    pub(crate) fn live_before(&self) -> &[SourceExprSiteV1] {
        &self.live_before
    }
    pub(crate) fn live_after(&self) -> &[SourceExprSiteV1] {
        &self.live_after
    }
    pub(crate) fn displaced(&self) -> Option<&SourceExprSiteV1> {
        self.displaced.as_ref()
    }
}

/// Build all successors before changing the caller's Normal-path local state.
/// Rejection cannot publish a partly transferred Map.
pub(super) fn observe_map<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &OwnedExprSiteV1,
    destination: BindingRefV1,
    keys: &[Box<str>],
    locals: &PrefixLocalFlow<'_>,
    homes: &[BindingRefV1],
    compatible: &mut impl FnMut(&OwnedExprSiteV1, BindingRefV1) -> Result<bool, E>,
) -> Result<Result<(MapHomeFlow, Vec<BindingRefV1>), HomePrefixUnavailableV1>, E> {
    let Some(shape) = input.body_shape() else {
        return Ok(Err(HomePrefixUnavailableV1::SourceMismatch));
    };
    let Ok((source_scope, target_function)) =
        crate::mir::resolved_control_flow::map_source_outward(input, site, destination)
    else {
        return Ok(Err(HomePrefixUnavailableV1::SourceMismatch));
    };
    let mut remaining = homes.to_vec();
    let mut used = std::collections::BTreeSet::new();
    let mut entries = Vec::new();
    // Install order is distinct from the Map's public sorted-key iteration.
    let mut live: Vec<(Box<str>, SourceExprSiteV1)> = Vec::new();
    for (ordinal, key) in keys.iter().enumerate() {
        let Ok(ordinal) = u32::try_from(ordinal) else {
            return Ok(Err(HomePrefixUnavailableV1::SourceMismatch));
        };
        let mut relations = shape.relations().iter().filter(|row| {
            row.parent() == site.site().node()
                && row.role() == &SourcePathSegmentV1::EntryValue(ordinal)
        });
        let Some(relation) = relations.next() else {
            return Ok(Err(HomePrefixUnavailableV1::SourceMismatch));
        };
        if relations.next().is_some() {
            return Ok(Err(HomePrefixUnavailableV1::SourceMismatch));
        }
        let child = relation.child();
        let Some((binding, acquisition)) = locals.direct_available_home(child) else {
            return Ok(Err(HomePrefixUnavailableV1::MapCandidateNotCovered(
                child.clone(),
            )));
        };
        if !used.insert(binding) || !compatible(acquisition, binding)? {
            return Ok(Err(HomePrefixUnavailableV1::MapCandidateNotCovered(
                child.clone(),
            )));
        }
        let Some(position) = remaining.iter().position(|home| *home == binding) else {
            return Ok(Err(HomePrefixUnavailableV1::SourceMismatch));
        };
        let precommit_outer = remaining.iter().rev().copied().collect();
        let live_before = live.iter().rev().map(|(_, site)| site.clone()).collect();
        remaining.remove(position);
        // Literal String key equality follows the existing canonical key law:
        // canonical integer spellings are unique; noncanonical spellings stay text.
        let displaced = live
            .iter()
            .position(|(old, _)| old == key)
            .map(|position| live.remove(position).1);
        live.push((key.clone(), child.clone()));
        entries.push(MapHomeEntry {
            site: child.clone(),
            key: key.clone(),
            acquisition: acquisition.clone(),
            binding,
            precommit_outer,
            committed_outer: remaining.iter().rev().copied().collect(),
            live_before,
            live_after: live.iter().rev().map(|(_, site)| site.clone()).collect(),
            displaced,
        });
    }
    Ok(Ok((
        MapHomeFlow {
            site: site.clone(),
            destination,
            source_scope,
            target_function,
            allocation_fault: homes.iter().rev().copied().collect(),
            entries: entries.into_boxed_slice(),
        },
        remaining,
    )))
}
