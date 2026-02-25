use super::MirBuilder;

impl MirBuilder {
    // ----------------------
    // Debug scope helpers (region_id for DebugHub events)
    // ----------------------
    #[inline]
    pub(crate) fn debug_next_join_id(&mut self) -> u32 {
        // Phase 136 Step 2/7 + Phase 2-2: Use core_ctx as SSOT (no sync needed)
        self.core_ctx.next_debug_join()
    }

    #[inline]
    pub(crate) fn debug_push_region<S: Into<String>>(&mut self, region: S) {
        // Phase 2-4: Use scope_ctx only (legacy field removed)
        let region = region.into();
        self.scope_ctx.debug_push_region(region);
    }

    #[inline]
    pub(crate) fn debug_pop_region(&mut self) {
        // Phase 2-4: Use scope_ctx only (legacy field removed)
        self.scope_ctx.debug_pop_region();
    }

    #[inline]
    #[allow(deprecated)]
    pub(crate) fn debug_current_region_id(&self) -> Option<String> {
        // Phase 136 Step 3/7: Read from scope_ctx (SSOT)
        self.scope_ctx.debug_current_region_id()
    }

    // ----------------------
    // Compile trace helpers (dev only; env-gated)
    // ----------------------
    #[inline]
    pub(super) fn compile_trace_enabled() -> bool {
        crate::config::env::builder_mir_compile_trace()
    }

    #[inline]
    pub(super) fn trace_compile<S: AsRef<str>>(&self, msg: S) {
        if Self::compile_trace_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!("[mir-compile] {}", msg.as_ref()));
        }
    }
}
