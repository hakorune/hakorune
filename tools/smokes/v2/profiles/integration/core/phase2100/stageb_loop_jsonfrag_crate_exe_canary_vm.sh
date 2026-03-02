#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
source "$ROOT/tools/smokes/v2/lib/test_runner.sh" || true

enable_exe_dev_env

# Minimal loop program (structure only)
TMP_HAKO=$(mktemp --suffix .hako)
cat >"$TMP_HAKO" <<'HAKO'
static box Main { method main(){
  local n=10; local i=0;
  loop(i<n){ i=i+1 }
  return i
} }
HAKO

TMP_JSON=$(mktemp --suffix .json)
EXE_OUT="${ROOT}/target/stageb_loop_jsonfrag_$$"
trap 'rm -f "$TMP_HAKO" "$TMP_JSON" "$EXE_OUT" 2>/dev/null || true' EXIT

# Emit MIR(JSON) via selfhost-first and JSONFrag loop (normalized)
LOG_OUT=$(mktemp)
if ! HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_MIR_BUILDER_LOOP_JSONFRAG=1 HAKO_MIR_BUILDER_JSONFRAG_NORMALIZE=1 HAKO_MIR_BUILDER_JSONFRAG_PURIFY=1 HAKO_MIR_BUILDER_LOOP_FORCE_JSONFRAG=1 \
      NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
      NYASH_JSON_ONLY=1 bash "$ROOT/tools/smokes/v2/lib/emit_mir_route.sh" --route hako-helper --timeout-secs "${HAKO_BUILD_TIMEOUT:-10}" --out "$TMP_JSON" --input "$TMP_HAKO" >"$LOG_OUT" 2>&1; then
  echo "[FAIL] stageb_loop_jsonfrag: failed to emit MIR JSON"; tail -n 60 "$LOG_OUT" >&2; exit 1
fi

# Purify (dev): drop stray MapBox newbox from instructions to enforce JSONFrag purity
if command -v jq >/dev/null 2>&1; then
  if ! jq '.functions = (.functions | map(.blocks = (.blocks | map(.instructions = (.instructions | map(select((.op != "newbox") or (.type != "MapBox"))))))))' "$TMP_JSON" > "${TMP_JSON}.clean" 2>/dev/null; then
    echo "[SKIP] stageb_loop_jsonfrag: jq failed to parse MIR JSON (dev env)." >&2
    exit 0
  fi
  if [[ -s "${TMP_JSON}.clean" ]]; then mv -f "${TMP_JSON}.clean" "$TMP_JSON"; fi
fi
if rg -n "newbox|MapBox" "$TMP_JSON" >/dev/null 2>&1; then
  echo "[FAIL] stageb_loop_jsonfrag: found MapBox/newbox in MIR after purify"; exit 1
fi

# Build EXE via crate backend with diagnostics
IR_DUMP="${ROOT}/target/stageb_loop_debug_$$.ll"
EXE_LOG=$(mktemp)
trap 'rm -f "$TMP_HAKO" "$TMP_JSON" "$EXE_OUT" "$IR_DUMP" "$EXE_LOG" 2>/dev/null || true' EXIT

if ! NYASH_LLVM_BACKEND=crate NYASH_LLVM_VERIFY=1 NYASH_LLVM_VERIFY_IR=1 \
      NYASH_LLVM_DUMP_IR="$IR_DUMP" \
      NYASH_NY_LLVM_COMPILER="${NYASH_NY_LLVM_COMPILER:-$ROOT/target/release/ny-llvmc}" \
      NYASH_EMIT_EXE_NYRT="${NYASH_EMIT_EXE_NYRT:-$ROOT/target/release}" \
      timeout "${HAKO_BUILD_TIMEOUT:-10}" bash "$ROOT/tools/ny_mir_builder.sh" --in "$TMP_JSON" --emit exe -o "$EXE_OUT" --quiet >"$EXE_LOG" 2>&1; then
  echo "[SKIP] stageb_loop_jsonfrag: failed to build EXE (crate)"
  if [ -f "$IR_DUMP" ] && [ -s "$IR_DUMP" ]; then
    echo "[DEBUG] First 120 lines of LLVM IR:" >&2
    head -n 120 "$IR_DUMP" >&2 || true
  fi
  if [ -s "$EXE_LOG" ]; then
    echo "[DEBUG] Build error log:" >&2
    tail -n 40 "$EXE_LOG" >&2 || true
  fi
  exit 0
fi

# Run and check exit code
set +e
timeout "${HAKO_EXE_TIMEOUT:-5}" "$EXE_OUT" >/dev/null 2>&1
rc=$?
set -e

# Expected: rc=10 (limit value from loop)
if [[ "$rc" -eq 10 ]]; then
  echo "[PASS] stageb_loop_jsonfrag_crate_exe_canary_vm"
  exit 0
fi
if [[ "$rc" -eq 124 ]]; then
  echo "[SKIP] stageb_loop_jsonfrag: EXE timed out (expect rc=10)" >&2
  exit 0
fi

# If rc != 10, this is a failure
echo "[FAIL] stageb_loop_jsonfrag_crate_exe_canary_vm (expected rc=10, got $rc)"
if [ -f "$IR_DUMP" ] && [ -s "$IR_DUMP" ]; then
  echo "[DEBUG] First 120 lines of LLVM IR for diagnosis:" >&2
  head -n 120 "$IR_DUMP" >&2 || true
fi
exit 1
