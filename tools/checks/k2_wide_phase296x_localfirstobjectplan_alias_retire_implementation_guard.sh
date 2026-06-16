#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="localfirstobjectplan-alias-retire-implementation"
CARD="docs/development/current/main/phases/phase-296x/296x-994-LOCALFIRSTOBJECTPLAN-ALIAS-RETIRE-IMPLEMENTATION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-993-LOCALFIRSTOBJECTPLAN-ALIAS-RETIRE-GUARD-COMPAT-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_localfirstobjectplan_alias_retire_implementation_guard.sh"
OBJECT_SRC="src/object_storage_plan.rs"
OBJECT_DIR="src/object_storage_plan"

for file in "$CARD" "$PREV_CARD" "$INDEX" "$OBJECT_SRC"; do
  [[ -f "$file" ]] || { echo "[$TAG] missing file: $file" >&2; exit 1; }
done
[[ -d "$OBJECT_DIR" ]] || { echo "[$TAG] missing object module dir: $OBJECT_DIR" >&2; exit 1; }

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
  "output_contract=hako-localfirstobjectplan-alias-retire-implementation-v0" \
  "source_evidence=296x-991,296x-992,296x-993" \
  "row_kind=implementation" \
  "localfirstobjectplan_alias_removed=1" \
  "objectplan_canonical_name_required=1" \
  "local_first_object_plan_alias_retired=1" \
  "local_first_object_plan_compat_alias_enabled=0" \
  "historical_guards_tolerate_alias_retire=1" \
  "public_api_reexport_preserved=1" \
  "vocabulary_merge_count=1" \
  "backend_lowering_changed=0" \
  "mir_json_metadata_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "summary=ok"; do
  require_card_line "$expected"
done

grep -R -F -q "pub struct ObjectPlan" "$OBJECT_SRC" "$OBJECT_DIR" || {
  echo "[$TAG] canonical ObjectPlan source is missing" >&2
  exit 1
}

if grep -R -F -q "pub type LocalFirstObjectPlan = ObjectPlan" "$OBJECT_SRC" "$OBJECT_DIR"; then
  echo "[$TAG] compatibility alias still exists in source" >&2
  exit 1
fi

if grep -R -F -q "LocalFirstObjectPlan::new" "$OBJECT_SRC" "$OBJECT_DIR"; then
  echo "[$TAG] tests/source still construct through retired alias" >&2
  exit 1
fi

grep -R -F -q "(\"local_first_object_plan_alias_retired\", \"1\")" "$OBJECT_SRC" "$OBJECT_DIR" || {
  echo "[$TAG] retired marker report field missing" >&2
  exit 1
}

if grep -R -F -q "(\"local_first_object_plan_compat_alias_enabled\", \"1\")" "$OBJECT_SRC" "$OBJECT_DIR"; then
  echo "[$TAG] compat alias enabled report field still present" >&2
  exit 1
fi

echo "[$TAG] ok"
