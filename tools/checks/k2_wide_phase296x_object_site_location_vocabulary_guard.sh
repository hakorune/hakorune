#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="object-site-location-vocabulary"
CARD="docs/development/current/main/phases/phase-296x/296x-996-OBJECT-SITE-LOCATION-VOCABULARY-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-995-OBJECT-STORAGE-PLAN-NEXT-VOCAB-CANDIDATE-SELECTION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_object_site_location_vocabulary_guard.sh"

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
  "output_contract=hako-object-site-location-vocabulary-v0" \
  "source_evidence=296x-995" \
  "row_kind=vocabulary_surface" \
  "object_site_location_vocabulary_defined=1" \
  "object_site_location_field_migration_enabled=0" \
  "publication_site_location_accessor_enabled=1" \
  "local_fastpath_fact_location_accessor_enabled=1" \
  "local_publication_inventory_location_accessor_enabled=1" \
  "public_field_shape_preserved=1" \
  "vocabulary_merge_count=0" \
  "backend_lowering_changed=0" \
  "mir_json_metadata_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "smallest_safe_next=OBJECT-SITE-LOCATION-FIELD-MIGRATION-PREFLIGHT-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

for token in \
  "pub struct ObjectSiteLocation" \
  "pub const fn new(" \
  "pub const fn location(&self) -> ObjectSiteLocation" \
  "(\"object_site_location_vocabulary_defined\", \"1\")" \
  "(\"object_site_location_field_migration_enabled\", \"0\")"; do
  grep -R -F -q "$token" src/object_storage_plan.rs src/object_storage_plan || {
    echo "[$TAG] missing source token: $token" >&2
    exit 1
  }
done

for token in \
  "pub block_id: ObjectBasicBlockId" \
  "pub instruction_index: ObjectInstructionIndex"; do
  grep -R -F -q "$token" src/object_storage_plan/publication.rs src/object_storage_plan/fastpath.rs src/object_storage_plan/inventory.rs || {
    echo "[$TAG] public field shape was unexpectedly migrated: $token" >&2
    exit 1
  }
done

echo "[$TAG] ok"
