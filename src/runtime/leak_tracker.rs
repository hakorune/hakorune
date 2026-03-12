//! Leak Tracker - Exit-time diagnostics for strong references still held
//!
//! Phase 285: Extended to report all global roots (modules, host_handles, plugin boxes).
//! Phase 29y.1: Added root category summary (SSOT: 30-OBSERVABILITY-SSOT.md)
//!
//! ## Environment Variable
//!
//! - `NYASH_LEAK_LOG=1` - Summary counts only (with category breakdown)
//! - `NYASH_LEAK_LOG=2` - Verbose (include names/entries, truncated to first 10)
//!
//! ## Output Format
//!
//! ```text
//! [lifecycle/leak] Roots still held at exit:
//! [lifecycle/leak]   modules: 3
//! [lifecycle/leak]   host_handles: 5
//! [lifecycle/leak]   plugin_boxes: 2
//! [lifecycle/leak] Root categories:
//! [lifecycle/leak]   handles: 8
//! [lifecycle/leak]   locals: 0
//! [lifecycle/leak]   temps: 4
//! [lifecycle/leak]   heap_fields: 2
//! [lifecycle/leak]   singletons: 305
//! ```

use crate::runtime::get_global_ring0;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

/// Leak log level: 0 = off, 1 = summary, 2 = verbose
static LEVEL: Lazy<u8> = Lazy::new(crate::config::env::leak_log_level);

/// Backward compatibility: enabled if level >= 1
static ENABLED: Lazy<bool> = Lazy::new(|| *LEVEL >= 1);

static LEAKS: Lazy<Mutex<HashMap<(String, u32), &'static str>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
const NO_OBSERVED_VALUE: usize = usize::MAX;
static OBSERVED_TEMPS_MAX: AtomicUsize = AtomicUsize::new(NO_OBSERVED_VALUE);
static OBSERVED_HEAP_FIELDS_MAX: AtomicUsize = AtomicUsize::new(NO_OBSERVED_VALUE);

fn observed_max(slot: &AtomicUsize) -> Option<usize> {
    let raw = slot.load(Ordering::Relaxed);
    if raw == NO_OBSERVED_VALUE {
        None
    } else {
        Some(raw)
    }
}

fn update_observed_max(slot: &AtomicUsize, count: usize) {
    let mut current = slot.load(Ordering::Relaxed);
    loop {
        if current == NO_OBSERVED_VALUE {
            match slot.compare_exchange_weak(current, count, Ordering::Relaxed, Ordering::Relaxed) {
                Ok(_) => return,
                Err(next) => {
                    current = next;
                    continue;
                }
            }
        }
        if count <= current {
            return;
        }
        match slot.compare_exchange_weak(current, count, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return,
            Err(next) => current = next,
        }
    }
}

/// Phase 29y.1: Root category summary for observability
///
/// Categories are defined in docs/development/current/main/phases/phase-29y/30-OBSERVABILITY-SSOT.md
#[derive(Debug, Default, Clone)]
pub struct RootSummary {
    /// host-visible registry/handle table
    pub handles: usize,
    /// runtime singleton/globals
    pub singletons: usize,
    /// object strong-owned fields (observed by VM lane when available)
    pub heap_fields: usize,
    /// temporary values/VM registers (observed by VM lane when available)
    pub temps: usize,
    /// local variable bindings (Phase 1: always 0 - VM teardown)
    pub locals: usize,
}

#[derive(Debug, Default, Clone)]
struct RootSummaryObservation {
    summary: RootSummary,
    temps_observed: bool,
    heap_fields_observed: bool,
    singletons_observed: bool,
}

/// Reset per-run root-surface observations.
pub fn reset_observed_roots() {
    OBSERVED_TEMPS_MAX.store(NO_OBSERVED_VALUE, Ordering::Relaxed);
    OBSERVED_HEAP_FIELDS_MAX.store(NO_OBSERVED_VALUE, Ordering::Relaxed);
}

/// Backward compatibility alias (X14 call sites).
pub fn reset_observed_temps() {
    reset_observed_roots();
}

/// Observe strong temp roots (VM regs etc.) and keep the per-run maximum.
pub fn observe_temps(count: usize) {
    update_observed_max(&OBSERVED_TEMPS_MAX, count);
}

/// Observe strong heap field roots and keep the per-run maximum.
pub fn observe_heap_fields(count: usize) {
    update_observed_max(&OBSERVED_HEAP_FIELDS_MAX, count);
}

/// Phase 29y.1/29x-X16: Collect root summary from available sources
///
/// Phase 1 limitation: Exit-time source is Rust-side global registries.
/// locals is currently emitted as a pinned baseline (0) to keep category contract stable.
/// singletons are sourced from runtime module globals.
fn collect_root_summary_observation() -> RootSummaryObservation {
    let host_handles = crate::runtime::host_handles::snapshot();
    let modules = crate::runtime::modules_registry::snapshot_names_and_strings();
    let temps_observed = observed_max(&OBSERVED_TEMPS_MAX);
    let heap_fields_observed = observed_max(&OBSERVED_HEAP_FIELDS_MAX);

    RootSummaryObservation {
        summary: RootSummary {
            handles: host_handles.len() + modules.len(),
            singletons: modules.len(),
            heap_fields: heap_fields_observed.unwrap_or(0),
            temps: temps_observed.unwrap_or(0),
            locals: 0, // Phase 1 limitation: VM state not available
        },
        temps_observed: temps_observed.is_some(),
        heap_fields_observed: heap_fields_observed.is_some(),
        singletons_observed: true,
    }
}

