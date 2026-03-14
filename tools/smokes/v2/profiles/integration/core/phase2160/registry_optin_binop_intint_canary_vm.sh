#!/usr/bin/env bash
# Opt-in canary for MirBuilder registry path: return.binop.intint
# Builds via Hako MirBuilderBox in VM from minimal Program(JSON v0) and checks registry/min tag.
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

# Minimal Program(JSON v0): Return(Binary op "+", Int 2, Int 5) → rc=7
PROG='{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Binary","op":"+","lhs":{"type":"Int","value":2},"rhs":{"type":"Int","value":5}}}]}'

run_registry_builder_tag_canary \
  "hako.mir.builder" \
  "$PROG" \
  "return.binop.intint" \
  "\\[mirbuilder/registry:return.binop.intint\\]" \
  "registry_optin_binop_intint" \
  "builder vm exec failed" \
  "registry tag not observed (binop.intint)" \
  "MIR missing functions"
