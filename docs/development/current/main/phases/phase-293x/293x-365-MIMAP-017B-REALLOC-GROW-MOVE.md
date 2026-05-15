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
- Validate the old known block before replacement allocation. This validation
  must stay facade-local and must not introduce page-map lookup.
- Expose scalar realloc grow/move observers:
  - old page id
  - old block id
  - new page id
  - new block id
  - requested size
  - status
  - reason code
- Add one proof app and one LLVM/EXE-primary guard.

## Implementation Order

Keep the row in this exact order:

1. Validate scalar inputs and old known block liveness.
2. Allocate the replacement block with the existing small allocation path.
3. If replacement allocation fails, keep the old block live and return a
   realloc reason.
4. If replacement allocation succeeds, release the old known block through the
   existing facade release route.
5. Record the grow/move result only after old-block release succeeds.

This row may add grow/move result observers, but it must not perform the
`MIMAP-FACADE-CLEAN-001` cleanup yet. Result capsule extraction, reason-code
SSOT, and known-page scan dedupe remain after `MIMAP-017B`.

## Reason Codes

`MIMAP-017B` extends the existing realloc reason space:

- `0`: grow/move accepted
- `1`: missing/stale old page id
- `2`: invalid old block id
- `3`: invalid requested size
- `4`: request is not a grow/move request for the old page block size
- `5`: old block is not live on the old page
- `6`: replacement allocation failed; old block was not released
- `7`: replacement allocation succeeded but old-block release failed

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

If this row wants to clean up result capsules, reason constants, or known-page
scan duplication, stop and leave that for `MIMAP-FACADE-CLEAN-001`.

## Follow-up

After `MIMAP-017B` lands:

```text
MIMAP-FACADE-CLEAN-001:
  facade result observer / reason-code SSOT cleanup

MIMAP-018A:
  stats snapshot observer integration
```

Cleanup TODO:

```text
docs/development/current/main/phases/phase-293x/293x-366-MIMAP-FACADE-CLEAN-001-RESULT-SSOT-TODO.md
```
