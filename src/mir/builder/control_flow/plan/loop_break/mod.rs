//! Phase 263 P0.2: loop_break module structure
//!
//! This module organizes loop_break route logic with a clear SSOT structure:
//! - `facts/` - Entry namespace for scattered loop_break fact extractors/types

pub(in crate::mir::builder) mod facts;
