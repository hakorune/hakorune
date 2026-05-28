#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
APP="$ROOT_DIR/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-264-PAGE-MODEL-RELEASE-KNOWN-LIVE-FIELD-TRAFFIC-PROBE.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-263-PAGE-MODEL-HOTPATH-SHAPE-OWNER-SELECTION-AFTER-RESULT-CAPSULE-RESET.md"
TOOL="$ROOT_DIR/tools/allocator/page_model_release_known_live_field_traffic_probe.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row264_release_known_live.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

MIR="$TMP_DIR/app.mir.json"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row264-release-known-live-probe] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=page-model-release-known-live-field-traffic-probe-v0"
require_line "$DOC" "target_method=HakoAllocPageModel.releaseLocalKnownLive/1"
require_line "$DOC" "field_op_count=12"
require_line "$DOC" "copy_count=13"
require_line "$DOC" "array_set_call_count=2"
require_line "$DOC" "array_bridge_field_get_count=2"
require_line "$DOC" "same_block_get_set_count=4"
require_line "$DOC" "rmw_candidate_count=4"
require_line "$DOC" "rmw_single_use_candidate_count=2"
require_line "$DOC" "rmw_multi_use_candidate_count=2"
require_line "$DOC" "receiver_copy_count=3"
require_line "$DOC" "recent_acquire_usize_copy_retry_blocked=1"
require_line "$DOC" "selected_next=page_model_release_known_live_owner_selection"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

if [ ! -x "$ROOT_DIR/target/release/hakorune" ]; then
  cargo build --release --bin hakorune >/tmp/hakorune_row264_hakorune_build.log
fi

NYASH_FEATURES=rune \
NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/target/release/hakorune" \
    --backend mir \
    --emit-mir-json "$MIR" \
    "$APP" >/tmp/hakorune_row264_mir_emit.log

python3 "$TOOL" --mir-json "$MIR" --owner-selection-report "$PREV" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=page-model-release-known-live-field-traffic-probe-v0"
require_line "$REPORT" "input_contract=page-model-hotpath-shape-owner-selection-v0"
require_line "$REPORT" "target_method=HakoAllocPageModel.releaseLocalKnownLive/1"
require_line "$REPORT" "target_method_pct=4.14"
require_line "$REPORT" "block_count=7"
require_line "$REPORT" "field_get_count=7"
require_line "$REPORT" "field_set_count=5"
require_line "$REPORT" "field_op_count=12"
require_line "$REPORT" "copy_count=13"
require_line "$REPORT" "call_count=2"
require_line "$REPORT" "branch_count=2"
require_line "$REPORT" "array_set_call_count=2"
require_line "$REPORT" "array_bridge_field_get_count=2"
require_line "$REPORT" "scalar_counter_field_op_count=10"
require_line "$REPORT" "same_block_get_set_count=4"
require_line "$REPORT" "rmw_candidate_count=4"
require_line "$REPORT" "rmw_single_use_candidate_count=2"
require_line "$REPORT" "rmw_multi_use_candidate_count=2"
require_line "$REPORT" "receiver_copy_count=3"
require_line "$REPORT" "recent_acquire_usize_copy_retry_blocked=1"
require_line "$REPORT" "selected_next=page_model_release_known_live_owner_selection"
require_line "$REPORT" "summary=ok"

cat "$REPORT"
