#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"
source tools/checks/lib/cargo_test_filter_group.sh

INTRIN_CORE_FILE="lang/src/runtime/substrate/intrin/intrin_core_box.hako"
VM_INTRIN_HELPER_FILE="lang/src/vm/boxes/mir_i64_intrinsics.hako"
VM_BOXCALL_FILE="lang/src/vm/boxes/mir_vm_s0_boxcall_builtin.hako"
VM_EXTERNCALL_FILE="lang/src/vm/boxes/mir_vm_s0_call_exec.hako"
VM_MIRCALL_FILE="lang/src/vm/boxes/mir_call_v1_handler.hako"
HOSTBRIDGE_FILE="lang/c-abi/include/hako_hostbridge.h"
KERNEL_FILE="lang/c-abi/shims/hako_kernel.c"

echo "[k2-wide-intrin-first-row] running narrow Intrin first-row acceptance pack"
run_cargo_test_filter_group "k2-wide-intrin-first-row" "vm-hako subset acceptance" \
  intrincore_bit_count \
  hako_intrin

echo "[k2-wide-intrin-first-row] --- substrate/vm/native route lock ---"
rg -F -q 'clz_i64(value)' "$INTRIN_CORE_FILE"
rg -F -q 'ctz_i64(value)' "$INTRIN_CORE_FILE"
rg -F -q 'popcnt_i64(value)' "$INTRIN_CORE_FILE"
rg -F -q 'externcall "hako_intrin_clz_i64"(value)' "$INTRIN_CORE_FILE"
rg -F -q 'externcall "hako_intrin_ctz_i64"(value)' "$INTRIN_CORE_FILE"
rg -F -q 'externcall "hako_intrin_popcnt_i64"(value)' "$INTRIN_CORE_FILE"
rg -F -q '[freeze:contract][intrin/non-negative-i64]' "$INTRIN_CORE_FILE"
rg -F -q 'static box MirI64IntrinsicsBox' "$VM_INTRIN_HELPER_FILE"
rg -F -q 'using selfhost.vm.boxes.mir_i64_intrinsics as MirI64IntrinsicsBox' "$VM_BOXCALL_FILE"
rg -F -q 'using selfhost.vm.boxes.mir_i64_intrinsics as MirI64IntrinsicsBox' "$VM_EXTERNCALL_FILE"
rg -F -q 'using selfhost.vm.boxes.mir_i64_intrinsics as MirI64IntrinsicsBox' "$VM_MIRCALL_FILE"
rg -F -q 'if method == "clz_i64"' "$VM_BOXCALL_FILE"
rg -F -q 'hako_intrin_clz_i64' "$VM_EXTERNCALL_FILE"
rg -F -q 'hako_intrin_clz_i64' "$VM_MIRCALL_FILE"
rg -F -q 'int64_t     hako_intrin_clz_i64(int64_t x);' "$HOSTBRIDGE_FILE"
rg -F -q 'int64_t hako_intrin_clz_i64(int64_t x)' "$KERNEL_FILE"
rg -F -q 'int64_t hako_intrin_ctz_i64(int64_t x)' "$KERNEL_FILE"
rg -F -q 'int64_t hako_intrin_popcnt_i64(int64_t x)' "$KERNEL_FILE"

echo "[k2-wide-intrin-first-row] ok"
