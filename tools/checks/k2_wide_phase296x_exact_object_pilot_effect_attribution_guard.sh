#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-725-EXACT-OBJECT-PILOT-EFFECT-ATTRIBUTION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-724-EXACT-OBJECT-PILOT-MEASUREMENT-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_exact_object_pilot_effect_attribution_guard.sh"

[[ -f "$CARD" ]] || { echo "[exact-object-effect-attribution] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[exact-object-effect-attribution] missing previous card: $PREV_CARD" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[exact-object-effect-attribution] row725 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[exact-object-effect-attribution] row724 card must be Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[exact-object-effect-attribution] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[exact-object-effect-attribution] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

require_line_in_file "$CARD" "output_contract=hako-exact-object-pilot-effect-attribution-v0"
require_line_in_file "$CARD" "source_evidence=296x-724"
require_line_in_file "$CARD" "target_front=object_lifecycle_body"
require_line_in_file "$CARD" "pilot_exact_object_enabled=1"
require_line_in_file "$CARD" "flattened_nested_route_expected=1"
require_line_in_file "$CARD" "flattened_nested_route_reached=0"
require_line_in_file "$CARD" "generated_ir_contains_synthetic_nested_fields=0"
require_line_in_file "$CARD" "runtime_handle_boundary_removed_for_nested_candidate=0"
require_line_in_file "$CARD" "body_elapsed_ratio_after=114.326"
require_line_in_file "$CARD" "alignment_result_last_requested_count=0"
require_line_in_file "$CARD" "alignment_result_last_normalized_count=0"
require_line_in_file "$CARD" "alignment_result_last_reason_count=0"
require_line_in_file "$CARD" "alignment_result_last_supported_count=0"
require_line_in_file "$CARD" "selected_owner=backend_route_reachability"
require_line_in_file "$CARD" "selected_next=EXACT-OBJECT-FLATTENED-NESTED-FIELD-BACKEND-REACHABILITY-001"
require_line_in_file "$CARD" "implementation_started=0"
require_line_in_file "$CARD" "summary=ok"

echo "[exact-object-effect-attribution] ok"