/// Phase 29x X17: Stable debug summary API for root categories.
///
/// Contract:
/// - Always returns the fixed 5-category vocabulary
///   (`handles/locals/temps/heap_fields/singletons`).
/// - Does not mutate runtime semantics; diagnostics only.
pub fn debug_root_summary() -> RootSummary {
    collect_root_summary_observation().summary
}

pub fn init() {
    let _ = &*REPORTER;
}

pub fn register_plugin(box_type: &str, instance_id: u32) {
    if !*ENABLED {
        return;
    }
    LEAKS
        .lock()
        .unwrap()
        .insert((box_type.to_string(), instance_id), "plugin");
}

pub fn finalize_plugin(box_type: &str, instance_id: u32) {
    if !*ENABLED {
        return;
    }
    LEAKS
        .lock()
        .unwrap()
        .remove(&(box_type.to_string(), instance_id));
}

struct Reporter;
impl Drop for Reporter {
    fn drop(&mut self) {
        if !*ENABLED {
            return;
        }
        emit_leak_report();
    }
}

static REPORTER: Lazy<Reporter> = Lazy::new(|| Reporter);

/// Emit exit-time leak report (Phase 285)
///
/// Called automatically on program exit via Reporter::drop.
/// Can also be called manually for testing.
pub fn emit_leak_report() {
    let level = *LEVEL;
    if level == 0 {
        return;
    }

    let ring0 = get_global_ring0();

    // Collect root counts
    let modules = crate::runtime::modules_registry::snapshot_names_and_strings();
    let host_handles = crate::runtime::host_handles::snapshot();
    let plugin_boxes = LEAKS.lock().map(|m| m.len()).unwrap_or(0);

    let modules_count = modules.len();
    let host_handles_count = host_handles.len();

    // Only print if there's something to report
    if modules_count == 0 && host_handles_count == 0 && plugin_boxes == 0 {
        return;
    }

    // Summary header
    ring0.log.warn("[lifecycle/leak] Roots still held at exit:");

    // Summary counts
    if modules_count > 0 {
        ring0
            .log
            .warn(&format!("[lifecycle/leak]   modules: {}", modules_count));
    }
    if host_handles_count > 0 {
        ring0.log.warn(&format!(
            "[lifecycle/leak]   host_handles: {}",
            host_handles_count
        ));
    }
    if plugin_boxes > 0 {
        ring0.log.warn(&format!(
            "[lifecycle/leak]   plugin_boxes: {}",
            plugin_boxes
        ));
    }

    // Phase 29y.1/29x-X17: Root category summary
    let obs = collect_root_summary_observation();
    ring0.log.warn("[lifecycle/leak] Root categories:");
    let debug = debug_root_summary();
    ring0
        .log
        .warn(&format!("[lifecycle/leak]   handles: {}", debug.handles));
    ring0
        .log
        .warn(&format!("[lifecycle/leak]   locals: {}", debug.locals));
    ring0
        .log
        .warn(&format!("[lifecycle/leak]   temps: {}", debug.temps));
    ring0.log.warn(&format!(
        "[lifecycle/leak]   heap_fields: {}",
        debug.heap_fields
    ));
    ring0.log.warn(&format!(
        "[lifecycle/leak]   singletons: {}",
        debug.singletons
    ));

    // min2: optional GC observability (dev/diagnostic only, default OFF).
    if crate::config::env::gc_metrics() {
        if matches!(
            crate::config::env::gc_mode_typed(),
            Ok(crate::runtime::gc_mode::GcMode::RcCycle)
        ) {
            let collect_sp = crate::config::env::gc_collect_sp_interval()
                .map(|v| v.to_string())
                .unwrap_or_else(|| "none".to_string());
            let collect_alloc = crate::config::env::gc_collect_alloc_bytes()
                .map(|v| v.to_string())
                .unwrap_or_else(|| "none".to_string());
            ring0.log.warn(&format!(
                "[gc/optional:mode] mode=rc+cycle collect_sp={} collect_alloc={}",
                collect_sp, collect_alloc
            ));
        }
    }

    let mut limitations: Vec<&str> = Vec::new();
    if !obs.temps_observed {
        limitations.push("temps source unavailable");
    }
    if !obs.heap_fields_observed {
        limitations.push("heap_fields source unavailable");
    }
    if !obs.singletons_observed {
        limitations.push("singletons source unavailable");
    }
    if !limitations.is_empty() {
        ring0.log.warn(&format!(
            "[lifecycle/leak]   (Phase 1 limitation: {})",
            limitations.join("; ")
        ));
    }

    // Verbose details (level 2)
    if level >= 2 {
        const MAX_ENTRIES: usize = 10;

        // Module names
        if !modules.is_empty() {
            ring0.log.warn("[lifecycle/leak]   module names:");
            for (i, (name, _value)) in modules.iter().take(MAX_ENTRIES).enumerate() {
                ring0
                    .log
                    .warn(&format!("[lifecycle/leak]     [{}] {}", i, name));
            }
            if modules.len() > MAX_ENTRIES {
                ring0.log.warn(&format!(
                    "[lifecycle/leak]     ... and {} more",
                    modules.len() - MAX_ENTRIES
                ));
            }
        }

        // Plugin box details
        if plugin_boxes > 0 {
            ring0.log.warn("[lifecycle/leak]   plugin box details:");
            if let Ok(m) = LEAKS.lock() {
                for (i, ((ty, id), _)) in m.iter().take(MAX_ENTRIES).enumerate() {
                    ring0.log.warn(&format!(
                        "[lifecycle/leak]     [{}] {}(id={}) not finalized",
                        i, ty, id
                    ));
                }
                if m.len() > MAX_ENTRIES {
                    ring0.log.warn(&format!(
                        "[lifecycle/leak]     ... and {} more",
                        m.len() - MAX_ENTRIES
                    ));
                }
            }
        }
    }
}
