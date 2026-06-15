#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-812-OBJECT-PLAN-LOCAL-FIRST-000.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-811-LOCAL-FIRST-OBJECT-MODEL-SSOT-001.md"
SRC="src/object_storage_plan.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_object_plan_local_first_guard.sh"

[[ -f "$CARD" ]] || { echo "[object-plan-local-first] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[object-plan-local-first] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$SRC" ]] || { echo "[object-plan-local-first] missing source: $SRC" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[object-plan-local-first] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$PREV_CARD" || {
  echo "[object-plan-local-first] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[object-plan-local-first] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[object-plan-local-first] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-object-plan-local-first-v0" \
  "source_evidence=296x-811,296x-711" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "object_plan_local_first_vocabulary_defined=1" \
  "object_plan_representation_field=ObjectStoragePlan" \
  "object_plan_publication_sites_defined=1" \
  "publication_site_reason_vocabulary_defined=1" \
  "standalone_publication_plan_enabled=0" \
  "publication_reason_plugin_or_extern=1" \
  "publication_reason_host_handle_required=1" \
  "publication_reason_dynamic_array_or_map=1" \
  "publication_reason_dynamic_nyashbox_api=1" \
  "publication_reason_return_as_dynamic_box=1" \
  "publication_reason_task_future_channel_boundary=1" \
  "publication_reason_unknown_fini_or_drop=1" \
  "publication_reason_unknown=1" \
  "unknown_publication_forces_generic_fallback=1" \
  "mirbuilder_object_management_enabled=0" \
  "mirbuilder_representation_owner=0" \
  "object_plan_execution_enabled=0" \
  "object_plan_mir_json_export_enabled=0" \
  "backend_consumes_object_plan=0" \
  "product_default_changed=0" \
  "next_task=OBJECT-PUBLICATION-INVENTORY-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for token in \
  "pub enum ObjectPublicationReason" \
  "pub struct ObjectPublicationSite" \
  "pub fn is_unpublished_local" \
  "pub fn requires_publication" \
  "(\"object_plan_execution_enabled\", \"0\")" \
  "(\"standalone_publication_plan_enabled\", \"0\")"; do
  grep -F -q "$token" "$SRC" || {
    echo "[object-plan-local-first] missing source token: $token" >&2
    exit 1
  }
done

if ! grep -F -q "pub struct LocalFirstObjectPlan" "$SRC" \
  && ! grep -F -q "pub type LocalFirstObjectPlan = ObjectPlan" "$SRC"; then
  echo "[object-plan-local-first] missing LocalFirstObjectPlan struct or compat alias" >&2
  exit 1
fi

for expected in \
  "do not add standalone PublicationPlan from this row" \
  "do not connect ObjectPlan to lowering from this row" \
  "do not let backend consume ObjectPlan from this row" \
  "do not move object representation ownership into MIRBuilder"; do
  grep -F -q "$expected" "$CARD" || {
    echo "[object-plan-local-first] missing stop line: $expected" >&2
    exit 1
  }
done

echo "[object-plan-local-first] ok"
