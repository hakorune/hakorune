#!/usr/bin/env bash
# Opt-in canary for MirBuilder registry path: return.binop.varvar
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT_DIR="$ROOT_GIT"
else
  ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
fi
BIN="${ROOT_DIR}/target/release/hakorune"
if [[ ! -x "${BIN}" ]]; then echo "[SKIP] hakorune not built"; exit 0; fi

source "$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2
SMOKES_DEV_PREINCLUDE=1 enable_mirbuilder_dev_env

# Program(JSON v0): a=2; b=5; return a+b
PROG='{"version":0,"kind":"Program","body":[{"type":"Local","name":"a","expr":{"type":"Int","value":2}},{"type":"Local","name":"b","expr":{"type":"Int","value":5}},{"type":"Return","expr":{"type":"Binary","op":"+","lhs":{"type":"Var","name":"a"},"rhs":{"type":"Var","name":"b"}}}]}'

run_registry_builder_tag_canary \
  "hako.mir.builder.min" \
  "$PROG" \
  "" \
  "\\[mirbuilder/(min|registry):return.binop.varvar\\]" \
  "registry_optin_return_binop_varvar" \
  "builder vm exec failed" \
  "registry/min tag not observed (return.binop.varvar)" \
  "MIR missing functions" \
  1
