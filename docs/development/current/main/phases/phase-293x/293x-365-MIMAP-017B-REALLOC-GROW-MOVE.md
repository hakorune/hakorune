# 293x-365 MIMAP-017B Realloc Grow Move Route

Status: ready
Date: 2026-05-15

## Decision

`MIMAP-017B` is the next primary allocator row after `MIMAP-017A`. It adds the
facade-owned realloc grow/move route by allocating a replacement block before
releasing the old known block.

## Scope

- Keep realloc routed through `HakoAllocObjectLifecycleFacade`.
- Reuse the existing small allocation route to allocate a replacement block.
- Release the old known `(page id, block id)` only after replacement allocation
  succeeds.
- Expose scalar realloc grow/move observers:
  - old page id
  - old block id
  - new page id
  - new block id
  - requested size
  - status
  - reason code
- Add one proof app and one LLVM/EXE-primary guard.

## Non-goals

- No byte copy.
- No page-map ownership lookup.
- No arbitrary pointer-to-page lookup.
- No raw unregister/register behavior.
- No OSVM/page-source route.
- No provider activation, hooks, host allocator replacement, or
  `#[global_allocator]`.
- No backend-specific matcher shortcuts.

## Expected Files

```text
apps/mimalloc-facade-realloc-grow-proof/main.hako
tools/checks/k2_wide_mimalloc_facade_realloc_grow_exe_guard.sh
```

Likely implementation owner:

```text
lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
```

## Acceptance

Expected proof output shape:

```text
mimalloc-facade-realloc-grow-proof
grow=<status>,<old page>,<old block>,<new page>,<new block>,<requested size>,<reason code>
reject=<status>,<reason code>
summary=ok
```

Required guard evidence:

```text
[mimap017b-mir-json] ok
[k2-wide-mimalloc-facade-realloc-grow-exe] ok
```

## Stop Lines

If this row needs byte copy, stop and split a copy-contract row.

If this row needs page-map lookup, arbitrary pointer resolution, or raw
register/unregister behavior, stop and split a page-map handoff row.

If this row needs OSVM/page-source behavior or backend-specific lowering, stop
and split a separate owner row.

## Follow-up

After `MIMAP-017B` lands:

```text
MIMAP-018A:
  stats snapshot observer integration
```
