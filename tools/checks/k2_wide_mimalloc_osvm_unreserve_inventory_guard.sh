#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-osvm-unreserve-inventory"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-293x/293x-456-MIMAP-031A-OSVM-UNRESERVE-CAPABILITY-INVENTORY.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-457-MIMAP-032A-OSVM-UNRESERVE-SUBSTRATE-ROUTE.md"
OSVM_CORE="lang/src/runtime/substrate/osvm/osvm_core_box.hako"
OSVM_README="lang/src/runtime/substrate/osvm/README.md"
PAGE_SOURCE="lang/src/hako_alloc/memory/page_source_policy_box.hako"
DECOMMIT_ADAPTER="lang/src/hako_alloc/memory/purge_page_source_decommit_adapter_box.hako"
DECOMMIT_ROUTE="lang/src/hako_alloc/memory/object_lifecycle_facade_huge_decommit_box.hako"
FAILFAST_ROUTE="lang/src/hako_alloc/memory/object_lifecycle_facade_huge_decommit_failfast_box.hako"
HOSTBRIDGE="lang/c-abi/include/hako_hostbridge.h"
KERNEL_SHIM="lang/c-abi/shims/hako_kernel.c"
NEED_VOCAB="lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_vocab.inc"
NEED_APPLY="lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_apply.inc"
NEED_RULES="lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_metadata_rules.inc"
SHELL_RULES="lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell.inc"
INDEX="docs/tools/check-scripts-index.md"

echo "[$TAG] checking MIMAP-031A OSVM unreserve inventory"

guard_require_command "$TAG" rg
guard_require_files \
  "$TAG" \
  "$CARD" \
  "$NEXT_CARD" \
  "$OSVM_CORE" \
  "$OSVM_README" \
  "$PAGE_SOURCE" \
  "$DECOMMIT_ADAPTER" \
  "$DECOMMIT_ROUTE" \
  "$FAILFAST_ROUTE" \
  "$HOSTBRIDGE" \
  "$KERNEL_SHIM" \
  "$NEED_VOCAB" \
  "$NEED_APPLY" \
  "$NEED_RULES" \
  "$SHELL_RULES" \
  "$INDEX" \
  "$0"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-031A card must be landed"
guard_expect_in_file "$TAG" 'MIMAP-032A OSVM unreserve substrate route' "$CARD" "MIMAP-031A must select MIMAP-032A"
guard_expect_in_file "$TAG" 'MIMAP-032A' "$NEXT_CARD" "MIMAP-032A card must exist"
guard_expect_in_file "$TAG" "$0" "$INDEX" "check script index must list MIMAP-031A guard"

for method in \
  'page_size_i64' \
  'reserve_bytes_i64' \
  'commit_bytes_i64' \
  'decommit_bytes_i64'
do
  guard_expect_in_file "$TAG" "$method" "$OSVM_CORE" "OsVmCoreBox live row missing: $method"
done

guard_expect_in_file "$TAG" 'HakoAllocPageSourcePolicy.decommitPage' "$DECOMMIT_ADAPTER" "M196 remains the decommit-only adapter owner"
guard_expect_in_file "$TAG" 'decommitPage' "$PAGE_SOURCE" "page-source decommit row must remain live"
guard_expect_in_file "$TAG" 'reservePage' "$PAGE_SOURCE" "page-source reserve row must remain live"
guard_expect_in_file "$TAG" 'commitPage' "$PAGE_SOURCE" "page-source commit row must remain live"

if rg -q 'Status: selected current' "$NEXT_CARD"; then
  if rg -n 'hako_osvm_(unreserve|release)|unreserve_bytes|release_bytes|unreservePage|releasePage' \
    src lang/c-abi crates/nyash_kernel lang/src >/tmp/"$TAG".active_unreserve_rows 2>&1; then
    echo "[$TAG] ERROR: OSVM unreserve/release symbols must remain unopened while MIMAP-032A is only selected" >&2
    cat /tmp/"$TAG".active_unreserve_rows >&2
    rm -f /tmp/"$TAG".active_unreserve_rows
    exit 1
  fi
  rm -f /tmp/"$TAG".active_unreserve_rows

  if rg -n 'provider[A-Za-z0-9_]*[[:space:]]*\(|install_hook|global_allocator|#\[global_allocator\]|GlobalAlloc|replace_allocator' \
    "$OSVM_CORE" "$PAGE_SOURCE" "$DECOMMIT_ADAPTER" "$DECOMMIT_ROUTE" "$FAILFAST_ROUTE" >/tmp/"$TAG".provider_leak 2>&1; then
    echo "[$TAG] ERROR: MIMAP-031A/MIMAP-032A planning must not open provider activation" >&2
    cat /tmp/"$TAG".provider_leak >&2
    rm -f /tmp/"$TAG".provider_leak
    exit 1
  fi
  rm -f /tmp/"$TAG".provider_leak
else
  echo "[$TAG] MIMAP-032A status has advanced; skipping pre-032A no-growth scan"
fi

echo "[$TAG] ok"
