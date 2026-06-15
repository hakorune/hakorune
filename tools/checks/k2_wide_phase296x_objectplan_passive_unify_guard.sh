#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-828-OBJECTPLAN-PASSIVE-UNIFY-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-827-SELFHOST-MIR-OBJECT-METADATA-001.md"
FINAL_SSOT="docs/development/current/main/design/compiler-object-final-shape-ssot.md"
SRC="src/object_storage_plan.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_objectplan_passive_unify_guard.sh"

[[ -f "$CARD" ]] || { echo "[objectplan-passive-unify] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[objectplan-passive-unify] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$FINAL_SSOT" ]] || { echo "[objectplan-passive-unify] missing final SSOT: $FINAL_SSOT" >&2; exit 1; }
[[ -f "$SRC" ]] || { echo "[objectplan-passive-unify] missing source: $SRC" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || {
  echo "[objectplan-passive-unify] card must be Landed" >&2
  exit 1
}
grep -q '^Status: Landed$' "$PREV_CARD" || {
  echo "[objectplan-passive-unify] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[objectplan-passive-unify] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[objectplan-passive-unify] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-objectplan-passive-unify-v0" \
  "source_evidence=296x-825,296x-827" \
  "objectplan_canonical_vocabulary_defined=1" \
  "objectplan_struct_name=ObjectPlan" \
  "objectplan_storage_field=ObjectStoragePlan" \
  "objectplan_publication_sites_field=Vec<ObjectPublicationSite>" \
  "objectplan_is_representation_truth=1" \
  "objectplan_is_publication_site_truth=1" \
  "local_first_object_plan_compat_alias_enabled=1" \
  "standalone_publication_plan_enabled=0" \
  "objectplan_execution_enabled=0" \
  "backend_consumes_objectplan=0" \
  "mirbuilder_object_management_enabled=0" \
  "product_default_changed=0" \
  "selected_next=ROUTEPLAN-OBJECTPLAN-HANDOFF-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  "objectplan_is_representation_truth=1" \
  "objectplan_is_publication_site_truth=1" \
  "standalone_publication_plan_enabled=0"; do
  require_line_in_file "$FINAL_SSOT" "$expected"
done

for token in \
  "pub struct ObjectPlan" \
  "pub type LocalFirstObjectPlan = ObjectPlan" \
  "impl ObjectPlan" \
  "pub storage: ObjectStoragePlan" \
  "pub publication_sites: Vec<ObjectPublicationSite>" \
  "(\"objectplan_canonical_vocabulary_defined\", \"1\")" \
  "(\"objectplan_is_representation_truth\", \"1\")" \
  "(\"objectplan_is_publication_site_truth\", \"1\")" \
  "(\"local_first_object_plan_compat_alias_enabled\", \"1\")" \
  "(\"object_plan_execution_enabled\", \"0\")" \
  "(\"standalone_publication_plan_enabled\", \"0\")"; do
  grep -F -q "$token" "$SRC" || {
    echo "[objectplan-passive-unify] missing source token: $token" >&2
    exit 1
  }
done

for stop_line in \
  "do not enable ObjectPlan execution in this row" \
  "do not make backend consume ObjectPlan in this row" \
  "do not split standalone PublicationPlan in this row" \
  "do not remove LocalFirstObjectPlan compatibility alias in this row" \
  "do not move object representation ownership into MIRBuilder"; do
  grep -F -q "$stop_line" "$CARD" || {
    echo "[objectplan-passive-unify] missing stop line: $stop_line" >&2
    exit 1
  }
done

echo "[objectplan-passive-unify] ok"
