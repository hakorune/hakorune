#!/usr/bin/env bash
# Opt-in canary for MirBuilder registry path: if.compare.varint
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

# Program(JSON v0): Local i=1; If (i == 1) then Return(12) else Return(0)
PROG='{"version":0,"kind":"Program","body":[{"type":"Local","name":"i","expr":{"type":"Int","value":1}},{"type":"If","cond":{"type":"Compare","op":"==","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":1}},"then":[{"type":"Return","expr":{"type":"Int","value":12}}],"else":[{"type":"Return","expr":{"type":"Int","value":0}}]}]}'

run_registry_builder_tag_canary \
  "hako.mir.builder" \
  "$PROG" \
  "" \
  "\\[mirbuilder/registry:if.compare.varint\\]" \
  "registry_optin_compare_varint" \
  "builder vm exec failed" \
  "registry/min tag not observed (if.compare.varint)" \
  "MIR missing functions"
