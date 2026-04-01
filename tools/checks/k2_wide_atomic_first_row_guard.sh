#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

ATOMIC_CORE_FILE="lang/src/runtime/substrate/atomic/atomic_core_box.hako"
VM_BOXCALL_FILE="lang/src/vm/boxes/mir_vm_s0_boxcall_builtin.hako"
VM_EXTERNCALL_FILE="lang/src/vm/boxes/mir_vm_s0_call_exec.hako"

echo "[k2-wide-atomic-first-row] running narrow Atomic first-row acceptance pack"
echo "[k2-wide-atomic-first-row] --- vm-hako subset acceptance ---"
cargo test -q subset_accepts_externcall_hako_barrier_touch_i64 -- --nocapture
cargo test -q subset_accepts_boxcall_atomiccore_fence_i64 -- --nocapture

echo "[k2-wide-atomic-first-row] --- substrate/vm route lock ---"
rg -F -q 'fence_i64()' "$ATOMIC_CORE_FILE"
rg -F -q 'externcall "hako_barrier_touch_i64"(0)' "$ATOMIC_CORE_FILE"
rg -F -q '[vm/adapter/atomic:fence_i64]' "$ATOMIC_CORE_FILE"
rg -F -q 'if method == "fence_i64"' "$VM_BOXCALL_FILE"
rg -F -q 'if func == "hako_barrier_touch_i64" || func == "hako_barrier_touch_i64/1"' "$VM_EXTERNCALL_FILE"

echo "[k2-wide-atomic-first-row] ok"
