# 293x-364 MIMAP-017A Realloc Shrink Same-Page Route

Status: landed
Date: 2026-05-15

## Decision

`MIMAP-017A` is the next primary allocator row after `MIMAP-016B`. It adds the
smallest realloc route: a same-page shrink/no-move result over the existing
facade-owned object lifecycle queue.

## Scope

- Keep realloc routed through `HakoAllocObjectLifecycleFacade`.
- Accept one known `(page id, block id)` that is still live on the selected page
  model.
- Treat new size `<=` current page block size as same-page/no-move success.
- Expose scalar realloc observers:
  - page id
  - block id
  - requested size
  - status
  - reason code
- Add one proof app and one LLVM/EXE-primary guard.

## Non-goals

- No grow/move allocation; that belongs to `MIMAP-017B`.
- No byte copy.
- No page-map ownership lookup.
- No arbitrary pointer-to-page lookup.
- No unregister/register behavior.
- No OSVM/page-source route.
- No provider activation, hooks, host allocator replacement, or
  `#[global_allocator]`.
- No backend-specific matcher shortcuts.

## Expected Files

```text
apps/mimalloc-facade-realloc-shrink-proof/main.hako
tools/checks/k2_wide_mimalloc_facade_realloc_shrink_exe_guard.sh
```

Likely implementation owner:

```text
lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
```

## Acceptance

Expected proof output shape:

```text
mimalloc-facade-realloc-shrink-proof
shrink=<status>,<page id>,<block id>,<requested size>,<reason code>
reject=<status>,<reason code>
summary=ok
```

Required guard evidence:

```text
[mimap017a-mir-json] ok
[k2-wide-mimalloc-facade-realloc-shrink-exe] ok
```

## Implementation

`MIMAP-017A` adds
`HakoAllocObjectLifecycleFacade.objectLifecycleReallocShrink(page_id, block_id,
requested_size)` as a facade-owned same-page/no-move observer route. It scans
the existing object lifecycle queue for one known page id, checks that the block
is live on that page, and accepts only requested sizes that fit in the current
page block size.

Reason codes:

- `0`: same-page shrink/no-move accepted
- `1`: missing/stale page id
- `2`: invalid block id
- `3`: invalid requested size
- `4`: requested size does not fit same-page shrink/no-move
- `5`: block is not live on the page

This row does not allocate a replacement block, copy bytes, use page-map lookup,
or register/unregister ownership.

## Evidence

```text
bash tools/checks/k2_wide_mimalloc_facade_realloc_shrink_exe_guard.sh
[mimap017a-mir-json] ok
[k2-wide-mimalloc-facade-realloc-shrink-exe] ok

for s in tools/checks/k2_wide_mimalloc_facade_aligned_alloc_exe_guard.sh tools/checks/k2_wide_mimalloc_facade_alignment_metadata_exe_guard.sh tools/checks/k2_wide_mimalloc_facade_small_alloc_exe_guard.sh tools/checks/k2_wide_mimalloc_facade_release_failfast_exe_guard.sh; do bash "$s"; done
[mimap016b-mir-json] ok
[k2-wide-mimalloc-facade-aligned-alloc-exe] ok
[mimap016a-mir-json] ok
[k2-wide-mimalloc-facade-alignment-metadata-exe] ok
[mimap014a-mir-json] ok
[k2-wide-mimalloc-facade-small-alloc-exe] ok
[mimap015b-mir-json] ok
[k2-wide-mimalloc-facade-release-failfast-exe] ok
```

## Stop Lines

If this row needs grow/move allocation, stop and split `MIMAP-017B`.

If this row needs page-map lookup or arbitrary pointer resolution, stop and add
a page-map handoff row.

If this row needs byte copy, unregister/register, OSVM/page-source behavior, or
backend-specific lowering, stop and split a separate owner row.

## Follow-up

After `MIMAP-017A` lands:

```text
MIMAP-017B:
  realloc grow / move route
```
