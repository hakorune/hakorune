#[inline(always)]
pub(crate) fn enabled() -> bool {
    nyash_rust::config::env::vm_route_trace()
}

pub(crate) fn emit(stage: &str, result: &str, reason: &str, extra: impl std::fmt::Display) {
    if !enabled() {
        return;
    }
    eprintln!(
        "[string/trace] stage={} result={} reason={} extra={}",
        if stage.is_empty() { "unknown" } else { stage },
        if result.is_empty() { "unknown" } else { result },
        if reason.is_empty() { "unknown" } else { reason },
        extra
    );
}
