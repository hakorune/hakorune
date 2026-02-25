#!/bin/bash
# llvmemit_canary_vm.sh — MIR(JSON v0) → .o box canary (provider-first; SKIP when provider absent)

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2
enable_mirbuilder_dev_env

tmp_hako="/tmp/llvmemit_canary_$$.hako"
cat > "$tmp_hako" <<'HAKO'
include "lang/src/llvm_ir/emit/LLVMEmitBox.hako"
static box Main { method main(args) {
  // Minimal MIR(JSON v0)
  local mir = "{\"functions\":{\"Main.main\":{\"params\":[],\"locals\":[],\"blocks\":[{\"label\":\"bb0\",\"instructions\":[{\"op\":\"const\",\"dst\":1,\"value\":{\"type\":\"Int\",\"value\":0}},{\"op\":\"ret\",\"value\":1}] }] }},\"blocks\":1}";
  // Call provider via extern directly to avoid name collision risks in this canary
  local argsA = new ArrayBox(); argsA.push(mir)
  local out = hostbridge.extern_invoke("env.codegen", "emit_object", argsA)
  if out == null { return 0 }
  // Print the path for the shell script to verify existence
  print("" + out)
  return 1;
} }
HAKO

set +e
out="$(HAKO_LLVM_EMIT_PROVIDER=ny-llvmc NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 \
  $NYASH_BIN --backend vm "$tmp_hako" 2>&1)"
rc=$?
set -e
rm -f "$tmp_hako" 2>/dev/null || true

path="$(echo "$out" | tail -n1 | tr -d '\r')"
if [ "$rc" -eq 1 ] && [ -n "$path" ] && [ -f "$path" ]; then
  echo "[PASS] llvmemit_canary_vm"
  rm -f "$path" || true
  exit 0
fi
if echo "$out" | grep -q "\[llvmemit/ny-llvmc/not-found\]"; then
  echo "[SKIP] llvmemit_canary (ny-llvmc not found)"
  exit 0
fi
if echo "$out" | grep -qi "call unresolved: 'hostbridge\.extern_invoke/3'\|call unresolved: 'LLVMEmitProviderBox\.emit_object/2'"; then
  echo "[SKIP] llvmemit_canary (provider reachable only via plugin; unresolved in this profile)"
  exit 0
fi
echo "[FAIL] llvmemit_canary_vm (rc=$rc)" >&2; exit 1
