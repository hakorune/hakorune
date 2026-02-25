//! Debug utilities for control flow tracing.
//!
//! This module provides diagnostic tools for tracing and debugging
//! control flow behavior during MIR construction.
//!
//! # Phase 195: Unified Tracing
//!
//! All JoinIR tracing now goes through the JoinLoopTrace module.
//! See `joinir/trace.rs` for the unified tracing interface.
//!
//! # Environment Variables
//!
//! - `NYASH_TRACE_VARMAP=1` - Enable variable map tracing
//! - `NYASH_JOINIR_DEBUG=1` - Enable general JoinIR debug output
//! - `NYASH_OPTION_C_DEBUG=1` - Enable PHI-related debug
//! - `NYASH_JOINIR_MAINLINE_DEBUG=1` - Enable mainline routing debug
//! - `NYASH_LOOPFORM_DEBUG=1` - Enable LoopForm debug

use super::super::MirBuilder;

impl MirBuilder {
    /// Trace variable_map state for debugging
    ///
    /// Phase 195: Delegates to JoinLoopTrace for unified tracing.
    /// Enable with NYASH_TRACE_VARMAP=1
    #[allow(dead_code)]
    pub(in crate::mir::builder) fn trace_varmap(&self, context: &str) {
        super::joinir::trace::trace().varmap(context, &self.variable_ctx.variable_map);
    }
}
