//! Map responsibility successors from the existing caller ownership walk.
//! No physical ID, package descriptor, or runtime storage policy is issued here.
use super::local_call_flow::LocalCallObservationV1;
use super::terminal_relation::{array_literal_element_count, map_literal_keys};
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use crate::mir::resolved_semantics::{RegionId, ScopeId, SourcePathSegmentV1};

#[path = "home_lifecycle_prefix.rs"]
mod lifecycle_prefix;
pub(crate) use lifecycle_prefix::LifecyclePrefixSourceV1;

#[path = "home_map_value_flow.rs"]
mod value_flow;
use value_flow::{map_value_leaf, observe_array_elements, transfer_home_v1};
pub(super) use value_flow::{observe_field_get_local_v1, observe_field_install_v1};
pub(crate) use value_flow::{MapFieldGetLocalV1, MapFieldInstallV1};

/// One verified explicit exit's site-bound obligation: the live Homes this
/// path must release (reverse live order) plus the local-call sites the
/// path observed, for the consumer's per-exit binding-group expectation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RootHomeExitV1 {
    homes: Box<[LiveHomeV1]>,
    implicit_origin: Option<(crate::mir::compiler::located::SourceBodySiteV1, u32)>,
    covered_calls: Box<[OwnedExprSiteV1]>,
    covered_statements: Box<[SourceStmtSiteV1]>,
}

impl RootHomeExitV1 {
    /// Every source exit snapshots the same newest-first Home obligation.
    pub(super) fn from_live_v1(
        homes: &[LiveHomeV1],
        covered_calls: Box<[OwnedExprSiteV1]>,
        covered_statements: Box<[SourceStmtSiteV1]>,
    ) -> Self {
        Self::issue(
            homes.iter().rev().cloned().collect(),
            covered_calls,
            covered_statements,
        )
    }
    pub(super) fn issue(
        homes: Box<[LiveHomeV1]>,
        covered_calls: Box<[OwnedExprSiteV1]>,
        covered_statements: Box<[SourceStmtSiteV1]>,
    ) -> Self {
        Self {
            homes,
            implicit_origin: None,
            covered_calls,
            covered_statements,
        }
    }
    pub(super) fn implicit_from_live_v1(
        body: crate::mir::compiler::located::SourceBodySiteV1,
        end: u32,
        homes: &[LiveHomeV1],
        calls: Box<[OwnedExprSiteV1]>,
        statements: Box<[SourceStmtSiteV1]>,
    ) -> Self {
        let mut row = Self::from_live_v1(homes, calls, statements);
        row.implicit_origin = Some((body, end));
        row
    }
    pub(crate) fn implicit_origin(
        &self,
    ) -> Option<(&crate::mir::compiler::located::SourceBodySiteV1, u32)> {
        self.implicit_origin
            .as_ref()
            .map(|(body, end)| (body, *end))
    }
    pub(crate) fn homes(&self) -> &[LiveHomeV1] {
        &self.homes
    }
    pub(crate) fn covered_calls(&self) -> &[OwnedExprSiteV1] {
        &self.covered_calls
    }
    pub(crate) fn covers_statement(&self, site: &SourceStmtSiteV1) -> bool {
        self.covered_statements.contains(site)
    }
}

