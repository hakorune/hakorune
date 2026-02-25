//! Phase 145 P0: ANF (A-Normal Form) module
//!
//! ## Responsibility
//!
//! Provides ANF transformation for impure expressions (Call/MethodCall) in Normalized JoinIR.
//! Ensures deterministic evaluation order (left-to-right, depth-first) for side-effecting expressions.
//!
//! ## Architecture (Box-First, 3-layer separation)
//!
//! - **contract.rs**: Diagnostic tags, out-of-scope reasons, plan structure (SSOT)
//! - **plan_box.rs**: AST pattern detection (requires_anf?, impure_count?)
//! - **execute_box.rs**: ANF transformation execution (P0: stub, P1+: implementation)
//!
//! ## Phase Scope
//!
//! - **P0**: Skeleton only (execute_box always returns Ok(None))
//! - **P1**: String.length() hoist (whitelist 1 intrinsic)
//! - **P2**: Compound expression ANF (recursive left-to-right linearization)
//!
//! ## Contract
//!
//! - Out-of-scope returns Ok(None) (graceful fallback)
//! - Default behavior unchanged (P0 is non-invasive skeleton)
//! - Strict mode (HAKO_ANF_DEV=1): Debug logging only (P0 has no fail-fast)

pub mod contract;
pub mod execute_box;
pub mod plan_box;

pub use contract::{AnfDiagnosticTag, AnfOutOfScopeReason, AnfPlan};
pub use execute_box::AnfExecuteBox;
pub use plan_box::AnfPlanBox;
