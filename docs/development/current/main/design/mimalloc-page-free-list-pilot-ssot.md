# MIMAP-008 page/free-list pilot SSOT

Decision: accepted.

`MIMAP-008` adopts the existing `HakoAllocPageModel` as the first Hakorune-side
page/free-list executable pilot.

## Scope

Owned by this row:

- direct page model construction
- free-list acquire path
- local-free release path through `HakoAllocPageModel.releaseLocal(block_id)`
- double-release rejection
- retire-on-empty behavior
- reactivation that drains local-free blocks back to the free list
- direct proof app and guard entry

Not owned by this row:

- OS virtual memory
- decommit/recommit substrate
- segment ownership
- thread-local or remote-free ownership
- allocator provider activation
- hooks or host allocator replacement
- `#[global_allocator]`

## Existing owner

The current code owner is:

```text
lang/src/hako_alloc/memory/page_box.hako
```

`HakoAllocPageModel` already owns the page-local block state and counters. This row
therefore avoids adding a second page model and instead fixes the pilot by direct
proof coverage.

## Proof and guard

Executable proof:

```text
apps/mimalloc-page-free-list-pilot-proof/main.hako
```

Guard entry:

```text
tools/checks/k2_wide_mimalloc_page_free_list_pilot_guard.sh
```

The guard must remain a direct page/free-list contract. It must not become a broad
allocator facade test.

## Transition to MIMAP-009

`MIMAP-009` is allowed to extend lifecycle integration around the page model, but
it must keep decommit/recommit/reuse separate from this free-list proof. Unsupported
substrate behavior must fail fast rather than silently falling back.
