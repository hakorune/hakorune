#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-mimalloc-comparison-hako-memory-evidence-runner"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

RUNNER="tools/allocator/hako_exe_memory_runner.sh"
APP="apps/hako-alloc-mimalloc-comparison-huge-osvm-slice-proof/main.hako"
TASKBOARD="docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md"
CARD="docs/development/current/main/phases/phase-294x/294x-61-MIMALLOC-COMPARISON-HAKO-MEMORY-EVIDENCE-RUNNER.md"
PREV_CARD="docs/development/current/main/phases/phase-294x/294x-60-MIMALLOC-COMPARISON-POST-CLOSEOUT-FOLLOW-ON-SELECTION.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_mimalloc_comparison_hako_memory_evidence_runner_guard.sh"
OUT="${TMPDIR:-/tmp}/hakorune_hako_memory_evidence_runner.out"

echo "[$TAG] checking hako EXE memory evidence runner"

guard_require_files "$TAG" "$RUNNER" "$APP" "$TASKBOARD" "$CARD" "$PREV_CARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$RUNNER" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'hako-exe-memory-evidence-v0' "$RUNNER" "runner must publish stable evidence contract"
guard_expect_in_file "$TAG" 'pure_first_route_preflight.py' "$RUNNER" "runner must preflight exact MIR before EXE build"
guard_expect_in_file "$TAG" '/usr/bin/time' "$RUNNER" "runner must use explicit RSS measurement tool"
guard_expect_in_file "$TAG" 'provider_activation=0' "$RUNNER" "runner must publish provider closed field"
guard_expect_in_file "$TAG" 'host_replacement=0' "$RUNNER" "runner must publish host replacement closed field"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HAKO-MEMORY-EVIDENCE-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HAKO-MEMORY-EVIDENCE-002' "$CARD" "card must select the follow-on blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HAKO-MEMORY-EVIDENCE-002' "$TASKBOARD" "taskboard must expose the follow-on blocker"
guard_expect_in_file "$TAG" "$RUNNER" "$INDEX" "check index must list the hako memory evidence runner"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

if rg -n 'LD_PRELOAD|replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|#\[global_allocator\]|provider_package_generated=1|hook_installed=1|global_allocator_installed=1' "$RUNNER" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: hako memory evidence runner opened replacement/provider/hook seams" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

bash "$RUNNER" --app "$APP" --workload huge-osvm-v1 --out "$OUT"

rg -F -q 'hako_exe_runner=1' "$OUT"
rg -F -q 'output_contract=hako-exe-memory-evidence-v0' "$OUT"
rg -F -q 'workload=huge-osvm-v1' "$OUT"
rg -F -q 'result_code=0' "$OUT"
rg -F -q 'run_count=1' "$OUT"
rg -F -q 'requested_bytes=4194321' "$OUT"
rg -F -q 'committed_bytes=4194433' "$OUT"
rg -F -q 'memory_usage_evidence=1' "$OUT"
rg -F -q 'output_summary_ok=1' "$OUT"
rg -F -q 'provider_activation=0' "$OUT"
rg -F -q 'host_replacement=0' "$OUT"
rg -F -q 'hook_installed=0' "$OUT"
rg -F -q 'global_allocator_installed=0' "$OUT"
rg -F -q 'summary=ok' "$OUT"

python3 - "$OUT" <<'PY'
import sys

values = {}
with open(sys.argv[1], encoding="utf-8") as fh:
    for line in fh:
        line = line.strip()
        if "=" in line:
            key, value = line.split("=", 1)
            values[key] = value

peak = int(values.get("peak_rss_bytes", "0"))
if peak <= 0:
    raise SystemExit(f"peak_rss_bytes must be nonzero, got {peak}")
print("[hako-memory-evidence-runner] ok")
PY

cat "$OUT"

echo "[$TAG] ok"
