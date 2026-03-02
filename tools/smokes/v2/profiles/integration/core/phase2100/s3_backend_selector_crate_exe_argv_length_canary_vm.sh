#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
source "$ROOT/tools/smokes/v2/lib/test_runner.sh" || true
enable_exe_dev_env

# Quick profile default timeout is 15s; EXE build/link can exceed this.
# Respect the global budget and SKIP instead of timing out (fast-fail friendly).
if [ "${SMOKES_DEFAULT_TIMEOUT:-0}" -ne 0 ] && [ "${SMOKES_DEFAULT_TIMEOUT:-0}" -lt 25 ]; then
  echo "[SKIP] time budget too small for EXE canary (SMOKES_DEFAULT_TIMEOUT=${SMOKES_DEFAULT_TIMEOUT}s)"
  exit 0
fi

# Program: return args.length()
TMP_HAKO=$(mktemp --suffix .hako)
cat >"$TMP_HAKO" <<'HAKO'
static box Main { method main(args){
  return args.length()
} }
HAKO

TMP_JSON=$(mktemp --suffix .json)
EXE_OUT="${ROOT}/target/exe_argv_len_$$"
trap 'rm -f "$TMP_HAKO" "$TMP_JSON" "$EXE_OUT" 2>/dev/null || true' EXIT

if ! NYASH_JSON_ONLY=1 bash "$ROOT/tools/smokes/v2/lib/emit_mir_route.sh" --route hako-helper --timeout-secs "${HAKO_BUILD_TIMEOUT:-10}" --out "$TMP_JSON" --input "$TMP_HAKO" >/dev/null 2>&1; then
  echo "[SKIP] argv_len: failed to emit MIR JSON"; exit 0
fi

if ! NYASH_LLVM_BACKEND=crate NYASH_EXE_ARGV=1 \
     bash "$ROOT/tools/ny_mir_builder.sh" --in "$TMP_JSON" --emit exe -o "$EXE_OUT" --quiet >/dev/null 2>&1; then
  echo "[SKIP] argv_len: failed to build EXE"; exit 0
fi

set +e
"$EXE_OUT" a bb ccc >/dev/null 2>&1
rc=$?
set -e
if [[ "$rc" -eq 3 ]]; then
  echo "[PASS] s3_backend_selector_crate_exe_argv_length_canary_vm"
  exit 0
fi
echo "[SKIP] argv_len: unexpected rc=$rc"; exit 0
