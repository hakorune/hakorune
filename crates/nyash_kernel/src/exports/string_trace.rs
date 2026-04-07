#[cfg(not(test))]
static ROUTE_TRACE_ENABLED_CACHE: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(2);

#[inline(always)]
fn route_trace_enabled() -> bool {
    #[cfg(test)]
    {
        match std::env::var("NYASH_LLVM_ROUTE_TRACE").ok().as_deref() {
            Some("1") | Some("on") | Some("true") | Some("yes") => true,
            _ => false,
        }
    }
    #[cfg(not(test))]
    {
        match ROUTE_TRACE_ENABLED_CACHE.load(std::sync::atomic::Ordering::Relaxed) {
            0 => false,
            1 => true,
            _ => {
                let enabled = nyash_rust::config::env::vm_route_trace();
                ROUTE_TRACE_ENABLED_CACHE
                    .store(enabled as u8, std::sync::atomic::Ordering::Relaxed);
                enabled
            }
        }
    }
}

#[inline(always)]
pub(crate) fn enabled() -> bool {
    route_trace_enabled()
}

pub(crate) fn emit(stage: &str, result: &str, reason: &str, extra: impl std::fmt::Display) {
    if !route_trace_enabled() {
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
