use super::{BasicBlockId, FunctionSignature, MirBuilder, MirFunction};

impl MirBuilder {
    // ---- Hint helpers (no-op by default) ----
    // Phase 136 Step 6/7: metadata_ctx is the hint metadata SSOT.
    #[inline]
    pub(crate) fn hint_scope_enter(&mut self, id: u32) {
        self.metadata_ctx.hint_scope_enter(id);
    }
    #[inline]
    pub(crate) fn hint_scope_leave(&mut self, id: u32) {
        self.metadata_ctx.hint_scope_leave(id);
    }
    #[inline]
    pub(crate) fn hint_join_result<S: Into<String>>(&mut self, var: S) {
        self.metadata_ctx.hint_join_result(var);
    }

    /// Hint for downstream metadata: set the logical source file name/path for the next build.
    /// Phase 136 Step 6/7: Delegate to metadata_ctx
    pub fn set_source_file_hint<S: Into<String>>(&mut self, source: S) {
        self.metadata_ctx.set_source_file(source);
    }

    /// Clear the source file hint (used when reusing the builder across modules).
    /// Phase 136 Step 6/7: Delegate to metadata_ctx
    pub fn clear_source_file_hint(&mut self) {
        self.metadata_ctx.clear_source_file();
    }

    /// Resolve current source file hint (builder field or env fallback).
    /// Phase 136 Step 6/7: Delegate to metadata_ctx
    pub(super) fn current_source_file(&self) -> Option<String> {
        self.metadata_ctx
            .current_source_file()
            .or_else(|| crate::config::env::builder_source_file_hint())
    }

    /// NCL-1: Externalize closure body into module metadata and return body_id.
    pub(super) fn intern_closure_body(
        &mut self,
        body: Vec<crate::ast::ASTNode>,
    ) -> Option<crate::mir::function::ClosureBodyId> {
        if body.is_empty() {
            return None;
        }
        self.current_module
            .as_mut()
            .map(|module| module.intern_closure_body(body))
    }

    /// Create a new MirFunction with source metadata applied.
    pub(super) fn new_function_with_metadata(
        &self,
        signature: FunctionSignature,
        entry_block: BasicBlockId,
    ) -> MirFunction {
        let mut f = MirFunction::new(signature, entry_block);
        f.metadata.source_file = self.current_source_file();
        f
    }

    pub(super) fn set_current_function_runes(&mut self, attrs: &crate::ast::DeclarationAttrs) {
        if let Some(function) = self.scope_ctx.current_function.as_mut() {
            function.metadata.runes = attrs.runes.clone();
        }
    }
}
