#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-process-timing-presentation-pack"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-73-MIMALLOC-COMPARISON-PROCESS-TIMING-PRESENTATION-PACK.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-72-MIMALLOC-COMPARISON-POST-LONG-TIMING-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_process_timing_presentation_pack_guard.sh"
PRESENTER="tools/allocator/mimalloc_process_timing_presentation.py"

echo "[$TAG] checking phase-295x process timing presentation pack"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$PRESENTER"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$PRESENTER"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-PROCESS-TIMING-PRESENTATION-PACK-295X-001' "$CARD" "card must identify current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-PROCESS-TIMING-PRESENTATION-CLOSEOUT-295X-001' "$CARD" "card must select closeout follow-on"
guard_expect_in_file "$TAG" 'allocator_body_timing=0' "$CARD" "card must close allocator-body timing"
guard_expect_in_file "$TAG" 'process_runtime_cost_included=1' "$PRESENTER" "presenter must mark runtime cost inclusion"
guard_expect_in_file "$TAG" 'evidence_output_cost_included=1' "$PRESENTER" "presenter must mark evidence output cost inclusion"
guard_expect_in_file "$TAG" 'winner_claim=0' "$PRESENTER" "presenter must keep winner claims closed"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-PROCESS-TIMING-PRESENTATION-PACK-295X-001' "$PREV_CARD" "previous card must select this pack"
guard_expect_in_file "$TAG" '295x-74' "$TASKBOARD" "taskboard must expose closeout row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_process_timing_presentation.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/repeated.out"
presentation="$tmp_dir/presentation.out"

cat >"$report" <<'EOF'
output_contract=mimalloc-comparison-repeated-measurement-v0
measurement_profile=phase295x-repeated-v0
warmup_count=1
sample_count=3
operation_repeat=128
timing_repeat_kind=process-invocation-v0
workload_count=2
workloads=representative-small-block-v0,representative-huge-ish-v0
canonical_rss_collector=external-time
summary=ok
provider_activation=0
host_replacement=0
hook_installed=0
global_allocator_installed=0
winner_claim=0
workload_0_id=representative-small-block-v0
workload_0_operation_family=small-block
workload_0_operation_repeat=128
workload_0_timing_repeat_kind=process-invocation-v0
workload_0_sample_count=3
workload_0_hako_external_rss_median_bytes=83886080
workload_0_c_external_rss_median_bytes=73400320
workload_0_hako_external_elapsed_median_ms=80
workload_0_c_external_elapsed_median_ms=60
workload_0_winner_claim=0
workload_1_id=representative-huge-ish-v0
workload_1_operation_family=huge-ish
workload_1_operation_repeat=128
workload_1_timing_repeat_kind=process-invocation-v0
workload_1_sample_count=3
workload_1_hako_external_rss_median_bytes=85983232
workload_1_c_external_rss_median_bytes=75497472
workload_1_hako_external_elapsed_median_ms=70
workload_1_c_external_elapsed_median_ms=80
workload_1_winner_claim=0
EOF

python3 "$PRESENTER" --report "$report" --out "$presentation"

rg -F -q 'mimalloc_process_timing_presentation=1' "$presentation"
rg -F -q 'output_contract=mimalloc-comparison-process-timing-presentation-v0' "$presentation"
rg -F -q 'timing_claim_kind=process-repeat-presentation-only' "$presentation"
rg -F -q 'allocator_body_timing=0' "$presentation"
rg -F -q 'process_runtime_cost_included=1' "$presentation"
rg -F -q 'evidence_output_cost_included=1' "$presentation"
rg -F -q 'workload_0_process_elapsed_median_delta_ms=20' "$presentation"
rg -F -q 'workload_1_process_elapsed_median_delta_ms=-10' "$presentation"
rg -F -q 'winner_claim=0' "$presentation"
rg -F -q 'summary=ok' "$presentation"

python3 - "$presentation" <<'PY'
import sys

values = {}
with open(sys.argv[1], encoding="utf-8") as fh:
    for line in fh:
        line = line.strip()
        if "=" in line:
            key, value = line.split("=", 1)
            values[key] = value

for idx in range(2):
    if values.get(f"workload_{idx}_allocator_body_timing") != "0":
        raise SystemExit(f"workload {idx} must not claim allocator-body timing")
    hako = int(values[f"workload_{idx}_hako_process_elapsed_median_ms"])
    c = int(values[f"workload_{idx}_c_process_elapsed_median_ms"])
    delta = int(values[f"workload_{idx}_process_elapsed_median_delta_ms"])
    if hako - c != delta:
        raise SystemExit(f"workload {idx} elapsed delta mismatch")
print("[phase295x-process-timing-presentation-pack] ok")
PY

cat "$presentation"
echo "[$TAG] ok"
