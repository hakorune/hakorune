//! Phase 263 P0.2: Pattern2 module structure
//!
//! This module organizes Pattern2 logic with a clear SSOT structure:
//! - `api/` - Public entry point for promotion logic (SSOT)

pub(in crate::mir::builder) mod api;
pub(in crate::mir::builder) mod contracts;
