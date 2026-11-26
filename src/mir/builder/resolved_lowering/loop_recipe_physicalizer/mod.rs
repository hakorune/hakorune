//! Common Loop physicalizer facade.
//!
//! `topology` owns shared physical services and entry transport,
//! `operation_emitter` owns private operation leaf seams, and segment modules
//! own the shared placement route. Generic G0 emission is covered by the
//! existing test-only canaries.

mod carrier_emitter;
#[cfg(test)]
mod generic_production_canary_tests;
// Caller-zero until the Main0 canonical-root handoff wires the installed App
// Main caller; recheck at the production-switch slice.
// Caller-zero until the same Main0 canonical-root handoff wires the second
// profile; recheck at the production-switch slice.
// Caller-zero until the same Main0 canonical-root handoff wires the third
// profile; recheck at the production-switch slice.
mod operation_dispatcher;
mod operation_emitter;
mod operation_ledger;
mod operation_target;
mod operation_type;
mod pure_operation_emitter;
mod recursive_after;
mod segment_allocator;
mod segment_dispatcher;
mod segment_topology;
mod topology;

// Caller-zero re-export until the Main0 canonical-root handoff wires the
// installed App Main caller; recheck at the production-switch slice.
// Caller-zero re-export for the second bounded Main0 profile; same handoff.
// Caller-zero re-export for the third bounded Main0 profile; same handoff.
pub(super) use operation_dispatcher::LoopOperationDispatchServicesV1;
#[cfg(test)]
pub(super) use segment_allocator::allocate_for_layout;
#[cfg(test)]
pub(super) use segment_dispatcher::emit_loop_segment_operation_dispatch_v1;
#[cfg(test)]
#[cfg(test)]
pub(super) use segment_topology::LoopPhysicalSegmentBlockReceiptV1;
#[cfg(test)]
pub(super) use topology::ReadyLoopEntryV1;
pub(super) use topology::*;
