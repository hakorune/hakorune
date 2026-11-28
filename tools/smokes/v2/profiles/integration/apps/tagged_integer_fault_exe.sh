#!/usr/bin/env bash
# A non-integer borrowed argument records TypeError before the EXE exits.
set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

runtime="${HAKO_LIFECYCLE_NYRT:-${NYASH_EMIT_EXE_NYRT:-$HAKO_ROOT/target/quick}}"
timeout_secs="${RUN_TIMEOUT_SECS:-120}"
work="$(mktemp -d "${TMPDIR:-/tmp}/hakorune-tagged-integer-fault.XXXXXXXX")"
trap 'rm -r -- "$work"' EXIT

export NYASH_DISABLE_PLUGINS=1
export HAKO_BACKEND_COMPILE_RECIPE=pure-first
export HAKO_BACKEND_COMPAT_REPLAY=none

for actual in true false null 9; do
  src="$work/sizebin-$actual.hako"
  cp "$HAKO_ROOT/lang/src/hako_alloc/memory/size_class_box.hako" "$src"
  printf '\nstatic box Main { main() { return SizeClassBox.size_to_bin(%s) } }\n' "$actual" >> "$src"
  if ! timeout "$timeout_secs" "$NYASH_BIN" --backend mir --verify \
      --emit-exe "$work/program-$actual" --emit-exe-nyrt "$runtime" "$src" >"$work/build.log" 2>&1; then
    cat "$work/build.log"
    test_fail "tagged_integer_fault_exe: EXE generation failed for $actual"
    exit 1
  fi

  status=0
  timeout "$timeout_secs" "$work/program-$actual" >"$work/stdout" 2>"$work/stderr" || status=$?
  primary="$(grep -F '[fault:primary]' "$work/stderr" || true)"
  expected_status=70
  expected_primary='[fault:primary] reason=302 site=0 details=0,0'
  if [ "$actual" = 9 ]; then
    expected_status=2
    expected_primary=''
  fi
  if [ "$status" -ne "$expected_status" ] || [ "$primary" != "$expected_primary" ]; then
    cat "$work/stdout" "$work/stderr"
    test_fail "tagged_integer_fault_exe: $actual exited $status, expected $expected_status and primary '$expected_primary'"
    exit 1
  fi
done

test_pass "tagged_integer_fault_exe"
