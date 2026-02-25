#!/bin/bash
# prod: nested prelude via alias using（JsonFieldExtractor まで解決）
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

code='using selfhost.vm.hakorune-vm.json_field_extractor as JsonFieldExtractor
static box Main { method main(args) {
  local v = JsonFieldExtractor.extract_int("{\\"dst\\":3}", "dst")
  return 0
} }'

set +e
rc=$(NYASH_USING_PROFILE=prod run_nyash_vm -c "$code" >/dev/null 2>&1; echo $?)
set -e

if [ "$rc" -eq 0 ]; then
  echo "[PASS] prod_nested_prelude_field_extractor_canary_vm"
  exit 0
fi
echo "[FAIL] prod_nested_prelude_field_extractor_canary_vm (rc=$rc)" >&2
exit 1

