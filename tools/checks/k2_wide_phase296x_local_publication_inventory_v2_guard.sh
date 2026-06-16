#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-publication-inventory-v2"
CARD="docs/development/current/main/phases/phase-296x/296x-898-LOCAL-PUBLICATION-INVENTORY-V2-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-897-LOCAL-ALIAS-CLASS-MVP-001.md"
CODE="src/object_storage_plan.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_publication_inventory_v2_guard.sh"

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
  "output_contract=hako-local-publication-inventory-v2-v0" \
  "source_evidence=296x-897" \
  "row_kind=passive_inventory_vocabulary" \
  "local_publication_inventory_v2_vocabulary_defined=1" \
  "local_publication_inventory_v2_report_only=1" \
  "local_publication_inventory_v2_backend_consumable=0" \
  "local_publication_inventory_v2_unknown_alias_fallback=1" \
  "local_publication_inventory_v2_maybe_published_fallback=1" \
  "publication_state_unpublished_fastpath_allowed=1" \
  "publication_state_published_fastpath_allowed=0" \
  "publication_state_maybe_published_fastpath_allowed=0" \
  "fallback_fact_enabled=0" \
  "backend_new_lowering_enabled=0" \
  "object_storage_plan_execution_enabled=0" \
  "next_task=LOCAL-KNOWN-RECEIVER-DIRECT-CALL-SHADOW-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

grep -F -q "next_task=LOCAL-PUBLICATION-INVENTORY-V2-001" "$PREV_CARD" || {
  echo "[$TAG] alias MVP does not hand off to publication inventory v2" >&2
  exit 1
}

for code_text in \
  "pub struct LocalPublicationInventoryRow" \
  "pub fn can_feed_fastpath_eligibility" \
  "LocalFastPathFallbackReason::AliasUnknown" \
  "(\"local_publication_inventory_v2_vocabulary_defined\", \"1\")" \
  "(\"local_publication_inventory_v2_backend_consumable\", \"0\")" \
  "local_publication_inventory_row_is_report_only_gate_input"; do
  grep -R -F -q "$code_text" src/object_storage_plan.rs src/object_storage_plan || {
    echo "[$TAG] missing code evidence: $code_text" >&2
    exit 1
  }
done

for text in \
  "passive report vocabulary" \
  "not itself backend-consumable" \
  "Inventory rows are observations, not backend proof." \
  "alias_class is known" \
  "publication_state == Unpublished" \
  "no backend consumption of inventory rows"; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing card text: $text" >&2
    exit 1
  }
done

echo "[$TAG] ok"
