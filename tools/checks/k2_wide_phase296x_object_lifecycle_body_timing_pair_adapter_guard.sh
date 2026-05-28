#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-174-OBJECT-LIFECYCLE-BODY-TIMING-PAIR-ADAPTER.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-173-OBJECT-LIFECYCLE-HAKO-BODY-TIMING-FIRST-PATTERN.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_object_lifecycle_body_timing_pair_adapter_guard.sh"
ADAPTER="tools/allocator/hako_mimalloc_object_lifecycle_body_timing_pair_adapter.py"
HAKO_RUNNER="tools/allocator/hako_exe_memory_runner.sh"
C_RUNNER="tools/allocator/c_mimalloc_explicit_runner.sh"
APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"

[[ -f "$CARD" ]] || { echo "[row174-body-pair] missing card: $CARD" >&2; exit 1; }
[[ -f "$ADAPTER" ]] || { echo "[row174-body-pair] missing adapter: $ADAPTER" >&2; exit 1; }

grep -q '^Status: Current$' "$CARD" || { echo "[row174-body-pair] row174 card must be Current" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[row174-body-pair] row173 card must be Landed" >&2; exit 1; }
grep -q 'latest_card = "296x-174-OBJECT-LIFECYCLE-BODY-TIMING-PAIR-ADAPTER"' "$STATE" || { echo "[row174-body-pair] CURRENT_STATE latest_card must point to row174" >&2; exit 1; }
grep -q 'current_blocker_token = "OBJECT-LIFECYCLE-BODY-TIMING-PAIR-ADAPTER-296X-001"' "$STATE" || { echo "[row174-body-pair] CURRENT_STATE blocker must point to row174" >&2; exit 1; }
grep -q '| 173 | `OBJECT-LIFECYCLE-HAKO-BODY-TIMING-FIRST-PATTERN-296X-001` | Landed |' "$TASKBOARD" || { echo "[row174-body-pair] taskboard row173 must be Landed" >&2; exit 1; }
grep -q '| 174 | `OBJECT-LIFECYCLE-BODY-TIMING-PAIR-ADAPTER-296X-001` | Current |' "$TASKBOARD" || { echo "[row174-body-pair] taskboard row174 must be Current" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[row174-body-pair] check index missing guard entry" >&2; exit 1; }

tmp_dir="$(mktemp -d /tmp/hakorune_row174_body_pair.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
hako_report="$tmp_dir/hako.out"
c_report="$tmp_dir/c.out"
pair_report="$tmp_dir/pair.out"

bash "$HAKO_RUNNER" --app "$APP" --workload representative-object-lifecycle-small-block-v0 --runtime-config empty --operation-repeat 1 --out "$hako_report" >/dev/null
bash "$C_RUNNER" --out "$c_report" --allow-ldconfig-discovery --workload representative-object-lifecycle-small-block-v0 --in-process-repeat 8192 --operation-repeat 1 >/dev/null
python3 "$ADAPTER" --hako-report "$hako_report" --c-report "$c_report" --out "$pair_report"

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$pair_report"; then
    echo "[row174-body-pair] missing report line: $expected" >&2
    cat "$pair_report" >&2
    exit 1
  fi
}

require_line "output_contract=hako-mimalloc-object-lifecycle-body-timing-pair-v0"
require_line "workload_id=representative-object-lifecycle-small-block-v0"
require_line "operation_family=small-block"
require_line "operation_sequence_id=representative-object-lifecycle-small-block-v0-seq"
require_line "free_order_id=even-odd-release-v0"
require_line "in_process_operation_repeat=8192"
require_line "allocation_count=524288"
require_line "free_count=524288"
require_line "requested_bytes=272416768"
require_line "body_elapsed_comparable=1"
require_line "hako_body_timing_available=1"
require_line "c_body_timing_available=1"
require_line "next_diagnostic=object_lifecycle_body_timing_gap_taxonomy"
require_line "next_optimization_allowed=0"
require_line "winner_claim=0"
require_line "replacement_active=0"
require_line "hook_installed=0"
require_line "global_allocator=0"
require_line "summary=ok"

for key in hako_body_elapsed_ns c_body_elapsed_ns; do
  value="$(awk -F= -v k="$key" '$1 == k { print $2 }' "$pair_report")"
  case "$value" in
    ''|*[!0-9]*|0)
      echo "[row174-body-pair] $key must be a positive integer" >&2
      cat "$pair_report" >&2
      exit 1
      ;;
  esac
done

if ! awk -F= '$1 == "body_elapsed_ratio" { exit !($2 + 0 > 0) }' "$pair_report"; then
  echo "[row174-body-pair] body_elapsed_ratio must be positive" >&2
  cat "$pair_report" >&2
  exit 1
fi

echo "[row174-body-pair] ok"
