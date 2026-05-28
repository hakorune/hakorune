#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
APP="$ROOT_DIR/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-250-SELECTED-METHOD-RECEIVER-BLOCK-ENTRY-COPY-FORWARDING-GUARD-SURFACE.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-249-PAGE-MODEL-ACQUIRE-USIZE-BLOCK-ENTRY-RECEIVER-COPY-POLICY-SELECTION.md"
TOOL="$ROOT_DIR/tools/allocator/selected_method_receiver_block_entry_copy_forwarding_guard_surface.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row250_receiver_guard.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

POLICY="$TMP_DIR/policy.out"
MIR="$TMP_DIR/app.mir.json"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row250-receiver-guard] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=selected-method-receiver-block-entry-copy-forwarding-guard-surface-v0"
require_line "$DOC" "candidate_count=9"
require_line "$DOC" "field_get_receiver_candidate_count=8"
require_line "$DOC" "field_set_receiver_candidate_count=1"
require_line "$DOC" "exclude_call_adjacent_receiver_copy=1"
require_line "$DOC" "exclude_cross_block_rewrite=1"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "summary=ok"

cat >"$POLICY" <<'REPORT'
output_contract=page-model-acquire-usize-block-entry-receiver-copy-policy-selection-v0
input_contract=page-model-acquire-usize-copy-materialization-probe-v0
target_method=HakoAllocPageModel.acquire_usize/1
selected_policy=selected_method_receiver_block_entry_copy_forwarding_guard_surface
summary=ok
REPORT

if [ ! -x "$ROOT_DIR/target/release/hakorune" ]; then
  cargo build --release --bin hakorune >/tmp/hakorune_row250_hakorune_build.log
fi

NYASH_FEATURES=rune \
NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/target/release/hakorune" \
    --backend mir \
    --emit-mir-json "$MIR" \
    "$APP" >/tmp/hakorune_row250_mir_emit.log

python3 "$TOOL" --mir-json "$MIR" --policy-report "$POLICY" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=selected-method-receiver-block-entry-copy-forwarding-guard-surface-v0"
require_line "$REPORT" "input_contract=page-model-acquire-usize-block-entry-receiver-copy-policy-selection-v0"
require_line "$REPORT" "target_method=HakoAllocPageModel.acquire_usize/1"
require_line "$REPORT" "candidate_count=9"
require_line "$REPORT" "field_get_receiver_candidate_count=8"
require_line "$REPORT" "field_set_receiver_candidate_count=1"
require_line "$REPORT" "receiver_source_value=0"
require_line "$REPORT" "candidate_position=block_entry"
require_line "$REPORT" "candidate_scope=selected_method_only"
require_line "$REPORT" "exclude_call_adjacent_receiver_copy=1"
require_line "$REPORT" "exclude_non_receiver_param_copy=1"
require_line "$REPORT" "exclude_cross_block_rewrite=1"
require_line "$REPORT" "implementation_open=0"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "candidate_0_block=block_34"
require_line "$REPORT" "candidate_7_block=block_45"
require_line "$REPORT" "candidate_8_sink=field_set_receiver"
require_line "$REPORT" "selected_next=selected_method_receiver_block_entry_copy_forwarding_implementation"
require_line "$REPORT" "summary=ok"

cat "$REPORT"
