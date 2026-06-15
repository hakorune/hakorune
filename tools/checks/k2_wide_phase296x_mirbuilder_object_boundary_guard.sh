#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

CARD="docs/development/current/main/phases/phase-296x/296x-826-MIRBUILDER-OBJECT-BOUNDARY-GUARD-001.md"
INDEX="docs/tools/check-scripts-index.md"
SEARCH_ROOT="src/mir/builder"

require_line_in_file() {
  local file="$1"
  local needle="$2"
  if ! grep -F -q "$needle" "$file"; then
    echo "[mirbuilder-object-boundary] missing '$needle' in $file" >&2
    exit 1
  fi
}

require_no_match() {
  local pattern="$1"
  if rg -n "$pattern" "$SEARCH_ROOT" >/tmp/mirbuilder_object_boundary_rg.out; then
    echo "[mirbuilder-object-boundary] forbidden pattern '$pattern' found under $SEARCH_ROOT" >&2
    cat /tmp/mirbuilder_object_boundary_rg.out >&2
    rm -f /tmp/mirbuilder_object_boundary_rg.out
    exit 1
  fi
  rm -f /tmp/mirbuilder_object_boundary_rg.out
}

for line in \
  "output_contract=hako-mirbuilder-object-boundary-guard-v0" \
  "guard_scope=src/mir/builder" \
  "mirbuilder_object_management_enabled=0" \
  "mirbuilder_object_storage_plan_reference_count=0" \
  "mirbuilder_local_first_object_plan_reference_count=0" \
  "mirbuilder_object_publication_reference_count=0" \
  "mirbuilder_hosthandle_bypass_reference_count=0" \
  "mirbuilder_arc_retirement_reference_count=0" \
  "mirbuilder_arcdynbox_reference_count=0" \
  "mirbuilder_helper_symbol_inference_reference_count=0" \
  "mirbuilder_method_name_special_case_reference_count=0" \
  "mirbuilder_variable_name_special_case_reference_count=0" \
  "product_default_changed=0" \
  "implementation_started=0" \
  "selected_next=SELFHOST-MIR-OBJECT-METADATA-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$line"
done

require_no_match "ObjectStoragePlan"
require_no_match "LocalFirstObjectPlan"
require_no_match "ObjectPublication"
require_no_match "HostHandleEscaped"
require_no_match "ArcDynBox"
require_no_match "hosthandle_bypass"
require_no_match "arc_retirement"
require_no_match "helper_symbol_inference"
require_no_match "method_name_special_case"
require_no_match "variable_name_special_case"

require_line_in_file "$INDEX" "k2_wide_phase296x_mirbuilder_object_boundary_guard.sh"

echo "[mirbuilder-object-boundary] ok"
