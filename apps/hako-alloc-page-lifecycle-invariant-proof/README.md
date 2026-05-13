# hako-alloc-page-lifecycle-invariant-proof

Purpose: prove M207 page lifecycle invariant freeze.

This fixture does not add allocator behavior. It uses the M207 observer to lock
the state table introduced by M205/M206:

| state | meaning |
| --- | --- |
| `1` | active |
| `2` | retired |
| `3` | decommitted |
| `4` | recommitted-active |

Stop line:

- no new allocation, release, decommit, recommit, or reactivation behavior
- no purge scheduler
- no verifier enforcement
- no object-return allocator API expansion
- no unreserve, OS release, provider hook, or process allocator replacement
