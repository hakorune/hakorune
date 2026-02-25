//! LoopScopeShape - ループ変数スコープの統合ビュー
//!
//! Box化して責務を分離:
//! - `shape`: 変数分類のSSOTと質問系API
//! - `builder`: LoopForm / 既存箱からの組み立て（Case-A ルーティング含む）
//! - `case_a`: Case-A minimal ターゲット判定（Phase 30 F-3.1）
//! - `case_a_lowering_shape`: Phase 170-A Case-A lowering shape detection (構造ベース)
//! - `context`: generic_case_a 用の共通コンテキスト
//! - `structural`: ループの構造的性質解析（Phase 48-5.5）

mod builder;
mod case_a;
mod case_a_lowering_shape;
mod context;
mod shape;
mod structural;

pub(crate) use case_a::is_case_a_minimal_target;
pub(crate) use case_a_lowering_shape::CaseALoweringShape;
pub(crate) use context::CaseAContext;
pub(crate) use shape::LoopScopeShape;

#[cfg(test)]
mod tests;
