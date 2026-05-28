#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
APP="$ROOT_DIR/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-248-PAGE-MODEL-ACQUIRE-USIZE-COPY-MATERIALIZATION-PROBE.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-247-PAGE-MODEL-HOTPATH-SHAPE-OWNER-SELECTION.md"
TOOL="$ROOT_DIR/tools/allocator/page_model_acquire_usize_copy_materialization_probe.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row248_acquire_copy.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

SELECTION="$TMP_DIR/selection.out"
MIR="$TMP_DIR/app.mir.json"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row248-acquire-copy] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=page-model-acquire-usize-copy-materialization-probe-v0"
require_line "$DOC" "dominant_copy_position=block_entry"
require_line "$DOC" "block_entry_receiver_param_copy_count=9"
require_line "$DOC" "recent_broad_local_ssa_nonkeeper_guard=1"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "summary=ok"

cat >"$SELECTION" <<'REPORT'
output_contract=page-model-hotpath-shape-owner-selection-v0
input_contract=page-model-hotpath-ir-shape-diff-inventory-v0
workload_id=representative-object-lifecycle-small-block-v0
selected_method=HakoAllocPageModel.acquire_usize/1
selected_method_shape_owner=copy_materialization
selected_owner=page_model_acquire_usize_copy_materialization_probe
summary=ok
REPORT

if [ ! -x "$ROOT_DIR/target/release/hakorune" ]; then
  cargo build --release --bin hakorune >/tmp/hakorune_row248_hakorune_build.log
fi

NYASH_FEATURES=rune \
NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/target/release/hakorune" \
    --backend mir \
    --emit-mir-json "$MIR" \
    "$APP" >/tmp/hakorune_row248_mir_emit.log

python3 "$TOOL" --mir-json "$MIR" --owner-selection-report "$SELECTION" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=page-model-acquire-usize-copy-materialization-probe-v0"
require_line "$REPORT" "input_contract=page-model-hotpath-shape-owner-selection-v0"
require_line "$REPORT" "target_method=HakoAllocPageModel.acquire_usize/1"
require_line "$REPORT" "block_count=12"
require_line "$REPORT" "copy_count=31"
require_line "$REPORT" "dominant_copy_position=block_entry"
require_line "$REPORT" "block_entry_copy_count=13"
require_line "$REPORT" "block_entry_receiver_param_copy_count=9"
require_line "$REPORT" "block_entry_requested_size_param_copy_count=1"
require_line "$REPORT" "block_entry_derived_value_copy_count=3"
require_line "$REPORT" "call_adjacent_copy_count=7"
require_line "$REPORT" "expression_materialization_copy_count=5"
require_line "$REPORT" "expression_param_copy_count=1"
require_line "$REPORT" "branch_condition_copy_count=4"
require_line "$REPORT" "field_set_value_copy_count=2"
require_line "$REPORT" "local_ssa_copy_count=0"
require_line "$REPORT" "phi_edge_copy_count=0"
require_line "$REPORT" "recent_broad_local_ssa_nonkeeper_guard=1"
require_line "$REPORT" "implementation_open=0"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "selected_next=page_model_acquire_usize_block_entry_receiver_copy_policy_selection"
require_line "$REPORT" "summary=ok"

cat "$REPORT"
