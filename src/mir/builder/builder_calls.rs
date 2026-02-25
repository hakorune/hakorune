//! 🎯 箱理論 Phase 2完了: builder_calls.rs → calls/* 完全移行
//!
//! **削減実績**:
//! - Phase 1: 982行 → 766行（216行削減、22%削減）
//! - Phase 2: 766行 → 49行（717行削減、94%削減）
//! - **合計削減**: 933行（95%削減達成！）
//!
//! **移行先**:
//! - `calls/emit.rs`: Call命令発行（emit_unified_call, emit_legacy_call等）
//! - `calls/build.rs`: Call構築（build_function_call, build_method_call等）
//! - `calls/lowering.rs`: 関数lowering（Phase 1で既に移行済み）
//! - `calls/utils.rs`: ユーティリティ（Phase 1で既に移行済み）
//!
//! **箱理論の原則**:
//! 1. ✅ 責務ごとに箱に分離: emit（発行）、build（構築）を明確に分離
//! 2. ✅ 境界を明確に: 各モジュールで公開インターフェース明確化
//! 3. ✅ いつでも戻せる: re-exportで既存API完全保持
//! 4. ✅ 巨大関数は分割: 100行超える関数を30-50行目標で分割

// Import from new modules (refactored with Box Theory)
pub use super::calls::call_target::CallTarget;

// ========================================
// Re-exports for backward compatibility
// ========================================

impl super::MirBuilder {
    // 🎯 Phase 2移行完了マーカー: すべての実装は calls/* に移行済み

    /// Map a user-facing type name to MIR type
    /// 実装: calls/utils.rs
    pub(super) fn parse_type_name_to_mir(name: &str) -> super::MirType {
        crate::mir::builder::calls::utils::parse_type_name_to_mir(name)
    }

    /// Extract string literal from AST node if possible
    /// 実装: calls/utils.rs
    pub(super) fn extract_string_literal(node: &crate::ast::ASTNode) -> Option<String> {
        crate::mir::builder::calls::utils::extract_string_literal(node)
    }

    // Note: All other methods (emit_unified_call, build_function_call, etc.)
    // are automatically available via `pub use super::calls::*;`
}
