#!/usr/bin/env bash
set -euo pipefail
# Purpose: Provider route with Match → Core rc=42

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

prog_json_path="/tmp/prog_2044_match_$$.json"
cat >"$prog_json_path" <<'JSON'
{"version":0,"kind":"Program","body":[
  {"type":"Return","expr":{"type":"Match","scrutinee":{"type":"Str","value":"A"},
   "arms":[{"label":"A","expr":{"type":"Int","value":42}}, {"label":"B","expr":{"type":"Int","value":0}}],
   "else":{"type":"Int","value":1}}}
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
  echo "[FAIL] provider match → core rc=$rc (expected 42)" >&2
  exit 1
fi

echo "[PASS] phase2044/mirbuilder_provider_match_core_exec_canary_vm"
exit 0