/// One normal-exit identity within its original function owner.
/// The implicit row retains original body/end provenance in its source payload.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum RootHomeExitKeyV1 {
    Explicit(SourceStmtSiteV1),
    ImplicitBodyEnd,
}
impl RootHomeExitKeyV1 {
    pub(crate) fn explicit_site(&self) -> Option<&SourceStmtSiteV1> {
        match self {
            Self::Explicit(site) => Some(site),
            Self::ImplicitBodyEnd => None,
        }
    }
}
impl PartialEq<SourceStmtSiteV1> for RootHomeExitKeyV1 {
    fn eq(&self, site: &SourceStmtSiteV1) -> bool {
        self.explicit_site() == Some(site)
    }
}
impl From<SourceStmtSiteV1> for RootHomeExitKeyV1 {
    fn from(site: SourceStmtSiteV1) -> Self {
        Self::Explicit(site)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RootHomeFlow {
    pub(super) named_arrays: Vec<super::NamedArrayHomeObservationV1>,
    /// One site-bound obligation per verified explicit exit. A completion
    /// with no statement site (implicit body end) keeps the obligation
    /// uncovered instead of borrowing another exit's evidence.
    pub(super) exits: BTreeMap<RootHomeExitKeyV1, Result<RootHomeExitV1, HomePrefixUnavailableV1>>,
    pub(super) maps: Vec<MapHomeObservation>,
    pub(super) local_calls: Vec<LocalCallObservationV1>,
    pub(super) expression_plans: Vec<HomeExpressionPlanV1>,
}
impl RootHomeFlow {
    pub(crate) fn field_array_loop_sources(
        &self,
    ) -> impl Iterator<Item = &std::rc::Rc<crate::mir::builder::VerifiedNoExitArrayHomeAfterV1>>
    {
        self.named_arrays
            .iter()
            .filter_map(super::NamedArrayHomeObservationV1::field_loop_proof)
    }
    pub(crate) fn named_arrays(&self) -> &[super::NamedArrayHomeObservationV1] {
        &self.named_arrays
    }
    pub(crate) fn statement_prefix_unavailable(&self, site: &SourceStmtSiteV1) -> bool {
        self.exits.values().any(|row| {
            matches!(row,
            Err(HomePrefixUnavailableV1::PrefixNotCovered(first)) if first == site)
        })
    }
    /// The sole exit's homes — only when the flow observed exactly one
    /// explicit site with no uncovered implicit end. Multi-exit functions
    /// must read `exit_row` per site.
    pub(crate) fn terminal_homes(&self) -> Result<&[LiveHomeV1], &HomePrefixUnavailableV1> {
        if self.exits.contains_key(&RootHomeExitKeyV1::ImplicitBodyEnd) || self.exits.len() != 1 {
            return Err(&HomePrefixUnavailableV1::TerminalNotCovered);
        }
        self.exits
            .values()
            .next()
            .expect("one exit")
            .as_ref()
            .map(RootHomeExitV1::homes)
    }

    /// The site-bound obligation for one verified explicit exit.
    pub(crate) fn exit_row(
        &self,
        site: &SourceStmtSiteV1,
    ) -> Option<Result<&RootHomeExitV1, &HomePrefixUnavailableV1>> {
        self.exits
            .get(&RootHomeExitKeyV1::Explicit(site.clone()))
            .map(|row| row.as_ref())
    }

    /// Every verified explicit exit has a proven obligation and no exit
    /// lacks a site — the only state where per-exit consumption may run.
    pub(crate) fn all_exits_ready(&self) -> bool {
        !self.exits.contains_key(&RootHomeExitKeyV1::ImplicitBodyEnd)
            && !self.exits.is_empty()
            && self.exits.values().all(|row| row.is_ok())
    }
    pub(crate) fn implicit_exit(
        &self,
    ) -> Option<Result<&RootHomeExitV1, &HomePrefixUnavailableV1>> {
        self.exits
            .get(&RootHomeExitKeyV1::ImplicitBodyEnd)
            .map(Result::as_ref)
    }
    pub(crate) fn maps(&self) -> &[MapHomeObservation] {
        &self.maps
    }
    pub(crate) fn local_calls(&self) -> &[LocalCallObservationV1] {
        &self.local_calls
    }
}

/// Every encountered Map requires the consumer, even if its ownership analysis
/// is unavailable. Unavailable is not an accepted partial transfer plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum MapHomeObservation {
    Complete(MapHomeFlow),
    /// Normal source delta for a checked procedural field install, not a Map allocation.
    FieldInstall(std::rc::Rc<MapFieldInstallV1>),
    FieldGet(std::rc::Rc<MapFieldGetLocalV1>),
    Unavailable {
        site: OwnedExprSiteV1,
    },
}
impl MapHomeObservation {
    pub(crate) fn site(&self) -> &OwnedExprSiteV1 {
        match self {
            Self::Complete(row) => row.site(),
            Self::FieldInstall(row) => row.site(),
            Self::FieldGet(row) => row.site(),
            Self::Unavailable { site } => site,
        }
    }
    pub(crate) fn complete(&self) -> Option<&MapHomeFlow> {
        match self {
            Self::Complete(row) => Some(row),
            Self::FieldGet(_) | Self::FieldInstall(_) | Self::Unavailable { .. } => None,
        }
    }
}

/// Where the constructed Map flows. A `local x = %{...}` initializer binds a
/// Local destination; a `return %{...}` transfers construction to the caller
/// through the function-return boundary — recorded exactly, never minted as a
/// binding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum MapDestinationV1 {
    LocalBinding(BindingRefV1),
    ReturnBoundary(SourceStmtSiteV1),
    /// A `%{...}` literal bound to a parent map's `EntryValue(ordinal)` slot.
    /// The slot is identified by the parent's exact map site and ordinal; the
    /// row's own `site` is the EntryValue child site.
    EntrySlot {
        parent_map: OwnedExprSiteV1,
        ordinal: u32,
    },
    /// A `%{...}` literal bound to a call's `Argument(ordinal)` slot. The
    /// slot is identified by the parent call's exact site and ordinal; the
    /// row's own `site` is the Argument child site. Destination evidence
    /// only — argument transfer semantics stay unclaimed.
    CallArgument {
        call: OwnedExprSiteV1,
        ordinal: u32,
    },
    /// A `%{...}` literal contained at an arbitrary sealed child position:
    /// `parent.node + role` is the map's exact site. Used for containment
    /// families without a more specific destination (array elements, call
    /// arguments under non-call-row parents, deeper nests). Destination
    /// evidence only.
    ContainedIn {
        parent: OwnedExprSiteV1,
        role: SourcePathSegmentV1,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MapHomeFlow {
    site: OwnedExprSiteV1,
    destination: MapDestinationV1,
    source_scope: ScopeId,
    target_function: RegionId,
    outer: Box<[MapOuterHome]>,
    entries: Box<[MapHomeEntry]>,
}
impl MapHomeFlow {
    pub(crate) fn site(&self) -> &OwnedExprSiteV1 {
        &self.site
    }
    pub(crate) fn destination(&self) -> &MapDestinationV1 {
        &self.destination
    }
    /// The bound local when the destination is a `local x = %{...}`
    /// initializer; `None` for return-boundary maps.
    pub(crate) fn local_binding(&self) -> Option<BindingRefV1> {
        match self.destination {
            MapDestinationV1::LocalBinding(binding) => Some(binding),
            MapDestinationV1::ReturnBoundary(_)
            | MapDestinationV1::EntrySlot { .. }
            | MapDestinationV1::CallArgument { .. }
            | MapDestinationV1::ContainedIn { .. } => None,
        }
    }
    pub(crate) fn source_scope(&self) -> ScopeId {
        self.source_scope
    }
    pub(crate) fn target_function(&self) -> RegionId {
        self.target_function
    }
    /// No Map exists on allocation Fault. Normal acquires an empty construction Map.
    pub(crate) fn allocation_fault(&self) -> impl DoubleEndedIterator<Item = LiveHomeV1> + '_ {
        self.outer.iter().map(|home| home.home.clone())
    }
    /// Remaining outer Homes in cleanup order after exactly `installed` entries.
    /// Before entry i installs use i; after Normal (including displaced-end Fault)
    /// use i+1. Projection reads issued transfers, never reclassifies source keys.
    pub(crate) fn outer_after_installs(
        &self,
        installed: usize,
    ) -> Option<impl DoubleEndedIterator<Item = LiveHomeV1> + '_> {
        if installed > self.entries.len() {
            return None;
        }
        Some(
            self.outer
                .iter()
                .filter(move |home| home.transferred_at.is_none_or(|entry| entry >= installed))
                .map(|home| home.home.clone()),
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
    home: LiveHomeV1,
    transferred_at: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MapHomeEntry {
    site: SourceExprSiteV1,
    key: Box<str>,
    ownership: MapEntryOwnership,
    replaced_at: Option<usize>,
    displaced: Option<SourceExprSiteV1>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum MapEntryOwnership {
    Value(MapValueSource),
    TransferHome {
        acquisition: OwnedExprSiteV1,
        binding: BindingRefV1,
    },
    /// The entry value is itself a `%{...}` literal; the child's own flow row
    /// carries the construction facts. `site` on the entry is the child site.
    NestedMap,
    /// The entry value is a `[...]` literal; each `Element(ordinal)` child is
    /// classified through the shared element chain — a leaf source or a
    /// nested `[...]` literal. Element Home transfers and `%{...}` elements
    /// stay uncovered at this boundary.
    NestedArray {
        elements: Box<[ArrayElementSource]>,
    },
}
/// A sealed array-element child with its source classification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ArrayElementSource {
    site: SourceExprSiteV1,
    kind: ArrayElementKindV1,
}
/// Element classes admitted inside a `[...]` map entry value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ArrayElementKindV1 {
    /// A leaf source — the same classes entry values admit.
    Leaf(MapValueSource),
    /// A nested `[...]` literal; its own `Element(ordinal)` children are
    /// classified recursively. No flow row — arrays are not MapLiteral rows.
    NestedArray(Box<[ArrayElementSource]>),
    /// A nested `%{...}` literal. The child emits its own `MapHomeFlow`
    /// row, while this element keeps the exact source identity needed to
    /// follow an Array -> Map read without re-reading AST or names.
    NestedMap(OwnedExprSiteV1),
}
impl ArrayElementSource {
    pub(crate) fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }
    /// `Some` for leaf elements only; container elements return `None` and
    /// stay fail-closed at every entry-level consumer.
    pub(crate) fn value_source(&self) -> Option<&MapValueSource> {
        match &self.kind {
            ArrayElementKindV1::Leaf(value) => Some(value),
            ArrayElementKindV1::NestedArray(_) | ArrayElementKindV1::NestedMap(_) => None,
        }
    }
    /// `Some` for nested `[...]` elements; leaves return `None`.
    pub(crate) fn nested_elements(&self) -> Option<&[ArrayElementSource]> {
        match &self.kind {
            ArrayElementKindV1::NestedArray(elements) => Some(elements),
            ArrayElementKindV1::Leaf(_) | ArrayElementKindV1::NestedMap(_) => None,
        }
    }
    /// `Some` for a nested map element; leaves and nested arrays return `None`.
    pub(crate) fn nested_map(&self) -> Option<&OwnedExprSiteV1> {
        match &self.kind {
            ArrayElementKindV1::NestedMap(site) => Some(site),
            ArrayElementKindV1::Leaf(_) | ArrayElementKindV1::NestedArray(_) => None,
        }
    }
}
/// Exact source payload or binding observation; unknown formal kind stays unknown.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum MapValueSource {
    Integer(i64),
    Bool(bool),
    Local {
        binding: BindingRefV1,
        kind: Option<SourceScalarKind>,
    },
    /// A sealed string literal; the UTF-8 payload is sealed with the row
    /// so a consumer never re-reads the source site.
    String(Box<str>),
    /// A self-rooted parameter handle (OpaqueHandle/DeclaredHandle/ExactText
    /// at install). Never a live Home/Map local — those stay on the
    /// transfer path only.
    BorrowedHandle(BindingRefV1),
    /// A live map-installed local borrowed by reference (`local m = %{...}`
    /// or an alias of it). The local stays the owner — the parent stores a
    /// reference, and the local still issues its own End through `outer`.
    MapLocal(BindingRefV1),
}

