//! Compatibility facade for aggregate storage planning vocabulary.
//!
//! The canonical passive vocabulary lives in `hakorune-mir-plans` with
//! `object_storage_plan`. Keep this facade during crate split so existing
//! main-crate users do not need broad import rewrites.

pub use hakorune_mir_plans::aggregate_storage_plan::*;
