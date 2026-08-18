//! Index-only physical bridge for S6C ExactText entry lanes.
//!
//! The source/co-seal and the physical entry sidecar remain the authorities.
//! This child only records the lane-index/root-index bijection needed by a
//! later runtime adapter; it never owns a runtime pair, handle, token, or
//! Residence.  The move-only plan is caller-zero evidence until a selected
//! backend consumes it.

use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1;
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1};
use crate::mir::{BasicBlockId, ValueId};

use super::common_v2_session::S6CTextEqOccurrencePhysicalViewV1;
use super::physical_entry_lane_adoption::PhysicalTextEntryLaneSidecarV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum CommonV2S6CTextRefEntryBridgeRejectV1 {
    EmptySidecar,
    ZeroPlanStamp,
    OwnerMismatch,
    BindingOwnerMismatch,
    OccurrenceBindingMismatch,
    LogicalOrdinalOrder,
    LaneIndexOverflow,
    LanePairMismatch,
    CarrierMismatch,
    RootIndexOverflow,
    LaneInputTooShort,
    ZeroPublishedPair,
    Consumer(String),
}

/// One source-bound row's physical lane/index projection.
///
/// `slot_lane_index` and `generation_lane_index` are indices into the already
/// published callable-entry lane values.  They are not runtime values and
/// must never be reinterpreted as a slot or generation themselves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct CommonV2S6CTextRefEntryBridgeRowV1 {
    binding: BindingRefV1,
    logical_ordinal: u32,
    slot_lane_index: u32,
    generation_lane_index: u32,
    published_pair_index: u32,
    root_index: u32,
    carrier: PhysicalCallableLaneCarrierV1,
}

impl CommonV2S6CTextRefEntryBridgeRowV1 {
    pub(in crate::mir::builder) const fn binding(self) -> BindingRefV1 {
        self.binding
    }

    pub(in crate::mir::builder) const fn logical_ordinal(self) -> u32 {
        self.logical_ordinal
    }

    pub(in crate::mir::builder) const fn slot_lane_index(self) -> u32 {
        self.slot_lane_index
    }

    pub(in crate::mir::builder) const fn generation_lane_index(self) -> u32 {
        self.generation_lane_index
    }

    pub(in crate::mir::builder) const fn published_pair_index(self) -> u32 {
        self.published_pair_index
    }

    pub(in crate::mir::builder) const fn root_index(self) -> u32 {
        self.root_index
    }

    pub(in crate::mir::builder) const fn carrier(self) -> PhysicalCallableLaneCarrierV1 {
        self.carrier
    }
}

/// Callback-scoped view of the pair vector built from one consumed plan.
/// The vector cannot be returned or stored by this API; the future runtime
/// consumer is the only intended callback recipient.
#[derive(Debug)]
pub(in crate::mir::builder) struct CommonV2S6CTextRefPublishedPairBatchRef<'batch> {
    pairs: &'batch [(u64, u64)],
}

impl CommonV2S6CTextRefPublishedPairBatchRef<'_> {
    pub(in crate::mir::builder) const fn len(&self) -> usize {
        self.pairs.len()
    }

    pub(in crate::mir::builder) fn with_pairs<R>(
        &self,
        callback: impl FnOnce(&[(u64, u64)]) -> R,
    ) -> R {
        callback(self.pairs)
    }
}

/// Move-only index/stamp plan.  It is consumed exactly once by the callback
/// consumer; no retry or second owner exists.
#[must_use = "an entry bridge plan must be consumed exactly once"]
#[derive(Debug)]
pub(in crate::mir::builder) struct CommonV2S6CTextRefEntryBridgePlanV1 {
    owner: FunctionOwnerIdV1,
    entry: BasicBlockId,
    plan_stamp: u64,
    max_lane_index: u32,
    rows: Box<[CommonV2S6CTextRefEntryBridgeRowV1]>,
}

