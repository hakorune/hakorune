//! Unified GC controller (skeleton)
//! Implements GcHooks and centralizes mode selection and metrics.

use std::sync::atomic::{AtomicU64, Ordering};

use super::gc::{BarrierKind, GcHooks};
use super::gc_mode::GcMode;
use super::gc_trigger_policy::GcTriggerPolicy;
use crate::runtime::gc_trace;
use crate::runtime::get_global_ring0;
use std::collections::{HashSet, VecDeque};

type DynBox = std::sync::Arc<dyn crate::box_trait::NyashBox>;

#[derive(Debug, Clone, Copy)]
struct ReachabilitySummary {
    nodes: u64,
    edges: u64,
}

pub struct GcController {
    mode: GcMode,
    safepoints: AtomicU64,
    barrier_reads: AtomicU64,
    barrier_writes: AtomicU64,
    alloc_bytes: AtomicU64,
    alloc_count: AtomicU64,
    sp_since_last: AtomicU64,
    bytes_since_last: AtomicU64,
    trigger_policy: GcTriggerPolicy,
    // Diagnostics: last trial reachability counters
    trial_nodes_last: AtomicU64,
    trial_edges_last: AtomicU64,
    // Diagnostics: collection counters and last duration/flags
    collect_count_total: AtomicU64,
    collect_by_sp: AtomicU64,
    collect_by_alloc: AtomicU64,
    trial_duration_last_ms: AtomicU64,
    trial_reason_last: AtomicU64, // bitflags: 1=sp, 2=alloc
}

impl GcController {
    pub fn new(mode: GcMode) -> Self {
        let controller = Self {
            mode,
            safepoints: AtomicU64::new(0),
            barrier_reads: AtomicU64::new(0),
            barrier_writes: AtomicU64::new(0),
            alloc_bytes: AtomicU64::new(0),
            alloc_count: AtomicU64::new(0),
            sp_since_last: AtomicU64::new(0),
            bytes_since_last: AtomicU64::new(0),
            trigger_policy: GcTriggerPolicy::from_env(),
            trial_nodes_last: AtomicU64::new(0),
            trial_edges_last: AtomicU64::new(0),
            collect_count_total: AtomicU64::new(0),
            collect_by_sp: AtomicU64::new(0),
            collect_by_alloc: AtomicU64::new(0),
            trial_duration_last_ms: AtomicU64::new(0),
            trial_reason_last: AtomicU64::new(0),
        };
        controller
    }
    pub fn mode(&self) -> GcMode {
        self.mode
    }
    pub fn snapshot(&self) -> (u64, u64, u64) {
        (
            self.safepoints.load(Ordering::Relaxed),
            self.barrier_reads.load(Ordering::Relaxed),
            self.barrier_writes.load(Ordering::Relaxed),
        )
    }
}

impl GcHooks for GcController {
    fn is_active(&self) -> bool {
        self.mode != GcMode::Off
    }

    fn safepoint(&self) {
        // Off mode: minimal overhead but still callable
        if self.mode != GcMode::Off {
            self.safepoints.fetch_add(1, Ordering::Relaxed);
            let sp = self.sp_since_last.fetch_add(1, Ordering::Relaxed) + 1;
            let sp_hit = self
                .trigger_policy
                .decide(sp, self.bytes_since_last.load(Ordering::Relaxed));
            if let Some(decision) = sp_hit {
                if decision.triggered_by_safepoint() {
                    self.collect_by_sp.fetch_add(1, Ordering::Relaxed);
                }
                if decision.triggered_by_alloc() {
                    self.collect_by_alloc.fetch_add(1, Ordering::Relaxed);
                }
                self.trial_reason_last
                    .store(decision.reason_bits(), Ordering::Relaxed);
                self.run_trial_collection();
            }
        }
        // Future: per-mode collection/cooperation hooks
    }
    fn barrier(&self, kind: BarrierKind) {
        if self.mode == GcMode::Off {
            return;
        }
        match kind {
            BarrierKind::Read => {
                self.barrier_reads.fetch_add(1, Ordering::Relaxed);
            }
            BarrierKind::Write => {
                self.barrier_writes.fetch_add(1, Ordering::Relaxed);
            }
        }
    }
    fn snapshot_counters(&self) -> Option<(u64, u64, u64)> {
        Some(self.snapshot())
    }
    fn alloc(&self, bytes: u64) {
        if self.mode == GcMode::Off {
            return;
        }
        self.alloc_count.fetch_add(1, Ordering::Relaxed);
        self.alloc_bytes.fetch_add(bytes, Ordering::Relaxed);
        self.bytes_since_last.fetch_add(bytes, Ordering::Relaxed);
    }
}

impl GcController {
    pub fn alloc_totals(&self) -> (u64, u64) {
        (
            self.alloc_count.load(Ordering::Relaxed),
            self.alloc_bytes.load(Ordering::Relaxed),
        )
    }
}

impl GcController {
    fn reset_collection_windows(&self) {
        self.sp_since_last.store(0, Ordering::Relaxed);
        self.bytes_since_last.store(0, Ordering::Relaxed);
    }

    fn snapshot_roots(&self) -> Vec<DynBox> {
        let mut roots = crate::runtime::host_handles::snapshot();
        let mut mod_roots = crate::runtime::modules_registry::snapshot_boxes();
        roots.append(&mut mod_roots);
        roots
    }

