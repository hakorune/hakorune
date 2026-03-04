//! Generic loop v0 module (facts + normalizer SSOT)
//!
//! SSOT: docs/development/current/main/design/plan-dir-shallowing-ssot.md
//! Flattened: facts/ and facts/body_check/ moved to generic_loop/ root

pub(in crate::mir::builder) mod body_check;
pub(in crate::mir::builder) mod body_check_extractors;
pub(in crate::mir::builder) mod body_check_shape_detectors;
#[cfg(test)]
pub(in crate::mir::builder) mod body_check_tests;
pub(in crate::mir::builder) mod facts;
pub(in crate::mir::builder) mod facts_helpers;
pub(in crate::mir::builder) mod facts_types;
pub(in crate::mir::builder) mod normalizer;
