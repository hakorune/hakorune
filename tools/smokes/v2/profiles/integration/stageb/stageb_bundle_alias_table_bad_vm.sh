#!/bin/bash
# stageb_bundle_alias_table_bad_vm.sh — Stage‑B: malformed alias table should fail (opt‑in)

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
source "$ROOT/tools/smokes/v2/lib/stageb_helpers.sh"
require_env || exit 2

if [ "${SMOKES_ENABLE_STAGEB:-0}" != "1" ]; then
  echo "[SKIP] stageb_bundle_alias_table_bad_vm (SMOKES_ENABLE_STAGEB=1 to enable)"
  exit 0
fi

main='static box Main { method main(args) { return 0 } }'

# Malformed table: missing colon; empty name/code entries
export HAKO_BUNDLE_ALIAS_TABLE='U1|||BadEntryNoColon|||
:codeOnly|||NameOnly:'

if json=$(stageb_compile_to_json_with_require "$main" "U1"); then
  echo "[FAIL] stageb_bundle_alias_table_bad_vm (unexpected success)" >&2
  test -f "$json" && rm -f "$json"
  exit 1
else
  echo "[PASS] stageb_bundle_alias_table_bad_vm"
  exit 0
fi

