//! Carrier variable metadata for JoinIR loop lowering
//!
//! This module defines metadata structures for tracking carrier variables
//! in loop lowering. This enables dynamic generation of exit bindings
//! without hardcoded variable names or ValueIds.
//!
//! Phase 193-2: Enhanced builder methods for flexible construction
//!
//! # Phase 183-2: Primary CarrierInfo Construction
//!
//! This module is the single source of truth for CarrierInfo initialization.
//! Both MIR and JoinIR contexts use `CarrierInfo::from_variable_map()` as the
//! primary construction method.
//!
//! - MIR context: `common_init.rs` delegates to this module
//! - JoinIR context: Uses `from_variable_map()` directly
//!
//! # Phase 76: BindingId-Based Promotion Tracking
//!
//! Replaces name-based promotion hacks (`"digit_pos"` → `"is_digit_pos"`) with
//! type-safe BindingId mapping. This eliminates fragile string matching while
//! maintaining backward compatibility through dual-path lookup.

mod carrier_info_impl;
mod carrier_var;
mod exit_meta;
mod types;

#[cfg(test)]
mod tests;

pub use types::{
    CarrierInfo, CarrierInit, CarrierRole, CarrierVar, ExitMeta, ExitReconnectMode,
    JoinFragmentMeta,
};
