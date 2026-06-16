#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-known-receiver-direct-call-shadow"
CARD="docs/development/current/main/phases/phase-296x/296x-899-LOCAL-KNOWN-RECEIVER-DIRECT-CALL-SHADOW-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-898-LOCAL-PUBLICATION-INVENTORY-V2-001.md"
CODE="src/object_storage_plan.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_known_receiver_direct_call_shadow_guard.sh"

for file in "$CARD" "$PREV_CARD" "$CODE" "$INDEX"; do
  [[ -f "$file" ]] || { echo "[$TAG] missing file: $file" >&2; exit 1; }
done

grep -q '^Status: Landed$' "$CARD" || {
  echo "[$TAG] card must be Landed" >&2
  exit 1
}

grep -F -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[$TAG] check index missing guard entry" >&2
  exit 1
}

require_card_line() {
  local expected="$1"
  if ! grep -F -x -q "$expected" "$CARD"; then
    echo "[$TAG] missing card line: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-local-known-receiver-direct-call-shadow-v0" \
  "source_evidence=296x-898" \
  "row_kind=passive_shadow_vocabulary" \
  "local_known_receiver_direct_call_shadow_defined=1" \
  "local_known_receiver_direct_call_shadow_backend_consumable=0" \
  "local_known_receiver_direct_call_shadow_fact_optional=1" \
  "local_known_receiver_direct_call_shadow_requires_routeplan=1" \
  "local_known_receiver_direct_call_shadow_requires_objectstorageplan=1" \
  "local_fastpath_fact_backend_consumable=1" \
  "fallback_evidence_backend_consumable=0" \
  "fallback_fact_enabled=0" \
  "backend_new_lowering_enabled=0" \
  "object_storage_plan_execution_enabled=0" \
  "next_task=LOCAL-KNOWN-RECEIVER-DIRECT-CALL-PILOT-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

grep -F -q "next_task=LOCAL-KNOWN-RECEIVER-DIRECT-CALL-SHADOW-001" "$PREV_CARD" || {
  echo "[$TAG] publication inventory does not hand off to shadow row" >&2
  exit 1
}

for code_text in \
  "pub struct LocalKnownReceiverDirectCallShadowRow" \
  "pub candidate_fact: Option<LocalFastPathFact>" \
  "LocalFastPathFact::known_receiver_direct_call" \
  "LocalFastPathFallbackReason::DynamicRoute" \
  "LocalFastPathFallbackReason::GenericStorage" \
  "(\"local_known_receiver_direct_call_shadow_defined\", \"1\")" \
  "local_known_receiver_direct_call_shadow_row_creates_fact_only_when_all_inputs_are_positive"; do
  grep -F -q "$code_text" "$CODE" || {
    echo "[$TAG] missing code evidence: $code_text" >&2
    exit 1
  }
done

for text in \
  "passive vocabulary" \
  "optional" \
  "LocalFastPathFact::KnownReceiverDirectCall" \
  "does not enable backend lowering" \
  "no backend consumption of shadow rows"; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing card text: $text" >&2
    exit 1
  }
done

echo "[$TAG] ok"
