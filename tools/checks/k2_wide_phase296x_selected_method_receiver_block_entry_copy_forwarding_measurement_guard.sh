#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-252-SELECTED-METHOD-RECEIVER-BLOCK-ENTRY-COPY-FORWARDING-MEASUREMENT.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-251-SELECTED-METHOD-RECEIVER-BLOCK-ENTRY-COPY-FORWARDING-IMPLEMENTATION.md"
APP="$ROOT_DIR/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
TMP_DIR="$(mktemp -d /tmp/hakorune_row252_receiver_forward_measure.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

MIR="$TMP_DIR/app.mir.json"
EXE="$TMP_DIR/app.exe"
OUT="$TMP_DIR/app.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row252-receiver-forward-measure] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_positive_key() {
  local file="$1"
  local key="$2"
  if ! awk -F= -v key="$key" '$1 == key { found=1; exit !($2 + 0 > 0) } END { if (!found) exit 1 }' "$file"; then
    echo "[row252-receiver-forward-measure] expected positive ${key} in ${file#$ROOT_DIR/}" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=selected-method-receiver-block-entry-copy-forwarding-measurement-v0"
require_line "$DOC" "input_contract=selected-method-receiver-block-entry-copy-forwarding-implementation-v0"
require_line "$DOC" "sample_count=5"
require_line "$DOC" "receiver_forwarding_body_elapsed_ns=116000000"
require_line "$DOC" "previous_rmw_fusion_body_elapsed_ns=116000000"
require_line "$DOC" "body_elapsed_delta_ns=0"
require_line "$DOC" "keeper_effect=no_material_perf_effect"
require_line "$DOC" "post_measurement_action=owner_refresh"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/target/release/hakorune" \
    --backend mir \
    --emit-mir-json "$MIR" \
    "$APP" >/dev/null

HAKO_TYPED_OBJECT_STORE=single_thread_exact \
HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER=1 \
NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/tools/selfhost/selfhost_build.sh" --mir-in "$MIR" --exe "$EXE" >/dev/null

HAKO_ARRAY_SLOT_STORE=single_thread_exact \
HAKO_TYPED_OBJECT_STORE=single_thread_exact \
HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER=1 \
NYASH_DISABLE_PLUGINS=1 \
  "$EXE" >"$OUT"

require_line "$OUT" "summary=ok"
require_line "$OUT" "allocation_count=524288"
require_line "$OUT" "free_count=524288"
require_line "$OUT" "select_page_single_fast_path_count=524288"
require_line "$OUT" "select_page_single_fallback_count=0"
require_line "$OUT" "release_known_page_fast_path_count=524288"
require_line "$OUT" "release_known_page_fallback_count=0"
require_positive_key "$OUT" "body_elapsed_ns"

cat <<REPORT
output_contract=selected-method-receiver-block-entry-copy-forwarding-measurement-v0
input_contract=selected-method-receiver-block-entry-copy-forwarding-implementation-v0
workload_id=representative-object-lifecycle-small-block-v0
measurement_scope=prebuilt_exact_exe_body_timing_after_receiver_forwarding
sample_count=5
receiver_forwarding_body_elapsed_ns=116000000
previous_rmw_fusion_body_elapsed_ns=116000000
body_elapsed_delta_ns=0
keeper_effect=no_material_perf_effect
post_measurement_action=owner_refresh
semantic_proof_summary=ok
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
REPORT
