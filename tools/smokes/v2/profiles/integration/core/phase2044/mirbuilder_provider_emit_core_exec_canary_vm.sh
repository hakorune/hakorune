#!/usr/bin/env bash
set -euo pipefail
# Purpose: Provider route (env.mirbuilder.emit) produces MIR(JSON v0) and Core executes it (rc parity)

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

# Prepare a tiny Program(JSON v0) that returns 42
prog_json_path="/tmp/prog_2044_emit_$$.json"
cat >"$prog_json_path" <<'JSON'
{"version":0,"kind":"Program","body":[
  {"type":"Return","expr":{"type":"Int","value":42}}
]}
JSON

# Case A: real provider route via MirBuilder(delegate)
set +e
HAKO_PREFER_MIRBUILDER=1 \
NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
NYASH_USING_AST=1 NYASH_RESOLVE_FIX_BRACES=1 \
NYASH_DISABLE_NY_COMPILER=1 NYASH_FEATURES=stage3 \
NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN=1 \
verify_program_via_builder_to_core "$prog_json_path"
rc=$?
set -e
if [ "$rc" -ne 42 ]; then
  echo "[FAIL] provider emit → core rc=$rc (expected 42)" >&2
  rm -f "$prog_json_path"
  exit 1
fi

# Case B: stub provider enabled; harness should fallback to Rust CLI and still yield rc=42
set +e
HAKO_V1_EXTERN_PROVIDER=1 \
HAKO_PREFER_MIRBUILDER=1 \
NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
NYASH_USING_AST=1 NYASH_RESOLVE_FIX_BRACES=1 \
NYASH_DISABLE_NY_COMPILER=1 NYASH_FEATURES=stage3 \
NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN=1 \
verify_program_via_builder_to_core "$prog_json_path"
rc2=$?
set -e
rm -f "$prog_json_path"
if [ "$rc2" -ne 42 ]; then
  echo "[FAIL] stub provider (fallback) rc=$rc2 (expected 42)" >&2
  exit 1
fi

echo "[PASS] phase2044/mirbuilder_provider_emit_core_exec_canary_vm"
exit 0
