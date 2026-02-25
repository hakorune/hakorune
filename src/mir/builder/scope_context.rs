//! ScopeContext - Lexical scope and control flow stack management
//!
//! Phase 136 Step 3/7: Extract scope-related state from MirBuilder
//!
//! # Responsibilities
//! - Lexical scope management (variable shadowing, block-scoped locals)
//! - Control flow stack management (loop header/exit, if merge)
//! - Function context management (current function, parameters)
//! - Debug scope tracking (region identifiers)
//!
//! # Design
//! - Encapsulates scope/control flow state for cleaner separation
//! - Provides type-safe push/pop operations
//! - Maintains deterministic iteration order (BTreeMap/BTreeSet)

use crate::mir::{BasicBlockId, MirFunction};
use std::collections::HashSet;

pub(in crate::mir::builder) use super::vars::lexical_scope::LexicalScopeFrame;

/// Scope and control flow context for MIR building
#[derive(Debug)]
pub(in crate::mir) struct ScopeContext {
    // ---- Lexical scope management ----
    /// Stack of lexical scopes for block-scoped `local` declarations
    /// Tracks per-block shadowing so variables restore on scope exit
    pub(super) lexical_scope_stack: Vec<LexicalScopeFrame>,

    // ---- Control flow stacks ----
    /// Stack of loop header blocks (innermost first)
    /// Used for break/continue target resolution
    pub(super) loop_header_stack: Vec<BasicBlockId>,

    /// Stack of loop exit blocks (innermost first)
    #[allow(dead_code)]
    pub(super) loop_exit_stack: Vec<BasicBlockId>,

    /// Stack of if/merge blocks (innermost first)
    /// Used for nested conditional lowering and jump generation
    pub(super) if_merge_stack: Vec<BasicBlockId>,

    // ---- Function context ----
    /// Current function being built
    pub(in crate::mir) current_function: Option<MirFunction>,

    /// Parameter names for current function
    /// Same lifecycle as current_function
    pub(in crate::mir) function_param_names: HashSet<String>,

    // ---- Debug scope ----
    /// Stack of region identifiers (e.g., "loop#1/header", "join#3/join")
    /// Zero-cost when unused (dev only)
    pub(super) debug_scope_stack: Vec<String>,
}

impl ScopeContext {
    /// Create new scope context (empty state)
    pub(super) fn new() -> Self {
        Self {
            lexical_scope_stack: Vec::new(),
            loop_header_stack: Vec::new(),
            loop_exit_stack: Vec::new(),
            if_merge_stack: Vec::new(),
            current_function: None,
            function_param_names: HashSet::new(),
            debug_scope_stack: Vec::new(),
        }
    }

    // ---- Lexical scope helpers ----

    /// Push new lexical scope frame
    #[inline]
    pub(super) fn push_lexical_scope(&mut self) {
        self.lexical_scope_stack.push(LexicalScopeFrame::default());
    }

    /// Pop lexical scope frame (returns frame for restoration)
    #[inline]
    pub(super) fn pop_lexical_scope(&mut self) -> Option<LexicalScopeFrame> {
        self.lexical_scope_stack.pop()
    }

    /// Get mutable reference to current scope frame
    #[inline]
    pub(super) fn current_scope_mut(&mut self) -> Option<&mut LexicalScopeFrame> {
        self.lexical_scope_stack.last_mut()
    }

    // ---- Control flow stack helpers ----

    /// Push loop header block
    #[inline]
    #[allow(dead_code)]
    pub(super) fn push_loop_header(&mut self, bb: BasicBlockId) {
        self.loop_header_stack.push(bb);
    }

    /// Pop loop header block
    #[inline]
    #[allow(dead_code)]
    pub(super) fn pop_loop_header(&mut self) -> Option<BasicBlockId> {
        self.loop_header_stack.pop()
    }

    /// Get innermost loop header
    #[inline]
    #[allow(dead_code)]
    pub(super) fn current_loop_header(&self) -> Option<BasicBlockId> {
        self.loop_header_stack.last().copied()
    }

