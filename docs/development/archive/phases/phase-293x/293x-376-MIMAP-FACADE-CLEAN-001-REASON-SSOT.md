# 293x-376 MIMAP-FACADE-CLEAN-001 Reason SSOT

Status: landed
Date: 2026-05-15

## Decision

`MIMAP-FACADE-CLEAN-001` starts with the facade reason-code and known-page scan
cleanup slice. The result observer scalar API stays stable for existing proof
apps and guards; this row only removes duplicated local decisions inside the
facade.

## Scope

- Add `object_lifecycle_facade_reason_box.hako` as the source-level
  reason-code SSOT for allocation, release, alignment, and realloc scalar
  observers.
- Route facade reason writes through named reason methods instead of numeric
  failure literals.
- Keep the public observer methods and stored field names unchanged.
- Centralize the release/realloc known-page scan in
  `objectLifecycleKnownPageIndexById(page_id)` while keeping typed page method
  calls in the caller routes.
- Update the memory README and cleanup guard so future changes can see the
  boundary from one entry.

## Non-goals

- No allocator behavior change.
- No result field rename or public observer API change.
- No page-map lookup or arbitrary pointer-to-page resolution.
- No byte copy, native aligned placement, OSVM/page-source execution, provider
  activation, hooks, host allocator replacement, or backend shortcut.

## Evidence

```text
bash tools/checks/k2_wide_mimalloc_facade_cleanup_reason_ssot_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_release_one_block_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_release_failfast_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_aligned_alloc_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_realloc_shrink_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_realloc_grow_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Next

Continue `MIMAP-FACADE-CLEAN-001` with the remaining result observer capsule
cleanup only after this reason/scan slice is green and committed.
