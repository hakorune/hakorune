#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"
source tools/checks/lib/cargo_test_filter_group.sh

RAW_BUF_README="lang/src/runtime/substrate/raw_buf/README.md"
RAW_BUF_CORE_FILE="lang/src/runtime/substrate/raw_buf/raw_buf_core_box.hako"
VM_SUBSET_FILE="src/runner/reference/vm_hako/subset_check/mod.rs"
VM_BOXCALL_FILE="lang/src/vm/boxes/mir_vm_s0_boxcall_builtin.hako"
SUBSTRATE_README="lang/src/runtime/substrate/README.md"
SUBSTRATE_LADDER_DOC="docs/development/current/main/design/substrate-capability-ladder-ssot.md"
HAKO_ALLOC_DOC="docs/development/current/main/design/hako-alloc-policy-state-contract-ssot.md"
HAKO_ALLOC_README="lang/src/hako_alloc/README.md"
DEV_GATE="tools/checks/dev_gate.sh"

echo "[k2-wide-rawbuf-first-row] running narrow RawBuf first-row acceptance pack"
run_cargo_test_filter_group "k2-wide-rawbuf-first-row" "vm-hako subset acceptance" \
  subset_accepts_boxcall_rawbufcore \
  subset_rejects_boxcall_rawbufcore

echo "[k2-wide-rawbuf-first-row] --- file/owner lock ---"
for file in \
  "$RAW_BUF_README" \
  "$RAW_BUF_CORE_FILE" \
  "$SUBSTRATE_README" \
  "$SUBSTRATE_LADDER_DOC" \
  "$HAKO_ALLOC_DOC" \
  "$HAKO_ALLOC_README"; do
  if [ ! -f "$file" ]; then
    echo "[k2-wide-rawbuf-first-row] missing file: $file" >&2
    exit 1
  fi
done

echo "[k2-wide-rawbuf-first-row] --- substrate route lock ---"
rg -F -q 'using selfhost.runtime.substrate.mem.mem_core_box as MemCoreBox' "$RAW_BUF_CORE_FILE"
rg -F -q 'alloc_bytes_i64(size)' "$RAW_BUF_CORE_FILE"
rg -F -q 'realloc_bytes_i64(ptr, new_size)' "$RAW_BUF_CORE_FILE"
rg -F -q 'free_bytes_i64(ptr)' "$RAW_BUF_CORE_FILE"
rg -F -q 'MemCoreBox.alloc_i64(size)' "$RAW_BUF_CORE_FILE"
rg -F -q 'MemCoreBox.realloc_i64(ptr, new_size)' "$RAW_BUF_CORE_FILE"
rg -F -q 'MemCoreBox.free_i64(ptr)' "$RAW_BUF_CORE_FILE"
rg -F -q '[vm/adapter/raw_buf:alloc_bytes_i64]' "$RAW_BUF_CORE_FILE"
rg -F -q '[vm/adapter/raw_buf:realloc_bytes_i64]' "$RAW_BUF_CORE_FILE"
rg -F -q '[vm/adapter/raw_buf:free_bytes_i64]' "$RAW_BUF_CORE_FILE"
rg -F -q '&& box_type != "RawBufCoreBox"' "$VM_SUBSET_FILE"
rg -F -q 'box_type == "RawBufCoreBox"' "$VM_SUBSET_FILE"
rg -F -q 'method != "alloc_bytes_i64"' "$VM_SUBSET_FILE"
rg -F -q 'method != "realloc_bytes_i64"' "$VM_SUBSET_FILE"
rg -F -q 'method != "free_bytes_i64"' "$VM_SUBSET_FILE"
rg -F -q 'if method == "alloc_bytes_i64"' "$VM_BOXCALL_FILE"
rg -F -q 'if method == "realloc_bytes_i64"' "$VM_BOXCALL_FILE"
rg -F -q 'if method == "free_bytes_i64"' "$VM_BOXCALL_FILE"
rg -F -q '__vm_hako_rawbuf_next_ptr' "$VM_BOXCALL_FILE"

echo "[k2-wide-rawbuf-first-row] --- stop-line lock ---"
rg -F -q 'No len/cap policy here.' "$RAW_BUF_README"
rg -F -q 'No `MaybeInit` here.' "$RAW_BUF_README"
rg -F -q 'No TLS / atomic / GC / OS VM policy here.' "$RAW_BUF_README"
if rg -F -q 'set_len' "$RAW_BUF_CORE_FILE"; then
  echo "[k2-wide-rawbuf-first-row] RawBufCoreBox widened into set_len" >&2
  exit 1
fi
if rg -F -q 'shrink' "$RAW_BUF_CORE_FILE"; then
  echo "[k2-wide-rawbuf-first-row] RawBufCoreBox widened into shrink" >&2
  exit 1
fi
if rg -F -q 'MaybeInit' "$RAW_BUF_CORE_FILE"; then
  echo "[k2-wide-rawbuf-first-row] RawBufCoreBox widened into MaybeInit" >&2
  exit 1
fi
if rg -F -q 'hako_osvm_' "$RAW_BUF_CORE_FILE"; then
  echo "[k2-wide-rawbuf-first-row] RawBufCoreBox reached OSVM directly" >&2
  exit 1
fi

echo "[k2-wide-rawbuf-first-row] --- docs/dev-gate lock ---"
rg -F -q 'raw_buf/raw_buf_core_box.hako' "$SUBSTRATE_README"
rg -F -q '### C2.5. `RawBuf`' "$SUBSTRATE_LADDER_DOC"
rg -F -q '`RawBuf` policy/state and native-layout-backed buffer ownership' "$HAKO_ALLOC_DOC"
rg -F -q 'The narrow `RawBufCoreBox` allocation facade lives under' "$HAKO_ALLOC_README"
rg -F -q 'k2_wide_rawbuf_first_row_guard.sh' "$DEV_GATE"

echo "[k2-wide-rawbuf-first-row] ok"
