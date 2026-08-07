//! Caller-zero common Loop topology physicalizer facade.
//!
//! This directory is a behavior-neutral BoxShape split. `topology` owns the
//! existing recursive block skeleton and `tests` owns its focused evidence.
//! Operation demand, leaf emission, and production activation remain outside
//! this row.

mod tests;
mod topology;

pub(super) use topology::*;
