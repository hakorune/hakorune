#!/usr/bin/env bash
set -euo pipefail

# Archive-later compat wrapper for the example/proof selfhost driver.
# This is compat-only. It preserves the historical shell contract while
# materializing the payload template onto the `vm-hako` lane.
# Usage:
#   tools/compat/legacy-codegen/run_compat_pure_selfhost.sh <json_file_or_-'stdin'> [exe_out]
# Env toggles:
#   HAKO_CAPI_PURE=1 (required)
#   HAKO_CAPI_TM=1   (optional: use TargetMachine path)

ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
JSON_IN="${1:-}"
EXE_OUT="${2:-/tmp/hako_selfhost_exe}"
DRIVER_HAKO="$ROOT/tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako"

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

if [[ ! -f "$DRIVER_HAKO" ]]; then
  echo "[ERR] missing compat selfhost driver: $DRIVER_HAKO" >&2
  exit 5
fi

if [[ -x "$ROOT/target/release/hakorune" ]]; then
  NYASH_BIN="$ROOT/target/release/hakorune"
elif [[ -x "$ROOT/target/release/nyash" ]]; then
  NYASH_BIN="$ROOT/target/release/nyash"
else
  echo "[ERR] missing hakorune/nyash binary under $ROOT/target/release" >&2
  exit 4
fi

MIR_JSON_Q="$(printf '%s' "$MIR_JSON" | jq -Rs .)"
EXE_OUT_Q="$(jq -Rn --arg x "$EXE_OUT" '$x')"
CODE_CONTENT="$(cat "$DRIVER_HAKO")"
CODE_CONTENT="${CODE_CONTENT//__MIR_JSON_LITERAL__/$MIR_JSON_Q}"
CODE_CONTENT="${CODE_CONTENT//__EXE_OUT_LITERAL__/$EXE_OUT_Q}"
TMP_HAKO="$(mktemp --suffix .hako)"
cleanup() {
  rm -f "$TMP_HAKO"
}
trap cleanup EXIT
printf '%s\n' "$CODE_CONTENT" > "$TMP_HAKO"

OUT="$(
  NYASH_LLVM_USE_CAPI=${NYASH_LLVM_USE_CAPI:-1} \
  HAKO_V1_EXTERN_PROVIDER_C_ABI=${HAKO_V1_EXTERN_PROVIDER_C_ABI:-1} \
  HAKO_CAPI_PURE=${HAKO_CAPI_PURE:-1} \
  timeout 120 "$NYASH_BIN" --backend vm-hako "$TMP_HAKO" 2>/dev/null
)" || true
EXE_PATH="$(echo "$OUT" | tail -n1 | tr -d '\r')"
if [[ ! -f "$EXE_PATH" ]]; then
  echo "[ERR] exe not produced: $EXE_PATH" >&2
  exit 4
fi
echo "$EXE_PATH"
"$EXE_PATH"
exit $?
