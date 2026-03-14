#!/usr/bin/env bash
# Opt-in canary for MirBuilderMin: return.binop var+var with prior Local Ints
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT_DIR="$ROOT_GIT"
else
  ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
fi
source "$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

# Program: Local a=2; Local b=5; Return(Binary '+', Var a, Var b)
PROG='{"version":0,"kind":"Program","body":[{"type":"Local","name":"a","expr":{"type":"Int","value":2}},{"type":"Local","name":"b","expr":{"type":"Int","value":5}},{"type":"Return","expr":{"type":"Binary","op":"+","lhs":{"type":"Var","name":"a"},"rhs":{"type":"Var","name":"b"}}}]}'

run_builder_module_tag_canary \
  "hako.mir.builder.min" \
  "$PROG" \
  "\[mirbuilder/min:return.binop.varvar\]" \
  "builder_min_return_binop_varvar" \
  "builder-min vm exec failed" \
  "min tag not observed (binop varvar)" \
  "MIR missing functions (min/binop varvar)" \
  0 \
  1
