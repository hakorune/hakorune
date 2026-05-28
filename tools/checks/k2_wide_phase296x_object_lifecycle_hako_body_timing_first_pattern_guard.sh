#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
RUNNER="tools/allocator/hako_exe_memory_runner.sh"
CARD="docs/development/current/main/phases/phase-296x/296x-173-OBJECT-LIFECYCLE-HAKO-BODY-TIMING-FIRST-PATTERN.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-172-OBJECT-LIFECYCLE-EXACT-C-RUNNER-FIRST-PATTERN.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_object_lifecycle_hako_body_timing_first_pattern_guard.sh"

[[ -f "$CARD" ]] || { echo "[row173-hako-body-timing] missing card: $CARD" >&2; exit 1; }

grep -q '^Status: Current$' "$CARD" || {
  echo "[row173-hako-body-timing] row173 card must be Current" >&2
  exit 1
}
grep -q '^Status: Landed$' "$PREV_CARD" || {
  echo "[row173-hako-body-timing] row172 card must be Landed" >&2
  exit 1
}
grep -q 'latest_card = "296x-173-OBJECT-LIFECYCLE-HAKO-BODY-TIMING-FIRST-PATTERN"' "$STATE" || {
  echo "[row173-hako-body-timing] CURRENT_STATE latest_card must point to row173" >&2
  exit 1
}
grep -q 'current_blocker_token = "OBJECT-LIFECYCLE-HAKO-BODY-TIMING-FIRST-PATTERN-296X-001"' "$STATE" || {
  echo "[row173-hako-body-timing] CURRENT_STATE blocker must point to row173" >&2
  exit 1
}
grep -q '| 172 | `OBJECT-LIFECYCLE-EXACT-C-RUNNER-FIRST-PATTERN-296X-001` | Landed |' "$TASKBOARD" || {
  echo "[row173-hako-body-timing] taskboard row172 must be Landed" >&2
  exit 1
}
grep -q '| 173 | `OBJECT-LIFECYCLE-HAKO-BODY-TIMING-FIRST-PATTERN-296X-001` | Current |' "$TASKBOARD" || {
  echo "[row173-hako-body-timing] taskboard row173 must be Current" >&2
  exit 1
}
grep -q 'hako_body_timing_available=1' "$APP" || {
  echo "[row173-hako-body-timing] app must print hako_body_timing_available=1" >&2
  exit 1
}
grep -q 'body_elapsed_ns=' "$APP" || {
  echo "[row173-hako-body-timing] app must print body_elapsed_ns" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[row173-hako-body-timing] check index missing guard entry" >&2
  exit 1
}

tmp_dir="$(mktemp -d /tmp/hakorune_row173_hako_body_timing.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/hako.out"

bash "$RUNNER" \
  --app "$APP" \
  --workload representative-object-lifecycle-small-block-v0 \
  --runtime-config empty \
  --operation-repeat 1 \
  --out "$report" >/dev/null

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$report"; then
    echo "[row173-hako-body-timing] missing report line: $expected" >&2
    cat "$report" >&2
    exit 1
  fi
}

require_line "output_contract=hako-exe-memory-evidence-v0"
require_line "workload=representative-object-lifecycle-small-block-v0"
require_line "operation_family=small-block"
require_line "operation_sequence_id=representative-object-lifecycle-small-block-v0-seq"
require_line "free_order_id=even-odd-release-v0"
require_line "in_process_operation_repeat=8192"
require_line "hako_body_timing_available=1"
require_line "body_timing_repeat_kind=workload-body-env-now-ms-v0"
require_line "body_timing_scope=allocator-workload-body"
require_line "body_timing_is_process_timing=0"
require_line "allocation_count=524288"
require_line "free_count=524288"
require_line "requested_bytes=272416768"
require_line "select_page_single_fast_path_count=524288"
require_line "select_page_single_fallback_count=0"
require_line "release_known_page_fast_path_count=524288"
require_line "release_known_page_fallback_count=0"
require_line "provider_activation=0"
require_line "host_replacement=0"
require_line "hook_installed=0"
require_line "global_allocator_installed=0"
require_line "summary=ok"

body_elapsed="$(awk -F= '$1 == "body_elapsed_ns" { print $2 }' "$report")"
case "$body_elapsed" in
  ''|*[!0-9]*|0)
    echo "[row173-hako-body-timing] body_elapsed_ns must be a positive integer" >&2
    cat "$report" >&2
    exit 1
    ;;
esac

echo "[row173-hako-body-timing] ok"
