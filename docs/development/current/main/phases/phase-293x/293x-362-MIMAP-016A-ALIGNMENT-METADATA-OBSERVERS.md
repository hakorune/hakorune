# 293x-362 MIMAP-016A Alignment Metadata Observers

Status: ready
Date: 2026-05-15

## Decision

`MIMAP-016A` is the next primary allocator row after `MIMAP-015B`. It adds
alignment request metadata and scalar observer results for the facade allocation
route without implementing aligned allocation execution.

## Scope

- Keep allocation routed through `HakoAllocObjectLifecycleFacade`.
- Accept and normalize one alignment request metadata value.
- Expose scalar alignment observers:
  - requested alignment
  - normalized alignment
  - alignment reason code
  - supported/unsupported summary
- Add one proof app and one LLVM/EXE-primary guard.

## Non-goals

- No aligned allocation success/fail route; that belongs to `MIMAP-016B`.
- No realloc route.
- No page-map ownership lookup.
- No OSVM/page-source route.
- No provider activation, hooks, host allocator replacement, or
  `#[global_allocator]`.
- No backend-specific matcher shortcuts.

## Expected Files

```text
apps/mimalloc-facade-alignment-metadata-proof/main.hako
tools/checks/k2_wide_mimalloc_facade_alignment_metadata_exe_guard.sh
```

Likely implementation owner:

```text
lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
```

## Acceptance

Expected proof output shape:

```text
mimalloc-facade-alignment-metadata-proof
align=<requested>,<normalized>,<reason code>
summary=ok
```

Required guard evidence:

```text
[mimap016a-mir-json] ok
[k2-wide-mimalloc-facade-alignment-metadata-exe] ok
```

## Stop Lines

If this row starts executing aligned allocation placement, stop and split that
behavior into `MIMAP-016B`.

If alignment metadata requires backend-specific lowering, stop and add a
compiler sidecar with a minimized fixture.

If the row needs page-map lookup or OSVM/page-source behavior, stop and split a
separate owner row.

## Follow-up

After `MIMAP-016A` lands:

```text
MIMAP-016B:
  aligned allocation success/fail route
```
