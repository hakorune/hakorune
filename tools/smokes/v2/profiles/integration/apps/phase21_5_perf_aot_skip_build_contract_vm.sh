#!/bin/bash
# phase21_5_perf_aot_skip_build_contract_vm.sh
#
# Contract pin:
# - PERF_AOT_SKIP_BUILD resolves as documented by perf_aot_resolve_skip_build:
#   - auto: 1 only when required release artifacts exist
#   - 0|1: explicit override
#   - invalid: fail-fast (rc!=0)

set -euo pipefail

SMOKE_NAME="phase21_5_perf_aot_skip_build_contract_vm"

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

HELPER="$NYASH_ROOT/tools/perf/lib/aot_helpers.sh"
if [ ! -f "$HELPER" ]; then
  test_fail "$SMOKE_NAME: helper missing: $HELPER"
  exit 2
fi

source "$HELPER"

TMP_ROOT="$(mktemp -d "/tmp/${SMOKE_NAME}.XXXXXX")"
cleanup() {
  rm -rf "$TMP_ROOT" >/dev/null 2>&1 || true
}
trap cleanup EXIT

resolve_skip_build() {
  local mode="$1"
  local root="$2"
  PERF_AOT_SKIP_BUILD="$mode" perf_aot_resolve_skip_build "$root"
}

assert_equals() {
  local expected="$1"
  local actual="$2"
  local label="$3"
  if [ "$expected" != "$actual" ]; then
    test_fail "$SMOKE_NAME: ${label} expected=$expected actual=$actual"
    exit 1
  fi
}

# auto / missing artifacts -> 0
actual="$(resolve_skip_build "auto" "$TMP_ROOT")"
assert_equals "0" "$actual" "auto-without-artifacts"

# auto / complete artifacts -> 1
mkdir -p "$TMP_ROOT/target/release"
touch "$TMP_ROOT/target/release/hakorune"
touch "$TMP_ROOT/target/release/ny-llvmc"
touch "$TMP_ROOT/target/release/libnyash_kernel.a"
chmod +x "$TMP_ROOT/target/release/hakorune"
chmod +x "$TMP_ROOT/target/release/ny-llvmc"
actual="$(resolve_skip_build "auto" "$TMP_ROOT")"
assert_equals "1" "$actual" "auto-with-artifacts"

# explicit override must win
actual="$(resolve_skip_build "0" "$TMP_ROOT")"
assert_equals "0" "$actual" "explicit-0"
actual="$(resolve_skip_build "1" "$TMP_ROOT")"
assert_equals "1" "$actual" "explicit-1"

# invalid value -> fail-fast
set +e
invalid_out="$(resolve_skip_build "invalid" "$TMP_ROOT" 2>&1)"
invalid_rc=$?
set -e
if [ "$invalid_rc" -eq 0 ]; then
  printf '%s\n' "$invalid_out"
  test_fail "$SMOKE_NAME: invalid value unexpectedly succeeded"
  exit 1
fi
if ! printf '%s\n' "$invalid_out" | grep -q 'PERF_AOT_SKIP_BUILD must be auto|0|1'; then
  printf '%s\n' "$invalid_out"
  test_fail "$SMOKE_NAME: missing fail-fast error for invalid value"
  exit 1
fi

test_pass "$SMOKE_NAME"
