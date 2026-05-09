# mimalloc-atomic-store-proof

Purpose: M29 pure-first EXE proof for the narrow `hako.atomic` fixed i64 store
slot seam.

The source uses `AtomicCoreBox.store_i64/2` and the already-live
`AtomicCoreBox.load_i64/1` to observe the stored value. MIR owns the route fact
for `hako_atomic_slot_store_i64`; pure-first only reads that fact.

This fixture intentionally avoids:

- generic atomic fetch_add
- pointer atomics
- memory-order arguments on store
- remote-free allocator policy
- app-specific backend matchers

Gate:

```bash
bash tools/checks/k2_wide_mimalloc_atomic_store_exe_guard.sh
```
