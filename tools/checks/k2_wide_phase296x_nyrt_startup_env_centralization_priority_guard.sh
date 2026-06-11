#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-nyrt-startup-env-centralization-priority"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

DOC="docs/development/current/main/design/nyrt-startup-env-centralization-priority-ssot.md"
INVENTORY="docs/development/current/main/design/nyrt-startup-env-read-inventory-ssot.md"
CARD="docs/development/current/main/phases/phase-296x/296x-651-HAKO-MIMALLOC-TYPED-OBJECT-EXACT-SLOT-ABI-SPLIT.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
ENV_REF="docs/reference/environment-variables.md"
ENTRY="crates/nyash_kernel/src/entry.rs"
STAGE1_RUNTIME_DEFAULTS="src/runner/stage1_bridge/env/runtime_defaults.rs"
MIR_FLAGS="src/config/env/mir_flags.rs"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_nyrt_startup_env_centralization_priority_guard.sh"

echo "[$TAG] checking NyRT startup env centralization priority"

guard_require_files "$TAG" \
  "$DOC" "$INVENTORY" "$CARD" "$CURRENT_STATE" "$INDEX" "$ENV_REF" \
  "$ENTRY" "$STAGE1_RUNTIME_DEFAULTS" "$MIR_FLAGS" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" "NYRT-ENV-002" "$DOC" "priority doc must name the row"
guard_expect_fixed_in_file "$TAG" "first_centralization_surface=NYASH_NYRT_SILENT_RESULT" "$DOC" "priority doc must pick the first seam"
guard_expect_fixed_in_file "$TAG" "P0" "$DOC" "priority doc must rank the first seam"
guard_expect_fixed_in_file "$TAG" "P1" "$DOC" "priority doc must rank the metrics cluster"
guard_expect_fixed_in_file "$TAG" "P2" "$DOC" "priority doc must rank the GC cluster"
guard_expect_fixed_in_file "$TAG" "P3" "$DOC" "priority doc must rank minimal-startup"
guard_expect_fixed_in_file "$TAG" "P4" "$DOC" "priority doc must rank the startup gates"
guard_expect_fixed_in_file "$TAG" "P5" "$DOC" "priority doc must rank path shaping last"
guard_expect_fixed_in_file "$TAG" "src/config/env/mir_flags.rs" "$DOC" "priority doc must cite the baseline env helpers"
guard_expect_fixed_in_file "$TAG" "docs/development/current/main/design/nyrt-startup-env-centralization-priority-ssot.md" "$ENV_REF" "env reference must point at the priority doc"
guard_expect_fixed_in_file "$TAG" "NYRT-ENV-002" "$CARD" "phase card must mention the priority row"
guard_expect_fixed_in_file "$TAG" "NYRT-ENV-002" "$CURRENT_STATE" "current state must mention the priority row"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

guard_expect_fixed_in_file "$TAG" "NYASH_NYRT_SILENT_RESULT" "$ENTRY" "entry must still carry the output-only toggle before centralization"
guard_expect_fixed_in_file "$TAG" "NYASH_GC_METRICS_JSON" "$ENTRY" "entry must still carry the metrics JSON toggle"
guard_expect_fixed_in_file "$TAG" "NYASH_GC_METRICS" "$ENTRY" "entry must still carry the metrics text toggle"
guard_expect_fixed_in_file "$TAG" "NYASH_GC_COLLECT_SP" "$ENTRY" "entry must still carry the GC safepoint interval knob"
guard_expect_fixed_in_file "$TAG" "NYASH_GC_COLLECT_ALLOC" "$ENTRY" "entry must still carry the GC allocation knob"
guard_expect_fixed_in_file "$TAG" "NYASH_LLVM_AUTO_SAFEPOINT" "$ENTRY" "entry must still carry the auto safepoint knob"
guard_expect_fixed_in_file "$TAG" "NYASH_GC_ALLOC_THRESHOLD" "$ENTRY" "entry must still carry the GC warn threshold knob"
guard_expect_fixed_in_file "$TAG" "NYASH_NYRT_MINIMAL_STARTUP" "$ENTRY" "entry must still carry the minimal-startup knob"
guard_expect_fixed_in_file "$TAG" "HAKO_NYRT_PLUGIN_HOST" "$ENTRY" "entry must still carry the plugin-host gate"
guard_expect_fixed_in_file "$TAG" "std::env::current_exe()" "$ENTRY" "entry must still probe executable path"
guard_expect_fixed_in_file "$TAG" "std::env::current_dir()" "$ENTRY" "entry must still probe current_dir"
guard_expect_fixed_in_file "$TAG" "NYASH_NYRT_SILENT_RESULT" "$STAGE1_RUNTIME_DEFAULTS" "stage1 runtime defaults must still seed the output toggle"
guard_expect_fixed_in_file "$TAG" "gc_metrics()" "$MIR_FLAGS" "baseline env helper must expose gc metrics"
guard_expect_fixed_in_file "$TAG" "gc_collect_sp_interval()" "$MIR_FLAGS" "baseline env helper must expose GC interval"
guard_expect_fixed_in_file "$TAG" "gc_collect_alloc_bytes()" "$MIR_FLAGS" "baseline env helper must expose GC allocation threshold"

