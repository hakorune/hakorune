#!/bin/bash
# RING1-CORE-09-min2: console provider smoke (VM)
#
# Contract pin:
# - ConsoleBox warn/error baseline stays stable while ring1 console provider is wired.
# - Exit code must be 0.
# - Output must match fixed multi-line contract.

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/ring1_console_provider/console_warn_error_min.hako"

if [ ! -f "$FIXTURE" ]; then
  test_fail "ring1_console_provider_vm: Fixture not found: $FIXTURE"
  exit 2
fi

set +e
output=$(NYASH_DISABLE_PLUGINS=1 \
         NYASH_VM_HAKO_PREFER_STRICT_DEV=0 \
         NYASH_VM_USE_FALLBACK=0 \
         NYASH_JOINIR_DEV=0 \
         NYASH_JOINIR_STRICT=0 \
         run_nyash_vm "$FIXTURE")
rc=$?
set -e

if [ "$rc" -ne 0 ]; then
  test_fail "ring1_console_provider_vm: expected rc=0, got $rc"
  exit 1
fi

expected=$(cat << 'TXT'
[Console WARN] ring1-console-warn
[Console ERROR] ring1-console-error
CONSOLE_PROVIDER_OK warn=1 error=1
TXT
)
compare_outputs "$expected" "$output" "ring1_console_provider_vm" || exit 1

test_pass "ring1_console_provider_vm"
