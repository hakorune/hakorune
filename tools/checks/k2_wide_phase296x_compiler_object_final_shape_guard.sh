#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

CARD="docs/development/current/main/phases/phase-296x/296x-825-COMPILER-OBJECT-FINAL-SHAPE-001.md"
SSOT="docs/development/current/main/design/compiler-object-final-shape-ssot.md"
INDEX="docs/tools/check-scripts-index.md"

require_line_in_file() {
  local file="$1"
  local needle="$2"
  if ! grep -F -q "$needle" "$file"; then
    echo "[compiler-object-final-shape] missing '$needle' in $file" >&2
    exit 1
  fi
}

for file in "$CARD" "$SSOT"; do
  for line in \
    "compiler_object_final_shape_contract=hako-compiler-object-final-shape-v0" \
    "mirbuilder_object_management_enabled=0" \
    "mirbuilder_records_object_meaning=1" \
    "semantic_refresh_owns_object_facts=1" \
    "box_callable_registry_is_callable_truth=1" \
    "routeplan_is_call_execution_truth=1" \
    "objectplan_is_representation_truth=1" \
    "objectplan_is_publication_site_truth=1" \
    "standalone_publication_plan_enabled=0" \
    "backend_consumes_routeplan_and_objectplan=1" \
    "backend_helper_symbol_inference_enabled=0" \
    "backend_method_name_special_case_enabled=0" \
    "backend_variable_name_special_case_enabled=0" \
    "runtime_generic_box_world_preserved=1" \
    "product_default_changed=0" \
    "selfhost_mirbuilder_metadata_only=1"; do
    require_line_in_file "$file" "$line"
  done
done

require_line_in_file "$CARD" "selected_next=MIRBUILDER-OBJECT-BOUNDARY-GUARD-001"
require_line_in_file "$CARD" "do not move object representation into MIRBuilder"
require_line_in_file "$SSOT" "Standalone"
require_line_in_file "$SSOT" "PublicationPlan"
require_line_in_file "$SSOT" "is allowed later only if ObjectPlan becomes too"
require_line_in_file "$SSOT" "do not lower from helper names"
require_line_in_file "$SSOT" "do not lower from source variable names"
require_line_in_file "$INDEX" "k2_wide_phase296x_compiler_object_final_shape_guard.sh"

echo "[compiler-object-final-shape] ok"
