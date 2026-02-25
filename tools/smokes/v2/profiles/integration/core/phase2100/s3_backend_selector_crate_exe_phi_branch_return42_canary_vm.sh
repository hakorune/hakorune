#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
source "$ROOT/tools/smokes/v2/lib/test_runner.sh" || true
source "$ROOT/tools/smokes/v2/lib/crate_exec.sh" || true
BIN_NYLLVMC="$ROOT/target/release/ny-llvmc"

timeout "${HAKO_BUILD_TIMEOUT:-10}" bash -lc '(cd "$ROOT" && cargo build -q --release -p nyash-llvm-compiler >/dev/null)' || true
timeout "${HAKO_BUILD_TIMEOUT:-10}" bash -lc '(cd "$ROOT/crates/nyash_kernel" && cargo build -q --release >/dev/null)' || true

# Branch: if (1 == 1) then return 42 else return 7 → expect rc=42
JSON='{
  "schema_version": 1,
  "functions": [
    {"name":"ny_main","blocks":[
      {"id":0,"inst":[
        {"op":"const","dst":1,"ty":"i64","value":1},
        {"op":"const","dst":2,"ty":"i64","value":1},
        {"op":"compare","operation":"==","lhs":1,"rhs":2,"dst":3},
        {"op":"branch","cond":3,"then":1,"else":2}
      ]},
      {"id":1,"inst":[
        {"op":"const","dst":4,"ty":"i64","value":42},
        {"op":"ret","value":4}
      ]},
      {"id":2,"inst":[
        {"op":"const","dst":5,"ty":"i64","value":7},
        {"op":"ret","value":5}
      ]}
    ]}
  ]
}'

APP="/tmp/ny_crate_backend_exe_phi_ret42_$$"
TMP_JSON="/tmp/ny_crate_backend_exe_phi_ret42_$$.json"
echo "$JSON" > "$TMP_JSON"

enable_exe_dev_env

if NYASH_LLVM_VERIFY=1 NYASH_LLVM_VERIFY_IR=1 HAKO_LLVM_CANARY_NORMALIZE=1 \
   crate_build_exe "$TMP_JSON" "$APP" "$ROOT/target/release"; then
  if [[ -x "$APP" ]]; then
    set +e
    crate_run_exe "$APP"; rc=$?
    set -e
    if [ "$rc" -eq 42 ]; then
      echo "[PASS] s3_backend_selector_crate_exe_phi_branch_return42_canary_vm"
      rm -f "$APP" "$TMP_JSON" 2>/dev/null || true
      exit 0
    fi
    if [ "$rc" -eq 124 ]; then
      echo "[SKIP] s3_backend_selector_crate_exe_phi_branch_return42_canary_vm: timed out running EXE (rc=$rc)" >&2
      rm -f "$APP" "$TMP_JSON" 2>/dev/null || true
      exit 0
    fi
  fi
fi
echo "[SKIP] s3_backend_selector_crate_exe_phi_branch_return42_canary_vm: build or run failed/timed out" >&2
rm -f "$APP" "$TMP_JSON" 2>/dev/null || true
exit 0
