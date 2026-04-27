//! Compatibility layer for route policies already moved to top-level owners.
//!
//! During folderization, `plan/policies/keep_plan/*` remains as a thin re-export shelf
//! for cleanup-side policy boxes that still have historical import paths.
//!
//! 詳細は [README.md](README.md) を参照してください。

pub use crate::mir::policies::PolicyDecision;

pub(in crate::mir::builder) mod p5b_escape_derived_policy;
pub(in crate::mir::builder) mod trim_policy; // Phase 132 P0.5
