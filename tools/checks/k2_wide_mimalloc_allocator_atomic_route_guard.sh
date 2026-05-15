#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-allocator-atomic-route"
cd "$ROOT_DIR"

ATOMIC_CORE="lang/src/runtime/substrate/atomic/atomic_core_box.hako"
ROUTE_PLAN="src/mir/extern_call_route_plan.rs"
ROUTE_TESTS="src/mir/extern_call_route_plan/tests.rs"
KERNEL_EXPORT="crates/nyash_kernel/src/exports/atomic.rs"
CARD="docs/development/current/main/phases/phase-293x/293x-395-MIMAP-ATOMIC-001-ALLOCATOR-ATOMIC-ROUTE-GUARD.md"
INDEX="docs/tools/check-scripts-index.md"

echo "[$TAG] checking allocator atomic route guard"

for file in \
  "$ATOMIC_CORE" \
  "$ROUTE_PLAN" \
  "$ROUTE_TESTS" \
  "$KERNEL_EXPORT" \
  "$CARD" \
  "$INDEX" \
  "tools/checks/k2_wide_mimalloc_atomic_cas_exe_guard.sh" \
  "tools/checks/k2_wide_mimalloc_atomic_load_exe_guard.sh" \
  "tools/checks/k2_wide_mimalloc_atomic_store_exe_guard.sh" \
  "tools/checks/k2_wide_mimalloc_atomic_fetch_add_exe_guard.sh" \
  "tools/checks/k2_wide_mimalloc_substrate_route_inventory_guard.sh"; do
  if [ ! -e "$file" ]; then
    echo "[$TAG] ERROR: missing required file: $file" >&2
    exit 1
  fi
done

for symbol in \
  hako_atomic_slot_cas_i64 \
  hako_atomic_slot_load_i64 \
  hako_atomic_slot_store_i64 \
  hako_atomic_slot_fetch_add_i64; do
  rg -F -q "$symbol" "$ATOMIC_CORE"
  rg -F -q "$symbol" "$ROUTE_PLAN"
  rg -F -q "$symbol" "$ROUTE_TESTS"
  rg -F -q "$symbol" "$KERNEL_EXPORT"
done

for guard in \
  k2_wide_mimalloc_atomic_cas_exe_guard.sh \
  k2_wide_mimalloc_atomic_load_exe_guard.sh \
  k2_wide_mimalloc_atomic_store_exe_guard.sh \
  k2_wide_mimalloc_atomic_fetch_add_exe_guard.sh; do
  rg -F -q "$guard" "$INDEX"
  rg -F -q "$guard" "$CARD"
done
rg -F -q "$(basename "$0")" "$INDEX"
rg -F -q "$(basename "$0")" "$CARD"

if rg -n 'hako_atomic_slot_(load|store|fetch_add|cas)_i64_ordered' \
  src lang/c-abi crates/nyash_kernel >/tmp/"$TAG".ordered 2>&1; then
  echo "[$TAG] ERROR: ordered fixed-slot atomic implementation rows are out of scope" >&2
  cat /tmp/"$TAG".ordered >&2
  rm -f /tmp/"$TAG".ordered
  exit 1
fi
rm -f /tmp/"$TAG".ordered

if rg -n 'ptr_fetch_add|hako_atomic_ptr_fetch_add' \
  src lang/c-abi crates/nyash_kernel >/tmp/"$TAG".ptr_fetch_add 2>&1; then
  echo "[$TAG] ERROR: pointer fetch_add row must stay inactive" >&2
  cat /tmp/"$TAG".ptr_fetch_add >&2
  rm -f /tmp/"$TAG".ptr_fetch_add
  exit 1
fi
rm -f /tmp/"$TAG".ptr_fetch_add

if rg -n 'remote[A-Za-z0-9_]*[[:space:]]*\(|Abandoned|PageOwner|page_owner|Channel|task_scope|worker_local|global_allocator|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(' \
  "$ATOMIC_CORE" >/tmp/"$TAG".scope 2>&1; then
  echo "[$TAG] ERROR: AtomicCoreBox leaked beyond fixed-slot substrate scope" >&2
  cat /tmp/"$TAG".scope >&2
  rm -f /tmp/"$TAG".scope
  exit 1
fi
rm -f /tmp/"$TAG".scope

bash tools/checks/k2_wide_mimalloc_atomic_cas_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_atomic_load_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_atomic_store_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_atomic_fetch_add_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_substrate_route_inventory_guard.sh

echo "[$TAG] ok"