/// The leaf kind one borrowed map entry shares. `Handle` is a
/// self-rooted parameter handle — physically an i64 value the selected
/// consumer stores through the existing `InstallValue` lane whose
/// payload end is a no-op (the map never owns it). `MapLocal` is a
/// live map-storage reference and `Local` a kind-less binding — neither
/// has a physical reference lane today, so their `OwnershipShare`
/// obligations stay uncovered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum MapEntryBorrowKindV1 {
    Handle,
    MapLocal,
    Local,
}

impl MapValueSource {
    pub(crate) fn scalar_kind(&self) -> Option<SourceScalarKind> {
        match self {
            Self::Integer(_) => Some(SourceScalarKind::Integer),
            Self::Bool(_) => Some(SourceScalarKind::Bool),
            Self::Local { kind, .. } => *kind,
            Self::String(_) | Self::BorrowedHandle(_) | Self::MapLocal(_) => None,
        }
    }
    /// The borrow kind and root binding one leaf carries when the entry
    /// stores a non-consuming reference; `None` for stored values and
    /// opaque child payloads. The kind is the sealed row's own
    /// classification — describe and the lowering consumer read the same
    /// predicate; nothing reclassifies a borrow downstream.
    pub(crate) fn borrowed_root(&self) -> Option<(MapEntryBorrowKindV1, BindingRefV1)> {
        match self {
            Self::BorrowedHandle(root) => Some((MapEntryBorrowKindV1::Handle, *root)),
            Self::MapLocal(root) => Some((MapEntryBorrowKindV1::MapLocal, *root)),
            Self::Local {
                binding,
                kind: None,
            } => Some((MapEntryBorrowKindV1::Local, *binding)),
            _ => None,
        }
    }
}

