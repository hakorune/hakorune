#!/bin/bash
# Phase 29ck runtime proof:
# `.hako VM` -> LlvmBackendBox -> env.codegen C-API -> object -> exe

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

if [ "${SMOKES_FORCE_LLVM:-0}" != "1" ] && ! can_run_llvm; then
    test_skip "phase29ck_vmhako_llvm_backend_runtime_proof: LLVM backend not available"
    exit 0
fi

INPUT_MIR="$NYASH_ROOT/apps/tests/hello_simple_llvm_native_probe.mir.json"
TMP_HAKO="${TMPDIR:-/tmp}/phase29ck_vmhako_llvm_backend_runtime_proof_$$.hako"
OUT_EXE="${TMPDIR:-/tmp}/phase29ck_vmhako_llvm_backend_runtime_proof_$$.exe"
BUILD_LOG="${TMPDIR:-/tmp}/phase29ck_vmhako_llvm_backend_runtime_proof_build_$$.log"

cleanup() {
    rm -f "$TMP_HAKO" "$OUT_EXE" "$BUILD_LOG"
}
trap cleanup EXIT

if [ ! -f "$INPUT_MIR" ]; then
    test_fail "phase29ck_vmhako_llvm_backend_runtime_proof: fixture missing: $INPUT_MIR"
    exit 1
fi

set +e
FFI_OUT=$(bash "$NYASH_ROOT/tools/build_hako_llvmc_ffi.sh" 2>&1)
FFI_RC=$?
set -e
echo "$FFI_OUT" >"$BUILD_LOG"
if [ "$FFI_RC" -ne 0 ]; then
    echo "[INFO] ffi build output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "phase29ck_vmhako_llvm_backend_runtime_proof: FFI build failed (rc=$FFI_RC)"
    exit 1
fi

cat >"$TMP_HAKO" <<HAKO
using selfhost.shared.backend.llvm_backend as LlvmBackendBox

static box Main {
  method main(args) {
    local obj = LlvmBackendBox.compile_obj("$INPUT_MIR")
    if obj == null { return 91 }
    local ok = LlvmBackendBox.link_exe(obj, "$OUT_EXE", null)
    if ok != 1 { return 92 }
    return 0
  }
}
HAKO

set +e
RUN_OUT=$(
  NYASH_LLVM_USE_CAPI=1 \
  HAKO_V1_EXTERN_PROVIDER_C_ABI=1 \
  HAKO_CAPI_PURE=1 \
  timeout 120 \
  "$NYASH_ROOT/target/release/hakorune" --backend vm-hako "$TMP_HAKO" 2>&1
)
RUN_RC=$?
set -e

if [ "$RUN_RC" -eq 124 ]; then
    test_fail "phase29ck_vmhako_llvm_backend_runtime_proof: vm-hako run timed out"
    exit 1
fi
if [ "$RUN_RC" -ne 0 ]; then
    echo "[INFO] vm-hako output:"
    echo "$RUN_OUT" | tail -n 120 || true
    test_fail "phase29ck_vmhako_llvm_backend_runtime_proof: vm-hako caller failed (rc=$RUN_RC)"
    exit 1
fi
if [ ! -x "$OUT_EXE" ]; then
    echo "[INFO] vm-hako output:"
    echo "$RUN_OUT" | tail -n 120 || true
    test_fail "phase29ck_vmhako_llvm_backend_runtime_proof: expected exe missing: $OUT_EXE"
    exit 1
fi

set +e
EXE_OUT=$(
  NYASH_NYRT_SILENT_RESULT=1 \
  timeout 120 "$OUT_EXE" 2>&1
)
EXE_RC=$?
set -e

if [ "$EXE_RC" -eq 124 ]; then
    test_fail "phase29ck_vmhako_llvm_backend_runtime_proof: executable run timed out"
    exit 1
fi
if [ "$EXE_RC" -ne 0 ]; then
    echo "[INFO] executable output:"
    echo "$EXE_OUT" | tail -n 120 || true
    test_fail "phase29ck_vmhako_llvm_backend_runtime_proof: executable exited non-zero (rc=$EXE_RC)"
    exit 1
fi

test_pass "phase29ck_vmhako_llvm_backend_runtime_proof: PASS (.hako VM -> LlvmBackendBox -> C-API -> exe)"
