---
Status: Done
Decision: accepted
Date: 2026-05-10
Scope: M41 pointer CAS remote-free list proof
---

# 293x-093 M41 Pointer CAS Remote-Free List Proof

## Decision

`M41 pointer CAS remote-free list proof` is live-narrow.

M41 adds no new route row. It composes the existing direct native pointer
atomic rows:

```text
hako_atomic_ptr_store_ordered(cell_ptr, value_ptr, order)
hako_atomic_ptr_load_ordered(cell_ptr, order)
hako_atomic_ptr_cas_ordered(cell_ptr, expected_ptr, desired_ptr, success_order, failure_order)
```

Accepted shape:

```text
head_cell = hako_mem_alloc(8)
block.next = old_head      via hako_atomic_ptr_store_ordered(block, old_head, Release)
publish head = block      via hako_atomic_ptr_cas_ordered(head_cell, old_head, block, AcqRel, Acquire)
observe list shape        via hako_atomic_ptr_load_ordered(head_cell/block, Acquire)
```

The fixture pushes two blocks and proves:

- first push: empty head becomes `block_a`, and `block_a.next` is null.
- second push: head becomes `block_b`, `block_b.next` is `block_a`, and
  `block_a.next` remains null.

## Owned

- `apps/mimalloc-ptr-remote-free-list-proof/`
- Guard:
  `tools/checks/k2_wide_mimalloc_ptr_remote_free_list_exe_guard.sh`
- Documentation/current pointers for M41.

## Not Owned

- New MIR extern route rows.
- New NyRT exports.
- New `.inc` route emit behavior.
- pointer `fetch_add`.
- AtomicCoreBox pointer methods.
- Production remote-free allocator policy.
- Pointer arithmetic, raw layout source syntax, or native pointer attrs.

## Gate

```bash
bash tools/checks/k2_wide_mimalloc_ptr_remote_free_list_exe_guard.sh
bash tools/checks/k2_wide_pointer_atomic_vocab_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The guard must verify:

- MIR JSON contains only existing M35/M39/M40 pointer atomic route facts.
- pure-first build logs hit store/load/CAS emit traces.
- EXE output proves the two-node list shape.
- `.inc` does not branch on the fixture app name.
- pointer fetch_add rows remain inactive.

## Result

Result on 2026-05-10:
`k2_wide_mimalloc_ptr_remote_free_list_exe_guard.sh` passes.
