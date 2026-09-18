//! Per-owner Map lifecycle undertaking — contract definition (C5a).
//!
//! This module defines the contract vocabulary only; admission connection
//! (calling `describe`/`verify` from `preflight_map_install` and carrying the
//! issued undertaking inside `PreparedNormalCallableSemanticPackageInstallV1`)
//! is the separate C5b slice. Nothing here issues new meaning: obligations
//! are derived projections of already-sealed `MapHomeFlow` rows, batch
//! membership, and completions — the aggregate proves one relation: *every
//! owner's obligation for this package is fully described AND the selected
//! lowering consumer's declared capability covers all of it.*
//!
//! Obligation enumeration starts from sealed batch membership
//! (`batch.declarations()` body-shape `MapLiteral` sites), never from the
//! set of owners whose completion happened to succeed — a failed completion
//! must not silently shrink the undertaking.
//!
//! `new MapBox()` construction sites carry the same operation vocabulary but
//! their evidence lives in OrdinaryNew claims, not `MapHomeFlow` rows; the
//! claim-family describe arm is a separate bounded row.

use crate::mir::resolved_semantics::home_new_prefix::{
    MapDestinationV1, MapEntryBorrowKindV1, MapEntryStoreClassV1, MapHomeFlow, MapHomeObservation,
    MapValueSource, TerminalRelationV1, TerminalReturnedSourceV1,
};
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, OwnedExprSiteV1, SourceExprSiteV1, SourceStmtSiteV1,
};
use std::collections::BTreeSet;

/// One lifecycle operation the selected lowering consumer must execute for
/// a Map. Derived — never reclassified — from the sealed row vocabulary:
/// destination class + entry ownership + overwrite chronology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum MapLifecycleOperationV1 {
    /// Construct the map at its literal site.
    ValueCreate,
    /// Store one key→value entry. The class is the sealed row's own
    /// `store_class()` — scalar, transferred, text, and empty-array
    /// stores are the declared consumer lanes; opaque classes (non-empty
    /// `[...]`, `%{...}` child
    /// values) stay uncovered and fail at verify. Borrowed entries carry
    /// `OwnershipShare` instead — a reference store is not an
    /// `EntryStore`.
    EntryStore(MapEntryStoreClassV1),
    /// Overwrite a previously stored key: release the displaced entry
    /// value, then complete the replacement store.
    EntryDisplace,
    /// Consume a live Home/binding into an entry (consuming transfer).
    OwnershipTransfer,
    /// Store a non-consuming borrow of a live map local or self-rooted
    /// parameter handle; the borrow target must outlive the map. The
    /// kind is the sealed leaf's own `borrowed_root()` classification —
    /// `Handle` borrows are the declared consumer lane while
    /// `MapLocal`/`Local` leaves stay uncovered until a borrowed
    /// storage-reference contract exists.
    OwnershipShare(MapEntryBorrowKindV1),
    /// Hand the map into its parent map's exact entry slot.
    SlotHandoff,
    /// Hand the map through the function-return boundary to the caller.
    ReturnHandoff,
    /// Hand the map into a call argument slot on a call edge.
    ArgumentHandoff,
    /// Hand the map into a sealed containment position with no more
    /// specific destination.
    ContainedHandoff,
    /// Reclaim untransferred outer acquisitions on the Normal end.
    NormalCleanup,
    /// Reclaim outer acquisitions and partially installed entries on Fault.
    FaultCleanup,
}

/// One binding borrowed (not consumed) into a map entry: `MapLocal` and
/// `BorrowedHandle` leaves, including leaves nested inside `[...]` entry
/// values. The consumer must keep the binding live until the owning map is
/// released; a `ReturnHandoff`/`SlotHandoff` map carrying borrows is the
/// dangling-borrow hazard the admission check must account for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MapEntryBorrowV1 {
    site: SourceExprSiteV1,
    binding: BindingRefV1,
}

impl MapEntryBorrowV1 {
    pub(crate) fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }
    pub(crate) fn binding(&self) -> BindingRefV1 {
        self.binding
    }
}

/// Every obligation one sealed `MapHomeFlow` row imposes on the consumer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MapSiteObligationV1 {
    site: OwnedExprSiteV1,
    destination: MapDestinationV1,
    operations: BTreeSet<MapLifecycleOperationV1>,
    borrows: Box<[MapEntryBorrowV1]>,
}

