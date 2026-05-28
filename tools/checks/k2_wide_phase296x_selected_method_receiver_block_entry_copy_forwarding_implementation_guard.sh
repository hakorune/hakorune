#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-251-SELECTED-METHOD-RECEIVER-BLOCK-ENTRY-COPY-FORWARDING-IMPLEMENTATION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-250-SELECTED-METHOD-RECEIVER-BLOCK-ENTRY-COPY-FORWARDING-GUARD-SURFACE.md"
APP="$ROOT_DIR/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
BODY_EMIT="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_same_module_body_emit.inc"
STATE="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_compiler_state.inc"
TYPED_EMIT="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_same_module_typed_object_emit.inc"
TMP_DIR="$(mktemp -d /tmp/hakorune_row251_receiver_forward.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

MIR="$TMP_DIR/app.mir.json"
EXE="$TMP_DIR/app.exe"
LL="$TMP_DIR/app.ll"
OUT="$TMP_DIR/app.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row251-receiver-forward] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_text() {
  local file="$1"
  local expected="$2"
  if ! grep -q "$expected" "$file"; then
    echo "[row251-receiver-forward] missing text in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=selected-method-receiver-block-entry-copy-forwarding-implementation-v0"
require_line "$DOC" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$DOC" "implementation_owner=c_abi_same_module_receiver_forwarding_alias"
require_line "$DOC" "forwarded_receiver_copy_count=9"
require_line "$DOC" "remaining_param0_copy_add_count=1"
require_line "$DOC" "semantic_proof_summary=ok"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "summary=ok"

require_text "$STATE" "set_receiver_forward_alias"
require_text "$BODY_EMIT" "same_module_function_should_forward_selected_receiver_copy"
require_text "$BODY_EMIT" "HakoAllocPageModel.acquire_usize/1"
require_text "$TYPED_EMIT" "get_receiver_forward_alias_src(box_reg, &box_ref_reg)"

bash "$ROOT_DIR/tools/build_hako_llvmc_ffi.sh" >/dev/null
cargo build --release -p nyash_kernel >/dev/null

NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/target/release/hakorune" \
    --backend mir \
    --emit-mir-json "$MIR" \
    "$APP" >/dev/null
python3 "$ROOT_DIR/tools/checks/pure_first_route_preflight.py" "$MIR" >/dev/null

HAKO_TYPED_OBJECT_STORE=single_thread_exact \
HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER=1 \
NYASH_LLVM_DUMP_IR="$LL" \
NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/tools/selfhost/selfhost_build.sh" --mir-in "$MIR" --exe "$EXE" >/dev/null

acquire_param0_add_count="$(
  awk '/define i64 @"HakoAllocPageModel.acquire_usize\/1"/{infn=1}
       infn&&/^}/{infn=0}
       infn&&/add i64 %r0, 0/{count++}
       END{print count+0}' "$LL"
)"
if [ "$acquire_param0_add_count" != "1" ]; then
  echo "[row251-receiver-forward] expected exactly one remaining acquire_usize param0 add, got $acquire_param0_add_count" >&2
  exit 1
fi

if ! awk '/define i64 @"HakoAllocPageModel.acquire_usize\/1"/{infn=1}
         infn&&/^}/{infn=0}
         infn&&/nyash\.object\.exact_slot_(get|set|rmw).*i64 %r0/{found=1}
         END{exit found?0:1}' "$LL"; then
  echo "[row251-receiver-forward] acquire_usize did not emit exact-slot receiver calls against %r0" >&2
  exit 1
fi

HAKO_ARRAY_SLOT_STORE=single_thread_exact \
HAKO_TYPED_OBJECT_STORE=single_thread_exact \
HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER=1 \
NYASH_DISABLE_PLUGINS=1 \
  "$EXE" >"$OUT"

require_line "$OUT" "summary=ok"

cat <<REPORT
output_contract=selected-method-receiver-block-entry-copy-forwarding-implementation-v0
selected_method=HakoAllocPageModel.acquire_usize/1
implementation_owner=c_abi_same_module_receiver_forwarding_alias
forwarded_receiver_copy_count=9
remaining_param0_copy_add_count=${acquire_param0_add_count}
semantic_proof_summary=ok
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
REPORT
