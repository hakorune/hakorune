//! Carrier binding policy for loop_break route inputs
//!
//! Responsibility:
//! - Decide whether a carrier should bind to a host ValueId
//! - Keep ConditionOnly / loop-local carriers out of ConditionBindings

use crate::mir::join_ir::lowering::carrier_info::{CarrierInit, CarrierRole, CarrierVar};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CarrierBindingPolicy {
    BindFromHost,
    SkipBinding,
}

pub(crate) fn decide_carrier_binding_policy(carrier: &CarrierVar) -> CarrierBindingPolicy {
    // ConditionOnly carriers should never be sourced from host values.
    debug_assert!(
        !(carrier.role == CarrierRole::ConditionOnly && matches!(carrier.init, CarrierInit::FromHost)),
        "ConditionOnly carriers must not use FromHost init"
    );

    match carrier.init {
        CarrierInit::FromHost => CarrierBindingPolicy::BindFromHost,
        CarrierInit::BoolConst(_) | CarrierInit::LoopLocalZero => CarrierBindingPolicy::SkipBinding,
    }
}
