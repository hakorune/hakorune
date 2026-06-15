#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-815-LOCAL-DIRECT-ARRAY-LEN-PILOT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-814-LOCAL-OBJECT-SHADOW-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_direct_array_len_pilot_guard.sh"

[[ -f "$CARD" ]] || { echo "[local-direct-array-len-pilot] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[local-direct-array-len-pilot] missing previous card: $PREV_CARD" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[local-direct-array-len-pilot] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$PREV_CARD" || {
  echo "[local-direct-array-len-pilot] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[local-direct-array-len-pilot] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[local-direct-array-len-pilot] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-local-direct-array-len-pilot-preflight-v0" \
  "source_evidence=296x-814,296x-813" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "local_direct_array_len_pilot_requested=1" \
  "array_length_direct_candidate_count=0" \
  "pre_publication_array_length_candidate_count=0" \
  "local_direct_array_len_pilot_open=0" \
  "implementation_allowed=0" \
  "measurement_required_before_winner_claim=1" \
  "blocked_reason=no_array_length_candidate_in_target_facade_body" \
  "design_consultation_required=1" \
  "candidate_alternative=local_page_direct_call_pilot" \
  "do_not_infer_from_helper_symbol=1" \
  "do_not_reopen_array_receiver_residence_chain=1" \
  "object_plan_execution_enabled=0" \
  "backend_consumes_object_plan=0" \
  "product_default_changed=0" \
  "summary=blocked"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  "do not implement Array.length direct lowering from this evidence" \
  "do not infer Array.length candidacy from nyash_array_length_h" \
  "do not reopen ArrayReceiverResidenceFact from fallback evidence" \
  "do not switch target to page direct calls without design consultation"; do
  grep -F -q "$expected" "$CARD" || {
    echo "[local-direct-array-len-pilot] missing stop line: $expected" >&2
    exit 1
  }
done

echo "[local-direct-array-len-pilot] ok"
