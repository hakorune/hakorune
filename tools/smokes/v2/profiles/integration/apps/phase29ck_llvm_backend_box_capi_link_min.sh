#!/bin/bash
# Phase 29ck BE0-min6a: LlvmBackendBox first-cut route
#
# Contract pin:
# 1) `.hako` caller uses `selfhost.shared.backend.llvm_backend`.
# 2) direct MIR emit accepts the official caller route (`selfhost.shared.backend.llvm_backend`).
# 3) official owner routes directly through canonical `env.codegen.*`.
# 4) non-empty `libs` reaches the thin backend boundary as arg 3.
# 5) downstream native route stays green on the existing app-seed parity smoke.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null || (cd "$SCRIPT_DIR/../../../../../../.." && pwd))"

TEST_HAKO="$(mktemp --suffix .hako)"
OUT_MIR="$(mktemp --suffix .json)"
LOG_OUT="$(mktemp --suffix .log)"
trap 'rm -f "$TEST_HAKO" "$OUT_MIR" "$LOG_OUT" || true' EXIT

cat > "$TEST_HAKO" <<'HAKO'
using selfhost.shared.backend.llvm_backend as LlvmBackendBox

static box Main {
  method main(args) {
    local obj = LlvmBackendBox.compile_obj("/tmp/hello_simple_llvm_native_probe.mir.json")
    local ok = LlvmBackendBox.link_exe(obj, "/tmp/hello_simple_llvm_native_probe.exe", "-lm")
    return ok
  }
}
HAKO

cp "$ROOT_DIR/apps/tests/hello_simple_llvm_native_probe.mir.json" /tmp/hello_simple_llvm_native_probe.mir.json

set +e
HAKO_MIR_BUILDER_FUNCS=1 \
"$ROOT_DIR/target/release/hakorune" --emit-mir-json "$OUT_MIR" "$TEST_HAKO" >"$LOG_OUT" 2>&1
RC=$?
set -e

if [ "$RC" -ne 0 ]; then
  echo "[FAIL] phase29ck_llvm_backend_box_capi_link_min (direct MIR emit failed rc=$RC)" >&2
  cat "$LOG_OUT" >&2
  exit 1
fi

if ! grep -q '"name": "LlvmBackendBox.compile_obj/1"' "$OUT_MIR"; then
  echo "[FAIL] phase29ck_llvm_backend_box_capi_link_min (LlvmBackendBox.compile_obj/1 missing from MIR)" >&2
  cat "$OUT_MIR" >&2
  exit 1
fi

if ! grep -q '"name": "LlvmBackendBox.link_exe/3"' "$OUT_MIR"; then
  echo "[FAIL] phase29ck_llvm_backend_box_capi_link_min (LlvmBackendBox.link_exe/3 missing from MIR)" >&2
  cat "$OUT_MIR" >&2
  exit 1
fi

if ! grep -q '"box_name": "LlvmBackendBox"' "$OUT_MIR"; then
  echo "[FAIL] phase29ck_llvm_backend_box_capi_link_min (LlvmBackendBox callsite missing from MIR)" >&2
  cat "$OUT_MIR" >&2
  exit 1
fi

if ! grep -q '"name": "compile_obj"' "$OUT_MIR"; then
  echo "[FAIL] phase29ck_llvm_backend_box_capi_link_min (compile_obj call missing from MIR)" >&2
  cat "$OUT_MIR" >&2
  exit 1
fi

if ! grep -q '"name": "link_exe"' "$OUT_MIR"; then
  echo "[FAIL] phase29ck_llvm_backend_box_capi_link_min (link_exe call missing from MIR)" >&2
  cat "$OUT_MIR" >&2
  exit 1
fi

if ! grep -Fq 'env.codegen.compile_json_path(json_path, "", recipe, compat)' "$ROOT_DIR/lang/src/shared/backend/llvm_backend_box.hako"; then
  echo "[FAIL] phase29ck_llvm_backend_box_capi_link_min (owner no longer routes via env.codegen.compile_json_path(json_path, \"\", recipe, compat))" >&2
  exit 1
fi

if ! grep -Fq 'env.codegen.link_object(obj, out, extra)' "$ROOT_DIR/lang/src/shared/backend/llvm_backend_box.hako"; then
  echo "[FAIL] phase29ck_llvm_backend_box_capi_link_min (owner no longer routes via env.codegen.link_object(obj, out, extra))" >&2
  exit 1
fi

SMOKES_FORCE_LLVM=1 bash "$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/phase29ck_native_llvm_cabi_link_min.sh" >/dev/null

echo "[PASS] phase29ck_llvm_backend_box_capi_link_min: PASS (official caller compiles via env.codegen.compile_json_path(json_path, \"\", recipe, compat) / env.codegen.link_object(obj, out, extra), libs 3rd arg reaches the thin boundary, native downstream parity stays green)"
