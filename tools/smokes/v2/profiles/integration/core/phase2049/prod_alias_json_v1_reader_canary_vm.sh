#!/bin/bash
# prod: alias using (selfhost.vm.hakorune-vm.json_v1_reader) resolves (rc=0)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

code='using selfhost.vm.hakorune-vm.json_v1_reader as JsonV1ReaderBox
static box Main { method main(args) { return 0 } }'

set +e
rc=$(NYASH_USING_PROFILE=prod run_nyash_vm -c "$code" >/dev/null 2>&1; echo $?)
set -e

if [ "$rc" -eq 0 ]; then
  echo "[PASS] prod_alias_json_v1_reader_canary_vm"
  exit 0
fi
echo "[FAIL] prod_alias_json_v1_reader_canary_vm (rc=$rc)" >&2
exit 1