impl MapSiteObligationV1 {
    pub(crate) fn site(&self) -> &OwnedExprSiteV1 {
        &self.site
    }
    pub(crate) fn destination(&self) -> &MapDestinationV1 {
        &self.destination
    }
    pub(crate) fn operations(&self) -> impl Iterator<Item = MapLifecycleOperationV1> + '_ {
        self.operations.iter().copied()
    }
    pub(crate) fn borrows(&self) -> &[MapEntryBorrowV1] {
        &self.borrows
    }
}

/// One owner's complete described map obligation set, enumerated from sealed
/// membership. Owners are grouped even if an owner ever holds more than one
/// declaration — the obligation set is what the contract verifies, not the
/// declaration count.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MapOwnerObligationsV1 {
    owner: FunctionOwnerIdV1,
    sites: Box<[MapSiteObligationV1]>,
}

impl MapOwnerObligationsV1 {
    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }
    pub(crate) fn sites(&self) -> &[MapSiteObligationV1] {
        &self.sites
    }
    pub(crate) fn operations(&self) -> impl Iterator<Item = MapLifecycleOperationV1> + '_ {
        self.sites.iter().flat_map(|site| site.operations())
    }
}

/// One call edge where a Map crosses a callable boundary. The callee side
/// is resolved at admission (C5b) from the sealed call-target rows; this
/// definition records which obligation pair the edge must satisfy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MapCallEdgeContractV1 {
    /// The call expression site inside the caller.
    call_site: OwnedExprSiteV1,
    kind: MapCallEdgeKindV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum MapCallEdgeKindV1 {
    /// The caller's `%{...}` flows into `Argument(ordinal)`: the callee
    /// convention must accept the handoff (transfer or borrow per the
    /// argument contract), and the caller loses release responsibility for
    /// what it transferred.
    Argument { ordinal: u32 },
    /// The callee's map egresses `return`: the caller acquires release
    /// responsibility for the received map, and borrows stored inside it
    /// must keep their targets live.
    ReturnReceive,
}

impl MapCallEdgeContractV1 {
    pub(crate) fn new(call_site: OwnedExprSiteV1, kind: MapCallEdgeKindV1) -> Self {
        Self { call_site, kind }
    }
    pub(crate) fn call_site(&self) -> &OwnedExprSiteV1 {
        &self.call_site
    }
    pub(crate) fn kind(&self) -> &MapCallEdgeKindV1 {
        &self.kind
    }
}

/// The operation set a selected lowering consumer declares it can execute.
/// A one-shot install token is not capability evidence — this is declared
/// and matched explicitly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MapLifecycleConsumerCapabilityV1 {
    operations: BTreeSet<MapLifecycleOperationV1>,
}

impl MapLifecycleConsumerCapabilityV1 {
    pub(crate) fn covering(operations: impl IntoIterator<Item = MapLifecycleOperationV1>) -> Self {
        Self {
            operations: operations.into_iter().collect(),
        }
    }
    pub(crate) fn covers(&self, operation: MapLifecycleOperationV1) -> bool {
        self.operations.contains(&operation)
    }
}

/// Describing one owner's obligations can fail before capability matching.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum MapObligationDescribeIssueV1 {
    /// Sealed membership says this owner declares Map sites but no sealed
    /// completion exists for it — the obligation set cannot be described
    /// and the gap must not silently shrink the undertaking.
    OwnerCompletionMissing { owner: FunctionOwnerIdV1 },
    /// The owner's completion carries no root flow at all.
    OwnerRootFlowMissing { owner: FunctionOwnerIdV1 },
    /// A declared Map site has no `Complete` flow row (`Unavailable` or
    /// absent) — partial transfer plans are not describable.
    ObligationUnavailable {
        owner: FunctionOwnerIdV1,
        site: OwnedExprSiteV1,
    },
    /// The owner's cleanup never reached a usable terminal Homes set —
    /// its completed Map rows cannot prove the exit they must survive.
    OwnerTerminalHomesUnavailable { owner: FunctionOwnerIdV1 },
    /// The owner has no sealed terminal relation at all — the exit the
    /// Map obligations must be ordered against does not exist.
    OwnerTerminalRelationMissing { owner: FunctionOwnerIdV1 },
    /// The sealed terminal relation hands a Map to the caller, but no
    /// described site satisfies it — sealed drift, never to be silently
    /// dropped.
    OwnerTerminalMapUnmatched { owner: FunctionOwnerIdV1 },
    /// The received-map call site still has live prior map homes: the
    /// bounded consumer owns no prior-home cleanup path, so the gap must
    /// surface here instead of at physical emission.
    CallPriorHomesUnsupported {
        owner: FunctionOwnerIdV1,
        site: OwnedExprSiteV1,
    },
}

