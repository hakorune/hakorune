#!/bin/bash
# phase21_5_perf_regression_guard_contract_vm.sh
#
# Contract pin:
# - Phase 21.5 medium/app regression guard must produce lock lines and end with ok.
# - Guard is expected to fail-fast when baseline contract is broken.
# - Smoke defaults are stability-tuned (repeat/retries/threshold) to avoid host jitter flakes.

set -euo pipefail

SMOKE_NAME="phase21_5_perf_regression_guard_contract_vm"

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2
GUARD_SCRIPT="$NYASH_ROOT/tools/checks/phase21_5_perf_regression_guard.sh"

if [ ! -f "$GUARD_SCRIPT" ]; then
  test_fail "$SMOKE_NAME: guard script not found: $GUARD_SCRIPT"
  exit 2
fi

OUT="$(
  PERF_REG_GUARD_WARMUP="${PERF_REG_GUARD_WARMUP:-1}" \
  PERF_REG_GUARD_REPEAT="${PERF_REG_GUARD_REPEAT:-3}" \
  PERF_REG_GUARD_APPS_RETRIES="${PERF_REG_GUARD_APPS_RETRIES:-5}" \
  PERF_REG_GUARD_AOT_MAX_DEGRADE_PCT="${PERF_REG_GUARD_AOT_MAX_DEGRADE_PCT:-60}" \
  PERF_REG_GUARD_APPS_MAX_DEGRADE_PCT="${PERF_REG_GUARD_APPS_MAX_DEGRADE_PCT:-35}" \
  PERF_REG_GUARD_PER_APP_MAX_DEGRADE_PCT="${PERF_REG_GUARD_PER_APP_MAX_DEGRADE_PCT:-40}" \
  NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
  bash "$GUARD_SCRIPT" 2>&1
)" || {
  echo "$OUT"
  test_fail "$SMOKE_NAME: regression guard failed"
  exit 1
}

if ! printf '%s\n' "$OUT" | grep -q '\[phase21_5-perf-regression-guard\] baseline_aot_ms='; then
  echo "$OUT"
  test_fail "$SMOKE_NAME: missing baseline_aot_ms lock line"
  exit 1
fi

if ! printf '%s\n' "$OUT" | grep -q '\[phase21_5-perf-regression-guard\] baseline_apps_total_ms='; then
  echo "$OUT"
  test_fail "$SMOKE_NAME: missing baseline_apps_total_ms lock line"
  exit 1
fi

if ! printf '%s\n' "$OUT" | grep -q '\[phase21_5-perf-regression-guard\] baseline_app_ms\['; then
  echo "$OUT"
  test_fail "$SMOKE_NAME: missing per-app lock lines"
  exit 1
fi

if ! printf '%s\n' "$OUT" | grep -q '\[phase21_5-perf-regression-guard\] apps_hotspot baseline='; then
  echo "$OUT"
  test_fail "$SMOKE_NAME: missing hotspot line"
  exit 1
fi

if printf '%s\n' "$OUT" | grep -q '\[phase21_5-perf-regression-guard\] baseline_entry_source_total_ms='; then
  if ! printf '%s\n' "$OUT" | grep -q '\[phase21_5-perf-regression-guard\] baseline_entry_prebuilt_total_ms='; then
    echo "$OUT"
    test_fail "$SMOKE_NAME: missing baseline_entry_prebuilt_total_ms lock line"
    exit 1
  fi
  if ! printf '%s\n' "$OUT" | grep -q '\[phase21_5-perf-regression-guard\] baseline_entry_delta_abs_ms='; then
    echo "$OUT"
    test_fail "$SMOKE_NAME: missing baseline_entry_delta_abs_ms lock line"
    exit 1
  fi
  if ! printf '%s\n' "$OUT" | grep -q '\[phase21_5-perf-regression-guard\] entry_mode_hotspot baseline='; then
    echo "$OUT"
    test_fail "$SMOKE_NAME: missing entry_mode_hotspot line"
    exit 1
  fi
elif ! printf '%s\n' "$OUT" | grep -q '\[phase21_5-perf-regression-guard\] entry_mode baseline missing; skip'; then
  echo "$OUT"
  test_fail "$SMOKE_NAME: missing entry_mode lock line (either baseline lock or skip line)"
  exit 1
fi

if ! printf '%s\n' "$OUT" | grep -q '\[phase21_5-perf-regression-guard\] ok'; then
  echo "$OUT"
  test_fail "$SMOKE_NAME: missing final ok line"
  exit 1
fi

test_pass "$SMOKE_NAME"
