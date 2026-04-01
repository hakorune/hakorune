#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

OSVM_CORE_FILE="lang/src/runtime/substrate/osvm/osvm_core_box.hako"
VM_SUBSET_FILE="src/runner/modes/vm_hako/subset_check/mod.rs"
VM_BOXCALL_FILE="lang/src/vm/boxes/mir_vm_s0_boxcall_builtin.hako"
VM_EXTERNCALL_FILE="lang/src/vm/boxes/mir_vm_s0_call_exec.hako"
VM_MIRCALL_FILE="lang/src/vm/boxes/mir_call_v1_handler.hako"
HOSTBRIDGE_FILE="lang/c-abi/include/hako_hostbridge.h"
KERNEL_SHIM_FILE="lang/c-abi/shims/hako_kernel.c"

echo "[k2-wide-osvm-first-row] running narrow OSVM first-widening acceptance pack"
echo "[k2-wide-osvm-first-row] --- vm-hako subset acceptance ---"
cargo test -q subset_accepts_externcall_hako_osvm_reserve_bytes_i64 -- --nocapture
cargo test -q subset_accepts_boxcall_osvmcore_reserve_bytes_i64 -- --nocapture
cargo test -q compile_v0_emits_mir_call_extern_hako_osvm_reserve_bytes_i64 -- --nocapture
cargo test -q compile_v0_emits_mir_call_extern_hako_osvm_commit_bytes_i64 -- --nocapture
cargo test -q compile_v0_emits_mir_call_extern_hako_osvm_decommit_bytes_i64 -- --nocapture
cargo test -q subset_accepts_boxcall_osvmcore_commit_bytes_i64 -- --nocapture
cargo test -q subset_accepts_externcall_hako_osvm_commit_bytes_i64 -- --nocapture
cargo test -q subset_accepts_boxcall_osvmcore_decommit_bytes_i64 -- --nocapture
cargo test -q subset_accepts_externcall_hako_osvm_decommit_bytes_i64 -- --nocapture
cargo test -q subset_rejects_boxcall_osvmcore_page_size_i64 -- --nocapture
cargo test -q subset_rejects_externcall_hako_osvm_page_size_i64 -- --nocapture

echo "[k2-wide-osvm-first-row] --- substrate/vm/abi route lock ---"
rg -F -q 'reserve_bytes_i64(len_bytes)' "$OSVM_CORE_FILE"
rg -F -q 'commit_bytes_i64(base, len_bytes)' "$OSVM_CORE_FILE"
rg -F -q 'decommit_bytes_i64(base, len_bytes)' "$OSVM_CORE_FILE"
rg -F -q 'externcall "hako_osvm_reserve_bytes_i64"(len_bytes)' "$OSVM_CORE_FILE"
rg -F -q 'externcall "hako_osvm_commit_bytes_i64"(base, len_bytes)' "$OSVM_CORE_FILE"
rg -F -q 'externcall "hako_osvm_decommit_bytes_i64"(base, len_bytes)' "$OSVM_CORE_FILE"
rg -F -q '[vm/adapter/osvm:reserve_bytes_i64]' "$OSVM_CORE_FILE"
rg -F -q '[vm/adapter/osvm:commit_bytes_i64]' "$OSVM_CORE_FILE"
rg -F -q '[vm/adapter/osvm:decommit_bytes_i64]' "$OSVM_CORE_FILE"
if rg -F -q 'page_size_i64' "$OSVM_CORE_FILE"; then
  echo "[k2-wide-osvm-first-row] osvm core widened beyond page-size park" >&2
  exit 1
fi
rg -F -q '&& method != "commit_bytes_i64"' "$VM_SUBSET_FILE"
rg -F -q '&& method != "decommit_bytes_i64"' "$VM_SUBSET_FILE"
rg -F -q 'if func == "hako_osvm_reserve_bytes_i64"' "$VM_SUBSET_FILE"
rg -F -q '|| func == "hako_osvm_reserve_bytes_i64/1"' "$VM_SUBSET_FILE"
rg -F -q 'if method == "reserve_bytes_i64"' "$VM_BOXCALL_FILE"
rg -F -q 'if method == "commit_bytes_i64"' "$VM_BOXCALL_FILE"
rg -F -q 'if method == "decommit_bytes_i64"' "$VM_BOXCALL_FILE"
rg -F -q 'if func == "hako_osvm_reserve_bytes_i64" || func == "hako_osvm_reserve_bytes_i64/1"' "$VM_EXTERNCALL_FILE"
rg -F -q 'if func == "hako_osvm_commit_bytes_i64" || func == "hako_osvm_commit_bytes_i64/2"' "$VM_EXTERNCALL_FILE"
rg -F -q 'if func == "hako_osvm_decommit_bytes_i64" || func == "hako_osvm_decommit_bytes_i64/2"' "$VM_EXTERNCALL_FILE"
rg -F -q 'if name == "hako_osvm_reserve_bytes_i64" || name == "hako_osvm_reserve_bytes_i64/1"' "$VM_MIRCALL_FILE"
rg -F -q 'if name == "hako_osvm_commit_bytes_i64" || name == "hako_osvm_commit_bytes_i64/2"' "$VM_MIRCALL_FILE"
rg -F -q 'if name == "hako_osvm_decommit_bytes_i64" || name == "hako_osvm_decommit_bytes_i64/2"' "$VM_MIRCALL_FILE"
rg -F -q 'hako_osvm_reserve_bytes_i64(int64_t len_bytes);' "$HOSTBRIDGE_FILE"
rg -F -q 'hako_osvm_commit_bytes_i64(int64_t base, int64_t len_bytes);' "$HOSTBRIDGE_FILE"
rg -F -q 'hako_osvm_decommit_bytes_i64(int64_t base, int64_t len_bytes);' "$HOSTBRIDGE_FILE"
rg -F -q 'int64_t hako_osvm_reserve_bytes_i64(int64_t len_bytes) {' "$KERNEL_SHIM_FILE"
rg -F -q 'int64_t hako_osvm_commit_bytes_i64(int64_t base, int64_t len_bytes) {' "$KERNEL_SHIM_FILE"
rg -F -q 'int64_t hako_osvm_decommit_bytes_i64(int64_t base, int64_t len_bytes) {' "$KERNEL_SHIM_FILE"

echo "[k2-wide-osvm-first-row] ok"
