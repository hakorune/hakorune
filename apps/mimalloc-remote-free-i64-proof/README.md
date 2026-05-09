# mimalloc-remote-free-i64-proof

Purpose: M31 pure-first EXE composition proof for a fixed-slot i64 remote-free
push sketch.

The source uses only already-live `AtomicCoreBox` fixed-slot i64 primitives:
`load_i64`, `store_i64`, `cas_i64`, and `fetch_add_i64`. MIR owns each route
fact independently; pure-first emits those existing rows and does not learn a
new remote-free helper.

This fixture intentionally avoids:

- pointer atomics
- memory-order arguments
- production remote-free allocator policy
- TLS coupling
- new backend route rows
- app-specific backend matchers

Gate:

```bash
bash tools/checks/k2_wide_mimalloc_remote_free_i64_exe_guard.sh
```
