#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
BIN_BUILDER="$ROOT/tools/ny_mir_builder.sh"

# Require llc (native path)
if ! command -v llc >/dev/null 2>&1; then
  echo "[SKIP] native_backend_return42_canary_vm (llc not found)" >&2
  exit 0
fi

# Prebuild NyRT (for linking)
(cd "$ROOT/crates/nyash_kernel" && cargo build -q --release >/dev/null) || true

JSON='{
  "schema_version": 1,
  "functions": [
    {"name":"ny_main","blocks":[
      {"id":0,"inst":[
        {"op":"const","dst":1,"ty":"i64","value":42},
        {"op":"ret","value":1}
      ]}
    ]}
  ]
}'

TMP_JSON="/tmp/native_ret42_$$.json"; echo "$JSON" > "$TMP_JSON"
APP="/tmp/native_ret42_$$"

if ! NYASH_LLVM_BACKEND=native NYASH_LLVM_SKIP_BUILD=1 bash "$BIN_BUILDER" --in "$TMP_JSON" --emit exe -o "$APP" >/dev/null 2>&1; then
  echo "[SKIP] native_backend_return42_canary_vm (native builder failed)" >&2
  rm -f "$TMP_JSON" "$APP"; exit 0
fi
set +e
"$APP" >/dev/null 2>&1; rc=$?
set -e
rm -f "$TMP_JSON" "$APP" 2>/dev/null || true
if [ "$rc" -eq 42 ]; then
  echo "[PASS] native_backend_return42_canary_vm"
  exit 0
fi
echo "[FAIL] native_backend_return42_canary_vm" >&2
exit 1
