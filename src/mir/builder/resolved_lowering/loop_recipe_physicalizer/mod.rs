//! Caller-zero common Loop physicalizer facade.
//!
//! `topology` owns the recursive block skeleton, `operation_emitter` owns the
//! private operation leaf seams, and each focused test module owns its own
//! evidence. Full physicalization and production activation remain closed.

mod operation_dispatcher;
mod operation_emitter;
#[cfg(test)]
#[path = "operation_family_tests.rs"]
mod operation_family_tests;
mod operation_ledger;
#[cfg(test)]
#[path = "read_emitter_tests.rs"]
mod read_emitter_tests;
mod tests;
mod topology;

use operation_dispatcher::*;
pub(super) use operation_emitter::*;
use operation_ledger::*;
pub(super) use topology::*;
