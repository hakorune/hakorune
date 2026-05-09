#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="k2-wide-atomic-memory-order-args-vocab"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-085-M33-ATOMIC-MEMORY-ORDER-ARGS-VOCAB-LOCK.md"
SUBSTRATE_DOC="docs/reference/runtime/substrate-capabilities.md"
ATOMIC_README="lang/src/runtime/substrate/atomic/README.md"
ATOMIC_CORE="lang/src/runtime/substrate/atomic/atomic_core_box.hako"

echo "[$TAG] running M33 atomic memory-order args vocabulary guard"

for path in "$TASKBOARD" "$CARD" "$SUBSTRATE_DOC" "$ATOMIC_README" "$ATOMIC_CORE"; do
  if [[ ! -f "$path" ]]; then
    echo "[$TAG] ERROR: missing required file: $path" >&2
    exit 1
  fi
done

rg -F -q '| `M33 atomic memory-order args docs/route vocabulary lock` | `live-docs` |' "$TASKBOARD"
rg -F -q '| `M34 pointer atomic vocabulary docs lock` | `next-card` |' "$TASKBOARD"
rg -F -q 'M33 Atomic Memory-Order Args Vocab Lock' "$CARD"
rg -F -q 'AtomicCoreBox.load_i64_ordered(slot, order)' "$CARD"
rg -F -q 'AtomicCoreBox.store_i64_ordered(slot, value, order)' "$CARD"
rg -F -q 'AtomicCoreBox.fetch_add_i64_ordered(slot, delta, order)' "$CARD"
rg -F -q 'AtomicCoreBox.cas_i64_ordered(slot, expected, desired, success_order, failure_order)' "$CARD"

for route in \
  'extern.hako_atomic.slot_load_i64_ordered' \
  'extern.hako_atomic.slot_store_i64_ordered' \
  'extern.hako_atomic.slot_fetch_add_i64_ordered' \
  'extern.hako_atomic.slot_cas_i64_ordered'; do
  rg -F -q "$route" "$CARD"
  rg -F -q "$route" "$SUBSTRATE_DOC"
done

for core_op in \
  'HakoAtomicSlotLoadI64Ordered' \
  'HakoAtomicSlotStoreI64Ordered' \
  'HakoAtomicSlotFetchAddI64Ordered' \
  'HakoAtomicSlotCasI64Ordered'; do
  rg -F -q "$core_op" "$CARD"
done

for symbol in \
  'hako_atomic_slot_load_i64_ordered' \
  'hako_atomic_slot_store_i64_ordered' \
  'hako_atomic_slot_fetch_add_i64_ordered' \
  'hako_atomic_slot_cas_i64_ordered'; do
  rg -F -q "$symbol" "$CARD"
  rg -F -q "$symbol" "$SUBSTRATE_DOC"
done

rg -F -q 'failure order is restricted to Relaxed, Acquire, or SeqCst' "$SUBSTRATE_DOC"
rg -F -q 'M33 reserves ordered' "$ATOMIC_README"

if rg -n 'load_i64_ordered|store_i64_ordered|fetch_add_i64_ordered|cas_i64_ordered' "$ATOMIC_CORE" >/tmp/"$TAG".atomic_core 2>&1; then
  echo "[$TAG] ERROR: ordered atomic methods must stay docs-only in M33" >&2
  cat /tmp/"$TAG".atomic_core >&2
  rm -f /tmp/"$TAG".atomic_core
  exit 1
fi
rm -f /tmp/"$TAG".atomic_core

if rg -n 'slot_(load|store|fetch_add|cas)_i64_ordered|HakoAtomicSlot.*Ordered' \
  src lang/c-abi/shims crates/nyash_kernel >/tmp/"$TAG".active_rows 2>&1; then
  echo "[$TAG] ERROR: ordered atomic route/export row leaked into active implementation" >&2
  cat /tmp/"$TAG".active_rows >&2
  rm -f /tmp/"$TAG".active_rows
  exit 1
fi
rm -f /tmp/"$TAG".active_rows

rg -F -q 'k2_wide_atomic_memory_order_args_vocab_guard.sh' docs/tools/check-scripts-index.md

echo "[$TAG] ok"
