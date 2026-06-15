#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-711-OBJECT-STORAGE-PLAN-SSOT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-710-OBJECT-BOUNDARY-INVENTORY-001.md"
NEXT_CARD="docs/development/current/main/phases/phase-296x/296x-712-EXACT-OBJECT-PLAN-SHADOW-001.md"
SSOT="docs/development/current/main/design/object-storage-plan-boundary-ssot.md"
SRC="src/object_storage_plan.rs"
LIB="src/lib.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_object_storage_plan_ssot_guard.sh"

[[ -f "$CARD" ]] || { echo "[object-storage-plan-ssot] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[object-storage-plan-ssot] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$NEXT_CARD" ]] || { echo "[object-storage-plan-ssot] missing next card: $NEXT_CARD" >&2; exit 1; }
[[ -f "$SRC" ]] || { echo "[object-storage-plan-ssot] missing source: $SRC" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[object-storage-plan-ssot] row711 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[object-storage-plan-ssot] row710 card must be Landed" >&2; exit 1; }
grep -Eq '^Status: (Active|Landed)$' "$NEXT_CARD" || { echo "[object-storage-plan-ssot] row712 card must be Active or Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[object-storage-plan-ssot] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[object-storage-plan-ssot] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

require_line_in_file "$CARD" "output_contract=hako-object-storage-plan-ssot-v0"
require_line_in_file "$CARD" "source_evidence=296x-710"
require_line_in_file "$CARD" "mirbuilder_object_management_enabled=0"
require_line_in_file "$CARD" "box_callable_registry_is_callable_truth=1"
require_line_in_file "$CARD" "routeplan_is_call_execution_truth=1"
require_line_in_file "$CARD" "object_storage_plan_is_representation_truth=1"
require_line_in_file "$CARD" "object_storage_plan_vocabulary_defined=1"
require_line_in_file "$CARD" "object_storage_plan_execution_enabled=0"
require_line_in_file "$CARD" "exact_object_shadow_ready=1"
require_line_in_file "$CARD" "summary=ok"

require_line_in_file "$SSOT" "object_storage_plan_is_representation_truth=1"
grep -q 'pub enum ObjectStoragePlan' "$SRC" || { echo "[object-storage-plan-ssot] missing ObjectStoragePlan enum" >&2; exit 1; }
grep -q 'ExactStackObject' "$SRC" || { echo "[object-storage-plan-ssot] missing ExactStackObject" >&2; exit 1; }
grep -q 'ExactNativeStruct' "$SRC" || { echo "[object-storage-plan-ssot] missing ExactNativeStruct" >&2; exit 1; }
grep -q 'Scalarized' "$SRC" || { echo "[object-storage-plan-ssot] missing Scalarized" >&2; exit 1; }
grep -q 'object_storage_plan_execution_enabled", "0"' "$SRC" || { echo "[object-storage-plan-ssot] execution must remain disabled" >&2; exit 1; }
grep -q '^pub mod object_storage_plan;' "$LIB" || { echo "[object-storage-plan-ssot] lib export missing" >&2; exit 1; }
require_line_in_file "$NEXT_CARD" "Task: EXACT-OBJECT-PLAN-SHADOW-001"

echo "[object-storage-plan-ssot] ok"
