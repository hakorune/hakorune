#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

CARD="docs/development/current/main/phases/phase-296x/296x-821-LOCAL-KNOWN-RECEIVER-DIRECT-CALL-MEASUREMENT-001.md"
MEASUREMENT_TOOL="tools/allocator/hako_local_known_receiver_direct_call_measurement.py"
INDEX="docs/tools/check-scripts-index.md"

require_line_in_file() {
  local file="$1"
  local needle="$2"
  if ! grep -F -q "$needle" "$file"; then
    echo "[local-known-receiver-direct-call-measurement] missing '$needle' in $file" >&2
    exit 1
  fi
}

python3 -m py_compile "$MEASUREMENT_TOOL"

for line in \
  "output_contract=hako-local-known-receiver-direct-call-measurement-v0" \
  "source_evidence=296x-820" \
  "primary_in_process_repeat=65536" \
  "body_elapsed_ratio=0.836" \
  "hako_not_slower_than_c=1" \
  "measurement_interpretation=current_front_no_longer_hako_slower" \
  "new_backend_lowering_code_added=0" \
  "page_specific_rule_enabled=0" \
  "method_name_special_case_enabled=0" \
  "helper_symbol_inference_enabled=0" \
  "storage_direct_enabled=0" \
  "hosthandle_bypass_enabled=0" \
  "arc_retirement_enabled=0" \
  "product_default_changed=0" \
  "winner_claim=1" \
  "selected_next=LOCAL-KNOWN-RECEIVER-DIRECT-CALL-CLOSEOUT-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$line"
done

require_line_in_file "$CARD" "do not attribute this result to a new code change"
require_line_in_file "$CARD" "do not open storage direct lowering"
require_line_in_file "$INDEX" "k2_wide_phase296x_local_known_receiver_direct_call_measurement_guard.sh"

echo "[local-known-receiver-direct-call-measurement] ok"
