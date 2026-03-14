#!/usr/bin/env bash
# Opt-in canary: MirBuilder registry if.compare.fold.binints (Binary(Int,Int) folded)
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

# Program(JSON v0): If( (2+3) < (4+6) ) then return 1 else return 0
PROG='{"version":0,"kind":"Program","body":[{"type":"If","cond":{"type":"Compare","op":"<","lhs":{"type":"Binary","op":"+","lhs":{"type":"Int","value":2},"rhs":{"type":"Int","value":3}},"rhs":{"type":"Binary","op":"+","lhs":{"type":"Int","value":4},"rhs":{"type":"Int","value":6}}},"then":[{"type":"Return","expr":{"type":"Int","value":1}}],"else":[{"type":"Return","expr":{"type":"Int","value":0}}]}]}'

run_registry_builder_tag_canary \
  "hako.mir.builder.min" \
  "$PROG" \
  "" \
  "\\[mirbuilder/(min|registry):(if.compare.fold.binints|if.compare.intint)]|registry):if.compare.fold.binints\\]" \
  "registry_optin_compare_fold_binints" \
  "builder vm exec failed" \
  "registry/min tag not observed (if.compare.fold.binints)" \
  "MIR missing functions"
