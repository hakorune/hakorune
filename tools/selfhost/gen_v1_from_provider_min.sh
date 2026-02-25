#!/usr/bin/env bash
# gen_v1_from_provider_min.sh — Generate MIR JSON v1 via provider env.mirbuilder.emit (minimal const42)
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
source "$ROOT/tools/smokes/v2/lib/test_runner.sh" 2>/dev/null || true

prog_json='{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Int","value":42}}]}'

code=$(cat <<'HCODE'
static box Main { method main(args) {
  local p = env.get("NYASH_PROGRAM_JSON")
  local out = hostbridge.extern_invoke("env.mirbuilder", "emit", p)
  print("" + out)
  return 0
} }
HCODE
)

NYASH_PROGRAM_JSON="$prog_json" \
NYASH_JSON_SCHEMA_V1=1 \
NYASH_USING_AST=1 NYASH_DISABLE_NY_COMPILER=1 HAKO_DISABLE_NY_COMPILER=1 \
NYASH_FEATURES=stage3 NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 \
run_nyash_vm -c "$code" 2>/dev/null | tr -d '\r'

