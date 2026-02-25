//! Phase 200-A: Function scope capture infrastructure
//!
//! This module provides types and analysis functions for capturing function-scoped variables
//! that are effectively immutable within a loop context.
//!
//! # Example
//!
//! For a function like JsonParser._atoi():
//!
//! ```nyash
//! method _atoi(s, pos, len) {
//!     local digits = "0123456789"  // <-- Captured variable
//!     local value = 0
//!     loop(pos < len) {
//!         local ch = s.charAt(pos)
//!         local digit = digits.indexOf(ch)  // Uses captured 'digits'
//!         if (digit < 0) { break }
//!         value = value * 10 + digit
//!         pos = pos + 1
//!     }
//!     return value
//! }
//! ```
//!
//! Here, `digits` is:
//! - Declared in function scope (before the loop)
//! - Never reassigned (effectively immutable)
//! - Referenced in loop body (digits.indexOf(ch))
//!
//! Phase 200-A creates the infrastructure to capture such variables.
//! Phase 200-B implements the actual detection logic.
//!
//! # Module Structure
//!
//! This module is organized following the Box-First principle:
//!
//! - `types` - Core type definitions (CapturedVar, CapturedEnv)
//! - `analyzers` - Analysis functions (analyze_captured_vars, analyze_captured_vars_v2)
//! - `helpers` - Helper functions for AST analysis and structural matching
//!
//! # Public API
//!
//! The primary entry points are:
//!
//! - `analyze_captured_vars()` - Main analysis function (uses pointer comparison)
//! - `analyze_captured_vars_v2()` - Alternative using structural matching (Phase 200-C)
//! - `CapturedVar` - Represents a captured variable
//! - `CapturedEnv` - Environment containing all captured variables

// Module declarations
mod analyzers;
mod helpers;
mod types;

// Public re-exports
pub(crate) use analyzers::analyze_captured_vars_v2;
pub use types::{CapturedEnv, CapturedKind, CapturedVar};
