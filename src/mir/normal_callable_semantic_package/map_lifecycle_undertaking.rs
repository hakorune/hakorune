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
    MapDestinationV1, MapHomeFlow, MapHomeObservation, MapValueSource,
};
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, OwnedExprSiteV1, SourceExprSiteV1,
};
use std::collections::BTreeSet;

/// One lifecycle operation the selected lowering consumer must execute for
/// a Map. Derived — never reclassified — from the sealed row vocabulary:
/// destination class + entry ownership + overwrite chronology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum MapLifecycleOperationV1 {
    /// Construct the map at its literal site.
    ValueCreate,
    /// Store one key→value entry.
    EntryStore,
    /// Overwrite a previously stored key: release the displaced entry
    /// value, then complete the replacement store.
    EntryDisplace,
    /// Consume a live Home/binding into an entry (consuming transfer).
    OwnershipTransfer,
    /// Store a non-consuming borrow of a live map local or self-rooted
    /// parameter handle; the borrow target must outlive the map.
    OwnershipShare,
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
}

/// The sealed relation the undertaking proves: described obligations exist
/// and the declared capability covers every one of them.
#[derive(Debug)]
pub(crate) struct MapLifecycleUndertakingV1 {
    owners: Box<[FunctionOwnerIdV1]>,
    capability: MapLifecycleConsumerCapabilityV1,
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
    /// The undertaking would be vacuous: no owner carries map obligations.
    EmptyUndertaking,
}

/// Describe one Complete flow row as a site obligation.
fn describe_flow(flow: &MapHomeFlow) -> MapSiteObligationV1 {
    use MapLifecycleOperationV1 as Op;
    let mut operations = BTreeSet::from([Op::ValueCreate, Op::NormalCleanup, Op::FaultCleanup]);
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
        operations.insert(Op::EntryStore);
        if entry.transfer_home().is_some() {
            operations.insert(Op::OwnershipTransfer);
        }
        if entry.displaced().is_some() {
            operations.insert(Op::EntryDisplace);
        }
        let mut collect_borrow = |site: &SourceExprSiteV1, source: &MapValueSource| match source {
            MapValueSource::MapLocal(binding) | MapValueSource::BorrowedHandle(binding) => {
                operations.insert(Op::OwnershipShare);
                borrows.push(MapEntryBorrowV1 {
                    site: site.clone(),
                    binding: *binding,
                });
            }
            _ => {}
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
        }
    }
    if owners.is_empty() {
        return Err(MapLifecycleUndertakingIssueV1::EmptyUndertaking);
    }
    Ok(MapLifecycleUndertakingV1 {
        owners: owners.into_boxed_slice(),
        capability,
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
        }
        let mut described = Vec::with_capacity(by_owner.len());
        for (owner, sites) in by_owner {
            let completion = self
                .ordinary_new_claim_ledger
                .completion_for_owner(owner)
                .ok_or(MapObligationDescribeIssueV1::OwnerCompletionMissing { owner })?;
            let flow = completion
                .cleanup()
                .root_flow()
                .ok_or(MapObligationDescribeIssueV1::OwnerRootFlowMissing { owner })?;
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
                rows.push(describe_flow(observation));
            }
            described.push(MapOwnerObligationsV1 {
                owner,
                sites: rows.into_boxed_slice(),
            });
        }
        Ok(described.into_boxed_slice())
    }
}
