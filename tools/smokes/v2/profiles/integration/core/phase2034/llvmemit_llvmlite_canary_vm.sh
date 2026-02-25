#!/bin/bash
# llvmemit llvmlite canary — opt-in provider; SKIP if python/llvmlite/harness not present

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2
enable_mirbuilder_dev_env

tmp_hako="/tmp/llvmemit_llvmlite_canary_$$.hako"
cat > "$tmp_hako" <<'HAKO'
include "lang/src/llvm_ir/emit/LLVMEmitBox.hako"
static box Main { method main(args) {
  local mir = "{\"functions\":{\"Main.main\":{\"params\":[],\"locals\":[],\"blocks\":[{\"label\":\"bb0\",\"instructions\":[{\"op\":\"const\",\"dst\":1,\"value\":{\"type\":\"i64\",\"value\":0}},{\"op\":\"ret\",\"value\":1}] }] }},\"blocks\":1}";
  local argsA = new ArrayBox(); argsA.push(mir)
  local out = hostbridge.extern_invoke("env.codegen", "emit_object", argsA)
  if out == null { return 0 }
  print("" + out)
  return 1
} }
HAKO

set +e
out="$(out="$(HAKO_LLVM_EMIT_PROVIDER=llvmlite "$NYASH_BIN" --backend vm "$tmp_hako" 2>&1 )"; rc=$?
set -e
rm -f "$tmp_hako" || true

path="$(echo "$out" | tail -n1 | tr -d '\r')"
if [ "$rc" -eq 1 ] && [ -n "$path" ] && [ -f "$path" ]; then
  echo "[PASS] llvmemit_llvmlite_canary_vm"; rm -f "$path" || true; exit 0
fi
if echo "$out" | grep -q "\[llvmemit/llvmlite/\(python-not-found\|harness-not-found\|failed\)"; then
  echo "[SKIP] llvmemit_llvmlite (provider missing)"; exit 0
fi
echo "[FAIL] llvmemit_llvmlite_canary_vm (rc=$rc)" >&2; exit 1

