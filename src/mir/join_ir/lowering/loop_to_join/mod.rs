//! Phase 31/33-23: Loop→JoinIR lowering entry (LoopToJoinLowerer)
//!
//! このモジュールは「LoopForm を JoinIR (JoinModule) に変換する統一エントリ」を提供する。
//! 本体ロジックは coordinator に集約し、責務を分割して保守性を上げる。
//!
//! ## 責務境界（Box化）
//! - `LoopPatternValidator`（`../loop_pattern_validator.rs`）: 構造検証（shape guard）
//! - `LoopViewBuilder`（`../loop_view_builder.rs`）: lowerer 選択（routing）
//! - `LoopToJoinLowerer`（このモジュール）: intake/scope 構築と strict ハンドリング（coordinator）
//!
//! ## 注意
//! - Phase 進捗ログはここに混ぜない（現役の導線のみ）。
//! - “とりあえず通す”フォールバックは増やさない。失敗は `None` で上位に返す。

mod case_a_entrypoints;
mod core;

pub use core::LoopToJoinLowerer;
