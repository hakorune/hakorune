//! Stable, opt-in timing output for MIR compilation stages.

use std::time::Duration;

pub(crate) fn trace_stage(stage: &str, elapsed: Duration) {
    if crate::config::env::builder_mir_compile_trace() {
        eprintln!(
            "[mir-compile/timing] stage={} elapsed_ms={}",
            stage,
            elapsed.as_millis()
        );
    }
}

pub(crate) fn trace_count(stage: &str, count: usize) {
    if crate::config::env::builder_mir_compile_trace() {
        eprintln!("[mir-compile/timing] stage={} count={}", stage, count);
    }
}
