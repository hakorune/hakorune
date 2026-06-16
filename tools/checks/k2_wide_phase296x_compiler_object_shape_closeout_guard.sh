#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-832-COMPILER-OBJECT-SHAPE-CLOSEOUT-001.md"
FINAL_SSOT="docs/development/current/main/design/compiler-object-final-shape-ssot.md"
OBJECT_SRC="src/object_storage_plan.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_compiler_object_shape_closeout_guard.sh"

[[ -f "$CARD" ]] || { echo "[compiler-object-shape-closeout] missing card: $CARD" >&2; exit 1; }
[[ -f "$FINAL_SSOT" ]] || { echo "[compiler-object-shape-closeout] missing final SSOT: $FINAL_SSOT" >&2; exit 1; }
[[ -f "$OBJECT_SRC" ]] || { echo "[compiler-object-shape-closeout] missing ObjectPlan source: $OBJECT_SRC" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || {
  echo "[compiler-object-shape-closeout] card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[compiler-object-shape-closeout] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[compiler-object-shape-closeout] missing line in $file: $expected" >&2
    exit 1
  fi
}

for card in \
  docs/development/current/main/phases/phase-296x/296x-825-COMPILER-OBJECT-FINAL-SHAPE-001.md \
  docs/development/current/main/phases/phase-296x/296x-826-MIRBUILDER-OBJECT-BOUNDARY-GUARD-001.md \
  docs/development/current/main/phases/phase-296x/296x-827-SELFHOST-MIR-OBJECT-METADATA-001.md \
  docs/development/current/main/phases/phase-296x/296x-828-OBJECTPLAN-PASSIVE-UNIFY-001.md \
  docs/development/current/main/phases/phase-296x/296x-829-ROUTEPLAN-OBJECTPLAN-HANDOFF-001.md \
  docs/development/current/main/phases/phase-296x/296x-830-PUBLICATION-SITE-INVENTORY-GENERIC-001.md \
  docs/development/current/main/phases/phase-296x/296x-831-BACKEND-PLAN-CONSUMER-GUARD-001.md; do
  [[ -f "$card" ]] || {
    echo "[compiler-object-shape-closeout] missing source evidence card: $card" >&2
    exit 1
  }
  grep -q '^Status: Landed$' "$card" || {
    echo "[compiler-object-shape-closeout] source evidence card not Landed: $card" >&2
    exit 1
  }
done

for script in \
  tools/checks/k2_wide_phase296x_compiler_object_final_shape_guard.sh \
  tools/checks/k2_wide_phase296x_mirbuilder_object_boundary_guard.sh \
  tools/checks/k2_wide_phase296x_selfhost_mir_object_metadata_guard.sh \
  tools/checks/k2_wide_phase296x_objectplan_passive_unify_guard.sh \
  tools/checks/k2_wide_phase296x_routeplan_objectplan_handoff_guard.sh \
  tools/checks/k2_wide_phase296x_publication_site_generic_inventory_guard.sh \
  tools/checks/k2_wide_phase296x_backend_plan_consumer_guard.sh; do
  [[ -x "$script" ]] || {
    echo "[compiler-object-shape-closeout] missing executable guard: $script" >&2
    exit 1
  }
  grep -q "$script" "$INDEX" || {
    echo "[compiler-object-shape-closeout] check index missing source guard: $script" >&2
    exit 1
  }
  bash "$script"
done

for expected in \
  "output_contract=hako-compiler-object-shape-closeout-v0" \
  "source_evidence=296x-825,296x-826,296x-827,296x-828,296x-829,296x-830,296x-831" \
  "compiler_object_shape_closeout=1" \
  "compiler_object_final_shape_contract=hako-compiler-object-final-shape-v0" \
  "mirbuilder_object_management_enabled=0" \
  "selfhost_mirbuilder_metadata_only=1" \
  "objectplan_canonical_vocabulary_defined=1" \
  "objectplan_is_representation_truth=1" \
  "objectplan_is_publication_site_truth=1" \
  "routeplan_objectplan_handoff_contract_defined=1" \
  "publication_site_generic_inventory_defined=1" \
  "backend_plan_consumer_guard_enabled=1" \
  "backend_helper_symbol_inference_enabled=0" \
  "backend_method_name_special_case_enabled=0" \
  "backend_variable_name_special_case_enabled=0" \
  "standalone_publication_plan_enabled=0" \
  "product_default_changed=0" \
  "implementation_gap_count=0" \
  "selected_next=MIMALLOC-FRESH-FRONT-SELECTION-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  "compiler_object_final_shape_contract=hako-compiler-object-final-shape-v0" \
  "mirbuilder_object_management_enabled=0" \
  "selfhost_mirbuilder_metadata_only=1" \
  "objectplan_is_representation_truth=1" \
  "objectplan_is_publication_site_truth=1" \
  "backend_plan_consumer_guard_enabled=1" \
  "backend_helper_symbol_inference_enabled=0" \
  "backend_method_name_special_case_enabled=0" \
  "backend_variable_name_special_case_enabled=0"; do
  require_line_in_file "$FINAL_SSOT" "$expected"
done

for token in \
  "(\"objectplan_canonical_vocabulary_defined\", \"1\")" \
  "(\"publication_site_generic_inventory_defined\", \"1\")" \
  "(\"backend_plan_consumer_guard_enabled\", \"1\")" \
  "(\"object_plan_execution_enabled\", \"0\")" \
  "(\"standalone_publication_plan_enabled\", \"0\")"; do
  grep -R -F -q "$token" src/object_storage_plan.rs src/object_storage_plan || {
    echo "[compiler-object-shape-closeout] missing ObjectPlan source token: $token" >&2
    exit 1
  }
done

for stop_line in \
  "do not resume object-shape implementation without a fresh owner" \
  "do not move object management into MIRBuilder" \
  "do not let selfhost MIRBuilder emit representation/publication truth" \
  "do not let backend lower from helper/method/variable names" \
  "do not split standalone PublicationPlan until ObjectPlan becomes too large" \
  "do not claim product runtime behavior changed"; do
  grep -F -q "$stop_line" "$CARD" || {
    echo "[compiler-object-shape-closeout] missing stop line: $stop_line" >&2
    exit 1
  }
done

echo "[compiler-object-shape-closeout] ok"
