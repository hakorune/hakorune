//! Generic Case A LoopForm → JoinIR Lowering (Modularized)
//!
//! Phase 192: Modularization into focused, single-responsibility modules.
//!
//! ## Overview
//!
//! This module provides Case A lowering for three minimal SSA loop route shapes:
//! - **skip_ws**: Whitespace skipping loop (Main.skip/1)
//! - **trim**: String trimming loop (FuncScannerBox.trim/1)
//! - **stage1_using_resolver**: lower-resolver compatibility loop (Stage1UsingResolverBox.resolve_for_source/5)
//!
//! ## Architecture
//!
//! ### Core Lowering Modules (Route-Specific)
//!
//! Each lowering module handles one specific loop route shape:
//!
//! - `skip_ws` - Skip whitespace loop lowering (~220 lines)
//! - `trim` - String trim loop lowering (~500 lines, largest)
//! - `stage1_using_resolver` - lower-resolver loop lowering (~180 lines)
//!
//! ### Helper Modules (Shared Utilities)
//!
//! - `entry_builder` - Entry function construction helper (~150 lines)
//!
//! ## Design Constraints (Critical)
//!
//! - **No condition analysis**: Compare/BinOp instructions are copied as-is from MIR
//! - **No multi-header loops**: Only single-header loops supported (v1 limitation)
//! - **Pinned/Carrier from LoopScopeShape**: Must be provided by caller
//! - **Fail-fast**: Returns `None` on route-shape mismatch, caller handles fallback
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
//! // Route-shape mismatch, fallback to other lowering
//! ```
//!
//! ## Module Organization (Phase 192)
//!
//! The current focused modules are listed above. Historical file-size totals are
//! intentionally not a routing or retention contract.
//!
//! ## See Also
//!
//! - `loop_scope_shape` - LoopScopeShape construction
//! - `value_id_ranges` - ValueId allocation strategy
//! - `loop_to_join` - Main loop lowering coordinator

// Route-specific lowering modules
pub mod skip_ws;
pub mod stage1_using_resolver;
pub mod trim;

// Helper modules
pub mod entry_builder;

// Re-export public lowering functions
pub(crate) use skip_ws::lower_case_a_skip_ws_with_scope;
pub(crate) use stage1_using_resolver::lower_case_a_stage1_usingresolver_with_scope;
pub(crate) use trim::lower_case_a_trim_with_scope;

// Re-export helper utilities
