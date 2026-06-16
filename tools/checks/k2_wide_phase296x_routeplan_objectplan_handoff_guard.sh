#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-829-ROUTEPLAN-OBJECTPLAN-HANDOFF-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-828-OBJECTPLAN-PASSIVE-UNIFY-001.md"
FINAL_SSOT="docs/development/current/main/design/compiler-object-final-shape-ssot.md"
OBJECT_SSOT="docs/development/current/main/design/object-storage-plan-boundary-ssot.md"
OBJECT_SRC="src/object_storage_plan.rs"
ROUTE_SRC="src/box_callable/route_plan.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_routeplan_objectplan_handoff_guard.sh"

[[ -f "$CARD" ]] || { echo "[routeplan-objectplan-handoff] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[routeplan-objectplan-handoff] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$FINAL_SSOT" ]] || { echo "[routeplan-objectplan-handoff] missing final SSOT: $FINAL_SSOT" >&2; exit 1; }
[[ -f "$OBJECT_SSOT" ]] || { echo "[routeplan-objectplan-handoff] missing ObjectPlan SSOT: $OBJECT_SSOT" >&2; exit 1; }
[[ -f "$OBJECT_SRC" ]] || { echo "[routeplan-objectplan-handoff] missing ObjectPlan source: $OBJECT_SRC" >&2; exit 1; }
[[ -f "$ROUTE_SRC" ]] || { echo "[routeplan-objectplan-handoff] missing RoutePlan source: $ROUTE_SRC" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || {
  echo "[routeplan-objectplan-handoff] card must be Landed" >&2
  exit 1
}
grep -q '^Status: Landed$' "$PREV_CARD" || {
  echo "[routeplan-objectplan-handoff] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[routeplan-objectplan-handoff] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[routeplan-objectplan-handoff] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-routeplan-objectplan-handoff-v0" \
  "source_evidence=296x-825,296x-828" \
  "routeplan_objectplan_handoff_contract_defined=1" \
  "routeplan_owns_execution_not_representation=1" \
  "objectplan_owns_representation_not_execution=1" \
  "backend_requires_routeplan_for_direct_call=1" \
  "backend_requires_objectplan_for_representation_bypass=1" \
  "backend_direct_call_without_routeplan_enabled=0" \
  "backend_representation_bypass_without_objectplan_enabled=0" \
  "backend_helper_symbol_inference_enabled=0" \
  "backend_method_name_special_case_enabled=0" \
  "backend_variable_name_special_case_enabled=0" \
  "objectplan_execution_enabled=0" \
  "routeplan_representation_truth_enabled=0" \
  "standalone_publication_plan_enabled=0" \
  "product_default_changed=0" \
  "selected_next=PUBLICATION-SITE-INVENTORY-GENERIC-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  "routeplan_is_call_execution_truth=1" \
  "objectplan_is_representation_truth=1" \
  "objectplan_is_publication_site_truth=1" \
  "backend_consumes_routeplan_and_objectplan=1"; do
  require_line_in_file "$FINAL_SSOT" "$expected"
done

for token in \
  "pub enum MethodCallRoutePlan" \
  "pub enum NewBoxRoutePlan" \
  "pub enum DropBoxRoutePlan" \
  "//! Semantic route plan vocabulary" \
  "Executable function" \
  "MethodCallRoutePlan::from_target"; do
  grep -F -q "$token" "$ROUTE_SRC" || {
    echo "[routeplan-objectplan-handoff] missing route source token: $token" >&2
    exit 1
  }
done

for token in \
  "pub struct ObjectPlan" \
  "(\"routeplan_objectplan_handoff_contract_defined\", \"1\")" \
  "(\"routeplan_owns_execution_not_representation\", \"1\")" \
  "(\"objectplan_owns_representation_not_execution\", \"1\")" \
  "(\"backend_requires_routeplan_for_direct_call\", \"1\")" \
  "\"backend_requires_objectplan_for_representation_bypass\"" \
  "(\"backend_helper_symbol_inference_enabled\", \"0\")" \
  "(\"backend_method_name_special_case_enabled\", \"0\")" \
  "(\"backend_variable_name_special_case_enabled\", \"0\")"; do
  grep -R -F -q "$token" src/object_storage_plan.rs src/object_storage_plan || {
    echo "[routeplan-objectplan-handoff] missing object source token: $token" >&2
    exit 1
  }
done

if ! grep -R -F -q "pub type LocalFirstObjectPlan = ObjectPlan" src/object_storage_plan.rs src/object_storage_plan \
  && ! grep -R -F -q "(\"local_first_object_plan_alias_retired\", \"1\")" src/object_storage_plan.rs src/object_storage_plan; then
  echo "[routeplan-objectplan-handoff] LocalFirstObjectPlan alias is neither present nor explicitly retired" >&2
  exit 1
fi

for stop_line in \
  "do not enable backend lowering in this row" \
  "do not make RoutePlan own object representation" \
  "do not make ObjectPlan own call execution" \
  "do not bypass HostHandle from RoutePlan alone" \
  "do not direct-call from ObjectPlan alone" \
  "do not infer proof from helper/method/variable names"; do
  grep -F -q "$stop_line" "$CARD" || {
    echo "[routeplan-objectplan-handoff] missing stop line: $stop_line" >&2
    exit 1
  }
done

grep -F -q "ObjectPlan:" "$OBJECT_SSOT" || {
  echo "[routeplan-objectplan-handoff] ObjectPlan boundary missing in Object SSOT" >&2
  exit 1
}
grep -F -q "does not execute, mutate MIR, or replace RoutePlan" "$OBJECT_SSOT" || {
  echo "[routeplan-objectplan-handoff] ObjectPlan non-execution boundary missing" >&2
  exit 1
}

echo "[routeplan-objectplan-handoff] ok"
