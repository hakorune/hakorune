#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-228-OBJECT-LIFECYCLE-FACADE-FIELD-OWNER-SELECTION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-227-OBJECT-LIFECYCLE-FACADE-EXACT-SLOT-FIELD-INVENTORY.md"
TOOL="$ROOT_DIR/tools/allocator/object_lifecycle_facade_field_owner_selection.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row228_facade_owner.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

INV="$TMP_DIR/inventory.out"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row228-facade-owner] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=object-lifecycle-facade-field-owner-selection-v0"
require_line "$DOC" "input_contract=object-lifecycle-facade-exact-slot-field-inventory-v0"
require_line "$DOC" "same_block_get_set_count=3"
require_line "$DOC" "same_receiver_repeated_get_count=1"
require_line "$DOC" "positive_net_cache_candidate_count=4"
require_line "$DOC" "selected_owner=selected_facade_same_block_get_set_fusion"
require_line "$DOC" "next_diagnostic=selected_facade_same_block_get_set_guard_surface"
require_line "$DOC" "planned_net_helper_call_delta=3"
require_line "$DOC" "planned_net_helper_call_delta_positive=1"
require_line "$DOC" "rejected_owner=generic_typed_field_residence_retry"
require_line "$DOC" "rejected_owner_1=facade_method_local_scalar_cache"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "summary=ok"

cat >"$INV" <<'REPORT'
output_contract=object-lifecycle-facade-exact-slot-field-inventory-v0
input_contract=typed-object-exact-slot-callsite-owner-selection-v0
workload_id=representative-object-lifecycle-small-block-v0
target_family=object_lifecycle_facade
target_family_pct=18.52
facade_method_count=3
facade_exact_slot_get_count=16
facade_exact_slot_set_count=9
facade_exact_slot_field_op_count=25
pattern.same_block_get_set_count=3
pattern.same_receiver_repeated_get_count=1
pattern.positive_net_cache_candidate_count=4
selected_next=facade_field_owner_selection
optimization_open=0
summary=ok
REPORT

python3 "$TOOL" --inventory-report "$INV" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=object-lifecycle-facade-field-owner-selection-v0"
require_line "$REPORT" "input_contract=object-lifecycle-facade-exact-slot-field-inventory-v0"
require_line "$REPORT" "selected_owner=selected_facade_same_block_get_set_fusion"
require_line "$REPORT" "selected_reason=same_block_get_set_candidates_dominate_positive_net_surface"
require_line "$REPORT" "next_diagnostic=selected_facade_same_block_get_set_guard_surface"
require_line "$REPORT" "planned_erased_get_set_helper_calls=6"
require_line "$REPORT" "planned_added_fused_helper_calls=3"
require_line "$REPORT" "planned_net_helper_call_delta=3"
require_line "$REPORT" "planned_net_helper_call_delta_positive=1"
require_line "$REPORT" "rejected_owner=generic_typed_field_residence_retry"
require_line "$REPORT" "rejected_owner_1=facade_method_local_scalar_cache"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"
