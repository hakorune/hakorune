/*!
 * Phase 136 Step 6/7: MetadataContext - Metadata/Span/Hint 管理の統一箱
 *
 * 責務:
 * - current_span: 現在の AST span（命令アノテーション用）
 * - source_file: ソースファイルヒント（メタデータ用）
 * - hint_sink: 型推論ヒント（ゼロコストガイダンス）
 * - current_region_stack: Region 観測用スタック（NYASH_REGION_TRACE=1 デバッグ用）
 *
 * 設計:
 * - HintSink は no-op デフォルトだが、将来の型推論最適化に備える
 * - Span は命令単位で保持され、エラー報告・デバッグ情報生成に使用
 * - source_file は関数メタデータに伝播
 * - current_region_stack は開発用トレース（本番コストゼロ）
 */

use crate::ast::Span;
use crate::mir::hints::HintSink;
use crate::mir::region::RegionId;
use crate::mir::ValueId;
use std::collections::HashMap;
use std::panic::Location;

/// Phase 136 Step 6/7: Metadata/Span/Hint 管理を統一した構造体だよ
#[derive(Debug, Clone)]
pub struct MetadataContext {
    /// 現在の AST span（命令アノテーション用）
    pub(super) current_span: Span,

    /// ソースファイルヒント（関数メタデータに伝播）
    pub(super) source_file: Option<String>,

    /// 型推論ヒント（ゼロコストガイダンス）
    pub(super) hint_sink: HintSink,

    /// Region 観測用のスタック（NYASH_REGION_TRACE=1 のデバッグ専用）
    /// - FunctionRegion がルート
    /// - 開発時のみ使用（本番コストゼロ）
    pub(super) current_region_stack: Vec<RegionId>,

    /// ValueId 起源 span（診断用）
    pub(super) value_origin_spans: HashMap<ValueId, Span>,

    /// ValueId 起源 caller（診断用）
    pub(super) value_origin_callers: HashMap<ValueId, String>,
}

impl MetadataContext {
    /// 新規 MetadataContext を生成（デフォルト状態）
    pub(crate) fn new() -> Self {
        Self {
            current_span: Span::unknown(),
            source_file: None,
            hint_sink: HintSink::new(),
            current_region_stack: Vec::new(),
            value_origin_spans: HashMap::new(),
            value_origin_callers: HashMap::new(),
        }
    }

    // ---- Span 管理 ----

    /// 現在の span を取得
    #[inline]
    pub(crate) fn current_span(&self) -> Span {
        self.current_span
    }

    /// 現在の span を設定
    #[inline]
    pub(crate) fn set_current_span(&mut self, span: Span) {
        self.current_span = span;
    }

    // ---- Source File 管理 ----

    /// ソースファイルヒントを設定
    #[inline]
    pub(crate) fn set_source_file<S: Into<String>>(&mut self, source: S) {
        self.source_file = Some(source.into());
    }

    /// ソースファイルヒントをクリア
    #[inline]
    pub(crate) fn clear_source_file(&mut self) {
        self.source_file = None;
    }

    /// 現在のソースファイルヒントを取得
    #[inline]
    pub(crate) fn current_source_file(&self) -> Option<String> {
        self.source_file.clone()
    }

    // ---- Hint Sink 管理（型推論ガイダンス）----

    /// スコープ開始ヒント（no-op デフォルト）
    #[inline]
    pub(crate) fn hint_scope_enter(&mut self, id: u32) {
        self.hint_sink.scope_enter(id);
    }

    /// スコープ終了ヒント（no-op デフォルト）
    #[inline]
    pub(crate) fn hint_scope_leave(&mut self, id: u32) {
        self.hint_sink.scope_leave(id);
    }

    /// Join 結果ヒント（no-op デフォルト）
    #[inline]
    pub(crate) fn hint_join_result<S: Into<String>>(&mut self, var: S) {
        self.hint_sink.join_result(var.into());
    }

    // ---- Region Stack 管理（デバッグ専用）----

    /// Region スタックに push（NYASH_REGION_TRACE=1 専用）
    #[inline]
    pub(crate) fn push_region(&mut self, region_id: RegionId) {
        self.current_region_stack.push(region_id);
    }

    /// Region スタックから pop（NYASH_REGION_TRACE=1 専用）
    #[inline]
    pub(crate) fn pop_region(&mut self) -> Option<RegionId> {
        self.current_region_stack.pop()
    }

    /// 現在の Region スタックを取得（読み取り専用）
    #[inline]
    pub(crate) fn current_region_stack(&self) -> &[RegionId] {
        &self.current_region_stack
    }

    // ---- ValueId 起源 span 管理（診断専用）----

    /// ValueId 起源 span を記録（診断用）
    #[inline]
    pub(crate) fn record_value_span(&mut self, value_id: ValueId, span: Span) {
        self.value_origin_spans.insert(value_id, span);
    }

    /// ValueId 起源 span を取得（診断用）
    #[inline]
    pub(crate) fn value_span(&self, value_id: ValueId) -> Option<Span> {
        self.value_origin_spans.get(&value_id).copied()
    }

    /// ValueId 起源 caller を記録（診断用）
    #[inline]
    pub(crate) fn record_value_caller(&mut self, value_id: ValueId, caller: &'static Location) {
        let loc = format!("{}:{}:{}", caller.file(), caller.line(), caller.column());
        self.value_origin_callers.insert(value_id, loc);
    }

    /// ValueId 起源 caller を取得（診断用）
    #[inline]
    pub(crate) fn value_caller(&self, value_id: ValueId) -> Option<&str> {
        self.value_origin_callers.get(&value_id).map(|s| s.as_str())
    }

    /// ValueId 起源 caller 全体（読み取り専用）
    #[inline]
    pub(crate) fn value_origin_callers(&self) -> &HashMap<ValueId, String> {
        &self.value_origin_callers
    }
}

impl Default for MetadataContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_context_creation() {
        let ctx = MetadataContext::new();
        assert!(ctx.source_file.is_none());
        assert_eq!(ctx.current_region_stack.len(), 0);
    }

    #[test]
    fn test_span_management() {
        let mut ctx = MetadataContext::new();
        let span = Span::new(0, 10, 1, 1);
        ctx.set_current_span(span);
        assert_eq!(ctx.current_span().start, 0);
        assert_eq!(ctx.current_span().end, 10);
    }

    #[test]
    fn test_source_file_management() {
        let mut ctx = MetadataContext::new();
        ctx.set_source_file("test.hako");
        assert_eq!(ctx.current_source_file(), Some("test.hako".to_string()));
        ctx.clear_source_file();
        assert!(ctx.current_source_file().is_none());
    }

    #[test]
    fn test_region_stack() {
        let mut ctx = MetadataContext::new();
        let region1 = RegionId(1);
        let region2 = RegionId(2);

        ctx.push_region(region1);
        ctx.push_region(region2);
        assert_eq!(ctx.current_region_stack().len(), 2);

        assert_eq!(ctx.pop_region(), Some(region2));
        assert_eq!(ctx.pop_region(), Some(region1));
        assert_eq!(ctx.pop_region(), None);
    }

    #[test]
    fn test_hint_operations_no_panic() {
        let mut ctx = MetadataContext::new();
        // These should not panic (no-op by default)
        ctx.hint_scope_enter(1);
        ctx.hint_scope_leave(1);
        ctx.hint_join_result("test_var");
    }
}
