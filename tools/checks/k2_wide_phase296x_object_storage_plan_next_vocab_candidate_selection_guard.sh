#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="object-storage-plan-next-vocab-candidate-selection"
CARD="docs/development/current/main/phases/phase-296x/296x-995-OBJECT-STORAGE-PLAN-NEXT-VOCAB-CANDIDATE-SELECTION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-994-LOCALFIRSTOBJECTPLAN-ALIAS-RETIRE-IMPLEMENTATION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_object_storage_plan_next_vocab_candidate_selection_guard.sh"

for file in "$CARD" "$PREV_CARD" "$INDEX"; do
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
  "output_contract=hako-object-storage-plan-next-vocab-candidate-selection-v0" \
  "source_evidence=296x-991,296x-994,rg-audit" \
  "row_kind=selection" \
  "candidate_count=3" \
  "selected_candidate=site_location_fields" \
  "selected_next=OBJECT-SITE-LOCATION-VOCABULARY-001" \
  "reason_enum_merge_selected=0" \
  "scalar_field_descriptor_merge_selected=0" \
  "site_location_field_pair_count=3" \
  "immediate_field_migration_allowed=0" \
  "vocabulary_merge_count=0" \
  "backend_lowering_changed=0" \
  "mir_json_metadata_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "summary=ok"; do
  require_card_line "$expected"
done

grep -R -F -q "pub struct ObjectBasicBlockId" src/object_storage_plan.rs src/object_storage_plan || {
  echo "[$TAG] ObjectBasicBlockId vocabulary missing" >&2
  exit 1
}
grep -R -F -q "pub struct ObjectInstructionIndex" src/object_storage_plan.rs src/object_storage_plan || {
  echo "[$TAG] ObjectInstructionIndex vocabulary missing" >&2
  exit 1
}

pair_count="$(rg -n "pub block_id: ObjectBasicBlockId" \
  src/object_storage_plan/publication.rs \
  src/object_storage_plan/fastpath.rs \
  src/object_storage_plan/inventory.rs | wc -l | tr -d ' ')"
if [[ "$pair_count" != "3" ]]; then
  echo "[$TAG] expected 3 object storage plan block_id field pairs, got $pair_count" >&2
  exit 1
fi

if grep -R -F -q "pub struct ObjectSiteLocation" src/object_storage_plan.rs src/object_storage_plan; then
  echo "[$TAG] ObjectSiteLocation should not be added in selection row" >&2
  exit 1
fi

echo "[$TAG] ok"
