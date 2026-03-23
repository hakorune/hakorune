#!/usr/bin/env bash
# Archived monitor-only probe: call LowerReturnMethodArrayMapBox.try_lower
# without full MirBuilder. This is no longer part of active closeout/gates.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT_DIR="$ROOT_GIT"
else
  ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
fi

source "$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

PROG='{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Method","recv":{"type":"Var","name":"a"},"method":"get","args":[{"type":"Int","value":0}]}}]}'

run_direct_lower_box_canary \
  "hako.mir.builder.internal.lower_return_method_array_map" \
  "try_lower" \
  "$PROG" \
  "registry_optin_method_arraymap_direct"
exit 0
