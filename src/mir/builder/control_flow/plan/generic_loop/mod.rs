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

// Re-export public API (maintains backward compatibility)
#[allow(unused_imports)] // Facade-style re-exports; not all are used in every build/profile.
pub(in crate::mir::builder) use facts::extract::{
    has_generic_loop_v1_recipe_hint, try_extract_generic_loop_v0_facts,
    try_extract_generic_loop_v1_facts,
};
#[allow(unused_imports)]
pub(in crate::mir::builder) use facts_types::{GenericLoopV0Facts, GenericLoopV1Facts};
#[allow(unused_imports)]
pub(in crate::mir::builder) use body_check::shape_resolution::resolve_v1_shape_matches;
