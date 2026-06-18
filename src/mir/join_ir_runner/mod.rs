//! JoinIR Runner - Development Harness (Structure Validation Only)
//!
//! # Two Routes
//!
//! ## Route A: JoinIR→MIR→VM (Recommended SSOT)
//! - Full semantic validation via MIR lowering pipeline
//! - Tests should use `JoinIrFrontendTestRunner` or `run_joinir_via_vm`
//! - Examples: Phase 34 tests (IfSelect, Loop, Break, Continue)
//! - **Use this route for ALL semantic tests**
//!
//! ## Route B: Direct JoinIR Runner (Structure Validation)
//! - For structure-only validation of JoinIR constructs
//! - Use `run_joinir_function` only when Route A is insufficient
//! - Examples: Handwritten JoinIR module tests, low-level instruction tests
//! - Note: Some operations (e.g., MethodCall) may be unimplemented in Runner
//!
//! # Phase 35-4 Unification Strategy
//! All semantic tests migrated to Route A. Route B kept only for fundamental
//! structure validation that cannot be verified through MIR→VM path.
//!
//! # Original Purpose (Phase 27.2)
//! hand-written / minimal JoinIR を VM と A/B 比較するための軽量ランナー。
//! - 対応値: i64 / bool / String / Unit
//! - 対応命令: Const / BinOp / Compare / BoxCall(StringBox: length, substring) /
//!              Call / Jump / Ret
//!
//! Phase 27.8: ops box 統合
//! - JoinValue / JoinIrOpError は join_ir_ops から再エクスポート
//! - eval_binop() / eval_compare() を使用（実装を一箇所に集約）

#[cfg(feature = "vm-reference")]
mod api;
#[cfg(feature = "vm-reference")]
mod exec;
#[cfg(test)]
mod tests;

// Phase 27.8: ops box からの再エクスポート
pub use crate::mir::join_ir_ops::{JoinIrOpError, JoinValue};

// Phase 27.8: 互換性のため JoinRuntimeError を JoinIrOpError の別名として保持
pub type JoinRuntimeError = JoinIrOpError;

#[cfg(feature = "vm-reference")]
pub use api::run_joinir_function;
