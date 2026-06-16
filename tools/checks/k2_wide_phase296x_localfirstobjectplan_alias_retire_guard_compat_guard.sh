#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="localfirstobjectplan-alias-retire-guard-compat"
CARD="docs/development/current/main/phases/phase-296x/296x-993-LOCALFIRSTOBJECTPLAN-ALIAS-RETIRE-GUARD-COMPAT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-992-LOCALFIRSTOBJECTPLAN-ALIAS-RETIRE-PREFLIGHT-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_localfirstobjectplan_alias_retire_guard_compat_guard.sh"

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
  "output_contract=hako-localfirstobjectplan-alias-retire-guard-compat-v0" \
  "source_evidence=296x-828,296x-991,296x-992" \
  "row_kind=guard_compat" \
  "historical_guards_tolerate_alias_retire=1" \
  "alias_present_or_retired_marker_required=1" \
  "objectplan_canonical_name_required=1" \
  "public_alias_currently_enabled=1" \
  "alias_removed=0" \
  "report_field_changed=0" \
  "vocabulary_merge_count=0" \
  "backend_lowering_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "smallest_safe_next=LOCALFIRSTOBJECTPLAN-ALIAS-RETIRE-IMPLEMENTATION-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

grep -R -F -q "pub struct ObjectPlan" src/object_storage_plan.rs src/object_storage_plan || {
  echo "[$TAG] canonical ObjectPlan source is missing" >&2
  exit 1
}

grep -R -F -q "pub type LocalFirstObjectPlan = ObjectPlan" src/object_storage_plan.rs src/object_storage_plan || {
  echo "[$TAG] alias must still be present in guard-compat row" >&2
  exit 1
}

grep -F -q "local_first_object_plan_alias_retired" tools/checks/k2_wide_phase296x_objectplan_passive_unify_guard.sh || {
  echo "[$TAG] passive unify guard does not tolerate future retired marker" >&2
  exit 1
}

grep -F -q "local_first_object_plan_alias_retired" tools/checks/k2_wide_phase296x_routeplan_objectplan_handoff_guard.sh || {
  echo "[$TAG] route/object handoff guard does not tolerate future retired marker" >&2
  exit 1
}

grep -F -q "pub struct ObjectPlan" tools/checks/k2_wide_phase296x_object_plan_local_first_guard.sh || {
  echo "[$TAG] local-first guard does not accept canonical ObjectPlan" >&2
  exit 1
}

grep -F -q "local_first_object_plan_alias_retired" tools/checks/k2_wide_phase296x_localfirstobjectplan_alias_retire_preflight_guard.sh || {
  echo "[$TAG] preflight guard does not tolerate future retired marker" >&2
  exit 1
}

echo "[$TAG] ok"
