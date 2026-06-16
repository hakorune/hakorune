#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="object-publication-site-location-field-migration"
CARD="docs/development/current/main/phases/phase-296x/296x-998-OBJECT-PUBLICATION-SITE-LOCATION-FIELD-MIGRATION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-997-OBJECT-SITE-LOCATION-FIELD-MIGRATION-PREFLIGHT-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_object_publication_site_location_field_migration_guard.sh"

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
  "output_contract=hako-object-publication-site-location-field-migration-v0" \
  "source_evidence=296x-996,296x-997" \
  "row_kind=implementation" \
  "selected_migration=ObjectPublicationSite" \
  "object_publication_site_location_field_migrated=1" \
  "object_site_location_field_migration_enabled=1" \
  "object_publication_site_block_instruction_accessors_preserved=1" \
  "local_fastpath_fact_field_migrated=0" \
  "local_publication_inventory_field_migrated=0" \
  "field_migration_count=1" \
  "backend_lowering_changed=0" \
  "mir_json_metadata_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "smallest_safe_next=OBJECT-SITE-LOCATION-REMAINING-FIELD-MIGRATION-SELECTION-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

grep -R -F -q "pub location: ObjectSiteLocation" src/object_storage_plan/publication.rs || {
  echo "[$TAG] ObjectPublicationSite location field missing" >&2
  exit 1
}

if grep -R -F -q "pub block_id: ObjectBasicBlockId" src/object_storage_plan/publication.rs; then
  echo "[$TAG] ObjectPublicationSite still exposes block_id field" >&2
  exit 1
fi

for token in \
  "pub const fn block_id(&self) -> ObjectBasicBlockId" \
  "pub const fn instruction_index(&self) -> ObjectInstructionIndex" \
  "(\"object_publication_site_location_field_migrated\", \"1\")"; do
  grep -R -F -q "$token" src/object_storage_plan.rs src/object_storage_plan || {
    echo "[$TAG] missing source token: $token" >&2
    exit 1
  }
done

grep -R -F -q "pub block_id: ObjectBasicBlockId" src/object_storage_plan/fastpath.rs src/object_storage_plan/inventory.rs || {
  echo "[$TAG] remaining field carriers were unexpectedly migrated" >&2
  exit 1
}

echo "[$TAG] ok"
