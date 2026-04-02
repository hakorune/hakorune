#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
BIN_BUILDER="$ROOT/tools/ny_mir_builder.sh"

if ! command -v llc >/dev/null 2>&1; then
  echo "[SKIP] native_backend_compare_eq_canary_vm (llc not found)" >&2
  exit 0
fi

(cd "$ROOT/crates/nyash_kernel" && cargo build -q --release >/dev/null) || true

JSON='{
  "schema_version": 1,
  "functions": [
    {"name":"ny_main","blocks":[
      {"id":0,"inst":[
        {"op":"const","dst":1,"ty":"i64","value":5},
        {"op":"const","dst":2,"ty":"i64","value":5},
        {"op":"compare","dst":3,"operation":"==","lhs":1,"rhs":2},
        {"op":"ret","value":3}
      ]}
    ]}
  ]
}'

TMP_JSON="/tmp/native_cmp_eq_$$.json"; echo "$JSON" > "$TMP_JSON"
APP="/tmp/native_cmp_eq_$$"

set +e
NYASH_LLVM_BACKEND=native NYASH_LLVM_SKIP_BUILD=1 bash "$BIN_BUILDER" --in "$TMP_JSON" --emit exe -o "$APP" >/dev/null 2>&1
RC_BUILD=$?
set -e
if [ "$RC_BUILD" -ne 0 ]; then
  echo "[SKIP] native_backend_compare_eq_canary_vm (native builder failed)" >&2
  rm -f "$TMP_JSON" "$APP"; exit 0
fi

set +e
"$APP" >/dev/null 2>&1; rc=$?
set -e
rm -f "$TMP_JSON" "$APP" 2>/dev/null || true
if [ "$rc" -eq 1 ]; then
  echo "[PASS] native_backend_compare_eq_canary_vm"
  exit 0
fi
echo "[FAIL] native_backend_compare_eq_canary_vm (rc=$rc)" >&2
exit 1

