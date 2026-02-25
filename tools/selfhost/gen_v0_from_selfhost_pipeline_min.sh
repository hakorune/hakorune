#!/usr/bin/env bash
# gen_v0_from_selfhost_pipeline_min.sh — Hako selfhost pipeline v2 → MIR(JSON v0) emit
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
source "$ROOT/tools/smokes/v2/lib/test_runner.sh" 2>/dev/null || true

code=$(cat <<'HCODE'
using lang.compiler.pipeline_v2.emit_return_box as EmitReturnBox
static box Main { method main(args) {
  // Emit MIR(JSON v0) for return 42 (minimal)
  local j = EmitReturnBox.emit_return_int2(42, 0)
  print("" + j)
  return 0
} }
HCODE
)

NYASH_USING_PROFILE=dev NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
NYASH_DISABLE_NY_COMPILER=1 HAKO_DISABLE_NY_COMPILER=1 \
NYASH_FEATURES=stage3 NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 \
run_nyash_vm -c "$code" 2>/dev/null | tr -d '\r'

