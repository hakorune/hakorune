//! ScopeContext - observation-only debug scope tracking
//!
//! Phase 136 Step 3/7: Extract scope-related state from MirBuilder
//!
//! Function-owned lexical/control-flow state lives in
//! `function_lowering_state::FunctionScopeStateV1`.
//!
/// Scope and control flow context for MIR building
#[derive(Debug)]
pub(in crate::mir) struct ScopeContext {
    // ---- Debug scope ----
    /// Stack of region identifiers (e.g., "loop#1/header", "join#3/join")
    /// Zero-cost when unused (dev only)
    pub(super) debug_scope_stack: Vec<String>,
}

impl ScopeContext {
    /// Create new scope context (empty state)
    pub(super) fn new() -> Self {
        Self {
            debug_scope_stack: Vec::new(),
        }
    }

    // ---- Debug scope helpers ----

    /// Push debug region identifier
    #[inline]
    pub(super) fn debug_push_region<S: Into<String>>(&mut self, region: S) {
        self.debug_scope_stack.push(region.into());
    }

    /// Pop debug region identifier
    #[inline]
    pub(super) fn debug_pop_region(&mut self) {
        let _ = self.debug_scope_stack.pop();
    }

    /// Get current debug region identifier
    #[inline]
    pub(super) fn debug_current_region_id(&self) -> Option<String> {
        self.debug_scope_stack.last().cloned()
    }

    pub(super) fn clear_debug_scope_for_function_entry(&mut self) {
        self.debug_scope_stack.clear();
    }
}

impl Default for ScopeContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_scope_stack() {
        let mut ctx = ScopeContext::new();

        // Initially empty
        assert!(ctx.debug_current_region_id().is_none());

        // Push region
        ctx.debug_push_region("loop#1/header");
        assert_eq!(
            ctx.debug_current_region_id(),
            Some("loop#1/header".to_string())
        );

        // Push nested region
        ctx.debug_push_region("join#3/join");
        assert_eq!(
            ctx.debug_current_region_id(),
            Some("join#3/join".to_string())
        );

        // Pop
        ctx.debug_pop_region();
        assert_eq!(
            ctx.debug_current_region_id(),
            Some("loop#1/header".to_string())
        );

        ctx.debug_pop_region();
        assert!(ctx.debug_current_region_id().is_none());
    }
}
