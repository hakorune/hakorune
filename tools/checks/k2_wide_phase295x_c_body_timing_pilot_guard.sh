#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-c-body-timing-pilot"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-76-MIMALLOC-COMPARISON-C-BODY-TIMING-PILOT.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-75-MIMALLOC-COMPARISON-ALLOCATOR-BODY-TIMING-CONTRACT.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_c_body_timing_pilot_guard.sh"
C_RUNNER="tools/allocator/c_mimalloc_explicit_runner.sh"
C_SOURCE="tools/allocator/c_mimalloc_explicit_runner.c"

echo "[$TAG] checking phase-295x C body timing pilot"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$C_RUNNER" "$C_SOURCE"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$C_RUNNER"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-C-BODY-TIMING-PILOT-295X-001' "$CARD" "card must identify current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HAKO-BODY-TIMING-FEASIBILITY-SELECTION-295X-001' "$CARD" "card must select hako feasibility follow-on"
guard_expect_in_file "$TAG" 'body_timing_repeat_kind=workload-body-monotonic-v0' "$CARD" "card must preserve body timing repeat kind"
guard_expect_in_file "$TAG" 'CLOCK_MONOTONIC' "$C_SOURCE" "C source must use monotonic body timing"
guard_expect_in_file "$TAG" 'c_body_timing_available=1' "$C_SOURCE" "C source must expose C body timing"
guard_expect_in_file "$TAG" 'hako_body_timing_available=0' "$C_SOURCE" "C source must keep hako timing unavailable"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-C-BODY-TIMING-PILOT-295X-001' "$PREV_CARD" "previous card must select this pilot"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HAKO-BODY-TIMING-FEASIBILITY-SELECTION-295X-001' "$TASKBOARD" "taskboard must expose hako feasibility follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_c_body_timing.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
out="$tmp_dir/c-body.out"

bash "$C_RUNNER" \
  --out "$out" \
  --workload representative-small-block-v0 \
  --operation-repeat 1 \
  --allow-ldconfig-discovery >/tmp/"$TAG".stdout

rg -F -q 'output_contract=allocator-comparison-c-mimalloc-explicit-runner-v0' "$out"
rg -F -q 'workload=representative-small-block-v0' "$out"
rg -F -q 'operation_family=small-block' "$out"
rg -F -q 'c_body_timing_available=1' "$out"
rg -F -q 'hako_body_timing_available=0' "$out"
rg -F -q 'body_timing_repeat_kind=workload-body-monotonic-v0' "$out"
rg -F -q 'body_timing_scope=allocator-workload-body' "$out"
rg -F -q 'body_timing_is_process_timing=0' "$out"
rg -F -q 'timing_repeat_kind=process-invocation-v0' "$out"
rg -F -q 'winner_claim=' "$out" || true

python3 - "$out" <<'PY'
import sys

values = {}
with open(sys.argv[1], encoding="utf-8") as fh:
    for line in fh:
        line = line.strip()
        if "=" in line:
            key, value = line.split("=", 1)
            values[key] = value

body_ns = int(values.get("body_elapsed_ns", "0"))
external_ms = int(values.get("external_elapsed_ms", "0"))
if body_ns <= 0:
    raise SystemExit("body_elapsed_ns must be positive")
if external_ms <= 0:
    raise SystemExit("external_elapsed_ms must remain positive")
for key in (
    "process_replacement_executed",
    "hook_installed",
    "backend_matcher_added",
    "global_allocator_installed",
    "provider_package_generated",
):
    if values.get(key) != "0":
        raise SystemExit(f"{key} must remain 0")
print("[phase295x-c-body-timing-pilot] ok")
PY

cat "$out"
echo "[$TAG] ok"
