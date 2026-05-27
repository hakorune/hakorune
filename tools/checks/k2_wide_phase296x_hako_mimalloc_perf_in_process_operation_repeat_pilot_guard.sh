#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-perf-in-process-operation-repeat-pilot"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_55="docs/development/current/main/phases/phase-296x/296x-55-HAKO-MIMALLOC-PERF-IN-PROCESS-OPERATION-REPEAT-PILOT.md"
CARD_56="docs/development/current/main/phases/phase-296x/296x-56-HAKO-MIMALLOC-PERF-IN-PROCESS-GAP-TAXONOMY-DECISION.md"
CARD_57="docs/development/current/main/phases/phase-296x/296x-57-HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
APP="apps/hako-alloc-mimalloc-comparison-in-process-small-block-proof/main.hako"
HAKO_RUNNER="tools/allocator/hako_exe_memory_runner.sh"
C_RUNNER_SRC="tools/allocator/c_mimalloc_explicit_runner.c"
C_RUNNER_SH="tools/allocator/c_mimalloc_explicit_runner.sh"
PILOT="tools/allocator/hako_mimalloc_in_process_operation_repeat_pilot.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_perf_in_process_operation_repeat_pilot_guard.sh"

echo "[$TAG] checking phase-296x in-process operation repeat pilot"

guard_require_files "$TAG" "$CARD_55" "$CARD_56" "$CARD_57" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$APP" "$HAKO_RUNNER" "$C_RUNNER_SRC" "$C_RUNNER_SH" "$PILOT" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$PILOT" "$SELF_SCRIPT" "$HAKO_RUNNER" "$C_RUNNER_SH"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_55" "pilot card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_56" "in-process gap taxonomy card must be current"
guard_expect_fixed_in_file "$TAG" 'Status: Planned' "$CARD_57" "first keeper optimization card must be planned"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-in-process-operation-repeat-measurement-v0' "$CARD_55" "pilot card must define output contract"
guard_expect_fixed_in_file "$TAG" 'process_invocation_repeat=0' "$CARD_55" "pilot card must close process repeat"
guard_expect_fixed_in_file "$TAG" 'external_elapsed_median_gap_ms=326' "$CARD_55" "pilot card must record pilot gap"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-55-HAKO-MIMALLOC-PERF-IN-PROCESS-OPERATION-REPEAT-PILOT"' "$CURRENT_STATE" "current state latest card must advance to row 55"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PERF-IN-PROCESS-GAP-TAXONOMY-DECISION-296X-001"' "$CURRENT_STATE" "current state must select row 56"
guard_expect_fixed_in_file "$TAG" '| 55 | `HAKO-MIMALLOC-PERF-IN-PROCESS-OPERATION-REPEAT-PILOT-296X-001` | Landed |' "$TASKBOARD" "taskboard row 55 must be landed"
guard_expect_fixed_in_file "$TAG" '| 56 | `HAKO-MIMALLOC-PERF-IN-PROCESS-GAP-TAXONOMY-DECISION-296X-001` | Current |' "$TASKBOARD" "taskboard row 56 must be current"
guard_expect_fixed_in_file "$TAG" '| 57 | `HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION-296X-001` | Planned |' "$TASKBOARD" "taskboard row 57 must be planned"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$PILOT" "$INDEX" "check index must list pilot tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_in_process_pilot.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/pilot.out"
python3 "$PILOT" \
  --out "$report" \
  --c-library /lib/x86_64-linux-gnu/libmimalloc.so.2 \
  --operation-repeat 8192 \
  --process-repeat 3 >/dev/null

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-in-process-operation-repeat-measurement-v0' "$report" "pilot must emit measurement contract"
guard_expect_fixed_in_file "$TAG" 'timing_repeat_kind=in-process-operation-loop-v0' "$report" "pilot must use in-process timing kind"
guard_expect_fixed_in_file "$TAG" 'operation_repeat=8192' "$report" "pilot must use inner repeat"
guard_expect_fixed_in_file "$TAG" 'process_repeat=3' "$report" "pilot must use sample process count"
guard_expect_fixed_in_file "$TAG" 'same_workload=1' "$report" "pilot must use same workload"
guard_expect_fixed_in_file "$TAG" 'same_operation_count=1' "$report" "pilot must use same operation count"
guard_expect_fixed_in_file "$TAG" 'process_invocation_repeat=0' "$report" "pilot must not use process invocation repeat"
guard_expect_fixed_in_file "$TAG" 'allocation_count=524288' "$report" "pilot must scale allocation count"
guard_expect_fixed_in_file "$TAG" 'free_count=524288' "$report" "pilot must scale free count"
guard_expect_fixed_in_file "$TAG" 'requested_bytes=272416768' "$report" "pilot must scale requested bytes"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "pilot must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$report" "pilot must keep provider closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "pilot must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "pilot must end ok"

echo "[$TAG] ok"
