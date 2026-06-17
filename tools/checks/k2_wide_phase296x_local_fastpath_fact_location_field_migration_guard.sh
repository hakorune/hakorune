#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-fastpath-fact-location-field-migration"
CARD="docs/development/current/main/phases/phase-296x/296x-1000-LOCAL-FASTPATH-FACT-LOCATION-FIELD-MIGRATION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-999-OBJECT-SITE-LOCATION-REMAINING-FIELD-MIGRATION-SELECTION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_fastpath_fact_location_field_migration_guard.sh"

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
  "output_contract=hako-local-fastpath-fact-location-field-migration-v0" \
  "source_evidence=296x-996,296x-999" \
  "row_kind=implementation" \
  "selected_migration=LocalFastPathFact" \
  "local_fastpath_fact_location_field_migrated=1" \
  "local_fastpath_fact_constructor_compat_preserved=1" \
  "local_fastpath_fact_block_instruction_accessors_preserved=1" \
  "mir_json_block_instruction_shape_preserved=1" \
  "local_publication_inventory_field_migrated=0" \
  "field_migration_count=1" \
  "backend_lowering_changed=0" \
  "mir_json_metadata_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "smallest_safe_next=LOCAL-PUBLICATION-INVENTORY-LOCATION-FIELD-MIGRATION-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

grep -R -F -q "pub location: ObjectSiteLocation" src/object_storage_plan/fastpath.rs || {
  echo "[$TAG] LocalFastPathFact location field missing" >&2
  exit 1
}

if grep -R -F -q "pub block_id: ObjectBasicBlockId" src/object_storage_plan/fastpath.rs; then
  echo "[$TAG] LocalFastPathFact still exposes block_id field" >&2
  exit 1
fi

for token in \
  "pub const fn block_id(&self) -> ObjectBasicBlockId" \
  "pub const fn instruction_index(&self) -> ObjectInstructionIndex" \
  "fact.block_id().0" \
  "fact.instruction_index().0" \
  "(\"local_fastpath_fact_location_field_migrated\", \"1\")"; do
  grep -R -F -q "$token" src/object_storage_plan.rs src/object_storage_plan src/runner/mir_json_emit || {
    echo "[$TAG] missing source token: $token" >&2
    exit 1
  }
done

grep -R -F -q "pub block_id: ObjectBasicBlockId" src/object_storage_plan/inventory.rs || {
  echo "[$TAG] LocalPublicationInventoryRow was unexpectedly migrated" >&2
  exit 1
}

echo "[$TAG] ok"
