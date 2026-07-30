//! Phase 27.10: Common utilities for JoinIR lowering.
//!
//! This file is the stable import facade. Keep implementation ownership in the
//! submodules below so CFG shape probes and dispatch policy do not
//! grow into one mixed helper shelf.

pub mod case_a;
mod cfg_shape;
mod dispatch;
pub(crate) mod string_whitespace;
mod target_adapter;

pub use cfg_shape::{
    construct_simple_while_loopform, ensure_entry_has_succs, has_binop, has_const_int,
    has_const_string, has_string_method,
};
pub use dispatch::{dispatch_lowering, log_fallback};
pub use target_adapter::try_generic_case_a_route;
