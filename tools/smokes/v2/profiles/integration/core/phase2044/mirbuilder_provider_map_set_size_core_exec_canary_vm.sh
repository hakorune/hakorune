#!/usr/bin/env bash
set -euo pipefail
# Purpose: Provider route with Map set/size → Core rc=1

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

prog_json_path="/tmp/prog_2044_map_$$.json"
cat >"$prog_json_path" <<'JSON'
{"version":0,"kind":"Program","body":[
  {"type":"Local","name":"m","expr":{"type":"New","class":"MapBox","args":[]}},
  {"type":"Expr","expr":{"type":"Method","recv":{"type":"Var","name":"m"},"method":"set","args":[{"type":"Str","value":"k"},{"type":"Int","value":7}]}},
  {"type":"Return","expr":{"type":"Method","recv":{"type":"Var","name":"m"},"method":"size","args":[]}}
]}
JSON

set +e
HAKO_VERIFY_BUILDER_ONLY=1 \
HAKO_PREFER_MIRBUILDER=1 \
NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
NYASH_USING_AST=1 NYASH_RESOLVE_FIX_BRACES=1 \
NYASH_DISABLE_NY_COMPILER=1 NYASH_FEATURES=stage3 \
NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN=1 \
verify_program_via_builder_to_core "$prog_json_path"
rc=$?
set -e
rm -f "$prog_json_path"
if [ "$rc" -ne 0 ]; then
  echo "[FAIL] provider map set/size (builder-only) rc=$rc (expected 0)" >&2
  exit 1
fi

echo "[PASS] phase2044/mirbuilder_provider_map_set_size_core_exec_canary_vm"
exit 0
