#!/bin/bash
# mimalloc-lite real-app smoke (VM)
#
# Contract pin:
# - fixed-size page selection works
# - free-list reuse and peak accounting stay deterministic

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

APP="$NYASH_ROOT/apps/mimalloc-lite/main.hako"

if [ ! -f "$APP" ]; then
  test_fail "mimalloc_lite_vm: App not found: $APP"
  exit 2
fi

output=$(SMOKES_CLEAN_ENV=1 \
  NYASH_DISABLE_PLUGINS=1 \
  NYASH_VM_HAKO_PREFER_STRICT_DEV=0 \
  NYASH_VM_USE_FALLBACK=0 \
  NYASH_JOINIR_DEV=0 \
  HAKO_JOINIR_DEV=0 \
  NYASH_JOINIR_STRICT=0 \
  HAKO_JOINIR_STRICT=0 \
  run_nyash_vm "$APP")

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
