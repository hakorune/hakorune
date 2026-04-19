//! Runtime-private bridge from named demand to executor lane action.
//!
//! MIR/lowering owns legality and provenance. This module only turns an already
//! selected `DemandSet` into the concrete runtime action used by executor code.
#![allow(dead_code)]

use super::value_demand::{
    DemandSet, PublishDemand, StorageDemand, ValueDemand, ARRAY_TEXT_DEGRADE_GENERIC,
    ARRAY_TEXT_OWNED_CELL,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ValueLaneAction {
    TextCellResidence,
    GenericBoxResidence,
    PublishBoundary,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ValueLanePlan {
    pub(crate) action: ValueLaneAction,
    pub(crate) demand: DemandSet,
}

impl ValueLanePlan {
    #[inline(always)]
    fn new(action: ValueLaneAction, demand: DemandSet) -> Self {
        Self { action, demand }
    }
}

#[inline(always)]
pub(crate) fn array_text_cell_store_lane_plan() -> ValueLanePlan {
    plan_text_cell_residence(ARRAY_TEXT_OWNED_CELL)
        .expect("ARRAY_TEXT_OWNED_CELL must map to TextCellResidence")
}

#[inline(always)]
pub(crate) fn array_text_degrade_generic_lane_plan() -> ValueLanePlan {
    plan_generic_box_residence(ARRAY_TEXT_DEGRADE_GENERIC)
        .expect("ARRAY_TEXT_DEGRADE_GENERIC must map to GenericBoxResidence")
}

#[inline(always)]
fn plan_text_cell_residence(demand: DemandSet) -> Option<ValueLanePlan> {
    if has_value(demand, ValueDemand::OwnedPayload)
        && has_storage(demand, StorageDemand::CellResidence)
        && demand.publish.is_empty()
        && !has_storage(demand, StorageDemand::GenericResidence)
    {
        Some(ValueLanePlan::new(
            ValueLaneAction::TextCellResidence,
            demand,
        ))
    } else {
        None
    }
}

#[inline(always)]
fn plan_generic_box_residence(demand: DemandSet) -> Option<ValueLanePlan> {
    if has_storage(demand, StorageDemand::GenericResidence)
        && (has_storage(demand, StorageDemand::DegradeGeneric)
            || has_value(demand, ValueDemand::StableObject))
        && demand.publish.is_empty()
    {
        Some(ValueLanePlan::new(
            ValueLaneAction::GenericBoxResidence,
            demand,
        ))
    } else {
        None
    }
}

#[inline(always)]
fn plan_publish_boundary(demand: DemandSet) -> Option<ValueLanePlan> {
    if has_publish(demand, PublishDemand::NeedStableObject)
        && has_value(demand, ValueDemand::StableObject)
    {
        Some(ValueLanePlan::new(ValueLaneAction::PublishBoundary, demand))
    } else {
        None
    }
}

#[inline(always)]
fn has_value(demand: DemandSet, needle: ValueDemand) -> bool {
    demand.value.iter().any(|value| *value == needle)
}

#[inline(always)]
fn has_storage(demand: DemandSet, needle: StorageDemand) -> bool {
    demand.storage.iter().any(|storage| *storage == needle)
}

#[inline(always)]
fn has_publish(demand: DemandSet, needle: PublishDemand) -> bool {
    demand.publish.iter().any(|publish| *publish == needle)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin::value_demand::{
        ARRAY_GENERIC_STORE_ANY, ARRAY_TEXT_READ_REF, PUBLISH_NEED_STABLE_OBJECT,
    };

    #[test]
    fn text_cell_store_demand_maps_to_text_cell_residence() {
        let plan = array_text_cell_store_lane_plan();
        assert_eq!(plan.action, ValueLaneAction::TextCellResidence);
        assert_eq!(plan.demand, ARRAY_TEXT_OWNED_CELL);
        assert!(plan.demand.publish.is_empty());
    }

    #[test]
    fn text_degrade_demand_maps_to_generic_box_residence() {
        let plan = array_text_degrade_generic_lane_plan();
        assert_eq!(plan.action, ValueLaneAction::GenericBoxResidence);
        assert_eq!(plan.demand, ARRAY_TEXT_DEGRADE_GENERIC);
        assert!(plan.demand.publish.is_empty());
    }

    #[test]
    fn generic_store_any_is_not_a_text_cell_plan() {
        assert_eq!(plan_text_cell_residence(ARRAY_GENERIC_STORE_ANY), None);
    }

    #[test]
    fn read_ref_demand_is_not_a_storage_plan() {
        assert_eq!(plan_text_cell_residence(ARRAY_TEXT_READ_REF), None);
        assert_eq!(plan_generic_box_residence(ARRAY_TEXT_READ_REF), None);
    }

    #[test]
    fn publish_need_stable_object_maps_to_boundary_action() {
        let plan = plan_publish_boundary(PUBLISH_NEED_STABLE_OBJECT)
            .expect("publish demand should map to boundary action");
        assert_eq!(plan.action, ValueLaneAction::PublishBoundary);
        assert_eq!(plan.demand, PUBLISH_NEED_STABLE_OBJECT);
    }
}
