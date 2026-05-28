#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-269-PAGE-MODEL-RELEASE-KNOWN-LIVE-SINGLE-USE-RMW-ROLLBACK.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-268-PAGE-MODEL-RELEASE-KNOWN-LIVE-SINGLE-USE-RMW-MEASUREMENT.md"
LOWER="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_same_module_body_emit.inc"
APP="$ROOT_DIR/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
TMP_DIR="$(mktemp -d /tmp/hakorune_row269_release_known_live_rmw_rollback.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

MIR="$TMP_DIR/app.mir.json"
EXE="$TMP_DIR/app.exe"
OUT="$TMP_DIR/app.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row269-release-known-live-rmw-rollback] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_contains() {
  local file="$1"
  local expected="$2"
  if ! grep -qF "$expected" "$file"; then
    echo "[row269-release-known-live-rmw-rollback] missing text in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_absent() {
  local file="$1"
  local unexpected="$2"
  if grep -qF "$unexpected" "$file"; then
    echo "[row269-release-known-live-rmw-rollback] unexpected text in ${file#$ROOT_DIR/}: $unexpected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=page-model-release-known-live-single-use-rmw-rollback-v0"
require_line "$DOC" "input_contract=page-model-release-known-live-single-use-rmw-measurement-v0"
require_line "$DOC" "rollback_reason=keeper_effect_no_effect"
require_line "$DOC" "removed_target=HakoAllocPageModel.releaseLocalKnownLive/1"
require_line "$DOC" "preserved_target=HakoAllocPageModel.acquire_usize/1"
require_line "$DOC" "semantic_proof_summary=ok"
require_line "$DOC" "post_rollback_action=owner_refresh"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

require_contains "$LOWER" "HakoAllocPageModel.acquire_usize/1"
require_absent "$LOWER" "same_module_function_name_is_selected_page_model_rmw_fusion_target"
require_absent "$LOWER" "HakoAllocPageModel.releaseLocalKnownLive/1"

bash "$ROOT_DIR/tools/build_hako_llvmc_ffi.sh" >/dev/null
cargo build --release --bin hakorune -p nyash-rust >/dev/null

NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/target/release/hakorune" --backend mir --emit-mir-json "$MIR" "$APP" >/dev/null
python3 "$ROOT_DIR/tools/checks/pure_first_route_preflight.py" "$MIR" >/dev/null

HAKO_TYPED_OBJECT_STORE=single_thread_exact \
HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER=1 \
NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/tools/selfhost/selfhost_build.sh" --mir-in "$MIR" --exe "$EXE" >/dev/null

HAKO_ARRAY_SLOT_STORE=single_thread_exact \
HAKO_TYPED_OBJECT_STORE=single_thread_exact \
HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER=1 \
NYASH_DISABLE_PLUGINS=1 \
  "$EXE" >"$OUT"

require_contains "$OUT" "summary=ok"

cat <<REPORT
output_contract=page-model-release-known-live-single-use-rmw-rollback-v0
rollback_reason=keeper_effect_no_effect
removed_target=HakoAllocPageModel.releaseLocalKnownLive/1
semantic_proof_summary=ok
post_rollback_action=owner_refresh
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
REPORT
