---
Status: Done
Decision: accepted
Date: 2026-05-10
Scope: M42 allocator remote-free list policy integration proof
---

# 293x-094 M42 Allocator Remote-Free List Policy Integration Proof

## Decision

`M42 allocator remote-free list policy integration proof` is live-narrow.

M42 adds no new route row. It moves the M41 two-node list push shape behind a
same-module policy box:

```text
AllocatorRemoteFreeListPolicy.init_head(head_cell)
AllocatorRemoteFreeListPolicy.push(head_cell, block_ptr)
AllocatorRemoteFreeListPolicy.peek_head(head_cell)
AllocatorRemoteFreeListPolicy.peek_next(block_ptr)
```

Accepted method-body shape:

```text
push(head_cell, block):
  old_head = hako_atomic_ptr_load_ordered(head_cell, Acquire)
  hako_atomic_ptr_store_ordered(block, old_head, Release)
  return hako_atomic_ptr_cas_ordered(head_cell, old_head, block, AcqRel, Acquire)
```

The app proves:

- first push publishes `block_a` and leaves `block_a.next` null.
- second push publishes `block_b`, sets `block_b.next = block_a`, and leaves
  `block_a.next` null.
- main reaches the policy box through same-module generic-i64 route facts.

## Owned

- `apps/mimalloc-remote-free-list-policy-proof/`
- Guard:
  `tools/checks/k2_wide_mimalloc_remote_free_list_policy_exe_guard.sh`
- Documentation/current pointers for M42.

## Not Owned

- New MIR extern route rows.
- New NyRT exports.
- New `.inc` route emit behavior.
- pointer `fetch_add`.
- AtomicCoreBox pointer methods.
- Production remote-free retry loop.
- Pointer arithmetic, raw layout source syntax, or native pointer attrs.

## Gate

```bash
bash tools/checks/k2_wide_mimalloc_remote_free_list_policy_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_ptr_remote_free_list_exe_guard.sh
bash tools/checks/k2_wide_pointer_atomic_vocab_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The guard must verify:

- main uses generic-i64 global routes to reach the policy box.
- policy method bodies own the pointer load/store/CAS extern route facts.
- pure-first build logs hit global-call and pointer atomic emit traces.
- EXE output proves the two-node list shape.
- `.inc` does not branch on the fixture app name.
- pointer fetch_add rows remain inactive.

## Result

Result on 2026-05-10:
`k2_wide_mimalloc_remote_free_list_policy_exe_guard.sh` passes.