    fn trace_reachability(&self, roots: Vec<DynBox>) -> ReachabilitySummary {
        let mut visited: HashSet<u64> = HashSet::new();
        let mut q: VecDeque<DynBox> = VecDeque::new();
        for root in roots {
            let id = root.box_id();
            if visited.insert(id) {
                q.push_back(root);
            }
        }
        let mut nodes: u64 = visited.len() as u64;
        let mut edges: u64 = 0;
        while let Some(cur) = q.pop_front() {
            gc_trace::trace_children(&*cur, &mut |child| {
                edges += 1;
                let id = child.box_id();
                if visited.insert(id) {
                    nodes += 1;
                    q.push_back(child);
                }
            });
        }
        ReachabilitySummary { nodes, edges }
    }

    fn record_reachability_summary(&self, summary: ReachabilitySummary) {
        self.trial_nodes_last
            .store(summary.nodes, Ordering::Relaxed);
        self.trial_edges_last
            .store(summary.edges, Ordering::Relaxed);
    }

    fn maybe_log_trial_summary(&self, summary: ReachabilitySummary) {
        if (summary.nodes + summary.edges) > 0 && crate::config::env::gc_metrics() {
            get_global_ring0().log.info(&format!(
                "[GC] trial: reachable nodes={} edges={} (roots=jit_handles)",
                summary.nodes, summary.edges
            ));
        }
    }

    fn finalize_collection_metrics(&self, started: std::time::Instant) {
        let ms = started.elapsed().as_millis() as u64;
        self.trial_duration_last_ms.store(ms, Ordering::Relaxed);
        self.collect_count_total.fetch_add(1, Ordering::Relaxed);
    }

    fn run_trial_collection(&self) {
        self.reset_collection_windows();
        // Only run for the active GC mode (rc+cycle). Off mode is handled above.
        match self.mode {
            GcMode::RcCycle => {
                let started = std::time::Instant::now();
                let roots = self.snapshot_roots();
                let summary = self.trace_reachability(roots);
                self.record_reachability_summary(summary);
                self.maybe_log_trial_summary(summary);
                self.finalize_collection_metrics(started);
                // Reason flags derive from current env thresholds vs last windows reaching triggers
                // Note: we set flags in safepoint() where triggers were decided.
            }
            GcMode::Off => {}
        }
    }
}

impl GcController {
    pub fn trial_reachability_last(&self) -> (u64, u64) {
        (
            self.trial_nodes_last.load(Ordering::Relaxed),
            self.trial_edges_last.load(Ordering::Relaxed),
        )
    }
    pub fn collection_totals(&self) -> (u64, u64, u64) {
        (
            self.collect_count_total.load(Ordering::Relaxed),
            self.collect_by_sp.load(Ordering::Relaxed),
            self.collect_by_alloc.load(Ordering::Relaxed),
        )
    }
    pub fn trial_duration_last_ms(&self) -> u64 {
        self.trial_duration_last_ms.load(Ordering::Relaxed)
    }
    pub fn trial_reason_last_bits(&self) -> u64 {
        self.trial_reason_last.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static GC_TRIGGER_ENV_LOCK: Mutex<()> = Mutex::new(());

    fn with_gc_trigger_env<F: FnOnce()>(
        collect_sp: Option<&str>,
        collect_alloc: Option<&str>,
        f: F,
    ) {
        let _guard = GC_TRIGGER_ENV_LOCK.lock().expect("env lock");
        let prev_sp = std::env::var("NYASH_GC_COLLECT_SP").ok();
        let prev_alloc = std::env::var("NYASH_GC_COLLECT_ALLOC").ok();

        match collect_sp {
            Some(value) => std::env::set_var("NYASH_GC_COLLECT_SP", value),
            None => std::env::remove_var("NYASH_GC_COLLECT_SP"),
        }
        match collect_alloc {
            Some(value) => std::env::set_var("NYASH_GC_COLLECT_ALLOC", value),
            None => std::env::remove_var("NYASH_GC_COLLECT_ALLOC"),
        }

        f();

        match prev_sp {
            Some(value) => std::env::set_var("NYASH_GC_COLLECT_SP", value),
            None => std::env::remove_var("NYASH_GC_COLLECT_SP"),
        }
        match prev_alloc {
            Some(value) => std::env::set_var("NYASH_GC_COLLECT_ALLOC", value),
            None => std::env::remove_var("NYASH_GC_COLLECT_ALLOC"),
        }
    }

    #[test]
    fn gc_controller_triggers_collection_on_safepoint_threshold() {
        with_gc_trigger_env(Some("1"), None, || {
            let controller = GcController::new(GcMode::RcCycle);
            controller.safepoint();
            assert_eq!(controller.collection_totals(), (1, 1, 0));
            assert_eq!(controller.trial_reason_last_bits(), 1);
        });
    }

    #[test]
    fn gc_controller_triggers_collection_on_alloc_threshold_after_safepoint() {
        with_gc_trigger_env(None, Some("64"), || {
            let controller = GcController::new(GcMode::RcCycle);
            controller.alloc(64);
            controller.safepoint();
            assert_eq!(controller.collection_totals(), (1, 0, 1));
            assert_eq!(controller.trial_reason_last_bits(), 2);
        });
    }

    #[test]
    fn gc_controller_triggers_collection_on_both_thresholds() {
        with_gc_trigger_env(Some("1"), Some("64"), || {
            let controller = GcController::new(GcMode::RcCycle);
            controller.alloc(64);
            controller.safepoint();
            assert_eq!(controller.collection_totals(), (1, 1, 1));
            assert_eq!(controller.trial_reason_last_bits(), 3);
        });
    }

    #[test]
    fn gc_controller_off_mode_ignores_trigger_thresholds() {
        with_gc_trigger_env(Some("1"), Some("64"), || {
            let controller = GcController::new(GcMode::Off);
            controller.alloc(64);
            controller.safepoint();
            assert_eq!(controller.collection_totals(), (0, 0, 0));
            assert_eq!(controller.trial_reason_last_bits(), 0);
        });
    }
}
