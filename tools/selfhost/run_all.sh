#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"

echo "[selfhost] Running phase2120 pure/TM reps"
export NYASH_LLVM_USE_CAPI=${NYASH_LLVM_USE_CAPI:-1}
export HAKO_V1_EXTERN_PROVIDER_C_ABI=${HAKO_V1_EXTERN_PROVIDER_C_ABI:-1}
export HAKO_CAPI_PURE=${HAKO_CAPI_PURE:-1}
# Optional: set HAKO_CAPI_TM=1 to exercise TargetMachine path

# Use curated runner to ensure ordering (pure first) and env toggles
bash "$ROOT/tools/smokes/v2/profiles/quick/core/phase2120/run_all.sh"

echo "[selfhost] Running minimal .hako → LLVM selfhost driver"
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
  bash "$ROOT/tools/selfhost/run_hako_llvm_selfhost.sh" "$TMP_JSON" "$EXE"
RC=$?
set -e
echo "[selfhost] exe=$EXE rc=$RC"
rm -f "$TMP_JSON" || true
exit 0
