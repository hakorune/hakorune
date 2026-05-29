#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-299-REPRESENTATION-CANDIDATE-INVENTORY.md"
SSOT="$ROOT_DIR/docs/development/current/main/design/representation-direct-lowering-ssot.md"
TOOL="$ROOT_DIR/tools/allocator/representation_candidate_inventory.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row299_representation_inventory.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row299-representation-candidate-inventory] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_positive_key() {
  local file="$1"
  local key="$2"
  if ! awk -F= -v key="$key" '$1 == key { found=1; exit !($2 + 0 > 0) } END { if (!found) exit 1 }' "$file"; then
    echo "[row299-representation-candidate-inventory] $key must be present and positive" >&2
    cat "$file" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$SSOT" "Status: Active"
require_line "$DOC" "output_contract=representation-candidate-inventory-v0"
require_line "$DOC" "input_contract=representation-direct-lowering-ssot-v0"
require_line "$DOC" "candidate_count=3"
require_line "$DOC" "positive_net_candidate_count=2"
require_line "$DOC" "top_positive_net_candidate=typed_object_exact_slot_residence"
require_line "$DOC" "lowest_risk_positive_net_candidate=array_slot_native_direct"
require_line "$DOC" "candidate_0_family=typed_object_exact_slot_residence"
require_line "$DOC" "candidate_0_candidate_representation=ResidentScalar"
require_line "$DOC" "candidate_0_net_helper_delta_positive=1"
require_line "$DOC" "candidate_0_implementation_risk=high"
require_line "$DOC" "candidate_1_family=result_capsule_value_aggregate"
require_line "$DOC" "candidate_1_candidate_representation=ValueAggregate"
require_line "$DOC" "candidate_1_net_helper_delta=0"
require_line "$DOC" "candidate_2_family=array_slot_native_direct"
require_line "$DOC" "candidate_2_candidate_representation=NativeDirect"
require_line "$DOC" "candidate_2_net_helper_delta_positive=1"
require_line "$DOC" "candidate_2_implementation_risk=low"
require_line "$DOC" "first_pilot_selection_required=1"
require_line "$DOC" "selected_next=first_representation_pilot_selection"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

"$TOOL" --out "$REPORT" >/tmp/hakorune_row299_representation_inventory.log

require_line "$REPORT" "output_contract=representation-candidate-inventory-v0"
require_line "$REPORT" "input_contract=representation-direct-lowering-ssot-v0"
require_line "$REPORT" "candidate_count=3"
require_line "$REPORT" "positive_net_candidate_count=2"
require_line "$REPORT" "top_positive_net_candidate=typed_object_exact_slot_residence"
require_line "$REPORT" "lowest_risk_positive_net_candidate=array_slot_native_direct"
require_line "$REPORT" "candidate_0_family=typed_object_exact_slot_residence"
require_line "$REPORT" "candidate_0_candidate_representation=ResidentScalar"
require_line "$REPORT" "candidate_1_family=result_capsule_value_aggregate"
require_line "$REPORT" "candidate_1_candidate_representation=ValueAggregate"
require_line "$REPORT" "candidate_1_net_helper_delta=0"
require_line "$REPORT" "candidate_2_family=array_slot_native_direct"
require_line "$REPORT" "candidate_2_candidate_representation=NativeDirect"
require_line "$REPORT" "first_pilot_selection_required=1"
require_line "$REPORT" "selected_next=first_representation_pilot_selection"
require_line "$REPORT" "implementation_open=0"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

require_positive_key "$REPORT" "candidate_0_net_helper_delta"
require_positive_key "$REPORT" "candidate_2_net_helper_delta"

echo "[row299-representation-candidate-inventory] ok"
