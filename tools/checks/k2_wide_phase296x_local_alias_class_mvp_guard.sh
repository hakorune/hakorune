#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-alias-class-mvp"
CARD="docs/development/current/main/phases/phase-296x/296x-897-LOCAL-ALIAS-CLASS-MVP-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-896-LOCAL-PUBLICATION-CLASSIFIER-000.md"
CODE="src/object_storage_plan.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_alias_class_mvp_guard.sh"

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
  "output_contract=hako-local-alias-class-mvp-v0" \
  "source_evidence=296x-896" \
  "row_kind=passive_vocabulary" \
  "local_alias_class_mvp_vocabulary_defined=1" \
  "local_alias_class_mvp_source_count=5" \
  "local_alias_source_local_assignment=1" \
  "local_alias_source_ssa_copy=1" \
  "local_alias_source_phi=1" \
  "local_alias_source_select=1" \
  "local_alias_source_simple_receiver_alias=1" \
  "local_alias_class_heap_graph_enabled=0" \
  "local_alias_class_field_sensitive_points_to_enabled=0" \
  "local_alias_class_collection_element_alias_enabled=0" \
  "local_alias_class_recursive_object_graph_enabled=0" \
  "object_storage_plan_execution_enabled=0" \
  "backend_new_lowering_enabled=0" \
  "next_task=LOCAL-PUBLICATION-INVENTORY-V2-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

grep -F -q "next_task=LOCAL-ALIAS-CLASS-MVP-001" "$PREV_CARD" || {
  echo "[$TAG] publication classifier does not hand off to alias MVP" >&2
  exit 1
}

for code_text in \
  "pub enum LocalAliasSourceKind" \
  "LocalAssignment" \
  "SsaCopy" \
  "Phi" \
  "Select" \
  "SimpleReceiverAlias" \
  "pub struct LocalAliasClassObservation" \
  "(\"local_alias_class_mvp_vocabulary_defined\", \"1\")" \
  "(\"local_alias_class_heap_graph_enabled\", \"0\")" \
  "local_alias_class_mvp_observation_is_passive_vocabulary"; do
  grep -R -F -q "$code_text" src/object_storage_plan.rs src/object_storage_plan || {
    echo "[$TAG] missing code evidence: $code_text" >&2
    exit 1
  }
done

for text in \
  "passive vocabulary only" \
  "does not run a" \
  "classifier" \
  "does not authorize backend lowering" \
  "smaller than a general escape / points-to engine" \
  "no heap graph traversal" \
  "no field-sensitive points-to"; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing card text: $text" >&2
    exit 1
  }
done

echo "[$TAG] ok"
