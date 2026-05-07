#!/bin/bash
# mimalloc-lite real-app smoke (VM)
#
# Contract pin:
# - fixed-size page selection works
# - free-list reuse and peak accounting stay deterministic

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

APP="$HAKO_ROOT/apps/mimalloc-lite/main.hako"

if [ ! -f "$APP" ]; then
  test_fail "mimalloc_lite_vm: App not found: $APP"
  exit 2
fi

output=$(run_hako_vm_release "$APP")

expected=$(cat << 'TXT'
mimalloc-lite
small_allocs=9 frees=3 reused=3 peak=6 free=2
medium_allocs=4 frees=1 reused=1 peak=3 free=1
requested_bytes=360
outstanding=9
summary=ok
TXT
)

compare_outputs "$expected" "$output" "mimalloc_lite_vm" || exit 1

test_pass "mimalloc_lite_vm"
