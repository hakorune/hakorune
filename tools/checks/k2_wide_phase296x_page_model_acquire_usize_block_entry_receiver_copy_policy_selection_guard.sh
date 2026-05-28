#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-249-PAGE-MODEL-ACQUIRE-USIZE-BLOCK-ENTRY-RECEIVER-COPY-POLICY-SELECTION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-248-PAGE-MODEL-ACQUIRE-USIZE-COPY-MATERIALIZATION-PROBE.md"
TOOL="$ROOT_DIR/tools/allocator/page_model_acquire_usize_block_entry_receiver_copy_policy_selection.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row249_receiver_policy.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

PROBE="$TMP_DIR/probe.out"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row249-receiver-policy] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=page-model-acquire-usize-block-entry-receiver-copy-policy-selection-v0"
require_line "$DOC" "selected_policy=selected_method_receiver_block_entry_copy_forwarding_guard_surface"
require_line "$DOC" "broad_local_ssa_reuse=0"
require_line "$DOC" "cross_block_value_rewrite=0"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "summary=ok"

cat >"$PROBE" <<'REPORT'
output_contract=page-model-acquire-usize-copy-materialization-probe-v0
input_contract=page-model-hotpath-shape-owner-selection-v0
target_method=HakoAllocPageModel.acquire_usize/1
copy_count=31
dominant_copy_position=block_entry
block_entry_copy_count=13
block_entry_receiver_param_copy_count=9
local_ssa_copy_count=0
phi_edge_copy_count=0
selected_next=page_model_acquire_usize_block_entry_receiver_copy_policy_selection
summary=ok
REPORT

python3 "$TOOL" --probe-report "$PROBE" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=page-model-acquire-usize-block-entry-receiver-copy-policy-selection-v0"
require_line "$REPORT" "input_contract=page-model-acquire-usize-copy-materialization-probe-v0"
require_line "$REPORT" "target_method=HakoAllocPageModel.acquire_usize/1"
require_line "$REPORT" "copy_count=31"
require_line "$REPORT" "block_entry_copy_count=13"
require_line "$REPORT" "block_entry_receiver_param_copy_count=9"
require_line "$REPORT" "local_ssa_copy_count=0"
require_line "$REPORT" "phi_edge_copy_count=0"
require_line "$REPORT" "selected_policy=selected_method_receiver_block_entry_copy_forwarding_guard_surface"
require_line "$REPORT" "next_row=selected_method_receiver_block_entry_copy_forwarding_guard_surface"
require_line "$REPORT" "policy_scope=selected_method_only"
require_line "$REPORT" "policy_shape=receiver_param_block_entry_copy_forwarding"
require_line "$REPORT" "broad_local_ssa_reuse=0"
require_line "$REPORT" "cross_block_value_rewrite=0"
require_line "$REPORT" "field_get_result_chain_rewrite=0"
require_line "$REPORT" "implementation_open=0"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"
