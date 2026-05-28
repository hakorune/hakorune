#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TOOL="tools/allocator/hako_mimalloc_object_lifecycle_body_timing_and_exact_c_pair_contract.py"
CARD="docs/development/current/main/phases/phase-296x/296x-171-OBJECT-LIFECYCLE-BODY-TIMING-AND-EXACT-C-PAIR-CONTRACT.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-170-POST-ROLLBACK-GAP-TAXONOMY-REFRESH.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_object_lifecycle_body_timing_and_exact_c_pair_contract_guard.sh"

[[ -f "$TOOL" ]] || { echo "[row171-body-timing-contract] missing tool: $TOOL" >&2; exit 1; }
[[ -f "$CARD" ]] || { echo "[row171-body-timing-contract] missing card: $CARD" >&2; exit 1; }

grep -q '^Status: Current$' "$CARD" || {
  echo "[row171-body-timing-contract] row171 card must be Current" >&2
  exit 1
}
grep -q '^Status: Landed$' "$PREV_CARD" || {
  echo "[row171-body-timing-contract] row170 card must be Landed" >&2
  exit 1
}
grep -q 'latest_card = "296x-171-OBJECT-LIFECYCLE-BODY-TIMING-AND-EXACT-C-PAIR-CONTRACT"' "$STATE" || {
  echo "[row171-body-timing-contract] CURRENT_STATE latest_card must point to row171" >&2
  exit 1
}
grep -q 'current_blocker_token = "OBJECT-LIFECYCLE-BODY-TIMING-AND-EXACT-C-PAIR-CONTRACT-296X-001"' "$STATE" || {
  echo "[row171-body-timing-contract] CURRENT_STATE blocker must point to row171" >&2
  exit 1
}
grep -q '| 170 | `POST-ROLLBACK-GAP-TAXONOMY-REFRESH-296X-001` | Landed |' "$TASKBOARD" || {
  echo "[row171-body-timing-contract] taskboard row170 must be Landed" >&2
  exit 1
}
grep -q '| 171 | `OBJECT-LIFECYCLE-BODY-TIMING-AND-EXACT-C-PAIR-CONTRACT-296X-001` | Current |' "$TASKBOARD" || {
  echo "[row171-body-timing-contract] taskboard row171 must be Current" >&2
  exit 1
}
grep -q "$TOOL" "$INDEX" || {
  echo "[row171-body-timing-contract] check index missing tool entry" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[row171-body-timing-contract] check index missing guard entry" >&2
  exit 1
}

tmp_dir="$(mktemp -d /tmp/hakorune_row171_body_timing_contract.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"

python3 "$TOOL" --out "$report"

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$report"; then
    echo "[row171-body-timing-contract] missing report line: $expected" >&2
    cat "$report" >&2
    exit 1
  fi
}

require_line "output_contract=hako-mimalloc-object-lifecycle-body-timing-and-exact-c-pair-contract-v0"
require_line "input_contract=hako-mimalloc-post-rollback-gap-taxonomy-refresh-v0"
require_line "workload_id=representative-object-lifecycle-small-block-v0"
require_line "operation_sequence_id=representative-object-lifecycle-small-block-v0-seq"
require_line "free_order_id=even-odd-release-v0"
require_line "required_hako_subject=hako_exact_exe_object_lifecycle"
require_line "required_c_subject=c_mimalloc_explicit_object_lifecycle"
require_line "required_allocation_count=524288"
require_line "required_free_count=524288"
require_line "required_requested_bytes=272416768"
require_line "hako_body_elapsed_ns_required=1"
require_line "c_body_elapsed_ns_required=1"
require_line "body_elapsed_comparable_required=1"
require_line "body_elapsed_role=primary_hot_loop_diagnostic"
require_line "external_elapsed_role=secondary_process_runtime_evidence"
require_line "exact_c_pair_required=1"
require_line "exact_c_pair_status=missing"
require_line "hako_body_timing_status=missing"
require_line "measurement_contract_gap_open=1"
require_line "next_diagnostic=object_lifecycle_exact_c_runner_first_pattern"
require_line "next_optimization_allowed=0"
require_line "winner_claim=0"
require_line "provider_active=0"
require_line "replacement_active=0"
require_line "hook_installed=0"
require_line "global_allocator=0"
require_line "summary=ok"

echo "[row171-body-timing-contract] ok"
