//! Caller-zero common Loop physicalizer facade.
//!
//! `topology` owns the recursive block skeleton, `operation_emitter` owns the
//! private operation leaf seams, and each focused test module owns its own
//! evidence. Full physicalization and production activation remain closed.

#[cfg(test)]
mod callable_canary;
#[cfg(test)]
mod callable_production_canary_tests;
mod carrier_emitter;
#[cfg(test)]
mod generic_production_canary_tests;
mod operation_dispatcher;
mod operation_emitter;
#[cfg(test)]
#[path = "operation_family_tests.rs"]
mod operation_family_tests;
mod operation_ledger;
mod operation_target;
mod operation_type;
#[cfg(test)]
#[path = "read_emitter_tests.rs"]
mod read_emitter_tests;
mod recursive_after;
mod segment_allocator;
mod segment_dispatcher;
mod segment_topology;
#[cfg(test)]
mod tail_completion;
mod tests;
mod topology;

use carrier_emitter::*;
use operation_dispatcher::*;
pub(super) use operation_emitter::*;
use operation_ledger::*;
pub(super) use segment_allocator::allocate_for_layout;
pub(super) use segment_dispatcher::preflight_loop_segment_operation_dispatch_v1;
pub(super) use segment_topology::LoopPhysicalSegmentBlockReceiptV1;
pub(super) use topology::ReadyLoopEntryV1;
pub(super) use topology::*;
