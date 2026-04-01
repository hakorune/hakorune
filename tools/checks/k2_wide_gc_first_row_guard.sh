#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GC_CORE_FILE="lang/src/runtime/substrate/gc/gc_core_box.hako"
VM_SUBSET_FILE="src/runner/modes/vm_hako/subset_check/mod.rs"
VM_BOXCALL_FILE="lang/src/vm/boxes/mir_vm_s0_boxcall_builtin.hako"
VM_EXTERNCALL_FILE="lang/src/vm/boxes/mir_vm_s0_call_exec.hako"

echo "[k2-wide-gc-first-row] running narrow GC first-row acceptance pack"
echo "[k2-wide-gc-first-row] --- vm-hako subset acceptance ---"
cargo test -q subset_accepts_externcall_nyash_gc_barrier_write -- --nocapture
cargo test -q subset_accepts_boxcall_gccore_write_barrier_i64 -- --nocapture

echo "[k2-wide-gc-first-row] --- substrate/vm route lock ---"
rg -F -q 'write_barrier_i64(handle_or_ptr)' "$GC_CORE_FILE"
rg -F -q 'externcall "nyash.gc.barrier_write"(handle_or_ptr)' "$GC_CORE_FILE"
rg -F -q '[vm/adapter/gc:write_barrier_i64]' "$GC_CORE_FILE"
rg -F -q '&& box_type != "GcCoreBox"' "$VM_SUBSET_FILE"
rg -F -q 'if func == "nyash.gc.barrier_write" || func == "nyash.gc.barrier_write/1"' "$VM_SUBSET_FILE"
rg -F -q 'if method == "write_barrier_i64"' "$VM_BOXCALL_FILE"
rg -F -q 'if func == "nyash.gc.barrier_write" || func == "nyash.gc.barrier_write/1"' "$VM_EXTERNCALL_FILE"

echo "[k2-wide-gc-first-row] ok"
