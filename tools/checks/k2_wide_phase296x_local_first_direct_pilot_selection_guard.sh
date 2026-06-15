#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-816-LOCAL-FIRST-DIRECT-PILOT-SELECTION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-815-LOCAL-DIRECT-ARRAY-LEN-PILOT-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_first_direct_pilot_selection_guard.sh"

[[ -f "$CARD" ]] || { echo "[local-first-direct-pilot-selection] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[local-first-direct-pilot-selection] missing previous card: $PREV_CARD" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[local-first-direct-pilot-selection] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$PREV_CARD" || {
  echo "[local-first-direct-pilot-selection] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[local-first-direct-pilot-selection] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[local-first-direct-pilot-selection] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-local-first-direct-pilot-selection-v0" \
  "source_evidence=296x-815,296x-814,296x-813" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "array_length_pilot_closed_for_current_front=1" \
  "array_length_direct_candidate_count=0" \
  "pre_publication_array_length_candidate_count=0" \
  "selected_next_pilot=local_known_receiver_direct_call" \
  "first_target_receiver=page" \
  "first_target_call_count=3" \
  "first_target_methods=acquire_usize,reuse" \
  "pilot_scope=direct_call_only" \
  "page_is_first_target_not_rule=1" \
  "page_specific_rule_enabled=0" \
  "method_name_special_case_enabled=0" \
  "storage_direct_enabled=0" \
  "hosthandle_bypass_enabled=0" \
  "arc_retirement_enabled=0" \
  "product_default_changed=0" \
  "implementation_started=0" \
  "next_task=LOCAL-PAGE-RECEIVER-CANDIDATE-PROBE-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  "do not implement page-specific branch" \
  "do not special-case acquire_usize or reuse" \
  "do not infer from helper symbol" \
  "do not open storage direct route" \
  "do not bypass HostHandle" \
  "do not retire Arc" \
  "do not change product default runtime behavior" \
  "do not reopen Array.length pilot for current front"; do
  grep -F -q "$expected" "$CARD" || {
    echo "[local-first-direct-pilot-selection] missing stop line: $expected" >&2
    exit 1
  }
done

for expected in \
  "LOCAL-PAGE-RECEIVER-CANDIDATE-PROBE-001" \
  "LOCAL-KNOWN-RECEIVER-DIRECT-CALL-GUARD-SURFACE-001" \
  "LOCAL-KNOWN-RECEIVER-DIRECT-CALL-SHADOW-001" \
  "LOCAL-KNOWN-RECEIVER-DIRECT-CALL-PILOT-001" \
  "LOCAL-KNOWN-RECEIVER-DIRECT-CALL-MEASUREMENT-001"; do
  grep -F -q "$expected" "$CARD" || {
    echo "[local-first-direct-pilot-selection] missing next task: $expected" >&2
    exit 1
  }
done

echo "[local-first-direct-pilot-selection] ok"
