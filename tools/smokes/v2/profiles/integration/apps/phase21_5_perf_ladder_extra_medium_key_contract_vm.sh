#!/bin/bash
# phase21_5_perf_ladder_extra_medium_key_contract_vm.sh
#
# Contract pin:
# - run_progressive_ladder supports opt-in extra medium keys via PERF_LADDER_EXTRA_MEDIUM_KEYS.
# - compare_reuse_small can be added without changing default medium key set.

set -euo pipefail

SMOKE_NAME="phase21_5_perf_ladder_extra_medium_key_contract_vm"

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

LADDER="$NYASH_ROOT/tools/perf/run_progressive_ladder_21_5.sh"
BENCH_HAKO="$NYASH_ROOT/benchmarks/bench_compare_reuse_small.hako"
BENCH_C="$NYASH_ROOT/benchmarks/c/bench_compare_reuse_small.c"

for f in "$LADDER" "$BENCH_HAKO" "$BENCH_C"; do
  if [ ! -f "$f" ]; then
    test_fail "$SMOKE_NAME: missing file: $f"
    exit 2
  fi
done

# Static contract: default medium keys remain unchanged, extras are appended via env.
if ! grep -q 'MEDIUM_KEYS=(box_create_destroy method_call_only)' "$LADDER"; then
  test_fail "$SMOKE_NAME: default MEDIUM_KEYS contract changed"
  exit 1
fi
if ! grep -q 'PERF_LADDER_EXTRA_MEDIUM_KEYS' "$LADDER"; then
  test_fail "$SMOKE_NAME: EXTRA_MEDIUM_KEYS wiring not found"
  exit 1
fi

set +e
OUT=$(PERF_LADDER_EXTRA_MEDIUM_KEYS=compare_reuse_small \
  PERF_LADDER_AOT_SMALL=0 \
  PERF_LADDER_AOT_MEDIUM=0 \
  PERF_LADDER_APPS=0 \
  PERF_LADDER_STABILITY=0 \
  PERF_LADDER_REGRESSION_GUARD=0 \
  NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
  bash "$LADDER" quick 2>&1)
RC=$?
set -e
if [ "$RC" -ne 0 ]; then
  printf '%s\n' "$OUT"
  test_fail "$SMOKE_NAME: ladder run failed rc=$RC"
  exit 1
fi

if ! printf '%s\n' "$OUT" | grep -q '\[ladder\] medium bench: compare_reuse_small'; then
  printf '%s\n' "$OUT"
  test_fail "$SMOKE_NAME: missing ladder medium step for compare_reuse_small"
  exit 1
fi

if ! printf '%s\n' "$OUT" | grep -q '\[bench\] name=compare_reuse_small'; then
  printf '%s\n' "$OUT"
  test_fail "$SMOKE_NAME: missing compare_reuse_small bench output"
  exit 1
fi

test_pass "$SMOKE_NAME"
