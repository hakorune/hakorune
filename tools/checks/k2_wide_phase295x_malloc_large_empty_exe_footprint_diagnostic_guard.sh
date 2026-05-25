#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-malloc-large-empty-exe-footprint-diagnostic"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-195-MIMALLOC-COMPARISON-MALLOC-LARGE-EMPTY-EXE-FOOTPRINT-DIAGNOSTIC.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-194-MIMALLOC-COMPARISON-MALLOC-LARGE-BASELINE-BREAKDOWN-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_malloc_large_empty_exe_footprint_diagnostic_guard.sh"
SCRIPT="tools/allocator/mimalloc_hako_empty_exe_footprint.py"
NOIO_APP="apps/hako-alloc-mimalloc-comparison-empty-noio-exe-proof/main.hako"
EVIDENCE_APP="apps/hako-alloc-mimalloc-comparison-empty-exe-proof/main.hako"

echo "[$TAG] checking phase-295x malloc-large empty EXE footprint diagnostic"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$SCRIPT" "$NOIO_APP" "$EVIDENCE_APP"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$SCRIPT"
guard_require_command "$TAG" readelf
guard_require_command "$TAG" cc

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MALLOC-LARGE-EMPTY-EXE-FOOTPRINT-DIAGNOSTIC-295X-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MALLOC-LARGE-EMPTY-EXE-FOOTPRINT-CLOSEOUT-295X-001' "$CARD" "card must select closeout follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MALLOC-LARGE-EMPTY-EXE-FOOTPRINT-DIAGNOSTIC-295X-001' "$PREV_CARD" "previous row must select this diagnostic row"
guard_expect_in_file "$TAG" 'mimalloc-comparison-hako-empty-exe-footprint-diagnostic-v0' "$SCRIPT" "script must define stable output contract"
guard_expect_in_file "$TAG" 'baseline_shrink_action=0' "$SCRIPT" "script must keep baseline shrink closed"
guard_expect_in_file "$TAG" 'winner_claim=0' "$SCRIPT" "script must keep winner claims closed"
guard_expect_in_file "$TAG" 'return 0' "$NOIO_APP" "no-output app must be a silent success control"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_malloc_large_empty_footprint.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
out="$tmp_dir/footprint.out"

python3 "$SCRIPT" --out "$out" --sample-count 5 --warmup-count 1 --allow-ldconfig-discovery

rg -F -q 'mimalloc_hako_empty_exe_footprint=1' "$out"
rg -F -q 'output_contract=mimalloc-comparison-hako-empty-exe-footprint-diagnostic-v0' "$out"
rg -F -q 'baseline_workload=representative-empty-v0' "$out"
rg -F -q 'diagnostic_workload=representative-empty-noio-v0' "$out"
rg -F -q 'static_footprint_evidence=1' "$out"
rg -F -q 'static_footprint_is_rss_claim=0' "$out"
rg -F -q 'baseline_shrink_action=0' "$out"
rg -F -q 'winner_claim=0' "$out"
rg -F -q 'summary=ok' "$out"

python3 - "$out" <<'PY'
import sys

values = {}
with open(sys.argv[1], encoding="utf-8") as fh:
    for line in fh:
        line = line.strip()
        if "=" in line:
            key, value = line.split("=", 1)
            values[key] = value

required_positive = [
    "hako_empty_evidence_external_rss_median_bytes",
    "hako_empty_noio_external_rss_median_bytes",
    "c_empty_external_rss_median_bytes",
    "hako_evidence_exe_file_bytes",
    "hako_noio_exe_file_bytes",
    "c_runner_file_bytes",
    "hako_evidence_exe_pt_load_mem_bytes",
    "hako_noio_exe_pt_load_mem_bytes",
    "c_runner_pt_load_mem_bytes",
    "c_mimalloc_library_file_bytes",
]
for key in required_positive:
    if int(values.get(key, "0")) <= 0:
        raise SystemExit(f"{key} must be positive")

expected = (
    int(values["hako_empty_evidence_external_rss_median_bytes"])
    - int(values["hako_empty_noio_external_rss_median_bytes"])
)
actual = int(values["hako_empty_evidence_minus_noio_rss_bytes"])
if actual != expected:
    raise SystemExit("evidence-minus-noio RSS mismatch")

print("[phase295x-malloc-large-empty-exe-footprint-diagnostic] ok")
PY

cat "$out"
echo "[$TAG] ok"
