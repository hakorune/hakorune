#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="object-site-location-remaining-field-migration-selection"
CARD="docs/development/current/main/phases/phase-296x/296x-999-OBJECT-SITE-LOCATION-REMAINING-FIELD-MIGRATION-SELECTION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-998-OBJECT-PUBLICATION-SITE-LOCATION-FIELD-MIGRATION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_object_site_location_remaining_field_migration_selection_guard.sh"

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
  "output_contract=hako-object-site-location-remaining-field-migration-selection-v0" \
  "source_evidence=296x-996,296x-998,rg-audit" \
  "row_kind=selection" \
  "remaining_candidate_count=2" \
  "selected_next_migration=LocalFastPathFact" \
  "deferred_migration=LocalPublicationInventoryRow" \
  "local_fastpath_fact_external_consumer_count=2" \
  "local_publication_inventory_feeds_fact_construction=1" \
  "constructor_compatibility_required=1" \
  "mir_json_metadata_shape_preserved_required=1" \
  "field_migration_count=0" \
  "backend_lowering_changed=0" \
  "mir_json_metadata_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "smallest_safe_next=LOCAL-FASTPATH-FACT-LOCATION-FIELD-MIGRATION-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

grep -R -F -q "pub block_id: ObjectBasicBlockId" src/object_storage_plan/fastpath.rs || {
  echo "[$TAG] LocalFastPathFact should not be migrated in selection row" >&2
  exit 1
}

grep -R -F -q "pub block_id: ObjectBasicBlockId" src/object_storage_plan/inventory.rs || {
  echo "[$TAG] LocalPublicationInventoryRow should not be migrated in selection row" >&2
  exit 1
}

grep -R -F -q "fact.block_id" src/runner/mir_json_emit src/mir/map_repr_plan || {
  echo "[$TAG] expected LocalFastPathFact external consumer evidence missing" >&2
  exit 1
}

echo "[$TAG] ok"
