//! Compatibility wrappers for facts-owned descriptive extractors.
//!
//! Only common helper wrappers remain here. Route-specific extractor facades
//! should not be regrown; callers import facts-owned route extractors directly.

pub(in crate::mir::builder) mod common_helpers;
