# mimalloc-atomic-fetch-add-proof

Purpose: M30 pure-first EXE proof for the narrow `hako.atomic` fixed i64 fetch-add
slot seam.

The source uses `AtomicCoreBox.fetch_add_i64/2` with the already-live
`store_i64` and `load_i64` helpers to observe previous-value semantics. MIR owns
the route fact for `hako_atomic_slot_fetch_add_i64`; pure-first only reads that
fact.

This fixture intentionally avoids:

- pointer atomics
- memory-order arguments on fetch-add
- remote-free allocator policy
- TLS coupling
- app-specific backend matchers

Gate:

```bash
bash tools/checks/k2_wide_mimalloc_atomic_fetch_add_exe_guard.sh
```
