# 293x-366 MIMAP-FACADE-CLEAN-001 Result SSOT TODO

Status: ready
Date: 2026-05-15

## Decision

`MIMAP-FACADE-CLEAN-001` is a BoxShape cleanup TODO for the allocator facade
after `MIMAP-017B`. It should not run before the grow/move realloc shape lands,
because that row will reveal the final short-term shape of facade result
observers.

This is cleanup only. It must not add allocator acceptance vocabulary or new
allocation behavior.

## TODO

- [ ] Split facade result observer state into small owned helper boxes or a
  clearly named result capsule:
  - allocation result
  - release result
  - alignment result
  - realloc result
- [ ] Move facade reason-code tables into one SSOT:
  - either a tracked README table next to `object_lifecycle_facade_box.hako`
  - or a tiny `object_lifecycle_facade_reason_box.hako` if source-level
    constants become useful
- [ ] Deduplicate the facade-local known-page scan used by release/realloc
  without introducing page-map lookup or arbitrary pointer resolution.
- [ ] Commonize allocator-facade guard forbidden-pattern wording where it
  reduces drift, while keeping row-specific stop lines readable.
- [ ] Update `lang/src/hako_alloc/memory/README.md` so future agents can see
  the facade boundary, reason SSOT, and cleanup stop lines from one entry.

## Non-goals

- No aligned allocation expansion.
- No realloc grow/move behavior.
- No byte copy.
- No page-map ownership lookup.
- No arbitrary pointer-to-page lookup.
- No OSVM/page-source route.
- No provider activation, hooks, host allocator replacement, or
  `#[global_allocator]`.
- No backend-specific matcher shortcuts.

## Acceptance

Expected proof shape:

```text
No behavior change.
```

Required evidence:

```text
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/dev_gate.sh quick
```

If code moves are included, also run the active facade row guard and the latest
two predecessor facade guards.

## Stop Lines

If cleanup wants to add a new allocator behavior, stop and split a normal
`MIMAP-*` behavior row.

If cleanup needs page-map lookup or arbitrary pointer resolution, stop and split
a page-map handoff row.

If cleanup becomes a broad file move touching unrelated allocator modules, stop
and narrow the owner boundary first.
