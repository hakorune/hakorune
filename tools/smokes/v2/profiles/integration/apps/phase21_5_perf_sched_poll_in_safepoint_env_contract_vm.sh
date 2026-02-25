#!/bin/bash
# phase21_5_perf_sched_poll_in_safepoint_env_contract_vm.sh
#
# Contract pin:
# - NYASH_SCHED_POLL_IN_SAFEPOINT accepts only 0|1|off|on|false|true.
# - invalid value must fail-fast with [freeze:contract][sched/poll_in_safepoint].
# - policy is independent from GC mode: both default and explicit 0 must execute
#   an AOT executable under NYASH_GC_MODE=off.

set -euo pipefail

SMOKE_NAME="phase21_5_perf_sched_poll_in_safepoint_env_contract_vm"
KEY="chip8_kernel_small"

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

source "$NYASH_ROOT/tools/perf/lib/aot_helpers.sh"

HAKORUNE_BIN="$NYASH_ROOT/target/release/hakorune"
HAKO_PROG="$NYASH_ROOT/benchmarks/bench_${KEY}.hako"
OUT_EXE="$NYASH_ROOT/target/tmp_perf_probe/${KEY}.sched_poll_contract.$$.exe"

if [ ! -x "$HAKORUNE_BIN" ]; then
  test_fail "$SMOKE_NAME: missing hakorune binary: $HAKORUNE_BIN"
  exit 2
fi
if [ ! -f "$HAKO_PROG" ]; then
  test_fail "$SMOKE_NAME: fixture missing: $HAKO_PROG"
  exit 2
fi

mkdir -p "$NYASH_ROOT/target/tmp_perf_probe"

if ! perf_emit_and_build_aot_exe "$NYASH_ROOT" "$HAKORUNE_BIN" "$HAKO_PROG" "$OUT_EXE"; then
  test_fail "$SMOKE_NAME: failed to emit/build AOT exe status=$PERF_AOT_LAST_STATUS reason=$PERF_AOT_LAST_REASON stage=$PERF_AOT_LAST_STAGE"
  exit 2
fi
trap 'rm -f "$OUT_EXE"' EXIT

# Case 1: invalid env value must fail-fast.
set +e
bad_out="$(
  NYASH_SCHED_POLL_IN_SAFEPOINT=2 \
  timeout 20s "$OUT_EXE" 2>&1
)"
bad_rc=$?
set -e
if [ "$bad_rc" -eq 0 ]; then
  printf '%s\n' "$bad_out"
  test_fail "$SMOKE_NAME: invalid NYASH_SCHED_POLL_IN_SAFEPOINT unexpectedly succeeded"
  exit 1
fi
if [ "$bad_rc" -eq 124 ]; then
  printf '%s\n' "$bad_out"
  test_fail "$SMOKE_NAME: invalid poll env case timed out"
  exit 1
fi
if ! printf '%s\n' "$bad_out" | grep -q '\[freeze:contract\]\[sched/poll_in_safepoint\]'; then
  printf '%s\n' "$bad_out"
  test_fail "$SMOKE_NAME: missing fail-fast tag for invalid poll env"
  exit 1
fi

# Case 2: default policy (unset) must run with GC off.
set +e
default_out="$(
  env -u NYASH_SCHED_POLL_IN_SAFEPOINT \
    NYASH_GC_MODE=off \
    timeout 20s "$OUT_EXE" 2>&1
)"
default_rc=$?
set -e
if [ "$default_rc" -eq 124 ]; then
  printf '%s\n' "$default_out"
  test_fail "$SMOKE_NAME: default poll policy timed out under GC off"
  exit 1
fi
if ! printf '%s\n' "$default_out" | grep -q '^Result:'; then
  printf '%s\n' "$default_out"
  test_fail "$SMOKE_NAME: default poll policy missing Result output"
  exit 1
fi

# Case 3: explicit poll-off must also run with GC off.
set +e
off_out="$(
  NYASH_GC_MODE=off \
  NYASH_SCHED_POLL_IN_SAFEPOINT=0 \
  timeout 20s "$OUT_EXE" 2>&1
)"
off_rc=$?
set -e
if [ "$off_rc" -eq 124 ]; then
  printf '%s\n' "$off_out"
  test_fail "$SMOKE_NAME: explicit poll-off timed out under GC off"
  exit 1
fi
if ! printf '%s\n' "$off_out" | grep -q '^Result:'; then
  printf '%s\n' "$off_out"
  test_fail "$SMOKE_NAME: explicit poll-off missing Result output"
  exit 1
fi

if [ "$default_rc" -ne "$off_rc" ]; then
  printf '%s\n' "$default_out"
  printf '%s\n' "$off_out"
  test_fail "$SMOKE_NAME: GC-off parity mismatch between default poll and explicit poll-off (default=$default_rc off=$off_rc)"
  exit 1
fi

test_pass "$SMOKE_NAME"
