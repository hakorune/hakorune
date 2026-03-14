#!/usr/bin/env bash
# Opt-in canary for MirBuilderMin: if.compare var-int with prior Local Int
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT_DIR="$ROOT_GIT"
else
  ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
fi
source "$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

# Program: Local i=3; If(Compare '<', Var i, 5) then Return(1) else Return(0)
PROG='{"version":0,"kind":"Program","body":[{"type":"Local","name":"i","expr":{"type":"Int","value":3}},{"type":"If","cond":{"type":"Compare","op":"<","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":5}},"then":[{"type":"Return","expr":{"type":"Int","value":1}}],"else":[{"type":"Return","expr":{"type":"Int","value":0}}]}]}'

run_builder_module_tag_canary \
  "hako.mir.builder.min" \
  "$PROG" \
  "\[mirbuilder/min:if.compare.varint\]" \
  "builder_min_if_compare_varint" \
  "builder-min vm exec failed" \
  "min tag not observed (if varint)" \
  "MIR missing functions (min/if varint)"
