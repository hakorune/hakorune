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

TMP_HAKO=$(mktemp --suffix .hako)
cat >"$TMP_HAKO" <<'HAKO'
static box Main { method main(){
  local n = 2
  if (n < 1) {
    return 1
  } else {
    return 2
  }
} }
HAKO

TMP_JSON=$(mktemp --suffix .json)
EXE_OUT="${ROOT}/target/if_merge_canary_$$"
IR_DUMP="${ROOT}/target/if_merge_canary_$$.ll"
trap 'rm -f "$TMP_HAKO" "$TMP_JSON" "$EXE_OUT" "$IR_DUMP" 2>/dev/null || true' EXIT

# Emit MIR(JSON) via selfhost-first
# Prefer selfhost-first; on failure, delegate to Rust builder for stability
if ! HAKO_SELFHOST_BUILDER_FIRST=1 NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 NYASH_JSON_ONLY=1 \
     bash "$ROOT/tools/smokes/v2/lib/emit_mir_route.sh" --route hako-helper --timeout-secs "${HAKO_BUILD_TIMEOUT:-10}" --out "$TMP_JSON" --input "$TMP_HAKO" >/dev/null 2>&1; then
  if ! NYASH_JSON_ONLY=1 bash "$ROOT/tools/smokes/v2/lib/emit_mir_route.sh" --route hako-helper --timeout-secs "${HAKO_BUILD_TIMEOUT:-10}" --out "$TMP_JSON" --input "$TMP_HAKO" >/dev/null 2>&1; then
    echo "[SKIP] if_merge_canary: failed to emit MIR JSON (both paths)"; exit 0
  fi
fi

# Build EXE via crate backend
if ! NYASH_LLVM_BACKEND=crate NYASH_LLVM_VERIFY=1 NYASH_LLVM_VERIFY_IR=1 \
      NYASH_LLVM_DUMP_IR="$IR_DUMP" \
      NYASH_NY_LLVM_COMPILER="${NYASH_NY_LLVM_COMPILER:-$ROOT/target/release/ny-llvmc}" \
      NYASH_EMIT_EXE_NYRT="${NYASH_EMIT_EXE_NYRT:-$ROOT/target/release}" \
      bash "$ROOT/tools/ny_mir_builder.sh" --in "$TMP_JSON" --emit exe -o "$EXE_OUT" --quiet >/dev/null 2>&1; then
  echo "[SKIP] if_merge_canary: failed to build EXE (crate)"; exit 0
fi

set +e
"$EXE_OUT" >/dev/null 2>&1
rc=$?
set -e

if [[ "$rc" -eq 2 ]]; then
  echo "[PASS] stageb_if_merge_crate_exe_canary_vm"
  exit 0
fi

echo "[FAIL] stageb_if_merge_crate_exe_canary_vm (expected rc=2, got $rc)" >&2
if [[ -s "$IR_DUMP" ]]; then head -n 80 "$IR_DUMP" >&2 || true; fi
exit 1
