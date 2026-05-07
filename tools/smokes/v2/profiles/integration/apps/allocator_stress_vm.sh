#!/bin/bash
# allocator-stress real-app smoke (VM)
#
# Contract pin:
# - hako_alloc page/free-list seam handles saturation and reuse
# - reject accounting stays deterministic

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

APP="$NYASH_ROOT/apps/allocator-stress/main.hako"

if [ ! -f "$APP" ]; then
  test_fail "allocator_stress_vm: App not found: $APP"
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
allocator-stress
small_allocs=11 frees=3 reused=3 peak=8 free=0
medium_allocs=6 frees=2 reused=2 peak=4 free=0
requested_bytes=454
outstanding=12
rejects=4
summary=ok
TXT
)

compare_outputs "$expected" "$output" "allocator_stress_vm" || exit 1

test_pass "allocator_stress_vm"
