//! Phase 255 P2: Common utilities for JoinIR route lowering
//!
//! This module provides shared helper functions used across different route
//! lowering implementations, eliminating code duplication and ensuring consistency.

mod carrier_binding_policy;
mod joinir_helpers; // Phase 256.8.5: JoinModule helpers

pub(crate) use carrier_binding_policy::{decide_carrier_binding_policy, CarrierBindingPolicy};
pub(crate) use joinir_helpers::get_entry_function; // Phase 256.8.5
