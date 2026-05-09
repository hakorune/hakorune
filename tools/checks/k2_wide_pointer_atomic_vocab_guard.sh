#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="k2-wide-pointer-atomic-vocab"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-086-M34-POINTER-ATOMIC-VOCAB-LOCK.md"
SUBSTRATE_DOC="docs/reference/runtime/substrate-capabilities.md"
ATOMIC_README="lang/src/runtime/substrate/atomic/README.md"
ATOMIC_CORE="lang/src/runtime/substrate/atomic/atomic_core_box.hako"

echo "[$TAG] running M34 pointer atomic vocabulary guard"

for path in "$TASKBOARD" "$CARD" "$SUBSTRATE_DOC" "$ATOMIC_README" "$ATOMIC_CORE"; do
  if [[ ! -f "$path" ]]; then
    echo "[$TAG] ERROR: missing required file: $path" >&2
    exit 1
  fi
done

rg -F -q '| `M34 pointer atomic vocabulary docs lock` | `live-docs` |' "$TASKBOARD"
rg -F -q 'M34 Pointer Atomic Vocabulary Lock' "$CARD"
rg -F -q 'AtomicCoreBox.ptr_load_ordered(cell_ptr, order)' "$CARD"
rg -F -q 'AtomicCoreBox.ptr_store_ordered(cell_ptr, value_ptr, order)' "$CARD"
rg -F -q 'AtomicCoreBox.ptr_cas_ordered(cell_ptr, expected_ptr, desired_ptr, success_order, failure_order)' "$CARD"

for route in \
  'extern.hako_atomic.ptr_load_ordered' \
  'extern.hako_atomic.ptr_store_ordered' \
  'extern.hako_atomic.ptr_cas_ordered'; do
  rg -F -q "$route" "$CARD"
  rg -F -q "$route" "$SUBSTRATE_DOC"
done

for core_op in \
  'HakoAtomicPtrLoadOrdered' \
  'HakoAtomicPtrStoreOrdered' \
  'HakoAtomicPtrCasOrdered'; do
  rg -F -q "$core_op" "$CARD"
done

for symbol in \
  'hako_atomic_ptr_load_ordered' \
  'hako_atomic_ptr_store_ordered' \
  'hako_atomic_ptr_cas_ordered'; do
  rg -F -q "$symbol" "$CARD"
  rg -F -q "$symbol" "$SUBSTRATE_DOC"
done

rg -F -q 'pointer atomic operands are native pointer transport values, not runtime' "$SUBSTRATE_DOC"
rg -F -q 'pointer fetch_add is not reserved by M34' "$SUBSTRATE_DOC"
rg -F -q 'M34 reserves pointer atomic load/store/CAS names' "$ATOMIC_README"

if rg -n 'ptr_load_ordered|ptr_store_ordered|ptr_cas_ordered|ptr_fetch_add' "$ATOMIC_CORE" >/tmp/"$TAG".atomic_core 2>&1; then
  echo "[$TAG] ERROR: pointer atomic methods must stay docs-only in M34" >&2
  cat /tmp/"$TAG".atomic_core >&2
  rm -f /tmp/"$TAG".atomic_core
  exit 1
fi
rm -f /tmp/"$TAG".atomic_core

if rg -n 'hako_atomic_ptr_fetch_add|ptr_fetch_add' \
  src lang/c-abi/shims crates/nyash_kernel >/tmp/"$TAG".active_rows 2>&1; then
  echo "[$TAG] ERROR: pointer atomic fetch_add row leaked into active implementation" >&2
  cat /tmp/"$TAG".active_rows >&2
  rm -f /tmp/"$TAG".active_rows
  exit 1
fi
rm -f /tmp/"$TAG".active_rows

rg -F -q 'k2_wide_pointer_atomic_vocab_guard.sh' docs/tools/check-scripts-index.md

echo "[$TAG] ok"
