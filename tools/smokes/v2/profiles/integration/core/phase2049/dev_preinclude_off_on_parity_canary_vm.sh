#!/bin/bash
# dev: relative using + preinclude OFF/ON で rc が一致（0）
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

code='using "lang/src/vm/boxes/mini_vm_entry.hako" as MiniVmEntryBox
static box Main { method main(args) { return 0 } }'

set +e
out1=$(NYASH_USING_PROFILE=dev NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 NYASH_ALLOW_USING_FILE=1 NYASH_PREINCLUDE=0 run_nyash_vm -c "$code" 2>&1)
rc1=$?
out2=$(NYASH_USING_PROFILE=dev NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 NYASH_ALLOW_USING_FILE=1 NYASH_PREINCLUDE=1 run_nyash_vm -c "$code" 2>&1)
rc2=$?
set -e

if [ "$rc1" -eq 0 ] && [ "$rc2" -eq 0 ]; then
  echo "[PASS] dev_preinclude_off_on_parity_canary_vm"
  exit 0
fi
echo "[FAIL] dev_preinclude_off_on_parity_canary_vm (rc1=$rc1 rc2=$rc2)" >&2
echo "$out1" | tail -n 20 >&2
echo "$out2" | tail -n 20 >&2
exit 1
