#!/usr/bin/env bash
# gen_v1_from_provider.sh — Generate MIR JSON v1 via provider env.mirbuilder.emit (dev)
# Produces a minimal v1 program that returns 42.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
source "$ROOT/tools/smokes/v2/lib/test_runner.sh" 2>/dev/null || true

# Build Program(JSON v0) inline (version:0, kind:"Program")
prog_json=$(cat <<'JSON'
{
  "version": 0,
  "kind": "Program",
  "body": [
    { "type": "Return", "expr": { "type": "Int", "value": 42 } }
  ]
}
JSON
)

code=$(cat <<'HCODE'
static box Main { method main(args) {
  local p = env.get("NYASH_PROGRAM_JSON")
  // 直接 hostbridge.extern_invoke を呼び出して v1 JSON を取得
  local out = hostbridge.extern_invoke("env.mirbuilder", "emit", p)
  print("" + out)
  return 0
} }
HCODE
)

# Run via Rust VM (dev flags similar to test_runner)
NYASH_PROGRAM_JSON="$prog_json" \
NYASH_JSON_SCHEMA_V1=1 \
NYASH_USING_AST=1 NYASH_DISABLE_NY_COMPILER=1 HAKO_DISABLE_NY_COMPILER=1 \
NYASH_FEATURES=stage3 NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 \
run_nyash_vm -c "$code" 2>/dev/null | tr -d '\r'
