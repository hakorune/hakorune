#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

CARD="docs/development/current/main/phases/phase-296x/296x-820-LOCAL-KNOWN-RECEIVER-DIRECT-CALL-PILOT-001.md"
CANDIDATE_TOOL="tools/allocator/hako_local_page_receiver_candidate_probe.py"
SHADOW_TOOL="tools/allocator/hako_local_known_receiver_direct_call_shadow.py"
PILOT_TOOL="tools/allocator/hako_local_known_receiver_direct_call_pilot.py"
INDEX="docs/tools/check-scripts-index.md"

require_line_in_file() {
  local file="$1"
  local needle="$2"
  if ! grep -F -q "$needle" "$file"; then
    echo "[local-known-receiver-direct-call-pilot] missing '$needle' in $file" >&2
    exit 1
  fi
}

require_line_in_report() {
  local report="$1"
  local needle="$2"
  if ! grep -F -q "$needle" "$report"; then
    echo "[local-known-receiver-direct-call-pilot] report missing '$needle'" >&2
    cat "$report" >&2
    exit 1
  fi
}

tmp_dir="$(mktemp -d /tmp/hako_local_known_receiver_direct_call_pilot.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

candidate_report="$tmp_dir/candidate.out"
shadow_report="$tmp_dir/shadow.out"
pilot_report="$tmp_dir/pilot.out"

python3 -m py_compile "$CANDIDATE_TOOL" "$SHADOW_TOOL" "$PILOT_TOOL"
python3 "$CANDIDATE_TOOL" --out "$candidate_report"
python3 "$SHADOW_TOOL" --probe-report "$candidate_report" --out "$shadow_report"
python3 "$PILOT_TOOL" \
  --candidate-report "$candidate_report" \
  --shadow-report "$shadow_report" \
  --out "$pilot_report"

for line in \
  "output_contract=hako-local-known-receiver-direct-call-pilot-v0" \
  "pilot_status=already_satisfied_existing_generic_route" \
  "generic_routeplan_backend_seam_ready=1" \
  "c_shim_user_box_method_route_consumer=1" \
  "c_shim_reads_user_box_method_routes=1" \
  "c_shim_emits_target_symbol_call=1" \
  "routeplan_direct_target_predicate_present=1" \
  "routeplan_same_module_definition_required=1" \
  "objectplan_pre_publication_shadow_used=1" \
  "routeplan_backend_consumable_proof_used=1" \
  "new_backend_lowering_code_added=0" \
  "page_specific_rule_enabled=0" \
  "method_name_special_case_enabled=0" \
  "helper_symbol_inference_enabled=0" \
  "storage_direct_enabled=0" \
  "hosthandle_bypass_enabled=0" \
  "arc_retirement_enabled=0" \
  "product_default_changed=0" \
  "measurement_required=1" \
  "next_task=LOCAL-KNOWN-RECEIVER-DIRECT-CALL-MEASUREMENT-001" \
  "summary=ok"; do
  require_line_in_report "$pilot_report" "$line"
  require_line_in_file "$CARD" "$line"
done

require_line_in_file "$CARD" "do not add a page receiver-name branch"
require_line_in_file "$CARD" "do not special-case acquire_usize or reuse"
require_line_in_file "$CARD" "do not infer direct calls from helper symbols"
require_line_in_file "$INDEX" "k2_wide_phase296x_local_known_receiver_direct_call_pilot_guard.sh"

echo "[local-known-receiver-direct-call-pilot] ok"
