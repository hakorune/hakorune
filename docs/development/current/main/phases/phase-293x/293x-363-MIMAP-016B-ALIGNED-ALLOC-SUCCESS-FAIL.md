# 293x-363 MIMAP-016B Aligned Alloc Success/Fail Route

Status: ready
Date: 2026-05-15

## Decision

`MIMAP-016B` is the next primary allocator row after `MIMAP-016A`. It adds a
facade-owned aligned allocation success/fail route over the existing object
lifecycle queue without adding page-map lookup, realloc, or native allocator
substrate behavior.

## Scope

- Keep allocation routed through `HakoAllocObjectLifecycleFacade`.
- Reuse the `MIMAP-016A` alignment request metadata observer seam.
- Add one aligned small-allocation facade method that validates alignment and
  then routes accepted requests through the existing small allocation path.
- Expose scalar aligned allocation result observers:
  - requested alignment
  - normalized alignment
  - allocation page id
  - allocation block id
  - fail-fast reason code
- Prove one supported aligned request and one unsupported request.
- Add one proof app and one LLVM/EXE-primary guard.

## Non-goals

- No page-map ownership lookup.
- No pointer-to-page lookup.
- No realloc route.
- No padded pointer arithmetic or native alignment claim.
- No OSVM/page-source route.
- No provider activation, hooks, host allocator replacement, or
  `#[global_allocator]`.
- No backend-specific matcher shortcuts.

## Expected Files

```text
apps/mimalloc-facade-aligned-alloc-proof/main.hako
tools/checks/k2_wide_mimalloc_facade_aligned_alloc_exe_guard.sh
```

Likely implementation owner:

```text
lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
```

## Acceptance

Expected proof output shape:

```text
mimalloc-facade-aligned-alloc-proof
aligned=<status>,<page id>,<block id>,<normalized alignment>,<reason code>
reject=<status>,<normalized alignment>,<reason code>
summary=ok
```

Required guard evidence:

```text
[mimap016b-mir-json] ok
[k2-wide-mimalloc-facade-aligned-alloc-exe] ok
```

## Stop Lines

If this row needs page-map lookup or pointer-to-page resolution, stop and split
a page-map handoff row.

If this row needs native aligned pointer placement, padded pointer arithmetic,
or backend-specific lowering, stop and add a separate compiler/substrate card.

If this row starts realloc or OSVM/page-source behavior, stop and split the
behavior into its own owner row.

## Follow-up

After `MIMAP-016B` lands:

```text
MIMAP-017A:
  realloc shrink / same-page route
```
