//! One bounded selected Stage-B instance-function session.
//!
//! F6-2 owns only the body schedule. It reuses the existing legacy block
//! driver once and keeps prefix, selected carrier publication, and suffix
//! descent behind one monotonic owner. Function preparation/finalization and
//! pending-session capture remain F6-3 responsibilities.

mod body_schedule;
mod rejection;

#[cfg(test)]
mod body_schedule_tests;
