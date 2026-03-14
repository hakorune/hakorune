#!/usr/bin/env bash
# Opt-in canary: MirBuilder registry if.compare.fold.varint (Binary(Var,Int) folded via local)
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
enable_mirbuilder_dev_env

# Program(JSON v0): local a=1; if (a+1)==2 then return 1 else return 0
PROG='{"version":0,"kind":"Program","body":[{"type":"Local","name":"a","expr":{"type":"Int","value":1}},{"type":"If","cond":{"type":"Compare","op":"==","lhs":{"type":"Binary","op":"+","lhs":{"type":"Var","name":"a"},"rhs":{"type":"Int","value":1}},"rhs":{"type":"Int","value":2}},"then":[{"type":"Return","expr":{"type":"Int","value":1}}],"else":[{"type":"Return","expr":{"type":"Int","value":0}}]}]}'

run_registry_builder_tag_canary \
  "hako.mir.builder.min" \
  "$PROG" \
  "" \
  "\\[mirbuilder/(min|registry):(if\\.compare\\.fold\\.varint|if\\.compare\\.varint)\\]" \
  "registry_optin_compare_fold_varint" \
  "builder vm exec failed" \
  "registry/min tag not observed (if.compare.fold.varint|varint)" \
  "MIR missing functions"
