#!/usr/bin/env bash
set -euo pipefail
# Purpose: Provider route (llvmlite) compiles a simple const 42 → ret 42 module and returns an object path

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

mir_json=$(cat <<'JSON'
{"version":0,"kind":"Module","functions":[{"name":"Main.main","params":[],"locals":[],"blocks":[{"label":"bb0","instructions":[{"op":"const","dst":1,"value":{"type":"i64","value":42}},{"op":"ret","value":1}]}]}]}
JSON
)

code=$(cat <<'HCODE'
static box Main { method main(args) {
  local j = env.get("_MIR_JSON")
  local a = new ArrayBox(); a.push(j)
  local p = hostbridge.extern_invoke("env.codegen", "emit_object", a)
  if p == null { print("NULL"); return 1 }
  print("" + p)
  return 0
} }
HCODE
)

export HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM=0
export NYASH_DISABLE_NY_COMPILER=1
export NYASH_FEATURES="${NYASH_FEATURES:-stage3}"
export NYASH_FEATURES=stage3
export NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN=1
export NYASH_ENABLE_USING=1
export HAKO_ENABLE_USING=1
export NYASH_USING_AST=1
export NYASH_RESOLVE_FIX_BRACES=1
export _MIR_JSON="$mir_json"
export HAKO_LLVM_EMIT_PROVIDER=llvmlite

out=$(run_nyash_vm -c "$code" 2>/dev/null || true)
path=$(echo "$out" | tail -n1 | tr -d '\r')
if [ -z "$path" ] || [ "$path" = "NULL" ]; then
  echo "[FAIL] provider returned empty path" >&2
  exit 1
fi
if [ ! -f "$path" ]; then
  echo "[FAIL] output object not found: $path" >&2
  exit 1
fi
echo "[PASS] compat/llvmlite-monitor-keep/codegen_provider_llvmlite_const42_canary_vm ($path)"
exit 0