/// The store class one entry's sealed ownership row requires. This is
/// the row's own classification — describe and the lowering consumer
/// read the same predicate; nothing reclassifies an entry downstream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum MapEntryStoreClassV1 {
    /// `InstallValue` lane: `Integer`/`Bool` literals and kind-sealed
    /// scalar locals.
    Scalar,
    /// `InstallIndexed` lane: a consuming Home transfer.
    Transferred,
    /// A borrowed reference (`MapLocal`, `BorrowedHandle`, kind-less
    /// `Local`) — the store obligation is `OwnershipShare`, never an
    /// `EntryStore`.
    Borrowed,
    /// `InstallText` lane: an owned UTF-8 payload sealed on the
    /// `MapValueSource::String` row.
    Text,
    /// `InstallEmptyArray` lane: a `[]` literal carries no elements and
    /// no child obligation — the map owns the empty-array meaning itself.
    EmptyArray,
    /// A non-empty array whose every direct element is an exact `MapLocal`
    /// binding. The array borrows the caller's live Map roots; it is not an
    /// owned child transfer and therefore requires a dedicated physical
    /// residence plus `OwnershipShare(MapLocal)` evidence.
    BorrowedArray,
    /// No install lane today: non-empty `[...]` entry values and
    /// `%{...}` child maps (indexed install requires a transfer
    /// acquisition the child does not carry).
    Opaque,
}

