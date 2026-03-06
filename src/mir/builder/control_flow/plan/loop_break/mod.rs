//! Phase 263 P0.2: loop_break module structure
//!
//! This module organizes loop_break route logic with a clear SSOT structure:
//! - `api/` - Public entry point for promotion logic (SSOT)

pub(in crate::mir::builder) mod api;
pub(in crate::mir::builder) mod contracts;
