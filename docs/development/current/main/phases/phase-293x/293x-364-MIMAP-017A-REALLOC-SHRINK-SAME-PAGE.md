# 293x-364 MIMAP-017A Realloc Shrink Same-Page Route

Status: ready
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
