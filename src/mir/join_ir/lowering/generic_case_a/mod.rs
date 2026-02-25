//! Generic Case A LoopForm → JoinIR Lowering (Modularized)
//!
//! Phase 192: Modularization into focused, single-responsibility modules.
//!
//! ## Overview
//!
//! This module provides Case A lowering for four minimal SSA loop patterns:
//! - **skip_ws**: Whitespace skipping loop (Main.skip/1)
//! - **trim**: String trimming loop (FuncScannerBox.trim/1)
//! - **append_defs**: Array concatenation loop (FuncScannerBox.append_defs/2)
//! - **stage1_using_resolver**: Using namespace resolution loop (Stage1UsingResolverBox.resolve_for_source/5)
//!
//! ## Architecture
//!
//! ### Core Lowering Modules (Pattern-Specific)
//!
//! Each lowering module handles one specific loop pattern:
//!
//! - `skip_ws` - Skip whitespace loop lowering (~220 lines)
//! - `trim` - String trim loop lowering (~500 lines, largest)
//! - `append_defs` - Array append loop lowering (~170 lines)
//! - `stage1_using_resolver` - Using resolver loop lowering (~180 lines)
//!
//! ### Helper Modules (Shared Utilities)
//!
//! - `entry_builder` - Entry function construction helper (~150 lines)
//! - `whitespace_check` - Whitespace detection utilities (~150 lines)
//!
//! ## Design Constraints (Critical)
//!
//! - **No condition analysis**: Compare/BinOp instructions are copied as-is from MIR
//! - **No multi-header loops**: Only single-header loops supported (v1 limitation)
//! - **Pinned/Carrier from LoopScopeShape**: Must be provided by caller
//! - **Fail-fast**: Returns `None` on pattern mismatch, caller handles fallback
//!
//! ## Public API
//!
//! All lowering functions follow the same signature:
//!
//! ```rust,ignore
//! pub(crate) fn lower_case_a_PATTERN_with_scope(
//!     scope: LoopScopeShape
//! ) -> Option<JoinModule>
//! ```
//!
//! ## Usage Example
//!
//! ```rust,ignore
//! use crate::mir::join_ir::lowering::generic_case_a;
//! use crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape;
//!
//! // Build LoopScopeShape from loop structure
//! let scope = LoopScopeShape::from_loop_form(&loop_form)?;
//!
//! // Try skip_ws lowering
//! if let Some(join_module) = generic_case_a::lower_case_a_skip_ws_with_scope(scope) {
//!     // JoinIR successfully generated
//!     return Some(join_module);
//! }
//! // Pattern mismatch, fallback to other lowering
//! ```
//!
//! ## Module Organization (Phase 192)
//!
//! **Before modularization**:
//! - generic_case_a.rs: 1,056 lines monolith (all 4 patterns + helpers)
//!
//! **After modularization** (Phase 192 complete):
//! - mod.rs: 93 lines (coordinator, **91% reduction**)
//! - skip_ws.rs: 258 lines (whitespace skipping)
//! - trim.rs: 537 lines (string trimming, largest module)
//! - append_defs.rs: 202 lines (array concatenation)
//! - stage1_using_resolver.rs: 228 lines (namespace resolution)
//! - entry_builder.rs: 165 lines (helper, shared initialization)
//! - whitespace_check.rs: 151 lines (helper, shared validation)
//!
//! **Total**: 1,634 lines modularized (7 focused modules)
//! **Average module size**: 233 lines (vs. 1,056-line monolith)
//!
//! ## See Also
//!
//! - `loop_scope_shape` - LoopScopeShape construction
//! - `value_id_ranges` - ValueId allocation strategy
//! - `loop_to_join` - Main loop lowering coordinator

// Pattern-specific lowering modules
pub mod append_defs;
pub mod skip_ws;
pub mod stage1_using_resolver;
pub mod trim;

// Helper modules
pub mod entry_builder;
pub mod whitespace_check;

// Re-export public lowering functions
pub(crate) use append_defs::lower_case_a_append_defs_with_scope;
pub(crate) use skip_ws::lower_case_a_skip_ws_with_scope;
pub(crate) use stage1_using_resolver::lower_case_a_stage1_usingresolver_with_scope;
pub(crate) use trim::lower_case_a_trim_with_scope;

// Re-export helper utilities
