#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

CARD="docs/development/current/main/phases/phase-296x/296x-823-MIMALLOC-BODY-TIMING-NEXT-OWNER-SELECTION-AFTER-LOCAL-KNOWN-RECEIVER-CLOSEOUT-001.md"
TOOL="tools/allocator/hako_mimalloc_next_owner_after_local_known_receiver_closeout.py"
INDEX="docs/tools/check-scripts-index.md"

require_line_in_file() {
  local file="$1"
  local needle="$2"
  if ! grep -F -q "$needle" "$file"; then
    echo "[next-owner-after-local-known-receiver-closeout] missing '$needle' in $file" >&2
    exit 1
  fi
}

python3 -m py_compile "$TOOL"

TMP_REPORT="$(mktemp)"
TMP_OUT="$(mktemp)"
trap 'rm -f "$TMP_REPORT" "$TMP_OUT"' EXIT

cat > "$TMP_REPORT" <<'REPORT'
output_contract=hako-local-known-receiver-direct-call-closeout-v0
lane_closed=1
body_elapsed_ratio=0.836
new_speedup_claim=0
new_backend_lowering_code_added=0
storage_direct_enabled=0
hosthandle_bypass_enabled=0
arc_retirement_enabled=0
product_default_changed=0
summary=ok
REPORT

python3 "$TOOL" --closeout-report "$TMP_REPORT" --out "$TMP_OUT"

for line in \
  "output_contract=hako-mimalloc-next-owner-after-local-known-receiver-closeout-v0" \
  "source_evidence=296x-822,296x-821" \
  "local_known_receiver_direct_call_lane_closed=1" \
  "current_body_elapsed_ratio=0.836" \
  "hako_slower_current_front=0" \
  "current_front_winner_from_previous=1" \
  "selected_next_owner=none_current_front_not_hako_slower" \
  "selected_owner_confidence=high" \
  "implementation_started=0" \
  "new_backend_lowering_code_added=0" \
  "storage_direct_enabled=0" \
  "hosthandle_bypass_enabled=0" \
  "arc_retirement_enabled=0" \
  "product_default_changed=0" \
  "startup_lane_reopened=0" \
  "source_hako_changed=0" \
  "winner_claim=0" \
  "next_task=MIMALLOC-CURRENT-FRONT-OPTIMIZATION-PAUSE-CHECKPOINT-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$line"
  require_line_in_file "$TMP_OUT" "$line"
done

require_line_in_file "$CARD" "do not select implementation owner while hako_slower_current_front=0"
require_line_in_file "$INDEX" "hako_mimalloc_next_owner_after_local_known_receiver_closeout.py"
require_line_in_file "$INDEX" "k2_wide_phase296x_next_owner_after_local_known_receiver_closeout_guard.sh"

echo "[next-owner-after-local-known-receiver-closeout] ok"