impl CommonV2S6CTextRefEntryBridgePlanV1 {
    pub(in crate::mir::builder) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) const fn entry(&self) -> BasicBlockId {
        self.entry
    }

    pub(in crate::mir::builder) const fn plan_stamp(&self) -> u64 {
        self.plan_stamp
    }

    pub(in crate::mir::builder) fn rows(&self) -> &[CommonV2S6CTextRefEntryBridgeRowV1] {
        &self.rows
    }

    /// Consume the plan and lend the root-index-ordered published pairs to
    /// one callback.  The runtime Residence adapter remains outside MIR.
    pub(in crate::mir::builder) fn with_published_pairs<R>(
        self,
        lane_values: &[u64],
        callback: impl FnOnce(CommonV2S6CTextRefPublishedPairBatchRef<'_>) -> Result<R, String>,
    ) -> Result<R, CommonV2S6CTextRefEntryBridgeRejectV1> {
        let max_index = usize::try_from(self.max_lane_index)
            .map_err(|_| CommonV2S6CTextRefEntryBridgeRejectV1::LaneIndexOverflow)?;
        if lane_values.len() <= max_index {
            return Err(CommonV2S6CTextRefEntryBridgeRejectV1::LaneInputTooShort);
        }

        let mut pairs = Vec::with_capacity(self.rows.len());
        for (expected_root, row) in self.rows.iter().enumerate() {
            let expected_root = u32::try_from(expected_root)
                .map_err(|_| CommonV2S6CTextRefEntryBridgeRejectV1::RootIndexOverflow)?;
            if row.root_index != expected_root || row.published_pair_index != expected_root {
                return Err(CommonV2S6CTextRefEntryBridgeRejectV1::RootIndexOverflow);
            }
            let slot = lane_values[row.slot_lane_index as usize];
            let generation = lane_values[row.generation_lane_index as usize];
            if slot == 0 {
                return Err(CommonV2S6CTextRefEntryBridgeRejectV1::ZeroPublishedPair);
            }
            if generation == 0 {
                return Err(CommonV2S6CTextRefEntryBridgeRejectV1::ZeroPublishedPair);
            }
            pairs.push((slot, generation));
        }
        callback(CommonV2S6CTextRefPublishedPairBatchRef { pairs: &pairs })
            .map_err(CommonV2S6CTextRefEntryBridgeRejectV1::Consumer)
    }
}

/// Issue the one physical plan from the already-installed ExactText sidecar.
/// No source meaning is reconstructed and no runtime value is captured here.
pub(in crate::mir::builder) fn issue_common_v2_s6c_textref_entry_bridge_plan_v1(
    sidecar: &PhysicalTextEntryLaneSidecarV1,
    plan_stamp: u64,
) -> Result<CommonV2S6CTextRefEntryBridgePlanV1, CommonV2S6CTextRefEntryBridgeRejectV1> {
    if sidecar.rows().is_empty() {
        return Err(CommonV2S6CTextRefEntryBridgeRejectV1::EmptySidecar);
    }
    if plan_stamp == 0 {
        return Err(CommonV2S6CTextRefEntryBridgeRejectV1::ZeroPlanStamp);
    }
    let owner = sidecar.owner();
    let entry = sidecar.entry();
    let mut previous_ordinal = None;
    let mut max_lane_index = 0u32;
    let mut rows = Vec::with_capacity(sidecar.rows().len());

    for (root_index, source_row) in sidecar.rows().iter().enumerate() {
        let root_index = u32::try_from(root_index)
            .map_err(|_| CommonV2S6CTextRefEntryBridgeRejectV1::RootIndexOverflow)?;
        if source_row.binding().owner() != owner {
            return Err(CommonV2S6CTextRefEntryBridgeRejectV1::BindingOwnerMismatch);
        }
        if previous_ordinal.is_some_and(|previous| source_row.logical_ordinal() <= previous) {
            return Err(CommonV2S6CTextRefEntryBridgeRejectV1::LogicalOrdinalOrder);
        }
        previous_ordinal = Some(source_row.logical_ordinal());
        if source_row.carrier() != PhysicalCallableLaneCarrierV1::U64BitsOnI64 {
            return Err(CommonV2S6CTextRefEntryBridgeRejectV1::CarrierMismatch);
        }

        let slot_lane_index = source_row.slot().as_u32();
        let generation_lane_index = source_row.generation().as_u32();
        if generation_lane_index != slot_lane_index.saturating_add(1) {
            return Err(CommonV2S6CTextRefEntryBridgeRejectV1::LanePairMismatch);
        }
        max_lane_index = max_lane_index.max(generation_lane_index);
        rows.push(CommonV2S6CTextRefEntryBridgeRowV1 {
            binding: source_row.binding(),
            logical_ordinal: source_row.logical_ordinal(),
            slot_lane_index,
            generation_lane_index,
            published_pair_index: root_index,
            root_index,
            carrier: source_row.carrier(),
        });
    }

    Ok(CommonV2S6CTextRefEntryBridgePlanV1 {
        owner,
        entry,
        plan_stamp,
        max_lane_index,
        rows: rows.into_boxed_slice(),
    })
}

