# mimalloc page/free-list pilot proof

Decision: accepted for `MIMAP-008`.

This app is the direct executable proof for the Hakorune-side page/free-list model pilot.
It uses `HakoAllocPageModel` directly, without OSVM, provider activation, allocator hooks,
or host allocator replacement.

The proof fixes these visible behaviors:

- `acquire` pops a free block and marks it live.
- oversize acquire rejects with the signed sentinel path.
- `releaseLocal` moves live blocks into the local-free list and rejects double release.
- `reactivate` rejects while the page is still in use.
- releasing the final live block retires the page.
- successful `reactivate` drains local-free blocks back to the free list.

This row intentionally does not model decommit/recommit/reuse. That remains `MIMAP-009`.
