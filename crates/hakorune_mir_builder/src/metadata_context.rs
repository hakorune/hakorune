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

#[derive(Debug, Clone, Default)]
struct HintSink;

impl HintSink {
    fn new() -> Self {
        Self
    }

    fn scope_enter(&mut self, _id: u32) {}

    fn scope_leave(&mut self, _id: u32) {}

    fn join_result<S: Into<String>>(&mut self, _var: S) {}
}

/// Phase 136 Step 6/7: Metadata/Span/Hint 管理を統一した構造体だよ
#[derive(Debug, Clone)]
pub struct MetadataContext<SpanT: Copy, RegionIdT: Copy> {
    /// 現在の AST span（命令アノテーション用）
    pub(super) current_span: SpanT,

    /// ソースファイルヒント（関数メタデータに伝播）
    pub(super) source_file: Option<String>,

    /// 型推論ヒント（ゼロコストガイダンス）
    hint_sink: HintSink,

    /// Region 観測用のスタック（NYASH_REGION_TRACE=1 のデバッグ専用）
    /// - FunctionRegion がルート
    /// - 開発時のみ使用（本番コストゼロ）
    pub(super) current_region_stack: Vec<RegionIdT>,
}

impl<SpanT: Copy, RegionIdT: Copy> MetadataContext<SpanT, RegionIdT> {
    /// 新規 MetadataContext を生成（デフォルト状態）
    pub fn new(current_span: SpanT) -> Self {
        Self {
            current_span,
            source_file: None,
            hint_sink: HintSink::new(),
            current_region_stack: Vec::new(),
        }
    }

    // ---- Span 管理 ----

    /// 現在の span を取得
    #[inline]
    pub fn current_span(&self) -> SpanT {
        self.current_span
    }

    /// 現在の span を設定
    #[inline]
    pub fn set_current_span(&mut self, span: SpanT) {
        self.current_span = span;
    }

    // ---- Source File 管理 ----

    /// ソースファイルヒントを設定
    #[inline]
    pub fn set_source_file<S: Into<String>>(&mut self, source: S) {
        self.source_file = Some(source.into());
    }

    /// ソースファイルヒントをクリア
    #[inline]
    pub fn clear_source_file(&mut self) {
        self.source_file = None;
    }

    /// 現在のソースファイルヒントを取得
    #[inline]
    pub fn current_source_file(&self) -> Option<String> {
        self.source_file.clone()
    }

    // ---- Hint Sink 管理（型推論ガイダンス）----

    /// スコープ開始ヒント（no-op デフォルト）
    #[inline]
    pub fn hint_scope_enter(&mut self, id: u32) {
        self.hint_sink.scope_enter(id);
    }

    /// スコープ終了ヒント（no-op デフォルト）
    #[inline]
    pub fn hint_scope_leave(&mut self, id: u32) {
        self.hint_sink.scope_leave(id);
    }

    /// Join 結果ヒント（no-op デフォルト）
    #[inline]
    pub fn hint_join_result<S: Into<String>>(&mut self, var: S) {
        self.hint_sink.join_result(var.into());
    }

    // ---- Region Stack 管理（デバッグ専用）----

    /// Region スタックに push（NYASH_REGION_TRACE=1 専用）
    #[inline]
    pub fn push_region(&mut self, region_id: RegionIdT) {
        self.current_region_stack.push(region_id);
    }

    /// Region スタックから pop（NYASH_REGION_TRACE=1 専用）
    #[inline]
    pub fn pop_region(&mut self) -> Option<RegionIdT> {
        self.current_region_stack.pop()
    }

    /// 現在の Region スタックを取得（読み取り専用）
    #[inline]
    pub fn current_region_stack(&self) -> &[RegionIdT] {
        &self.current_region_stack
    }
}

impl<SpanT: Copy + Default, RegionIdT: Copy> Default for MetadataContext<SpanT, RegionIdT> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct TestSpan {
        start: usize,
        end: usize,
    }

    type TestMetadataContext = MetadataContext<TestSpan, u32>;

    fn test_context() -> TestMetadataContext {
        MetadataContext::new(TestSpan { start: 0, end: 0 })
    }

    #[test]
    fn test_metadata_context_creation() {
        let ctx = test_context();
        assert!(ctx.source_file.is_none());
        assert_eq!(ctx.current_region_stack.len(), 0);
    }

    #[test]
    fn test_span_management() {
        let mut ctx = test_context();
        let span = TestSpan { start: 0, end: 10 };
        ctx.set_current_span(span);
        assert_eq!(ctx.current_span().start, 0);
        assert_eq!(ctx.current_span().end, 10);
    }

    #[test]
    fn test_source_file_management() {
        let mut ctx = test_context();
        ctx.set_source_file("test.hako");
        assert_eq!(ctx.current_source_file(), Some("test.hako".to_string()));
        ctx.clear_source_file();
        assert!(ctx.current_source_file().is_none());
    }

    #[test]
    fn test_region_stack() {
        let mut ctx = test_context();
        let region1 = 1u32;
        let region2 = 2u32;

        ctx.push_region(region1);
        ctx.push_region(region2);
        assert_eq!(ctx.current_region_stack().len(), 2);

        assert_eq!(ctx.pop_region(), Some(region2));
        assert_eq!(ctx.pop_region(), Some(region1));
        assert_eq!(ctx.pop_region(), None);
    }

    #[test]
    fn test_hint_operations_no_panic() {
        let mut ctx = test_context();
        // These should not panic (no-op by default)
        ctx.hint_scope_enter(1);
        ctx.hint_scope_leave(1);
        ctx.hint_join_result("test_var");
    }
}
