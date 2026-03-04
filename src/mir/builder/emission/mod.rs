//! emission: MIR命令の薄い発行箱（仕様不変）。
//! - constant.rs: Const発行を一箇所に集約
//! - compare.rs: Compare命令の薄い発行
//! - branch.rs: Branch/Jump 発行の薄い関数
//! - phi.rs: PHI挿入の薄いラッパー（builder context extraction）
//! - phi_lifecycle.rs: PHI lifecycle SSOT（Reserve→Define→Populate→Finalize）
//! - loop_split_scan.rs: Pattern7 split scan EdgeCFG Frag (Phase 272 P0.2)
//!
//! Phase 273 P3: loop_scan_with_init.rs removed (replaced by generalized Frag API)

pub mod branch;
pub mod compare;
pub mod constant;
pub(crate) mod copy_emitter;
pub(in crate::mir::builder) mod phi;  // Phase 272 P0.2 Refactoring
pub(in crate::mir::builder) mod phi_lifecycle;  // PHI lifecycle SSOT
pub(in crate::mir::builder) mod value_lifecycle;  // Value lifecycle contract (typed → defined)
pub(in crate::mir::builder) mod loop_split_scan;  // Phase 272 P0.2