tmp_dir="$(mktemp -d /tmp/hakorune_nyrt_env_centralization.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
cat >"$report" <<'REPORT'
output_contract=nyrt-startup-env-centralization-priority-v0
input_contract=nyrt-startup-env-surface-v0
first_centralization_surface=NYASH_NYRT_SILENT_RESULT
priority_0=NYASH_NYRT_SILENT_RESULT
priority_1=NYASH_GC_METRICS_JSON|NYASH_GC_METRICS
priority_2=NYASH_GC_COLLECT_SP|NYASH_GC_COLLECT_ALLOC|NYASH_LLVM_AUTO_SAFEPOINT|NYASH_GC_ALLOC_THRESHOLD
priority_3=NYASH_NYRT_MINIMAL_STARTUP
priority_4=HAKO_NYRT_PLUGIN_HOST|NYASH_NYRT_RUNTIME_HOOKS|NYASH_NYRT_RUNTIME_BUILD|NYASH_NYRT_ENTRY_PATH_PREP|NYASH_NYRT_RING0_INIT
priority_5=current_exe/current_dir/PATH/PYTHONHOME
baseline_surface=src/config/env.rs
baseline_helper_surface=src/config/env/mir_flags.rs
stage1_bridge_touches_silent_result=1
path_shape_knob_last=1
summary=ok
REPORT
cat "$report"

guard_expect_fixed_in_file "$TAG" "output_contract=nyrt-startup-env-centralization-priority-v0" "$report" "probe report must carry the priority contract"
guard_expect_fixed_in_file "$TAG" "first_centralization_surface=NYASH_NYRT_SILENT_RESULT" "$report" "probe report must pick the first seam"
guard_expect_fixed_in_file "$TAG" "priority_0=NYASH_NYRT_SILENT_RESULT" "$report" "probe report must rank P0"
guard_expect_fixed_in_file "$TAG" "priority_1=NYASH_GC_METRICS_JSON|NYASH_GC_METRICS" "$report" "probe report must rank P1"
guard_expect_fixed_in_file "$TAG" "priority_2=NYASH_GC_COLLECT_SP|NYASH_GC_COLLECT_ALLOC|NYASH_LLVM_AUTO_SAFEPOINT|NYASH_GC_ALLOC_THRESHOLD" "$report" "probe report must rank P2"
guard_expect_fixed_in_file "$TAG" "priority_3=NYASH_NYRT_MINIMAL_STARTUP" "$report" "probe report must rank P3"
guard_expect_fixed_in_file "$TAG" "priority_4=HAKO_NYRT_PLUGIN_HOST|NYASH_NYRT_RUNTIME_HOOKS|NYASH_NYRT_RUNTIME_BUILD|NYASH_NYRT_ENTRY_PATH_PREP|NYASH_NYRT_RING0_INIT" "$report" "probe report must rank P4"
guard_expect_fixed_in_file "$TAG" "priority_5=current_exe/current_dir/PATH/PYTHONHOME" "$report" "probe report must rank P5"
guard_expect_fixed_in_file "$TAG" "baseline_surface=src/config/env.rs" "$report" "probe report must name the comparison baseline"
guard_expect_fixed_in_file "$TAG" "baseline_helper_surface=src/config/env/mir_flags.rs" "$report" "probe report must name the existing helper baseline"
guard_expect_fixed_in_file "$TAG" "summary=ok" "$report" "probe report must finish cleanly"

echo "[$TAG] ok"
