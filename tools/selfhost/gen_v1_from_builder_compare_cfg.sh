#!/usr/bin/env bash
# gen_v1_from_builder_compare_cfg.sh — v1 JSON (compare/branch CFG)
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
source "$ROOT/tools/smokes/v2/lib/test_runner.sh" 2>/dev/null || true

code=$(cat <<'HCODE'
using lang.compiler.pipeline_v2.emit_compare_box as EmitCompareBox
static box Main { method main(args) {
  // if (3 < 5) ret 1 else ret 0
  local j = EmitCompareBox.emit_compare_cfg(3, 5, "Lt")
  print("" + j)
  return 0
} }
HCODE
)

NYASH_USING_AST=1 NYASH_DISABLE_NY_COMPILER=1 HAKO_DISABLE_NY_COMPILER=1 \
NYASH_FEATURES=stage3 NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 \
run_nyash_vm -c "$code" 2>/dev/null | tr -d '\r'

