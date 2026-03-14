#!/usr/bin/env bash
# Opt-in canary for MirBuilderMin: return.method.arraymap (len/size alias)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT_DIR="$ROOT_GIT"
else
  ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
fi
source "$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

# Return(Method Var recv 'a', method 'len', args []) -> alias to size
PROG='{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Method","recv":{"type":"Var","name":"a"},"method":"len","args":[]}}]}'

run_builder_module_tag_canary \
  "hako.mir.builder.min" \
  "$PROG" \
  "\[mirbuilder/min:return.method.arraymap\]" \
  "builder_min_method_arraymap_len" \
  "builder-min vm exec failed" \
  "min tag not observed (len)" \
  "MIR missing functions (min/len)"
