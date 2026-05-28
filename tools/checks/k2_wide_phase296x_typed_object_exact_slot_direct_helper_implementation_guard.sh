#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-215-TYPED-OBJECT-EXACT-SLOT-DIRECT-HELPER-IMPLEMENTATION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-214-TYPED-OBJECT-EXACT-SLOT-DIRECT-HELPER-SELECTION.md"
APP="$ROOT_DIR/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
TMP_DIR="$(mktemp -d /tmp/hakorune_row215_exact_slot_impl.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

MIR="$TMP_DIR/app.mir.json"
EXE_ON="$TMP_DIR/app_on.exe"
EXE_OFF="$TMP_DIR/app_off.exe"
ON_OUT="$TMP_DIR/on.out"
OFF_OUT="$TMP_DIR/off.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row215-exact-slot-impl] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_contains() {
  local file="$1"
  local expected="$2"
  if ! grep -q "$expected" "$file"; then
    echo "[row215-exact-slot-impl] missing content in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=typed-object-exact-slot-direct-helper-implementation-v0"
require_line "$DOC" "default_helper_abi_unchanged=1"
require_line "$DOC" "generic_helper_codepath_unchanged=1"
require_line "$DOC" "new_symbol_count=6"
require_line "$DOC" "runtime_helper_env_check=0"
require_line "$DOC" "runtime_helper_safe_mutex_fallback=0"
require_line "$DOC" "default_exact_helper_emission=0"
require_line "$DOC" "safe_mutex_default_smoke=ok"
require_line "$DOC" "single_thread_exact_direct_helper_smoke=ok"
require_line "$DOC" "exact_lane_helper_emission_count_positive=1"
require_line "$DOC" "summary=ok"

bash "$ROOT_DIR/tools/build_hako_llvmc_ffi.sh" >/dev/null
cargo build --release --bin hakorune -p nyash-rust >/dev/null
cargo build --release -p nyash_kernel >/dev/null

if ! command -v llvm-nm >/dev/null 2>&1; then
  echo "[row215-exact-slot-impl] llvm-nm is required for the export check" >&2
  exit 1
fi

llvm-nm -g "$ROOT_DIR/target/release/libnyash_kernel.a" >"$TMP_DIR/kernel.nm" 2>/dev/null
for symbol in \
  'nyash.object.exact_slot_get_i64_hii' \
  'nyash.object.exact_slot_set_i64_hii' \
  'nyash.object.exact_slot_get_u64_hii' \
  'nyash.object.exact_slot_set_u64_hiu' \
  'nyash.object.exact_slot_get_handle_hii' \
  'nyash.object.exact_slot_set_handle_hii'
do
  if ! grep -q " ${symbol}$" "$TMP_DIR/kernel.nm"; then
    echo "[row215-exact-slot-impl] missing exported runtime symbol: $symbol" >&2
    exit 1
  fi
done

NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/target/release/hakorune" --backend mir --emit-mir-json "$MIR" "$APP" >/dev/null
python3 "$ROOT_DIR/tools/checks/pure_first_route_preflight.py" "$MIR" >/dev/null

HAKO_TYPED_OBJECT_STORE=single_thread_exact \
HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER=1 \
NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/tools/selfhost/selfhost_build.sh" --mir-in "$MIR" --exe "$EXE_ON" >/dev/null

NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/tools/selfhost/selfhost_build.sh" --mir-in "$MIR" --exe "$EXE_OFF" >/dev/null

exact_on_count="$(strings "$EXE_ON" | grep -c 'nyash.object.exact_slot' || true)"
exact_off_count="$(strings "$EXE_OFF" | grep -c 'nyash.object.exact_slot' || true)"
if [ "$exact_on_count" -lt 1 ]; then
  echo "[row215-exact-slot-impl] exact-lane EXE does not reference exact_slot helpers" >&2
  exit 1
fi
if [ "$exact_off_count" -ne 0 ]; then
  echo "[row215-exact-slot-impl] default EXE unexpectedly references exact_slot helpers" >&2
  exit 1
fi

HAKO_ARRAY_SLOT_STORE=single_thread_exact \
HAKO_TYPED_OBJECT_STORE=single_thread_exact \
HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER=1 \
NYASH_DISABLE_PLUGINS=1 \
  "$EXE_ON" >"$ON_OUT"
NYASH_DISABLE_PLUGINS=1 "$EXE_OFF" >"$OFF_OUT"

require_contains "$ON_OUT" '^summary=ok$'
require_contains "$OFF_OUT" '^summary=ok$'

cat <<REPORT
output_contract=typed-object-exact-slot-direct-helper-implementation-v0
runtime_symbol_exported_count=6
exact_lane_helper_emission_count=${exact_on_count}
default_exact_helper_emission=${exact_off_count}
safe_mutex_default_smoke=ok
single_thread_exact_direct_helper_smoke=ok
provider_activation=0
allocator_replacement=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
REPORT
