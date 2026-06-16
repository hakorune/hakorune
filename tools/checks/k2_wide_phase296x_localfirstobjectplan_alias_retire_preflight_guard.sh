#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="localfirstobjectplan-alias-retire-preflight"
CARD="docs/development/current/main/phases/phase-296x/296x-992-LOCALFIRSTOBJECTPLAN-ALIAS-RETIRE-PREFLIGHT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-991-OBJECT-STORAGE-PLAN-VOCAB-AUDIT-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_localfirstobjectplan_alias_retire_preflight_guard.sh"
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
  "output_contract=hako-localfirstobjectplan-alias-retire-preflight-v0" \
  "source_evidence=296x-828,296x-991,worker-rg-audit" \
  "row_kind=preflight" \
  "preflight_input_exact_token_reference_count=27" \
  "live_api_compat_reference_count=5" \
  "historical_doc_reference_count=8" \
  "guard_expectation_reference_count=7" \
  "mirbuilder_forbidden_term_guard_preserved=1" \
  "public_alias_currently_enabled=1" \
  "report_field_currently_enabled=1" \
  "historical_guard_requires_alias_count=3" \
  "immediate_alias_removal_allowed=0" \
  "vocabulary_merge_count=0" \
  "backend_lowering_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "smallest_safe_next=LOCALFIRSTOBJECTPLAN-ALIAS-RETIRE-GUARD-COMPAT-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

grep -F -q "next_task=LOCALFIRSTOBJECTPLAN-ALIAS-RETIRE-PREFLIGHT-001" "$PREV_CARD" || {
  echo "[$TAG] previous card does not hand off to alias retire preflight" >&2
  exit 1
}

if ! grep -R -F -q "pub type LocalFirstObjectPlan = ObjectPlan" "$OBJECT_SRC" "$OBJECT_DIR" \
  && ! grep -R -F -q "(\"local_first_object_plan_alias_retired\", \"1\")" "$OBJECT_SRC" "$OBJECT_DIR"; then
  echo "[$TAG] public compatibility alias is neither present nor explicitly retired" >&2
  exit 1
fi

if grep -R -F -q "pub type LocalFirstObjectPlan = ObjectPlan" "$OBJECT_SRC" "$OBJECT_DIR"; then
  grep -R -F -q "(\"local_first_object_plan_compat_alias_enabled\", \"1\")" "$OBJECT_SRC" "$OBJECT_DIR" || {
    echo "[$TAG] report field for compat alias is missing while alias is present" >&2
    exit 1
  }
fi

if grep -R -F -q "pub type LocalFirstObjectPlan = ObjectPlan" "$OBJECT_SRC" "$OBJECT_DIR" \
  && ! grep -R -F -q "LocalFirstObjectPlan::new" "$OBJECT_SRC" "$OBJECT_DIR"; then
  echo "[$TAG] compatibility alias test coverage is missing while alias is present" >&2
  exit 1
fi

for guard in \
  tools/checks/k2_wide_phase296x_object_plan_local_first_guard.sh \
  tools/checks/k2_wide_phase296x_objectplan_passive_unify_guard.sh \
  tools/checks/k2_wide_phase296x_routeplan_objectplan_handoff_guard.sh; do
  grep -F -q "LocalFirstObjectPlan" "$guard" || {
    echo "[$TAG] historical guard no longer references LocalFirstObjectPlan: $guard" >&2
    exit 1
  }
done

grep -F -q 'require_no_match "LocalFirstObjectPlan"' tools/checks/k2_wide_phase296x_mirbuilder_object_boundary_guard.sh || {
  echo "[$TAG] MIRBuilder forbidden-term guard was not preserved" >&2
  exit 1
}

echo "[$TAG] ok"
