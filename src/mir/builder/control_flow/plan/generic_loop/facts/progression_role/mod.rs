//! Pure progression-role observations for generic-loop candidates.
//!
//! This module does not select a candidate, build a Recipe, or change loop
//! acceptance. A2 consumes these observations after A1 fixes their contract.

pub(in crate::mir::builder) mod observation;
