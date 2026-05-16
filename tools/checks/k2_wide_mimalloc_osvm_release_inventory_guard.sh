#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-osvm-release-inventory"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-293x/293x-528-MIMAP-048A-OSVM-RELEASE-CAPABILITY-INVENTORY.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-529-MIMAP-048B-POST-RELEASE-INVENTORY-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/mimalloc-osvm-release-capability-inventory-ssot.md"
UNRESERVE_CLOSEOUT="docs/development/current/main/design/mimalloc-osvm-fast-path-unreserve-closeout-ssot.md"
OSVM_CORE="lang/src/runtime/substrate/osvm/osvm_core_box.hako"
PAGE_SOURCE="lang/src/hako_alloc/memory/page_source_policy_box.hako"
UNRESERVE_ADAPTER="lang/src/hako_alloc/memory/purge_page_source_unreserve_adapter_box.hako"
FAST_PATH_UNRESERVE="lang/src/hako_alloc/memory/osvm_fast_path_unreserve_route_box.hako"
FAST_PATH_FAILFAST="lang/src/hako_alloc/memory/osvm_fast_path_unreserve_failfast_box.hako"
HOSTBRIDGE="lang/c-abi/include/hako_hostbridge.h"
KERNEL_SHIM="lang/c-abi/shims/hako_kernel.c"
EXTERN_PLAN="src/mir/extern_call_route_plan.rs"
VM_S0="lang/src/vm/boxes/mir_vm_s0_call_exec.hako"
INC_DIR="lang/c-abi/shims"
INDEX="docs/tools/check-scripts-index.md"

echo "[$TAG] checking MIMAP-048A OSVM release capability inventory"

guard_require_command "$TAG" rg
guard_require_files \
  "$TAG" \
  "$CARD" \
  "$NEXT_CARD" \
  "$DESIGN" \
  "$UNRESERVE_CLOSEOUT" \
  "$OSVM_CORE" \
  "$PAGE_SOURCE" \
  "$UNRESERVE_ADAPTER" \
  "$FAST_PATH_UNRESERVE" \
  "$FAST_PATH_FAILFAST" \
  "$HOSTBRIDGE" \
  "$KERNEL_SHIM" \
  "$EXTERN_PLAN" \
  "$VM_S0" \
  "$INDEX" \
  "$0"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-048A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "release inventory design must be accepted"
guard_expect_in_file "$TAG" 'Status: SSOT' "$UNRESERVE_CLOSEOUT" "unreserve closeout SSOT must remain present"
guard_expect_in_file "$TAG" 'MIMAP-048B' "$NEXT_CARD" "MIMAP-048B follow-up card must exist"
guard_expect_in_file "$TAG" "$0" "$INDEX" "check script index must list MIMAP-048A guard"

guard_expect_in_file "$TAG" 'hako_osvm_unreserve_bytes_i64' "$OSVM_CORE" "OsVmCoreBox must still own unreserve"
guard_expect_in_file "$TAG" 'unreservePage' "$PAGE_SOURCE" "page-source unreserve must remain explicit"
guard_expect_in_file "$TAG" 'HakoAllocPageSourceUnreserveAdapter' "$UNRESERVE_ADAPTER" "page-source unreserve adapter must remain explicit"
guard_expect_in_file "$TAG" 'HakoAllocOsVmFastPathUnreserveRoute' "$FAST_PATH_UNRESERVE" "fast-path unreserve route must remain explicit"
guard_expect_in_file "$TAG" 'HakoAllocOsVmFastPathUnreserveFailFastRoute' "$FAST_PATH_FAILFAST" "fast-path unreserve diagnostics must remain explicit"

if rg -n 'page-source release rows|release beyond the adapter seam' \
  "$UNRESERVE_ADAPTER" "$FAST_PATH_UNRESERVE" "$FAST_PATH_FAILFAST" >/tmp/"$TAG".release_comment_drift 2>&1; then
  echo "[$TAG] ERROR: unreserve owners must not describe themselves as release rows" >&2
  cat /tmp/"$TAG".release_comment_drift >&2
  rm -f /tmp/"$TAG".release_comment_drift
  exit 1
fi
rm -f /tmp/"$TAG".release_comment_drift

if rg -n 'hako_osvm_release|release_bytes_(i64|usize)|releasePage|HakoOsvmRelease' \
  "$OSVM_CORE" \
  "$PAGE_SOURCE" \
  "$UNRESERVE_ADAPTER" \
  "$FAST_PATH_UNRESERVE" \
  "$FAST_PATH_FAILFAST" \
  "$HOSTBRIDGE" \
  "$KERNEL_SHIM" \
  "$EXTERN_PLAN" \
  "$VM_S0" \
  "$INC_DIR" >/tmp/"$TAG".release_leak 2>&1; then
  echo "[$TAG] ERROR: OS release route vocabulary must remain inactive in runtime/backend/allocator code" >&2
  cat /tmp/"$TAG".release_leak >&2
  rm -f /tmp/"$TAG".release_leak
  exit 1
fi
rm -f /tmp/"$TAG".release_leak

if rg -n 'mimalloc-osvm-release|HakoAllocOsVm.*Release|osvm.*release.*matcher' \
  "$INC_DIR" >/tmp/"$TAG".inc_matcher_leak 2>&1; then
  echo "[$TAG] ERROR: release-specific backend/app matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_matcher_leak >&2
  rm -f /tmp/"$TAG".inc_matcher_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_matcher_leak

echo "[$TAG] ok"
