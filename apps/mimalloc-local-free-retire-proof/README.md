# mimalloc-local-free-retire-proof

M169 proof fixture for the `.hako` mimalloc port.

This app proves page-local local-free collection before remote-free integration.
`HakoAllocPageModel.releaseLocal(...)` records same-thread frees into
`local_free`; `acquire(...)` collects one local-free block when the normal free
stack is empty; empty-page retire state is recorded by the final local release.

Scope:

- Same-thread `local_free` collection into reusable page-local free blocks.
- Empty-page retire state and idempotent retire accounting.
- Retired pages reject later page-local allocation attempts.

Non-goals:

- No remote-free atomics.
- No abandoned-page reclaim.
- No page-map lookup or arbitrary pointer free.
- No OSVM unreserve/release row.
- No provider, hook, or process allocator replacement.
- No production `usize` field migration.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_local_free_retire_guard.sh
```
