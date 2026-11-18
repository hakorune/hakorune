//! JoinIR integration for control flow
//!
//! This module contains JoinIR-related control flow logic:
//! - Route lowerers (active module surface and physical path: route_entry/)
//! - Routing logic (routing.rs) ✅
//! - Parity verification (parity_checker.rs) ✅ Phase 138
//! - Unified tracing (trace.rs) ✅ Phase 195
//! - Control tree capability guard (control_tree_capability_guard.rs) ✅ Phase 112

pub(in crate::mir::builder) mod control_tree_capability_guard;
pub(in crate::mir::builder) mod route_entry;
pub(in crate::mir::builder) mod routing;
pub(in crate::mir::builder) mod trace;
