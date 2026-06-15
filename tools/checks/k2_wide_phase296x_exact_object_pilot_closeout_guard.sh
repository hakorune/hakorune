#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-731-EXACT-OBJECT-PILOT-CLOSEOUT-001.md"
MEASURE_CARD="docs/development/current/main/phases/phase-296x/296x-730-EXACT-OBJECT-PILOT-MEASUREMENT-002.md"
SSOT="docs/development/current/main/design/object-storage-plan-boundary-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
MEASURE_GUARD="tools/checks/k2_wide_phase296x_exact_object_pilot_measurement_002_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_exact_object_pilot_closeout_guard.sh"

[[ -f "$CARD" ]] || { echo "[exact-object-pilot-closeout] missing card: $CARD" >&2; exit 1; }
[[ -f "$MEASURE_CARD" ]] || { echo "[exact-object-pilot-closeout] missing measurement card: $MEASURE_CARD" >&2; exit 1; }
[[ -f "$SSOT" ]] || { echo "[exact-object-pilot-closeout] missing SSOT: $SSOT" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[exact-object-pilot-closeout] row731 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$MEASURE_CARD" || { echo "[exact-object-pilot-closeout] row730 card must be Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[exact-object-pilot-closeout] check index missing closeout guard entry" >&2; exit 1; }
grep -q "$MEASURE_GUARD" "$INDEX" || { echo "[exact-object-pilot-closeout] check index missing measurement guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[exact-object-pilot-closeout] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

for file in "$CARD" "$SSOT"; do
  require_line_in_file "$file" "output_contract=hako-exact-object-pilot-closeout-v0"
  require_line_in_file "$file" "source_evidence=296x-730"
  require_line_in_file "$file" "target_front=object_lifecycle_body"
  require_line_in_file "$file" "object_storage_plan_route_reached=1"
  require_line_in_file "$file" "pilot_exact_object_enabled=1"
  require_line_in_file "$file" "body_elapsed_ratio_before=114.326"
  require_line_in_file "$file" "body_elapsed_ratio_after=117.038"
  require_line_in_file "$file" "winner_claim=0"
  require_line_in_file "$file" "keeper_claim=0"
  require_line_in_file "$file" "global_arc_retirement_claim=0"
  require_line_in_file "$file" "global_host_handle_retirement_claim=0"
  require_line_in_file "$file" "product_default_changed=0"
  require_line_in_file "$file" "mirbuilder_object_management_enabled=0"
  require_line_in_file "$file" "benchmark_name_branch_count=0"
  require_line_in_file "$file" "helper_name_branch_count=0"
  require_line_in_file "$file" "type_abi_execution_truth=0"
  require_line_in_file "$file" "selected_next=MIMALLOC-BODY-TIMING-FRESH-OWNER-SELECTION-002"
  require_line_in_file "$file" "summary=ok"
done

bash "$MEASURE_GUARD"

echo "[exact-object-pilot-closeout] ok"
