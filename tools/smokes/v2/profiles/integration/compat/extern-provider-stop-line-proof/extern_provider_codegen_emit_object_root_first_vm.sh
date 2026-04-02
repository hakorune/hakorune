#!/usr/bin/env bash
set -euo pipefail
# Purpose: exact W4 proof lane for the extern-provider stop-line.
# `.hako VM` -> LlvmBackendEvidenceAdapterBox.compile_obj_provider_stopline(...)
# -> current compat/provider stop-line -> object -> LlvmBackendBox.link_exe(...) -> exe

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

if [ "${SMOKES_FORCE_LLVM:-0}" != "1" ] && ! can_run_llvm; then
  test_skip "compat/extern-provider-stop-line-proof/extern_provider_codegen_emit_object_root_first_vm: LLVM backend not available"
  exit 0
fi

INPUT_MIR="$NYASH_ROOT/apps/tests/hello_simple_llvm_native_probe.mir.json"
if [ ! -f "$INPUT_MIR" ]; then
  test_fail "compat/extern-provider-stop-line-proof/extern_provider_codegen_emit_object_root_first_vm: fixture missing: $INPUT_MIR"
  exit 1
fi

if ! command -v jq >/dev/null 2>&1; then
  test_skip "compat/extern-provider-stop-line-proof/extern_provider_codegen_emit_object_root_first_vm: jq not available"
  exit 0
fi

TMP_HAKO="$(mktemp --suffix .hako)"
OUT_EXE="$(mktemp --suffix .exe)"
BUILD_LOG="${TMPDIR:-/tmp}/extern_provider_stop_line_proof_build_$$.log"
rm -f "$OUT_EXE"

cleanup() {
  rm -f "$TMP_HAKO" "$OUT_EXE" "$BUILD_LOG"
}
trap cleanup EXIT

set +e
FFI_OUT=$(bash "$NYASH_ROOT/tools/build_hako_llvmc_ffi.sh" 2>&1)
FFI_RC=$?
set -e
printf "%s\n" "$FFI_OUT" >"$BUILD_LOG"
if [ "$FFI_RC" -ne 0 ]; then
  echo "[INFO] ffi build output:"
  tail -n 120 "$BUILD_LOG" || true
  test_fail "compat/extern-provider-stop-line-proof/extern_provider_codegen_emit_object_root_first_vm: FFI build failed (rc=$FFI_RC)"
  exit 1
fi

MIR_JSON_Q="$(jq -Rs . "$INPUT_MIR")"

cat >"$TMP_HAKO" <<HAKO
using selfhost.shared.backend.llvm_backend as LlvmBackendBox
using selfhost.shared.backend.llvm_backend_evidence_adapter as LlvmBackendEvidenceAdapterBox

static box Main {
  method main(args) {
    local obj = LlvmBackendEvidenceAdapterBox.compile_obj_provider_stopline($MIR_JSON_Q)
    if obj == null || obj == "" { return 91 }
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
  timeout 120 \
  "$NYASH_ROOT/target/release/hakorune" --backend vm-hako "$TMP_HAKO" 2>&1
)
RUN_RC=$?
set -e

if [ "$RUN_RC" -eq 124 ]; then
  test_fail "compat/extern-provider-stop-line-proof/extern_provider_codegen_emit_object_root_first_vm: vm-hako run timed out"
  exit 1
fi
if [ "$RUN_RC" -ne 0 ]; then
  echo "[INFO] vm-hako output:"
  echo "$RUN_OUT" | tail -n 120 || true
  test_fail "compat/extern-provider-stop-line-proof/extern_provider_codegen_emit_object_root_first_vm: vm-hako caller failed (rc=$RUN_RC)"
  exit 1
fi
if [ ! -x "$OUT_EXE" ]; then
  echo "[INFO] vm-hako output:"
  echo "$RUN_OUT" | tail -n 120 || true
  test_fail "compat/extern-provider-stop-line-proof/extern_provider_codegen_emit_object_root_first_vm: expected exe missing: $OUT_EXE"
  exit 1
fi

set +e
timeout 120 "$OUT_EXE" >/dev/null 2>&1
EXE_RC=$?
set -e

if [ "$EXE_RC" -eq 124 ]; then
  test_fail "compat/extern-provider-stop-line-proof/extern_provider_codegen_emit_object_root_first_vm: executable run timed out"
  exit 1
fi
if [ "$EXE_RC" -ne 0 ]; then
  test_fail "compat/extern-provider-stop-line-proof/extern_provider_codegen_emit_object_root_first_vm: executable exited $EXE_RC (expected 0)"
  exit 1
fi

test_pass "compat/extern-provider-stop-line-proof/extern_provider_codegen_emit_object_root_first_vm: PASS"