/// The sealed relation the undertaking proves: described obligations exist
/// and the declared capability covers every one of them. Call edges are
/// attached by the install preflight's own co-seal — each row pairs a
/// described `CallArgument` obligation with a lifecycle-admitted loan edge
/// whose callee formal ABI and contract kind both agree at the ordinal.
#[derive(Debug)]
pub(crate) struct MapLifecycleUndertakingV1 {
    owners: Box<[FunctionOwnerIdV1]>,
    capability: MapLifecycleConsumerCapabilityV1,
    call_edges: Box<[MapCallEdgeContractV1]>,
    _seal: MapLifecycleUndertakingSealV1,
}

#[derive(Debug)]
struct MapLifecycleUndertakingSealV1;

impl MapLifecycleUndertakingV1 {
    pub(crate) fn owners(&self) -> &[FunctionOwnerIdV1] {
        &self.owners
    }
    pub(crate) fn capability(&self) -> &MapLifecycleConsumerCapabilityV1 {
        &self.capability
    }
    pub(crate) fn call_edges(&self) -> &[MapCallEdgeContractV1] {
        &self.call_edges
    }

    /// Attach the co-sealed call-edge contracts. Only the package's
    /// install preflight may do this — the pairs it records were already
    /// matched against both the described obligations and the sealed
    /// loan rows; the undertaking issues no new meaning.
    pub(in crate::mir::normal_callable_semantic_package) fn with_call_edges(
        mut self,
        call_edges: impl IntoIterator<Item = MapCallEdgeContractV1>,
    ) -> Self {
        self.call_edges = call_edges.into_iter().collect();
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum MapLifecycleUndertakingIssueV1 {
    Describe(MapObligationDescribeIssueV1),
    /// The declared capability cannot execute an operation an owner owes.
    UncoveredOperation {
        owner: FunctionOwnerIdV1,
        site: OwnedExprSiteV1,
        operation: MapLifecycleOperationV1,
    },
    /// A site carrying borrowed entries together with a handoff
    /// (Slot/Return/Argument/Contained) would move the borrow across a
    /// boundary the sealed vocabulary cannot prove — the target's
    /// liveness beyond the owner is unsealed — so the escape fails here
    /// instead of silently riding the handoff.
    BorrowedEntryEscape {
        owner: FunctionOwnerIdV1,
        site: OwnedExprSiteV1,
        binding: BindingRefV1,
    },
    /// The undertaking would be vacuous: no owner carries map obligations.
    EmptyUndertaking,
}

/// A Map the sealed terminal relation hands to the caller. `Literal`
/// rows already describe `ReturnHandoff` through their `ReturnBoundary`
/// destination — the terminal join is a consistency check. `Local`
/// rows carry the map to the boundary without a `ReturnBoundary`
/// destination, so the handoff obligation is added here.
enum ReturnedMapV1<'a> {
    Literal {
        site: &'a OwnedExprSiteV1,
        return_site: &'a SourceStmtSiteV1,
    },
    Local(BindingRefV1),
}

/// Describe one Complete flow row as a site obligation.
fn describe_flow(flow: &MapHomeFlow, returned_local: bool) -> MapSiteObligationV1 {
    use MapLifecycleOperationV1 as Op;
    let mut operations = BTreeSet::from([Op::ValueCreate, Op::NormalCleanup, Op::FaultCleanup]);
    if returned_local {
        operations.insert(Op::ReturnHandoff);
    }
    match flow.destination() {
        MapDestinationV1::LocalBinding(_) => {}
        MapDestinationV1::ReturnBoundary(_) => {
            operations.insert(Op::ReturnHandoff);
        }
        MapDestinationV1::EntrySlot { .. } => {
            operations.insert(Op::SlotHandoff);
        }
        MapDestinationV1::CallArgument { .. } => {
            operations.insert(Op::ArgumentHandoff);
        }
        MapDestinationV1::ContainedIn { .. } => {
            operations.insert(Op::ContainedHandoff);
        }
    }
    let mut borrows = Vec::new();
    for entry in flow.entries() {
        match entry.store_class() {
            MapEntryStoreClassV1::Borrowed => {}
            class => {
                operations.insert(Op::EntryStore(class));
            }
        }
        if entry.transfer_home().is_some() {
            operations.insert(Op::OwnershipTransfer);
        }
        if entry.displaced().is_some() {
            operations.insert(Op::EntryDisplace);
        }
        // A `Local` entry stores a scalar copy only when the source kind
        // is sealed; a kind-less local is a non-consuming borrow of a live
        // binding — the same sharing obligation as `MapLocal`/
        // `BorrowedHandle`, not an `EntryStore` alone. The leaf's own
        // `borrowed_root()` classification carries the kind precision.
        let mut collect_borrow = |site: &SourceExprSiteV1, source: &MapValueSource| {
            if let Some((kind, binding)) = source.borrowed_root() {
                operations.insert(Op::OwnershipShare(kind));
                borrows.push(MapEntryBorrowV1 {
                    site: site.clone(),
                    binding,
                });
            }
        };
        if let Some(source) = entry.value_source() {
            collect_borrow(entry.site(), source);
        }
        if let Some(elements) = entry.array_elements() {
            collect_array_borrows(elements, &mut collect_borrow);
        }
    }
    MapSiteObligationV1 {
        site: flow.site().clone(),
        destination: flow.destination().clone(),
        operations,
        borrows: borrows.into_boxed_slice(),
    }
}

fn collect_array_borrows(
    elements: &[crate::mir::resolved_semantics::home_new_prefix::ArrayElementSource],
    collect: &mut impl FnMut(&SourceExprSiteV1, &MapValueSource),
) {
    for element in elements {
        if let Some(source) = element.value_source() {
            collect(element.site(), source);
        }
        if let Some(nested) = element.nested_elements() {
            collect_array_borrows(nested, collect);
        }
    }
}

/// Verify the declared capability covers every described obligation and
/// seal the undertaking. Call-edge conformance rows are matched separately
/// at admission; this seal covers the per-owner obligation half.
pub(crate) fn verify_map_lifecycle_undertaking(
    obligations: &[MapOwnerObligationsV1],
    capability: MapLifecycleConsumerCapabilityV1,
) -> Result<MapLifecycleUndertakingV1, MapLifecycleUndertakingIssueV1> {
    use MapLifecycleOperationV1 as Op;
    let mut owners = Vec::with_capacity(obligations.len());
    for owner_obligations in obligations {
        owners.push(owner_obligations.owner());
        for site in owner_obligations.sites() {
            for operation in site.operations() {
                if !capability.covers(operation) {
                    return Err(MapLifecycleUndertakingIssueV1::UncoveredOperation {
                        owner: owner_obligations.owner(),
                        site: site.site().clone(),
                        operation,
                    });
                }
            }
            // A borrowed entry must not ride a handoff: the sealed borrow
            // rows prove the target outlives the map only while the map
            // stays inside this owner. Slot/Return/Argument/Contained
            // handoffs carry no borrow-liveness contract yet, so a site
            // mixing them with borrows is refused even when every named
            // operation is declared covered.
            if let Some(borrow) = site.borrows().first() {
                if site.operations().any(|operation| {
                    matches!(
                        operation,
                        Op::SlotHandoff
                            | Op::ReturnHandoff
                            | Op::ArgumentHandoff
                            | Op::ContainedHandoff
                    )
                }) {
                    return Err(MapLifecycleUndertakingIssueV1::BorrowedEntryEscape {
                        owner: owner_obligations.owner(),
                        site: site.site().clone(),
                        binding: borrow.binding(),
                    });
                }
            }
        }
    }
    if owners.is_empty() {
        return Err(MapLifecycleUndertakingIssueV1::EmptyUndertaking);
    }
    Ok(MapLifecycleUndertakingV1 {
        owners: owners.into_boxed_slice(),
        capability,
        call_edges: Box::new([]),
        _seal: MapLifecycleUndertakingSealV1,
    })
}

impl super::VerifiedNormalCallableSemanticPackageV1 {
    /// Describe every sealed member's map obligations, enumerated from batch
    /// membership (declared `MapLiteral` sites per declaration), never from
    /// the set of owners whose completion happened to succeed.
    pub(in crate::mir::normal_callable_semantic_package) fn describe_map_lifecycle_obligations(
        &self,
    ) -> Result<Box<[MapOwnerObligationsV1]>, MapObligationDescribeIssueV1> {
        use crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1;
        use crate::mir::resolved_semantics::BodyExpressionShapeV1;
        let mut by_owner: std::collections::BTreeMap<FunctionOwnerIdV1, Vec<SourceExprSiteV1>> =
            std::collections::BTreeMap::new();
        for declaration in self.batch.declarations() {
            for expression in declaration.body_shape().expressions() {
                if let BodyExpressionShapeV1::MapLiteral { site, .. } = expression {
                    by_owner
                        .entry(declaration.owner())
                        .or_default()
                        .push(site.clone());
                }
            }
            // A map-result local call makes its owner a map owner even
            // without a `%{...}` literal: the received map must be
            // released at the caller's exit.
            if let Some(flow) = self
                .ordinary_new_claim_ledger
                .completion_for_owner(declaration.owner())
                .and_then(|completion| completion.cleanup().root_flow())
            {
                if flow.local_calls().iter().any(|call| {
                    call.owner() == declaration.owner()
                        && call.result() == LocalCallResultClassV1::Map
                }) {
                    by_owner.entry(declaration.owner()).or_default();
                }
            }
        }
        let mut described = Vec::with_capacity(by_owner.len());
        for (owner, sites) in by_owner {
            let completion = self
                .ordinary_new_claim_ledger
                .completion_for_owner(owner)
                .ok_or(MapObligationDescribeIssueV1::OwnerCompletionMissing { owner })?;
            // Co-seal the owner's exit evidence regardless of any
            // AppMain loan: completed Map rows alone cannot prove the
            // cleanup the consumer must execute at the terminal.
            if !matches!(completion.cleanup().terminal_homes(), Some(Ok(_))) {
                return Err(MapObligationDescribeIssueV1::OwnerTerminalHomesUnavailable { owner });
            }
            let flow = completion
                .cleanup()
                .root_flow()
                .ok_or(MapObligationDescribeIssueV1::OwnerRootFlowMissing { owner })?;
            let terminal = self
                .ordinary_new_claim_ledger
                .terminal_relation_for_owner(owner)
                .ok_or(MapObligationDescribeIssueV1::OwnerTerminalRelationMissing { owner })?;
            let returned_map = match terminal {
                TerminalRelationV1::Value(row) => match row.returned() {
                    TerminalReturnedSourceV1::MapLiteral(site) => Some(ReturnedMapV1::Literal {
                        site,
                        return_site: row.return_site(),
                    }),
                    TerminalReturnedSourceV1::MapLocal(binding) => {
                        Some(ReturnedMapV1::Local(*binding))
                    }
                    _ => None,
                },
                _ => None,
            };
            let mut returned_matched = returned_map.is_none();
            let mut rows = Vec::with_capacity(sites.len());
            for site in sites {
                let owned = OwnedExprSiteV1::new(owner, site);
                let observation = flow
                    .maps()
                    .iter()
                    .find(|row| row.site() == &owned)
                    .and_then(MapHomeObservation::complete)
                    .ok_or_else(|| MapObligationDescribeIssueV1::ObligationUnavailable {
                        owner,
                        site: owned.clone(),
                    })?;
                let mut returned_local = false;
                match &returned_map {
                    Some(ReturnedMapV1::Literal {
                        site: returned_site,
                        return_site,
                    }) if observation.site() == *returned_site => {
                        if !matches!(
                            observation.destination(),
                            MapDestinationV1::ReturnBoundary(statement)
                                if statement.node() == return_site.node()
                        ) {
                            return Err(MapObligationDescribeIssueV1::OwnerTerminalMapUnmatched {
                                owner,
                            });
                        }
                        returned_matched = true;
                    }
                    Some(ReturnedMapV1::Local(binding))
                        if observation.local_binding() == Some(*binding) =>
                    {
                        returned_local = true;
                        returned_matched = true;
                    }
                    _ => {}
                }
                rows.push(describe_flow(observation, returned_local));
            }
            // A map-result local call receives a map the caller must
            // release at its exit. `ReturnHandoff` stays literal-row only:
            // returning a received map is not yet an admitted shape.
            for call in flow
                .local_calls()
                .iter()
                .filter(|call| call.result() == LocalCallResultClassV1::Map)
            {
                if call.owner() != owner {
                    return Err(MapObligationDescribeIssueV1::ObligationUnavailable {
                        owner,
                        site: call.site().clone(),
                    });
                }
                if !call.prior_homes().is_empty() {
                    return Err(MapObligationDescribeIssueV1::CallPriorHomesUnsupported {
                        owner,
                        site: call.site().clone(),
                    });
                }
                rows.push(MapSiteObligationV1 {
                    site: call.site().clone(),
                    destination: MapDestinationV1::LocalBinding(call.destination()),
                    operations: [
                        MapLifecycleOperationV1::NormalCleanup,
                        MapLifecycleOperationV1::FaultCleanup,
                    ]
                    .into_iter()
                    .collect(),
                    borrows: Box::default(),
                });
            }
            if !returned_matched {
                return Err(MapObligationDescribeIssueV1::OwnerTerminalMapUnmatched { owner });
            }
            described.push(MapOwnerObligationsV1 {
                owner,
                sites: rows.into_boxed_slice(),
            });
        }
        Ok(described.into_boxed_slice())
    }
}
