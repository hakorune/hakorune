use super::super::MirBuilder;
use crate::runtime::get_global_ring0;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::OnceLock;

// Dev-only KPI: resolve.choose Known rate
static TOTAL_CHOOSE: AtomicUsize = AtomicUsize::new(0);
static KNOWN_CHOOSE: AtomicUsize = AtomicUsize::new(0);
static KPI_ENABLED: OnceLock<bool> = OnceLock::new();
static SAMPLE_EVERY: OnceLock<usize> = OnceLock::new();

fn kpi_enabled() -> bool {
    *KPI_ENABLED.get_or_init(|| crate::config::env::builder_debug_kpi_known())
}

fn sample_every() -> usize {
    *SAMPLE_EVERY.get_or_init(|| crate::config::env::builder_debug_sample_every().unwrap_or(0))
}

/// Dev‑only: emit a resolve.try event（candidates inspection）。
pub(crate) fn emit_try(builder: &MirBuilder, meta: serde_json::Value) {
    let fn_name = builder
        .scope_ctx
        .current_function
        .as_ref()
        .map(|f| f.signature.name.as_str());
    let region = builder.debug_current_region_id();
    crate::debug::hub::emit("resolve", "try", fn_name, region.as_deref(), meta);
}

/// Dev‑only: emit a resolve.choose event（decision）。
pub(crate) fn emit_choose(builder: &MirBuilder, meta: serde_json::Value) {
    let fn_name = builder
        .scope_ctx
        .current_function
        .as_ref()
        .map(|f| f.signature.name.as_str());
    let region = builder.debug_current_region_id();
    // KPI (dev-only)
    record_kpi(&meta);
    crate::debug::hub::emit("resolve", "choose", fn_name, region.as_deref(), meta);
}

/// Internal: Call from emit_choose wrapper to record KPI if enabled.
fn record_kpi(meta: &serde_json::Value) {
    if !kpi_enabled() {
        return;
    }
    let total = TOTAL_CHOOSE.fetch_add(1, Ordering::Relaxed) + 1;
    let certainty = meta.get("certainty").and_then(|v| v.as_str()).unwrap_or("");
    if certainty == "Known" {
        KNOWN_CHOOSE.fetch_add(1, Ordering::Relaxed);
    }
    let n = sample_every();
    if n > 0 && total % n == 0 {
        let known = KNOWN_CHOOSE.load(Ordering::Relaxed);
        let rate = if total > 0 {
            (known as f64) * 100.0 / (total as f64)
        } else {
            0.0
        };
        get_global_ring0().log.info(&format!(
            "[NYASH-KPI] resolve.choose Known={} Total={} ({:.1}%)",
            known, total, rate
        ));
    }
}
