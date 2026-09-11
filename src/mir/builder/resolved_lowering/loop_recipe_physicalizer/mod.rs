//! Common Loop physicalizer facade.
//!
//! `topology` owns shared physical services and entry transport,
//! `operation_emitter` owns private operation leaf seams, and segment modules
//! own the current production placement route. The bounded CallableSingleLoop
//! and Generic G0 consumers are production-connected; broader GenericLoop
//! physicalization remains closed.

mod callable_canary;
#[cfg(test)]
mod callable_production_canary_tests;
mod callable_lowerer;
mod carrier_emitter;
mod compare_i64_operands;
mod compare_i64_writer;
#[cfg(test)]
mod compare_i64_writer_tests;
#[cfg(test)]
mod compare_result_ledger;
mod generic_lowerer;
#[cfg(test)]
mod generic_production_canary_tests;
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
mod tail_completion;
mod topology;

pub(super) use operation_dispatcher::LoopOperationDispatchServicesV1;
pub(in crate::mir::builder) use callable_lowerer::lower_callable_single_loop_function_draft_v1;
pub(in crate::mir::builder) use generic_lowerer::lower_generic_g0_function_draft_v1;
pub(in crate::mir::builder) use generic_lowerer::lower_generic_g0_function_draft_pending_v1;
pub(super) use segment_allocator::allocate_for_layout;
pub(super) use segment_dispatcher::emit_loop_segment_operation_dispatch_v1;
pub(super) use segment_dispatcher::preflight_loop_segment_operation_dispatch_v1;
pub(super) use segment_topology::LoopPhysicalSegmentBlockReceiptV1;
pub(super) use topology::ReadyLoopEntryV1;
pub(super) use topology::*;
