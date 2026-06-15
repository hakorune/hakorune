#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

CARD="docs/development/current/main/phases/phase-296x/296x-827-SELFHOST-MIR-OBJECT-METADATA-001.md"
SSOT="docs/development/current/main/design/selfhost-mir-object-metadata-ssot.md"
README="lang/src/compiler/mirbuilder/README.md"
INDEX="docs/tools/check-scripts-index.md"

require_line_in_file() {
  local file="$1"
  local needle="$2"
  if ! grep -F -q "$needle" "$file"; then
    echo "[selfhost-mir-object-metadata] missing '$needle' in $file" >&2
    exit 1
  fi
}

for file in "$CARD" "$SSOT"; do
  for line in \
    "selfhost_mir_object_metadata_contract=hako-selfhost-mir-object-metadata-v0" \
    "selfhost_mirbuilder_metadata_only=1" \
    "selfhost_mirbuilder_representation_truth_enabled=0" \
    "selfhost_mirbuilder_publication_truth_enabled=0" \
    "selfhost_mirbuilder_backend_route_truth_enabled=0"; do
    require_line_in_file "$file" "$line"
  done
done

for line in \
  "source_span" \
  "receiver_origin" \
  "known_type_hint" \
  "field_key" \
  "call_site_id" \
  "newbox_origin"; do
  require_line_in_file "$SSOT" "$line"
done

for line in \
  "object_storage_plan" \
  "publication_site" \
  "hosthandle_bypass" \
  "arc_retirement" \
  "backend_direct_route"; do
  require_line_in_file "$SSOT" "$line"
done

require_line_in_file "$CARD" "selfhost_mirbuilder_fail_fast_prefix=[freeze:contract][hako_mirbuilder]"
require_line_in_file "$CARD" "selected_next=OBJECTPLAN-PASSIVE-UNIFY-001"
require_line_in_file "$README" "selfhost-mir-object-metadata-ssot.md"
require_line_in_file "$INDEX" "k2_wide_phase296x_selfhost_mir_object_metadata_guard.sh"

echo "[selfhost-mir-object-metadata] ok"
