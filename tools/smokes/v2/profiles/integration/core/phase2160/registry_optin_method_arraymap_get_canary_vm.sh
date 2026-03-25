#!/usr/bin/env bash
# Opt-in canary for MirBuilder registry path: return.method.arraymap (get)
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

# Program(JSON v0): Return(Method Var recv 'a', method 'get', args [Int 0])
PROG='{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Method","recv":{"type":"Var","name":"a"},"method":"get","args":[{"type":"Int","value":0}]}}]}'

run_registry_method_arraymap_canary \
  "$PROG" \
  "return.method.arraymap" \
  "[mirbuilder/registry:return.method.arraymap]" \
  "registry_optin_method_arraymap_get" \
  '"method":"get"' \
  '"args":\[[0-9]' \
  1
