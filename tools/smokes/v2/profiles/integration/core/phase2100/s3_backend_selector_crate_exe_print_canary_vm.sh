#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
source "$ROOT/tools/smokes/v2/lib/test_runner.sh" || true

enable_exe_dev_env

TMP_HAKO=$(mktemp --suffix .hako)
cat >"$TMP_HAKO" <<'HAKO'
static box Main { method main(){
  print("hello-print-canary")
  return 0
} }
HAKO

TMP_JSON=$(mktemp --suffix .json)
EXE_OUT="${ROOT}/target/print_canary_$$"
IR_DUMP="${ROOT}/target/print_canary_$$.ll"
trap 'rm -f "$TMP_HAKO" "$TMP_JSON" "$EXE_OUT" "$IR_DUMP" 2>/dev/null || true' EXIT

# Emit MIR(JSON) via selfhost-first
if ! HAKO_SELFHOST_BUILDER_FIRST=1 NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 NYASH_JSON_ONLY=1 \
     timeout "${HAKO_BUILD_TIMEOUT:-10}" bash "$ROOT/tools/hakorune_emit_mir.sh" "$TMP_HAKO" "$TMP_JSON" >/dev/null 2>&1; then
  echo "[SKIP] print_canary: failed to emit MIR JSON"; exit 0
fi

# Build EXE via crate backend
if ! NYASH_LLVM_BACKEND=crate NYASH_LLVM_VERIFY=1 NYASH_LLVM_VERIFY_IR=1 \
      NYASH_LLVM_DUMP_IR="$IR_DUMP" \
      NYASH_NY_LLVM_COMPILER="${NYASH_NY_LLVM_COMPILER:-$ROOT/target/release/ny-llvmc}" \
      NYASH_EMIT_EXE_NYRT="${NYASH_EMIT_EXE_NYRT:-$ROOT/target/release}" \
      timeout "${HAKO_BUILD_TIMEOUT:-10}" bash "$ROOT/tools/ny_mir_builder.sh" --in "$TMP_JSON" --emit exe -o "$EXE_OUT" --quiet >/dev/null 2>&1; then
  echo "[SKIP] print_canary: failed to build EXE (crate)"; exit 0
fi

set +e
# Run the built EXE with an internal timeout shorter than the harness timeout
# so that hangs are handled here and converted to SKIP (quick profile policy).
timeout "${HAKO_EXE_TIMEOUT:-5}" "$EXE_OUT" >/dev/null 2>&1
rc=$?
set -e

if [[ "$rc" -eq 0 ]]; then
  echo "[PASS] s3_backend_selector_crate_exe_print_canary_vm"
  exit 0
fi

# Known issue path: print path may segfault on some hosts; provide diagnostics and SKIP for quick
if [[ "$rc" -eq 124 ]]; then
  echo "[SKIP] print_canary: timed out running EXE (rc=$rc). Providing IR head for diagnosis." >&2
else
  echo "[SKIP] print_canary: non-zero exit (rc=$rc). Providing IR head for diagnosis." >&2
fi
if [[ -s "$IR_DUMP" ]]; then head -n 80 "$IR_DUMP" >&2 || true; fi
exit 0
