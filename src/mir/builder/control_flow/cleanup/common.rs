//! Common cleanup helpers for JoinIR route lowering.

mod carrier_binding_policy;
mod joinir_helpers;

pub(crate) use carrier_binding_policy::{decide_carrier_binding_policy, CarrierBindingPolicy};
pub(crate) use joinir_helpers::get_entry_function;
