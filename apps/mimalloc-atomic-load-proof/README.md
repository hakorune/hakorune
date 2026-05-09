# mimalloc-atomic-load-proof

Purpose: M28 pure-first EXE proof for the narrow `hako.atomic` fixed i64 load
slot seam.

The source uses `AtomicCoreBox.load_i64/1` and the already-live
`AtomicCoreBox.cas_i64/3` to seed/reset the slot. MIR owns the route fact for
`hako_atomic_slot_load_i64`; pure-first only reads that fact.

This fixture intentionally avoids:

- generic atomic store/fetch_add
- pointer atomics
- memory-order arguments on load
- remote-free allocator policy
- app-specific backend matchers

Gate:

```bash
bash tools/checks/k2_wide_mimalloc_atomic_load_exe_guard.sh
```
