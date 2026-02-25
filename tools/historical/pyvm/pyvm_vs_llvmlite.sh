#!/usr/bin/env bash
set -euo pipefail

if [[ "${NYASH_CLI_VERBOSE:-0}" == "1" ]]; then
  set -x
fi

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/../../.." && pwd)
source "$SCRIPT_DIR/common.sh"

APP="${1:-$ROOT_DIR/apps/tests/esc_dirname_smoke.hako}"
OUT="app_pyvm_cmp"

if [[ ! -f "$APP" ]]; then
  echo "error: app not found: $APP" >&2
  exit 2
fi

# 1) Build nyash with llvm harness enabled (build_llvm.sh does the right thing)
echo "[cmp] building AOT via llvmlite harness ..." >&2
"$ROOT_DIR/tools/build_llvm.sh" "$APP" -o "$OUT" >/dev/null

# 2) Run AOT executable and capture stdout + exit code
echo "[cmp] running AOT (llvmlite) ..." >&2
set +e
OUT_LL=$("$ROOT_DIR/$OUT" 2>&1)
CODE_LL=$?
set -e

# 3) Run PyVM path (direct historical runner)
echo "[cmp] running PyVM ..." >&2
set +e
OUT_PY=$(pyvm_run_source_capture "$APP" 2>&1)
CODE_PY=$?
set -e

echo "=== llvmlite (AOT) stdout ==="
echo "$OUT_LL" | sed -n '1,120p'
echo "=== PyVM stdout ==="
echo "$OUT_PY" | sed -n '1,120p'
echo "=== exit codes ==="
echo "llvmlite: $CODE_LL"
echo "PyVM    : $CODE_PY"

# Strict compare only when requested. Default: exit code parity.
STRICT=${CMP_STRICT:-0}
if [[ "$STRICT" == "1" ]]; then
  DIFF=0
  if [[ "$OUT_LL" != "$OUT_PY" ]]; then
    echo "[cmp] stdout differs" >&2
    DIFF=1
  fi
  if [[ "$CODE_LL" -ne "$CODE_PY" ]]; then
    echo "[cmp] exit code differs" >&2
    DIFF=1
  fi
  if [[ "$DIFF" -eq 0 ]]; then
    echo "✅ parity OK (stdout + exit code)"
  else
    echo "❌ parity mismatch" >&2
    exit 1
  fi
else
  if [[ "$CODE_LL" -eq "$CODE_PY" ]]; then
    echo "✅ parity OK (exit code)"
  else
    echo "❌ exit code mismatch" >&2
    exit 1
  fi
fi
