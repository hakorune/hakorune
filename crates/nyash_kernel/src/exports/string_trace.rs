#[cfg(not(test))]
use std::sync::OnceLock;

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
        static TRACE_ENABLED: OnceLock<bool> = OnceLock::new();
        *TRACE_ENABLED.get_or_init(|| nyash_rust::config::env::vm_route_trace())
    }
}

pub(crate) fn enabled() -> bool {
    route_trace_enabled()
}

pub(crate) fn emit(stage: &str, result: &str, reason: &str, extra: &str) {
    if !route_trace_enabled() {
        return;
    }
    eprintln!(
        "[string/trace] stage={} result={} reason={} extra={}",
        if stage.is_empty() { "unknown" } else { stage },
        if result.is_empty() { "unknown" } else { result },
        if reason.is_empty() { "unknown" } else { reason },
        if extra.is_empty() { "" } else { extra }
    );
}
