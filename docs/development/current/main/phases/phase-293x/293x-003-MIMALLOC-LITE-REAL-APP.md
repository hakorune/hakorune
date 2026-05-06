# 293x-003 mimalloc-lite Real App

- Status: Landed
- Lane: `phase-293x real-app bringup`
- Scope: add a deterministic allocator-shaped app before the real allocator
  port.

## Changes

- Added `apps/mimalloc-lite/main.hako`.
- Added app-local `apps/mimalloc-lite/test.sh`.
- Added `tools/smokes/v2/profiles/integration/apps/mimalloc_lite_vm.sh`.
- Added mimalloc-lite to the `real-apps` integration suite.
- Updated `apps/README.md` and the phase-293x taskboard.

## Contract

- Small page: `9` allocations, `3` frees, `3` reuses, peak `6`, free `2`.
- Medium page: `4` allocations, `1` free, `1` reuse, peak `3`, free `1`.
- Total requested bytes: `360`.
- Outstanding blocks: `9`.

## Implementation Note

- Block metadata is kept in parallel numeric arrays in this slice.
- A loop that created `MiBlock` objects and stored them in an `ArrayBox`
  exposed an object-in-loop initialization route that is not needed for this
  app contract. If the real allocator port needs that shape, handle it as a
  compiler seam rather than adding app-side workaround logic.

## Verification

```bash
apps/mimalloc-lite/test.sh
tools/smokes/v2/profiles/integration/apps/mimalloc_lite_vm.sh
tools/smokes/v2/run.sh --profile integration --suite real-apps --skip-preflight
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
