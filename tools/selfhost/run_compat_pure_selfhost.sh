#!/usr/bin/env bash
set -euo pipefail

# Historical pure-lowering selfhost helper.
# Usage:
#   tools/selfhost/run_compat_pure_selfhost.sh <json_file_or_-'stdin'> [exe_out]
# Env toggles:
#   HAKO_CAPI_PURE=1 (required)
#   HAKO_CAPI_TM=1   (optional: use TargetMachine path)

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
JSON_IN="${1:-}"
EXE_OUT="${2:-/tmp/hako_selfhost_exe}"

if [[ -z "$JSON_IN" ]]; then
  echo "Usage: $0 <json_file_or_-'stdin'> [exe_out]" >&2
  exit 2
fi

if [[ "$JSON_IN" == "-" ]]; then
  MIR_JSON="$(cat)"
else
  MIR_JSON="$(cat "$JSON_IN")"
fi

export NYASH_LLVM_USE_CAPI=${NYASH_LLVM_USE_CAPI:-1}
export HAKO_V1_EXTERN_PROVIDER_C_ABI=${HAKO_V1_EXTERN_PROVIDER_C_ABI:-1}
export HAKO_CAPI_PURE=${HAKO_CAPI_PURE:-1}

if [[ "${NYASH_LLVM_USE_CAPI}" != "1" || "${HAKO_V1_EXTERN_PROVIDER_C_ABI}" != "1" || "${HAKO_CAPI_PURE}" != "1" ]]; then
  echo "[ERR] require NYASH_LLVM_USE_CAPI=1 HAKO_V1_EXTERN_PROVIDER_C_ABI=1 HAKO_CAPI_PURE=1" >&2
  exit 3
fi

export _MIR_JSON="$MIR_JSON"
export _EXE_OUT="$EXE_OUT"

CODE_CONTENT="$(cat "$ROOT/tools/selfhost/examples/hako_llvm_selfhost_driver.hako")"
OUT="$(bash "$ROOT/tools/dev/hako_debug_run.sh" --safe -c "$CODE_CONTENT" 2>/dev/null)" || true
EXE_PATH="$(echo "$OUT" | tail -n1 | tr -d '\r')"
if [[ ! -f "$EXE_PATH" ]]; then
  echo "[ERR] exe not produced: $EXE_PATH" >&2
  exit 4
fi
echo "$EXE_PATH"
"$EXE_PATH"
exit $?
