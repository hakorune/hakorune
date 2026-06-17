//! Compatibility facade for object storage planning vocabulary.
//!
//! The canonical passive vocabulary now lives in `hakorune-mir-plans`.
//! Keep this facade so existing main-crate call sites do not need a broad
//! import rewrite during the crate split.

pub use hakorune_mir_plans::object_storage_plan::*;