impl MapHomeEntry {
    pub(crate) fn store_class(&self) -> MapEntryStoreClassV1 {
        match &self.ownership {
            MapEntryOwnership::TransferHome { .. } => MapEntryStoreClassV1::Transferred,
            MapEntryOwnership::NestedMap => MapEntryStoreClassV1::Opaque,
            MapEntryOwnership::NestedArray { elements } => {
                if elements.is_empty() {
                    MapEntryStoreClassV1::EmptyArray
                } else if elements.iter().all(|element| {
                    matches!(element.value_source(), Some(MapValueSource::MapLocal(_)))
                }) {
                    MapEntryStoreClassV1::BorrowedArray
                } else {
                    MapEntryStoreClassV1::Opaque
                }
            }
            MapEntryOwnership::Value(value) => match value {
                _ if value.scalar_kind().is_some() => MapEntryStoreClassV1::Scalar,
                MapValueSource::String(_) => MapEntryStoreClassV1::Text,
                MapValueSource::BorrowedHandle(_)
                | MapValueSource::MapLocal(_)
                | MapValueSource::Local { kind: None, .. } => MapEntryStoreClassV1::Borrowed,
                _ => MapEntryStoreClassV1::Opaque,
            },
        }
    }
    pub(crate) fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }
    pub(crate) fn key(&self) -> &str {
        &self.key
    }
    pub(crate) fn transfer_home(&self) -> Option<(&OwnedExprSiteV1, BindingRefV1)> {
        match &self.ownership {
            MapEntryOwnership::TransferHome {
                acquisition,
                binding,
            } => Some((acquisition, *binding)),
            MapEntryOwnership::Value(_)
            | MapEntryOwnership::NestedMap
            | MapEntryOwnership::NestedArray { .. } => None,
        }
    }
    pub(crate) fn binding(&self) -> Option<BindingRefV1> {
        match &self.ownership {
            MapEntryOwnership::TransferHome { binding, .. } => Some(*binding),
            MapEntryOwnership::Value(MapValueSource::Local { binding, .. })
            | MapEntryOwnership::Value(MapValueSource::BorrowedHandle(binding))
            | MapEntryOwnership::Value(MapValueSource::MapLocal(binding)) => Some(*binding),
            MapEntryOwnership::Value(_)
            | MapEntryOwnership::NestedMap
            | MapEntryOwnership::NestedArray { .. } => None,
        }
    }
    pub(crate) fn value_source(&self) -> Option<&MapValueSource> {
        match &self.ownership {
            MapEntryOwnership::Value(value) => Some(value),
            MapEntryOwnership::TransferHome { .. }
            | MapEntryOwnership::NestedMap
            | MapEntryOwnership::NestedArray { .. } => None,
        }
    }
    /// Sealed leaf sources for a `[...]` entry value; `None` otherwise.
    pub(crate) fn array_elements(&self) -> Option<&[ArrayElementSource]> {
        match &self.ownership {
            MapEntryOwnership::NestedArray { elements } => Some(elements),
            _ => None,
        }
    }
    /// Exact direct `MapLocal` roots for the bounded borrowed-array lane.
    /// Returning `None` shares the same sealed predicate as `store_class()`;
    /// callers must not reconstruct the classification from source names.
    pub(crate) fn borrowed_array_bindings(&self) -> Option<Box<[BindingRefV1]>> {
        if self.store_class() != MapEntryStoreClassV1::BorrowedArray {
            return None;
        }
        let elements = self.array_elements()?;
        Some(
            elements
                .iter()
                .map(|element| match element.value_source() {
                    Some(MapValueSource::MapLocal(binding)) => *binding,
                    _ => unreachable!("BorrowedArray classification drift"),
                })
                .collect(),
        )
    }
    pub(crate) fn displaced(&self) -> Option<&SourceExprSiteV1> {
        self.displaced.as_ref()
    }
}

