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
    outer: Box<[MapOuterHome]>,
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
    pub(crate) fn allocation_fault(&self) -> impl DoubleEndedIterator<Item = BindingRefV1> + '_ {
        self.outer.iter().map(|home| home.binding)
    }
    /// Remaining outer Homes in cleanup order after exactly `installed` entries.
    /// Before entry i installs use i; after Normal (including displaced-end Fault)
    /// use i+1. Projection reads issued transfers, never reclassifies source keys.
    pub(crate) fn outer_after_installs(
        &self,
        installed: usize,
    ) -> Option<impl DoubleEndedIterator<Item = BindingRefV1> + '_> {
        if installed > self.entries.len() {
            return None;
        }
        Some(
            self.outer
                .iter()
                .filter(move |home| home.transferred_at.is_none_or(|entry| entry >= installed))
                .map(|home| home.binding),
        )
    }
    /// Source audit projection. Runtime cleanup consumes MapEnd, not this list.
    #[cfg(test)]
    pub(crate) fn live_after_installs(
        &self,
        installed: usize,
    ) -> Option<impl DoubleEndedIterator<Item = &SourceExprSiteV1> + '_> {
        let prefix = self.entries.get(..installed)?;
        Some(
            prefix
                .iter()
                .rev()
                .filter(move |entry| entry.replaced_at.is_none_or(|next| next >= installed))
                .map(|entry| &entry.site),
        )
    }
    pub(crate) fn entries(&self) -> &[MapHomeEntry] {
        &self.entries
    }
}

/// Initial reverse acquisition order, with the sole source-issued transfer point.
#[derive(Debug, Clone, PartialEq, Eq)]
struct MapOuterHome {
    binding: BindingRefV1,
    transferred_at: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MapHomeEntry {
    site: SourceExprSiteV1,
    key: Box<str>,
    acquisition: OwnedExprSiteV1,
    binding: BindingRefV1,
    replaced_at: Option<usize>,
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
    let mut entries: Vec<MapHomeEntry> = Vec::new();
    let mut outer: Vec<_> = homes
        .iter()
        .rev()
        .map(|binding| MapOuterHome {
            binding: *binding,
            transferred_at: None,
        })
        .collect();
    // Key comparison stays at this source issuer; consumers see issued deltas.
    let mut last_install = std::collections::BTreeMap::<Box<str>, usize>::new();
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
        remaining.remove(position);
        // Literal String key equality follows the existing canonical key law:
        // canonical integer spellings are unique; noncanonical spellings stay text.
        let entry_index = entries.len();
        let displaced = last_install.insert(key.clone(), entry_index).map(|prior| {
            entries[prior].replaced_at = Some(entry_index);
            entries[prior].site.clone()
        });
        // Binding membership was checked above; update the same initial Home row.
        let Some(home) = outer.iter_mut().find(|home| home.binding == binding) else {
            return Ok(Err(HomePrefixUnavailableV1::SourceMismatch));
        };
        home.transferred_at = Some(entry_index);
        entries.push(MapHomeEntry {
            site: child.clone(),
            key: key.clone(),
            acquisition: acquisition.clone(),
            binding,
            replaced_at: None,
            displaced,
        });
    }
    Ok(Ok((
        MapHomeFlow {
            site: site.clone(),
            destination,
            source_scope,
            target_function,
            outer: outer.into_boxed_slice(),
            entries: entries.into_boxed_slice(),
        },
        remaining,
    )))
}