    /// Push loop exit block
    #[inline]
    #[allow(dead_code)]
    pub(super) fn push_loop_exit(&mut self, bb: BasicBlockId) {
        self.loop_exit_stack.push(bb);
    }

    /// Pop loop exit block
    #[inline]
    #[allow(dead_code)]
    pub(super) fn pop_loop_exit(&mut self) -> Option<BasicBlockId> {
        self.loop_exit_stack.pop()
    }

    /// Push if/merge block
    #[inline]
    pub(super) fn push_if_merge(&mut self, bb: BasicBlockId) {
        self.if_merge_stack.push(bb);
    }

    /// Pop if/merge block
    #[inline]
    pub(super) fn pop_if_merge(&mut self) -> Option<BasicBlockId> {
        self.if_merge_stack.pop()
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

    pub(super) fn clear_for_function_entry(&mut self) {
        self.lexical_scope_stack.clear();
        self.loop_header_stack.clear();
        self.loop_exit_stack.clear();
        self.if_merge_stack.clear();
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
    fn test_lexical_scope_stack() {
        let mut ctx = ScopeContext::new();

        // Initially empty
        assert_eq!(ctx.lexical_scope_stack.len(), 0);

        // Push scope
        ctx.push_lexical_scope();
        assert_eq!(ctx.lexical_scope_stack.len(), 1);

        // Push another
        ctx.push_lexical_scope();
        assert_eq!(ctx.lexical_scope_stack.len(), 2);

        // Pop scope
        let frame = ctx.pop_lexical_scope();
        assert!(frame.is_some());
        assert_eq!(ctx.lexical_scope_stack.len(), 1);

        // Pop last
        let frame = ctx.pop_lexical_scope();
        assert!(frame.is_some());
        assert_eq!(ctx.lexical_scope_stack.len(), 0);

        // Pop from empty
        let frame = ctx.pop_lexical_scope();
        assert!(frame.is_none());
    }

    #[test]
    fn test_loop_stacks() {
        let mut ctx = ScopeContext::new();

        let header1 = BasicBlockId(1);
        let header2 = BasicBlockId(2);

        // Initially empty
        assert!(ctx.current_loop_header().is_none());

        // Push first loop
        ctx.push_loop_header(header1);
        assert_eq!(ctx.current_loop_header(), Some(header1));

        // Push nested loop (innermost)
        ctx.push_loop_header(header2);
        assert_eq!(ctx.current_loop_header(), Some(header2));

        // Pop innermost
        assert_eq!(ctx.pop_loop_header(), Some(header2));
        assert_eq!(ctx.current_loop_header(), Some(header1));

        // Pop outermost
        assert_eq!(ctx.pop_loop_header(), Some(header1));
        assert!(ctx.current_loop_header().is_none());
    }

    #[test]
    fn test_if_merge_stack() {
        let mut ctx = ScopeContext::new();

        let merge1 = BasicBlockId(10);
        let merge2 = BasicBlockId(20);

        // Push merge blocks
        ctx.push_if_merge(merge1);
        ctx.push_if_merge(merge2);

        // Pop in LIFO order
        assert_eq!(ctx.pop_if_merge(), Some(merge2));
        assert_eq!(ctx.pop_if_merge(), Some(merge1));
        assert_eq!(ctx.pop_if_merge(), None);
    }

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

    #[test]
    fn test_function_context() {
        let mut ctx = ScopeContext::new();

        // Initially no function
        assert!(ctx.current_function.is_none());
        assert!(ctx.function_param_names.is_empty());

        // Simulate setting function context
        ctx.function_param_names.insert("x".to_string());
        ctx.function_param_names.insert("y".to_string());

        assert_eq!(ctx.function_param_names.len(), 2);
        assert!(ctx.function_param_names.contains("x"));
        assert!(ctx.function_param_names.contains("y"));
    }
}