/// Bind the physical plan to the already-issued source/segment occurrence.
/// The sidecar remains the lane-index authority; this check prevents a plan
/// for one entry/cohort from being paired with a different TextEq occurrence.
pub(in crate::mir::builder) fn issue_common_v2_s6c_textref_entry_bridge_plan_for_occurrence_v1(
    occurrence: &S6CTextEqOccurrencePhysicalViewV1<'_>,
    sidecar: &PhysicalTextEntryLaneSidecarV1,
    plan_stamp: u64,
) -> Result<CommonV2S6CTextRefEntryBridgePlanV1, CommonV2S6CTextRefEntryBridgeRejectV1> {
    if occurrence.owner() != sidecar.owner() || occurrence.entry() != sidecar.entry() {
        return Err(CommonV2S6CTextRefEntryBridgeRejectV1::OwnerMismatch);
    }
    let matching_rows = sidecar.rows().iter().filter(|row| {
        row.binding() == occurrence.binding()
            && row.logical_ordinal() == occurrence.logical_ordinal()
    });
    if matching_rows.count() != 1 {
        return Err(CommonV2S6CTextRefEntryBridgeRejectV1::OccurrenceBindingMismatch);
    }
    issue_common_v2_s6c_textref_entry_bridge_plan_v1(sidecar, plan_stamp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::resolved_lowering::physical_entry_lane_adoption::PhysicalTextEntryLaneSidecarRowV1;
    use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1;
    use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;
    use crate::mir::BindingId;

    fn sidecar(
        owner: FunctionOwnerIdV1,
        rows: &[(u32, u32, u32, u32)],
    ) -> PhysicalTextEntryLaneSidecarV1 {
        let rows = rows
            .iter()
            .map(|&(ordinal, slot, generation, binding)| {
                PhysicalTextEntryLaneSidecarRowV1::new(
                    BindingRefV1::new(owner, BindingId::new(binding)),
                    ordinal,
                    ValueId::new(slot),
                    ValueId::new(generation),
                    PhysicalCallableLaneCarrierV1::U64BitsOnI64,
                )
            })
            .collect();
        PhysicalTextEntryLaneSidecarV1::new(owner, BasicBlockId::new(3), rows)
    }

    fn owner() -> FunctionOwnerIdV1 {
        let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("owner issuer");
        issuer.issue().expect("owner")
    }

    #[test]
    fn positive_plan_orders_published_pairs_by_explicit_root_index() {
        let owner = owner();
        let plan = issue_common_v2_s6c_textref_entry_bridge_plan_v1(
            &sidecar(owner, &[(0, 1, 2, 0), (2, 5, 6, 1)]),
            9,
        )
        .expect("plan");
        assert_eq!(plan.owner(), owner);
        assert_eq!(plan.entry(), BasicBlockId::new(3));
        assert_eq!(plan.rows().len(), 2);
        let pairs = plan
            .with_published_pairs(&[0, 11, 12, 0, 0, 21, 22], |batch| {
                batch.with_pairs(|pairs| Ok(pairs.to_vec()))
            })
            .expect("pairs");
        assert_eq!(pairs, vec![(11, 12), (21, 22)]);
    }

    #[test]
    fn plan_rejects_non_monotonic_source_ordinals() {
        let owner = owner();
        let result = issue_common_v2_s6c_textref_entry_bridge_plan_v1(
            &sidecar(owner, &[(2, 1, 2, 0), (1, 5, 6, 1)]),
            1,
        );
        assert!(matches!(
            result,
            Err(CommonV2S6CTextRefEntryBridgeRejectV1::LogicalOrdinalOrder)
        ));
    }

    #[test]
    fn plan_rejects_non_adjacent_lanes_and_foreign_binding() {
        let owner_id = owner();
        let lane_result = issue_common_v2_s6c_textref_entry_bridge_plan_v1(
            &sidecar(owner_id, &[(0, 1, 3, 0)]),
            1,
        );
        assert!(matches!(
            lane_result,
            Err(CommonV2S6CTextRefEntryBridgeRejectV1::LanePairMismatch)
        ));

        let foreign = owner();
        let foreign_sidecar = PhysicalTextEntryLaneSidecarV1::new(
            owner_id,
            BasicBlockId::new(3),
            vec![PhysicalTextEntryLaneSidecarRowV1::new(
                BindingRefV1::new(foreign, BindingId::new(0)),
                0,
                ValueId::new(1),
                ValueId::new(2),
                PhysicalCallableLaneCarrierV1::U64BitsOnI64,
            )]
            .into_boxed_slice()
            .into_vec(),
        );
        assert!(matches!(
            issue_common_v2_s6c_textref_entry_bridge_plan_v1(&foreign_sidecar, 1),
            Err(CommonV2S6CTextRefEntryBridgeRejectV1::BindingOwnerMismatch)
        ));
    }

    #[test]
    fn consumed_plan_rejects_short_or_zero_runtime_lanes() {
        let owner = owner();
        let plan =
            issue_common_v2_s6c_textref_entry_bridge_plan_v1(&sidecar(owner, &[(0, 1, 2, 0)]), 1)
                .expect("plan");
        assert_eq!(
            plan.with_published_pairs(&[0, 7], |_| Ok(())),
            Err(CommonV2S6CTextRefEntryBridgeRejectV1::LaneInputTooShort)
        );

        let plan =
            issue_common_v2_s6c_textref_entry_bridge_plan_v1(&sidecar(owner, &[(0, 1, 2, 0)]), 1)
                .expect("plan");
        assert_eq!(
            plan.with_published_pairs(&[0, 0, 8], |_| Ok(())),
            Err(CommonV2S6CTextRefEntryBridgeRejectV1::ZeroPublishedPair)
        );
    }
}
