# mimalloc-atomic-cas-proof

Purpose: M27 pure-first EXE proof for the narrow `hako.atomic` fixed i64 CAS
slot seam.

The source uses `AtomicCoreBox.cas_i64/3`. MIR owns the route fact for
`hako_atomic_slot_cas_i64`; pure-first only reads that fact.

This fixture intentionally avoids:

- generic atomic load/store/fetch_add
- pointer atomics
- memory-order arguments on CAS
- remote-free allocator policy
- app-specific backend matchers

Gate:

```bash
bash tools/checks/k2_wide_mimalloc_atomic_cas_exe_guard.sh
```
