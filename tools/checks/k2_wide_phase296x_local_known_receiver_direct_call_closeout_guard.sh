#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

CARD="docs/development/current/main/phases/phase-296x/296x-822-LOCAL-KNOWN-RECEIVER-DIRECT-CALL-CLOSEOUT-001.md"
CLOSEOUT_TOOL="tools/allocator/hako_local_known_receiver_direct_call_closeout.py"
INDEX="docs/tools/check-scripts-index.md"

require_line_in_file() {
  local file="$1"
  local needle="$2"
  if ! grep -F -q "$needle" "$file"; then
    echo "[local-known-receiver-direct-call-closeout] missing '$needle' in $file" >&2
    exit 1
  fi
}

python3 -m py_compile "$CLOSEOUT_TOOL"

TMP_REPORT="$(mktemp)"
TMP_OUT="$(mktemp)"
trap 'rm -f "$TMP_REPORT" "$TMP_OUT"' EXIT

cat > "$TMP_REPORT" <<'REPORT'
output_contract=hako-local-known-receiver-direct-call-measurement-v0
hako_body_elapsed_ns=24000000
c_body_elapsed_ns=28710275
body_elapsed_ratio=0.836
hako_not_slower_than_c=1
measurement_interpretation=current_front_no_longer_hako_slower
new_backend_lowering_code_added=0
page_specific_rule_enabled=0
method_name_special_case_enabled=0
helper_symbol_inference_enabled=0
storage_direct_enabled=0
hosthandle_bypass_enabled=0
arc_retirement_enabled=0
product_default_changed=0
winner_claim=1
summary=ok
REPORT

python3 "$CLOSEOUT_TOOL" --measurement-report "$TMP_REPORT" --out "$TMP_OUT"

for line in \
  "output_contract=hako-local-known-receiver-direct-call-closeout-v0" \
  "source_evidence=296x-821,296x-820,296x-819" \
  "closed_lane=local_known_receiver_direct_call" \
  "lane_closed=1" \
  "closeout_reason=current_front_no_longer_hako_slower_and_no_new_lowering_needed" \
  "body_elapsed_ratio=0.836" \
  "winner_claim=1" \
  "winner_claim_source=current_front_measurement" \
  "new_speedup_claim=0" \
  "new_backend_lowering_code_added=0" \
  "page_specific_rule_enabled=0" \
  "method_name_special_case_enabled=0" \
  "helper_symbol_inference_enabled=0" \
  "storage_direct_enabled=0" \
  "hosthandle_bypass_enabled=0" \
  "arc_retirement_enabled=0" \
  "product_default_changed=0" \
  "next_owner_selection_required=1" \
  "selected_next=MIMALLOC-BODY-TIMING-NEXT-OWNER-SELECTION-AFTER-LOCAL-KNOWN-RECEIVER-CLOSEOUT-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$line"
  require_line_in_file "$TMP_OUT" "$line"
done

require_line_in_file "$CARD" "do not attribute the measurement to a new code change"
require_line_in_file "$CARD" "do not open storage direct lowering from this lane"
require_line_in_file "$INDEX" "hako_local_known_receiver_direct_call_closeout.py"
require_line_in_file "$INDEX" "k2_wide_phase296x_local_known_receiver_direct_call_closeout_guard.sh"

echo "[local-known-receiver-direct-call-closeout] ok"
