#!/usr/bin/env bash
set -euo pipefail
# Purpose: Provider route with Ternary → Core rc=42

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

prog_json_path="/tmp/prog_2044_ternary_$$.json"
cat >"$prog_json_path" <<'JSON'
{"version":0,"kind":"Program","body":[
  {"type":"Return","expr":{"type":"Ternary","cond":{"type":"Compare","op":"<","lhs":{"type":"Int","value":1},"rhs":{"type":"Int","value":2}},"then":{"type":"Int","value":42},"else":{"type":"Int","value":0}}}
]}
JSON

set +e
HAKO_PREFER_MIRBUILDER=1 \
NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
NYASH_USING_AST=1 NYASH_RESOLVE_FIX_BRACES=1 \
NYASH_DISABLE_NY_COMPILER=1 NYASH_FEATURES=stage3 \
NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN=1 \
verify_program_via_builder_to_core "$prog_json_path"
rc=$?
set -e
rm -f "$prog_json_path"
if [ "$rc" -ne 42 ]; then
  echo "[FAIL] provider ternary → core rc=$rc (expected 42)" >&2
  exit 1
fi

echo "[PASS] phase2044/mirbuilder_provider_ternary_core_exec_canary_vm"
exit 0

