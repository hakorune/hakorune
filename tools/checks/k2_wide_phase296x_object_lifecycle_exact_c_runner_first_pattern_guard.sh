#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

C_RUNNER="tools/allocator/c_mimalloc_explicit_runner.c"
C_RUNNER_SH="tools/allocator/c_mimalloc_explicit_runner.sh"
CARD="docs/development/current/main/phases/phase-296x/296x-172-OBJECT-LIFECYCLE-EXACT-C-RUNNER-FIRST-PATTERN.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-171-OBJECT-LIFECYCLE-BODY-TIMING-AND-EXACT-C-PAIR-CONTRACT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_object_lifecycle_exact_c_runner_first_pattern_guard.sh"

[[ -f "$CARD" ]] || { echo "[row172-exact-c-runner] missing card: $CARD" >&2; exit 1; }

grep -q '^Status: Current$' "$CARD" || {
  echo "[row172-exact-c-runner] row172 card must be Current" >&2
  exit 1
}
grep -q '^Status: Landed$' "$PREV_CARD" || {
  echo "[row172-exact-c-runner] row171 card must be Landed" >&2
  exit 1
}
grep -q 'latest_card = "296x-172-OBJECT-LIFECYCLE-EXACT-C-RUNNER-FIRST-PATTERN"' "$STATE" || {
  echo "[row172-exact-c-runner] CURRENT_STATE latest_card must point to row172" >&2
  exit 1
}
grep -q 'current_blocker_token = "OBJECT-LIFECYCLE-EXACT-C-RUNNER-FIRST-PATTERN-296X-001"' "$STATE" || {
  echo "[row172-exact-c-runner] CURRENT_STATE blocker must point to row172" >&2
  exit 1
}
grep -q '| 171 | `OBJECT-LIFECYCLE-BODY-TIMING-AND-EXACT-C-PAIR-CONTRACT-296X-001` | Landed |' "$TASKBOARD" || {
  echo "[row172-exact-c-runner] taskboard row171 must be Landed" >&2
  exit 1
}
grep -q '| 172 | `OBJECT-LIFECYCLE-EXACT-C-RUNNER-FIRST-PATTERN-296X-001` | Current |' "$TASKBOARD" || {
  echo "[row172-exact-c-runner] taskboard row172 must be Current" >&2
  exit 1
}
grep -q 'representative-object-lifecycle-small-block-v0' "$C_RUNNER" || {
  echo "[row172-exact-c-runner] C runner must include object-lifecycle workload" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[row172-exact-c-runner] check index missing guard entry" >&2
  exit 1
}

tmp_dir="$(mktemp -d /tmp/hakorune_row172_exact_c_runner.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/c.out"

bash "$C_RUNNER_SH" \
  --out "$report" \
  --allow-ldconfig-discovery \
  --workload representative-object-lifecycle-small-block-v0 \
  --in-process-repeat 8192 \
  --operation-repeat 1 >/dev/null

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$report"; then
    echo "[row172-exact-c-runner] missing report line: $expected" >&2
    cat "$report" >&2
    exit 1
  fi
}

require_line "output_contract=allocator-comparison-c-mimalloc-explicit-runner-v0"
require_line "workload=representative-object-lifecycle-small-block-v0"
require_line "operation_family=small-block"
require_line "operation_sequence_id=representative-object-lifecycle-small-block-v0-seq"
require_line "free_order_id=even-odd-release-v0"
require_line "in_process_operation_repeat=8192"
require_line "allocation_count=524288"
require_line "free_count=524288"
require_line "requested_bytes=272416768"
require_line "c_body_timing_available=1"
require_line "hako_body_timing_available=0"
require_line "body_timing_repeat_kind=workload-body-monotonic-v0"
require_line "body_timing_is_process_timing=0"
require_line "process_replacement_executed=0"
require_line "hook_installed=0"
require_line "global_allocator_installed=0"
require_line "summary=ok"

body_elapsed="$(awk -F= '$1 == "body_elapsed_ns" { print $2 }' "$report")"
case "$body_elapsed" in
  ''|*[!0-9]*|0)
    echo "[row172-exact-c-runner] body_elapsed_ns must be a positive integer" >&2
    cat "$report" >&2
    exit 1
    ;;
esac

echo "[row172-exact-c-runner] ok"
