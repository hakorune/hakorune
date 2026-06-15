#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

CARD="docs/development/current/main/phases/phase-296x/296x-824-MIMALLOC-CURRENT-FRONT-OPTIMIZATION-PAUSE-CHECKPOINT-001.md"
TOOL="tools/allocator/hako_mimalloc_current_front_pause_checkpoint.py"
INDEX="docs/tools/check-scripts-index.md"

require_line_in_file() {
  local file="$1"
  local needle="$2"
  if ! grep -F -q "$needle" "$file"; then
    echo "[current-front-pause-checkpoint] missing '$needle' in $file" >&2
    exit 1
  fi
}

python3 -m py_compile "$TOOL"

TMP_REPORT="$(mktemp)"
TMP_OUT="$(mktemp)"
trap 'rm -f "$TMP_REPORT" "$TMP_OUT"' EXIT

cat > "$TMP_REPORT" <<'REPORT'
output_contract=hako-mimalloc-next-owner-after-local-known-receiver-closeout-v0
current_body_elapsed_ratio=0.836
hako_slower_current_front=0
selected_next_owner=none_current_front_not_hako_slower
implementation_started=0
new_backend_lowering_code_added=0
storage_direct_enabled=0
hosthandle_bypass_enabled=0
arc_retirement_enabled=0
product_default_changed=0
summary=ok
REPORT

python3 "$TOOL" --owner-selection-report "$TMP_REPORT" --out "$TMP_OUT"

for line in \
  "output_contract=hako-mimalloc-current-front-optimization-pause-checkpoint-v0" \
  "source_evidence=296x-823,296x-822,296x-821" \
  "body_elapsed_ratio=0.836" \
  "current_front_paused=1" \
  "pause_reason=current_front_not_hako_slower" \
  "local_known_receiver_direct_call_lane_closed=1" \
  "implementation_owner_selected=0" \
  "implementation_started=0" \
  "new_backend_lowering_code_added=0" \
  "storage_direct_enabled=0" \
  "hosthandle_bypass_enabled=0" \
  "arc_retirement_enabled=0" \
  "product_default_changed=0" \
  "fresh_front_selection_allowed=1" \
  "remeasure_if_environment_changes=1" \
  "no_current_front_patch_without_new_evidence=1" \
  "selected_next=MIMALLOC-FRESH-FRONT-SELECTION-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$line"
  require_line_in_file "$TMP_OUT" "$line"
done

require_line_in_file "$CARD" "do not patch the current front without new Hako-slower evidence"
require_line_in_file "$INDEX" "hako_mimalloc_current_front_pause_checkpoint.py"
require_line_in_file "$INDEX" "k2_wide_phase296x_current_front_pause_checkpoint_guard.sh"

echo "[current-front-pause-checkpoint] ok"
