#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-267-PAGE-MODEL-RELEASE-KNOWN-LIVE-SINGLE-USE-RMW-IMPLEMENTATION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-266-PAGE-MODEL-RELEASE-KNOWN-LIVE-SINGLE-USE-RMW-GUARD-SURFACE.md"
LOWER="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_same_module_body_emit.inc"
APP="$ROOT_DIR/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
TMP_DIR="$(mktemp -d /tmp/hakorune_row267_release_known_live_rmw.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

MIR="$TMP_DIR/app.mir.json"
EXE="$TMP_DIR/app.exe"
OUT="$TMP_DIR/app.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row267-release-known-live-rmw-impl] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_contains() {
  local file="$1"
  local expected="$2"
  if ! grep -qF "$expected" "$file"; then
    echo "[row267-release-known-live-rmw-impl] missing text in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=page-model-release-known-live-single-use-rmw-implementation-v0"
require_line "$DOC" "implementation_owner=c_abi_same_module_typed_field_rmw_fusion"
require_line "$DOC" "selected_method=HakoAllocPageModel.releaseLocalKnownLive/1"
require_line "$DOC" "selected_field_0=local_free_count"
require_line "$DOC" "selected_field_1=retire_count"
require_line "$DOC" "existing_helper_symbol=nyash.object.exact_slot_rmw_add_u64_hiii"
require_line "$DOC" "new_runtime_helper_added=0"
require_line "$DOC" "hako_source_change=0"
require_line "$DOC" "semantic_proof_summary=ok"
require_line "$DOC" "planned_net_helper_call_delta=2"
require_line "$DOC" "multi_use_rmw_fused=0"
require_line "$DOC" "array_bridge_fused=0"
require_line "$DOC" "generic_typed_field_residence_open=0"
require_line "$DOC" "generic_cse_open=0"
require_line "$DOC" "summary=ok"

require_contains "$LOWER" "same_module_function_name_is_selected_page_model_rmw_fusion_target"
require_contains "$LOWER" "HakoAllocPageModel.acquire_usize/1"
require_contains "$LOWER" "HakoAllocPageModel.releaseLocalKnownLive/1"
require_contains "$LOWER" "nyash.object.exact_slot_rmw_add_u64_hiii"

bash "$ROOT_DIR/tools/build_hako_llvmc_ffi.sh" >/dev/null
cargo build --release --bin hakorune -p nyash-rust >/dev/null

NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/target/release/hakorune" --backend mir --emit-mir-json "$MIR" "$APP" >/dev/null
python3 "$ROOT_DIR/tools/checks/pure_first_route_preflight.py" "$MIR" >/dev/null

HAKO_TYPED_OBJECT_STORE=single_thread_exact \
HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER=1 \
NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/tools/selfhost/selfhost_build.sh" --mir-in "$MIR" --exe "$EXE" >/dev/null

rmw_symbol_count="$(strings "$EXE" | grep -c 'nyash.object.exact_slot_rmw_add_u64_hiii' || true)"
if [ "$rmw_symbol_count" -lt 1 ]; then
  echo "[row267-release-known-live-rmw-impl] exact-EXE does not reference RMW helper" >&2
  exit 1
fi

HAKO_ARRAY_SLOT_STORE=single_thread_exact \
HAKO_TYPED_OBJECT_STORE=single_thread_exact \
HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER=1 \
NYASH_DISABLE_PLUGINS=1 \
  "$EXE" >"$OUT"

require_contains "$OUT" "summary=ok"

cat <<REPORT
output_contract=page-model-release-known-live-single-use-rmw-implementation-v0
implementation_owner=c_abi_same_module_typed_field_rmw_fusion
selected_method=HakoAllocPageModel.releaseLocalKnownLive/1
exact_exe_rmw_symbol_present=1
semantic_proof_summary=ok
planned_net_helper_call_delta=2
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
REPORT
