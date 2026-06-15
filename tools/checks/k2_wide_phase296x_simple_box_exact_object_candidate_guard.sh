#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-738-SIMPLE-BOX-EXACT-OBJECT-CANDIDATE-001.md"
SOURCE_CARD="docs/development/current/main/phases/phase-296x/296x-731-EXACT-OBJECT-PILOT-CLOSEOUT-001.md"
SSOT="docs/development/current/main/design/record-box-two-surface-one-substrate-ssot.md"
OBJECT_SSOT="docs/development/current/main/design/object-storage-plan-boundary-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_simple_box_exact_object_candidate_guard.sh"
SOURCE_GUARD="tools/checks/k2_wide_phase296x_exact_object_pilot_closeout_guard.sh"

[[ -f "$CARD" ]] || { echo "[simple-box-exact-object-candidate] missing card: $CARD" >&2; exit 1; }
[[ -f "$SOURCE_CARD" ]] || { echo "[simple-box-exact-object-candidate] missing source card: $SOURCE_CARD" >&2; exit 1; }
[[ -f "$SSOT" ]] || { echo "[simple-box-exact-object-candidate] missing SSOT: $SSOT" >&2; exit 1; }
[[ -f "$OBJECT_SSOT" ]] || { echo "[simple-box-exact-object-candidate] missing object SSOT: $OBJECT_SSOT" >&2; exit 1; }

grep -q '^Status: Landed / Parked$' "$CARD" || { echo "[simple-box-exact-object-candidate] row738 card must be Landed / Parked" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[simple-box-exact-object-candidate] check index missing guard entry" >&2; exit 1; }
grep -q "$SOURCE_GUARD" "$INDEX" || { echo "[simple-box-exact-object-candidate] check index missing source guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[simple-box-exact-object-candidate] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-simple-box-exact-object-candidate-v0" \
  "source_evidence=296x-731" \
  "record_box_surface_model=two_surface_one_substrate" \
  "simple_box_exact_object_candidate_allowed=1" \
  "fresh_high_confidence_owner_evidence=0" \
  "implementation_allowed=0" \
  "record_semantics_used_as_box_proof=0" \
  "object_storage_plan_required=1" \
  "routeplan_required=1" \
  "mirbuilder_object_management_enabled=0" \
  "benchmark_name_branch_count=0" \
  "helper_name_branch_count=0" \
  "product_default_changed=0" \
  "selected_next=MIMALLOC-BODY-TIMING-FRESH-OWNER-SELECTION-002" \
  "summary=parked"; do
  require_line_in_file "$CARD" "$expected"
done

grep -F -q "SIMPLE-BOX-EXACT-OBJECT-CANDIDATE-001:" "$SSOT" || {
  echo "[simple-box-exact-object-candidate] SSOT missing task entry" >&2
  exit 1
}
grep -F -q "fresh_high_confidence_owner_evidence=0" "$SSOT" || {
  echo "[simple-box-exact-object-candidate] SSOT missing parked reason" >&2
  exit 1
}

bash "$SOURCE_GUARD"

echo "[simple-box-exact-object-candidate] ok"
