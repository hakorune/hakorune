#!/bin/bash
# phase21_5_perf_shellcheck_contract_vm.sh
#
# Contract pin:
# - Shell lint for perf/check scripts is available as optional gate.
# - If shellcheck is not installed, this contract is a skip-pass (non-blocking).

set -euo pipefail

SMOKE_NAME="phase21_5_perf_shellcheck_contract_vm"

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

if ! command -v shellcheck >/dev/null 2>&1; then
  test_pass "$SMOKE_NAME: skip (shellcheck not installed)"
  exit 0
fi

FILES=(
  "$NYASH_ROOT/tools/checks/lib/perf_guard_common.sh"
  "$NYASH_ROOT/tools/checks/lib/perf_guard_apps.sh"
  "$NYASH_ROOT/tools/checks/lib/perf_guard_entry_mode.sh"
  "$NYASH_ROOT/tools/checks/phase21_5_perf_regression_guard.sh"
  "$NYASH_ROOT/tools/perf/lib/compare_mode_common.sh"
  "$NYASH_ROOT/tools/perf/bench_apps_entry_mode_compare.sh"
  "$NYASH_ROOT/tools/perf/bench_apps_mir_mode_compare.sh"
)

for f in "${FILES[@]}"; do
  if [ ! -f "$f" ]; then
    test_fail "$SMOKE_NAME: target not found: $f"
    exit 2
  fi
done

OUT="$(shellcheck -x "${FILES[@]}" 2>&1)" || {
  echo "$OUT"
  test_fail "$SMOKE_NAME: shellcheck failed"
  exit 1
}

test_pass "$SMOKE_NAME"
