#!/usr/bin/env bash
# Opt-in canary for MirBuilder registry path (HAKO_MIR_BUILDER_REGISTRY=1).
# Runs Hako MirBuilderBox in VM with a minimal Program(JSON v0) and checks MIR(JSON) presence.
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

# Minimal Program(JSON v0): If(Compare Int==Int) → Return(Int)/Return(Int)
PROG='{"version":0,"kind":"Program","body":[{"type":"If","cond":{"type":"Compare","op":"==","lhs":{"type":"Int","value":1},"rhs":{"type":"Int","value":1}},"then":[{"type":"Return","expr":{"type":"Int","value":42}}],"else":[{"type":"Return","expr":{"type":"Int","value":0}}]}]}'

run_registry_builder_tag_canary \
  "hako.mir.builder" \
  "$PROG" \
  "" \
  "\\[mirbuilder/registry:if.compare.intint\\]" \
  "registry_optin" \
  "builder vm exec failed" \
  "registry tag not observed" \
  "MIR without functions (registry)"
