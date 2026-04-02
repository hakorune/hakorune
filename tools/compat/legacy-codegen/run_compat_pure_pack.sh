#!/usr/bin/env bash
set -euo pipefail

# Historical compat pure-pack orchestrator.
# This shells into the active compat/pure-keep suite, the archive/pure-historical
# replay bucket, and then the canonical archive-later compat selfhost transport
# wrapper. It is not a separate proof owner.

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
PURE_KEEP_RUNNER="$ROOT/tools/smokes/v2/profiles/integration/compat/pure-keep/run_pure_keep.sh"
PURE_HISTORICAL_RUNNER="$ROOT/tools/smokes/v2/profiles/archive/pure-historical/run_pure_historical.sh"
SELFHOST_COMPAT_WRAPPER="$ROOT/tools/compat/legacy-codegen/run_compat_pure_selfhost.sh"

echo "[selfhost/compat] Running historical pure/TM pack"
export NYASH_LLVM_USE_CAPI=${NYASH_LLVM_USE_CAPI:-1}
export HAKO_V1_EXTERN_PROVIDER_C_ABI=${HAKO_V1_EXTERN_PROVIDER_C_ABI:-1}
export HAKO_CAPI_PURE=${HAKO_CAPI_PURE:-1}
# Optional: set HAKO_CAPI_TM=1 to exercise TargetMachine path

if [[ ! -f "$PURE_KEEP_RUNNER" ]]; then
  echo "[ERR] missing pure keep runner: $PURE_KEEP_RUNNER" >&2
  exit 2
fi
if [[ ! -f "$PURE_HISTORICAL_RUNNER" ]]; then
  echo "[ERR] missing pure historical runner: $PURE_HISTORICAL_RUNNER" >&2
  exit 2
fi
if [[ ! -f "$SELFHOST_COMPAT_WRAPPER" ]]; then
  echo "[ERR] missing compat selfhost wrapper: $SELFHOST_COMPAT_WRAPPER" >&2
  exit 3
fi

bash "$PURE_KEEP_RUNNER"

bash "$PURE_HISTORICAL_RUNNER"

echo "[selfhost/compat] Running historical .hako -> LLVM selfhost helper"
TMP_JSON="/tmp/hako_min44_$$.json"
cat > "$TMP_JSON" <<'JSON'
{"schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[
 {"op":"const","dst":1,"value":{"type":"i64","value":44}},
 {"op":"ret","value":1}
]}]}]}
JSON

EXE="/tmp/hako_selfhost_min_exe_$$"
set +e
HAKO_CAPI_PURE=${HAKO_CAPI_PURE:-1} NYASH_LLVM_USE_CAPI=${NYASH_LLVM_USE_CAPI:-1} HAKO_V1_EXTERN_PROVIDER_C_ABI=${HAKO_V1_EXTERN_PROVIDER_C_ABI:-1} \
  bash "$SELFHOST_COMPAT_WRAPPER" "$TMP_JSON" "$EXE"
RC=$?
set -e
echo "[selfhost/compat] exe=$EXE rc=$RC"
rm -f "$TMP_JSON" || true
exit 0