/// Build all successors before changing the caller's Normal-path local state.
/// Rejection cannot publish a partly transferred Map. `used` and `nested_out`
/// are shared across the recursive nested-`%{...}` tree; a transfer consumes
/// the local at its point, and a home consumed inside a child subtree is
/// marked in the parent's `outer` at the parent's entry index.
pub(super) fn observe_map<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &OwnedExprSiteV1,
    destination: MapDestinationV1,
    keys: &[Box<str>],
    locals: &mut PrefixLocalFlow<'_>,
    homes: &[LiveHomeV1],
    used: &mut std::collections::BTreeSet<BindingRefV1>,
    nested_out: &mut Vec<MapHomeObservation>,
    compatible: &mut impl FnMut(&OwnedExprSiteV1, BindingRefV1) -> Result<bool, E>,
) -> Result<Result<(MapHomeFlow, Vec<LiveHomeV1>), HomePrefixUnavailableV1>, E> {
    let Some(shape) = input.body_shape() else {
        return Ok(Err(HomePrefixUnavailableV1::SourceMismatch));
    };
    let outward = match &destination {
        MapDestinationV1::LocalBinding(binding) => {
            crate::mir::resolved_control_flow::map_source_outward(input, site, *binding)
        }
        MapDestinationV1::ReturnBoundary(return_site) => {
            crate::mir::resolved_control_flow::map_return_outward(input, site, return_site)
        }
        MapDestinationV1::EntrySlot {
            parent_map,
            ordinal,
        } => {
            crate::mir::resolved_control_flow::map_entry_outward(input, site, parent_map, *ordinal)
        }
        MapDestinationV1::CallArgument { call, ordinal } => {
            crate::mir::resolved_control_flow::map_argument_outward(input, site, call, *ordinal)
        }
        MapDestinationV1::ContainedIn { parent, role } => {
            crate::mir::resolved_control_flow::map_contained_outward(input, site, parent, role)
        }
    };
    let Ok((source_scope, target_function)) = outward else {
        return Ok(Err(HomePrefixUnavailableV1::SourceMismatch));
    };
    let mut remaining = homes.to_vec();
    let mut entries: Vec<MapHomeEntry> = Vec::new();
    let mut outer: Vec<_> = homes
        .iter()
        .rev()
        .map(|binding| MapOuterHome {
            home: binding.clone(),
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
        let ownership = if let Some((binding, acquisition)) = locals
            .direct_available_home(child)
            .map(|(binding, acquisition)| (binding, acquisition.clone()))
        {
            if !used.insert(binding) || !compatible(&acquisition, binding)? {
                return Ok(Err(HomePrefixUnavailableV1::MapCandidateNotCovered(
                    child.clone(),
                )));
            }
            let Some(home) = outer.iter_mut().find(|home| home.home.binding() == binding) else {
                return Ok(Err(HomePrefixUnavailableV1::SourceMismatch));
            };
            if let Err(issue) = transfer_home_v1(locals, &mut remaining, binding) {
                return Ok(Err(issue));
            }
            home.transferred_at = Some(entries.len());
            MapEntryOwnership::TransferHome {
                acquisition,
                binding,
            }
        } else if let Some(child_keys) = map_literal_keys(input, child) {
            // A nested `%{...}` is its own MapLiteral row, not a value leaf:
            // recurse so the child issues an EntrySlot-destination row, then
            // record the slot as the parent's entry ownership.
            let child_owned = OwnedExprSiteV1::new(input.owner(), child.clone());
            match observe_map(
                input,
                &child_owned,
                MapDestinationV1::EntrySlot {
                    parent_map: site.clone(),
                    ordinal,
                },
                child_keys,
                locals,
                &remaining,
                used,
                nested_out,
                compatible,
            )? {
                Ok((child_flow, child_remaining)) => {
                    for binding in child_flow
                        .outer
                        .iter()
                        .filter(|home| home.transferred_at.is_some())
                        .map(|home| home.home.clone())
                    {
                        let Some(home) = outer.iter_mut().find(|home| home.home == binding) else {
                            return Ok(Err(HomePrefixUnavailableV1::SourceMismatch));
                        };
                        home.transferred_at = Some(entries.len());
                    }
                    remaining = child_remaining;
                    nested_out.push(MapHomeObservation::Complete(child_flow));
                    MapEntryOwnership::NestedMap
                }
                Err(issue) => {
                    nested_out.push(MapHomeObservation::Unavailable { site: child_owned });
                    return Ok(Err(issue));
                }
            }
        } else if let Some(element_count) = array_literal_element_count(input, child) {
            // A `[...]` entry value carries classified elements: leaf
            // sources or nested `[...]` literals. Elements issue no
            // transfer and consume no Home.
            let elements = match observe_array_elements(
                input,
                shape,
                locals,
                child,
                element_count,
                used,
                nested_out,
                compatible,
            )? {
                Ok(elements) => elements,
                Err(issue) => return Ok(Err(issue)),
            };
            MapEntryOwnership::NestedArray {
                elements: elements.into_boxed_slice(),
            }
        } else {
            let Some(value) = map_value_leaf(input, locals, child) else {
                return Ok(Err(HomePrefixUnavailableV1::MapCandidateNotCovered(
                    child.clone(),
                )));
            };
            MapEntryOwnership::Value(value)
        };
        // Literal String key equality follows the existing canonical key law:
        // canonical integer spellings are unique; noncanonical spellings stay text.
        let entry_index = entries.len();
        let displaced = last_install.insert(key.clone(), entry_index).map(|prior| {
            entries[prior].replaced_at = Some(entry_index);
            entries[prior].site.clone()
        });
        entries.push(MapHomeEntry {
            site: child.clone(),
            key: key.clone(),
            ownership,
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
