//! Conservative PHI Generation - Box Theory Application
//!
//! # Theory: Conservative ∘ Elimination = Minimal SSA
//!
//! - Conservative: correctness-first, generate all PHIs
//! - Elimination (future): efficiency optimization, remove unused PHIs
//!
//! # Phase 58 削除完了（2025-11-29）
//!
//! ## 削除内容
//!
//! - **ConservativeMerge struct**: 削除（約60行）
//! - **analyze メソッド**: `phi_merge.rs::merge_all_vars` にインライン化
//! - **テストコード**: 削除（約35行）
//!
//! ## 移行先
//!
//! `src/mir/builder/phi_merge.rs` の `PhiMergeHelper::merge_all_vars` メソッド内に
//! すべてのロジックがインライン化されている。
//!
//! ## 理論コメント保持理由
//!
//! Conservative ∘ Elimination = Minimal SSA の理論は重要なため、
//! このファイルはドキュメントとして残す。
//!
//! ## 旧コード履歴
//!
//! - Phase 25.1q: Conservative PHI戦略の一元化
//! - Phase 41-1: get_conservative_values 削除
//! - Phase 42: trace_if_enabled 削除
//! - Phase 47: compute_modified_names インライン化
//! - Phase 58: ConservativeMerge struct 完全削除
//!
//! ## 参照
//!
//! - Phase 37分析: `docs/.../phase-37-if-phi-reduction/conservative_responsibility_table.md`
//! - Phase 39設計: `docs/.../phase-39-if-phi-level2/joinir_extension_design.md`

// ========================================
// Phase 58削除済み（2025-11-29）
// ========================================
//
// 以下のコードは phi_merge.rs にインライン化済み:
//
// pub struct ConservativeMerge {
//     pub all_vars: HashSet<String>,
//     pub changed_vars: HashSet<String>,
// }
//
// impl ConservativeMerge {
//     pub fn analyze(...) -> Self { ... }
// }
//
// テストも削除:
// - test_conservative_merge_both_defined
// - test_conservative_merge_union
