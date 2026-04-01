#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GC_CONTROLLER_FILE="src/runtime/gc_controller.rs"
GC_TRIGGER_POLICY_FILE="src/runtime/gc_trigger_policy.rs"

echo "[k2-wide-hako-alloc-gc-trigger] running GC trigger threshold policy acceptance pack"
echo "[k2-wide-hako-alloc-gc-trigger] --- parser/decision acceptance ---"
cargo test -q gc_trigger_policy_ -- --nocapture

echo "[k2-wide-hako-alloc-gc-trigger] --- controller seam acceptance ---"
cargo test -q gc_controller_triggers_collection_on_safepoint_threshold -- --nocapture
cargo test -q gc_controller_triggers_collection_on_alloc_threshold_after_safepoint -- --nocapture
cargo test -q gc_controller_triggers_collection_on_both_thresholds -- --nocapture
cargo test -q gc_controller_off_mode_ignores_trigger_thresholds -- --nocapture

echo "[k2-wide-hako-alloc-gc-trigger] --- policy/body route lock ---"
rg -F -q 'trigger_policy: GcTriggerPolicy,' "$GC_CONTROLLER_FILE"
rg -F -q 'trigger_policy: GcTriggerPolicy::from_env(),' "$GC_CONTROLLER_FILE"
rg -F -q '.trigger_policy' "$GC_CONTROLLER_FILE"
rg -F -q '.decide(sp, self.bytes_since_last.load(Ordering::Relaxed));' "$GC_CONTROLLER_FILE"
rg -F -q 'gc_collect_sp_interval()' "$GC_TRIGGER_POLICY_FILE"
rg -F -q 'gc_collect_alloc_bytes()' "$GC_TRIGGER_POLICY_FILE"
rg -F -q 'gc_controller_triggers_collection_on_safepoint_threshold' "$GC_CONTROLLER_FILE"
rg -F -q 'gc_controller_triggers_collection_on_alloc_threshold_after_safepoint' "$GC_CONTROLLER_FILE"
rg -F -q 'gc_controller_triggers_collection_on_both_thresholds' "$GC_CONTROLLER_FILE"
rg -F -q 'gc_controller_off_mode_ignores_trigger_thresholds' "$GC_CONTROLLER_FILE"

echo "[k2-wide-hako-alloc-gc-trigger] ok"
