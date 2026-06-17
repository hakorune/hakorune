#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="object-site-location-closeout"
CARD="docs/development/current/main/phases/phase-296x/296x-1002-OBJECT-SITE-LOCATION-CLOSEOUT-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_object_site_location_closeout_guard.sh"

[[ -f "$CARD" ]] || { echo "[$TAG] missing card: $CARD" >&2; exit 1; }
[[ -f "$INDEX" ]] || { echo "[$TAG] missing index: $INDEX" >&2; exit 1; }

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
  "output_contract=hako-object-site-location-closeout-v0" \
  "source_evidence=296x-996,296x-998,296x-1000,296x-1001" \
  "row_kind=closeout" \
  "object_site_location_vocabulary_defined=1" \
  "object_publication_site_location_field_migrated=1" \
  "local_fastpath_fact_location_field_migrated=1" \
  "local_publication_inventory_location_field_migrated=1" \
  "repeated_public_block_instruction_field_count=0" \
  "mir_json_block_instruction_shape_preserved=1" \
  "backend_lowering_changed=0" \
  "mir_json_metadata_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "reason_enum_merge_opened=0" \
  "scalar_field_descriptor_merge_opened=0" \
  "next_task=FRESH-COMPILER-OWNER-SELECTION-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

for token in \
  "(\"object_publication_site_location_field_migrated\", \"1\")" \
  "(\"local_fastpath_fact_location_field_migrated\", \"1\")" \
  "(\"local_publication_inventory_location_field_migrated\", \"1\")"; do
  grep -R -F -q "$token" src/object_storage_plan.rs src/object_storage_plan || {
    echo "[$TAG] missing report token: $token" >&2
    exit 1
  }
done

if rg -q "pub block_id: ObjectBasicBlockId|pub instruction_index: ObjectInstructionIndex" \
  src/object_storage_plan/publication.rs \
  src/object_storage_plan/fastpath.rs \
  src/object_storage_plan/inventory.rs; then
  echo "[$TAG] repeated public block/instruction fields remain outside ObjectSiteLocation" >&2
  exit 1
fi

grep -R -F -q "fact.block_id().0" src/runner/mir_json_emit || {
  echo "[$TAG] MIR JSON block accessor shape missing" >&2
  exit 1
}
grep -R -F -q "fact.instruction_index().0" src/runner/mir_json_emit || {
  echo "[$TAG] MIR JSON instruction accessor shape missing" >&2
  exit 1
}

echo "[$TAG] ok"
