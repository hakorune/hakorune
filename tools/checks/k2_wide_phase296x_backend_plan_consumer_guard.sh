#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-831-BACKEND-PLAN-CONSUMER-GUARD-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-830-PUBLICATION-SITE-INVENTORY-GENERIC-001.md"
FINAL_SSOT="docs/development/current/main/design/compiler-object-final-shape-ssot.md"
OBJECT_SRC="src/object_storage_plan.rs"
FLATTENED="src/llvm_py/instructions/flattened_nested_fields.py"
FIELD_ACCESS="src/llvm_py/instructions/field_access.py"
METHOD_CALL="src/llvm_py/instructions/mir_call/method_call.py"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_backend_plan_consumer_guard.sh"

[[ -f "$CARD" ]] || { echo "[backend-plan-consumer] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[backend-plan-consumer] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$FINAL_SSOT" ]] || { echo "[backend-plan-consumer] missing final SSOT: $FINAL_SSOT" >&2; exit 1; }
[[ -f "$OBJECT_SRC" ]] || { echo "[backend-plan-consumer] missing ObjectPlan source: $OBJECT_SRC" >&2; exit 1; }
[[ -f "$FLATTENED" ]] || { echo "[backend-plan-consumer] missing flattened consumer: $FLATTENED" >&2; exit 1; }
[[ -f "$FIELD_ACCESS" ]] || { echo "[backend-plan-consumer] missing field access consumer: $FIELD_ACCESS" >&2; exit 1; }
[[ -f "$METHOD_CALL" ]] || { echo "[backend-plan-consumer] missing method call consumer: $METHOD_CALL" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || {
  echo "[backend-plan-consumer] card must be Landed" >&2
  exit 1
}
grep -q '^Status: Landed$' "$PREV_CARD" || {
  echo "[backend-plan-consumer] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[backend-plan-consumer] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[backend-plan-consumer] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-backend-plan-consumer-guard-v0" \
  "source_evidence=296x-829,296x-830" \
  "backend_plan_consumer_guard_enabled=1" \
  "backend_plan_consumer_requires_routeplan_and_objectplan=1" \
  "backend_existing_flattened_nested_consumer_allowed=1" \
  "backend_new_lowering_enabled=0" \
  "backend_direct_call_without_routeplan_enabled=0" \
  "backend_representation_bypass_without_objectplan_enabled=0" \
  "backend_helper_symbol_inference_enabled=0" \
  "backend_method_name_special_case_enabled=0" \
  "backend_variable_name_special_case_enabled=0" \
  "routeplan_owns_execution_not_representation=1" \
  "objectplan_owns_representation_not_execution=1" \
  "product_default_changed=0" \
  "selected_next=COMPILER-OBJECT-SHAPE-CLOSEOUT-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  "backend_consumes_routeplan_and_objectplan=1" \
  "backend_requires_routeplan_for_direct_call=1" \
  "backend_requires_objectplan_for_representation_bypass=1" \
  "backend_helper_symbol_inference_enabled=0" \
  "backend_method_name_special_case_enabled=0" \
  "backend_variable_name_special_case_enabled=0"; do
  require_line_in_file "$FINAL_SSOT" "$expected"
done

for token in \
  "(\"backend_plan_consumer_guard_enabled\", \"1\")" \
  "\"backend_plan_consumer_requires_routeplan_and_objectplan\"" \
  "(\"backend_existing_flattened_nested_consumer_allowed\", \"1\")" \
  "(\"backend_new_lowering_enabled\", \"0\")" \
  "(\"backend_helper_symbol_inference_enabled\", \"0\")" \
  "(\"backend_method_name_special_case_enabled\", \"0\")" \
  "(\"backend_variable_name_special_case_enabled\", \"0\")"; do
  grep -R -F -q "$token" src/object_storage_plan.rs src/object_storage_plan || {
    echo "[backend-plan-consumer] missing object source token: $token" >&2
    exit 1
  }
done

for token in \
  "validate_flattened_nested_state_plan" \
  "is_flattened_nested_view" \
  "try_lower_nested_method_call" \
  "try_lower_owner_field_get" \
  "try_lower_owner_field_set"; do
  grep -F -q "$token" "$FLATTENED" || {
    echo "[backend-plan-consumer] missing flattened consumer token: $token" >&2
    exit 1
  }
done

grep -F -q "_flattened_nested_field_access_route_enabled" "$FIELD_ACCESS" || {
  echo "[backend-plan-consumer] field access hook missing" >&2
  exit 1
}
grep -F -q "_flattened_nested_method_call_route_enabled" "$METHOD_CALL" || {
  echo "[backend-plan-consumer] method call hook missing" >&2
  exit 1
}

for stop_line in \
  "do not add a new backend lowering path in this row" \
  "do not infer direct call from helper names" \
  "do not infer direct call from method names" \
  "do not infer object representation from variable names" \
  "do not bypass HostHandle without ObjectPlan" \
  "do not bypass RoutePlan for callable target"; do
  grep -F -q "$stop_line" "$CARD" || {
    echo "[backend-plan-consumer] missing stop line: $stop_line" >&2
    exit 1
  }
done

echo "[backend-plan-consumer] ok"
