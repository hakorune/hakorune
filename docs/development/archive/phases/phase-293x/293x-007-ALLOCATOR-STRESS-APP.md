# 293x-007 Allocator-Stress App

- Status: Landed
- Lane: `phase-293x real-app bringup`
- Scope: add real-app stress coverage over the `hako_alloc` VM-only
  page/free-list seam.

## Decision

Add `apps/allocator-stress` as the next real-app slice after the page-heap port.
The app uses `HakoAllocHeap` through the public `hako_alloc` module and fixes
deterministic behavior for:

- small and medium page saturation
- release then reuse order
- oversize allocation rejection
- double-free rejection
- requested-byte and outstanding-block accounting

## Non-Goals

- No native allocator backend migration.
- No EXE parity claim.
- No app-side workaround for pure-first general user-box `newbox`.

## Changes

- Added `apps/allocator-stress`.
- Added app-local and integration smoke entries.
- Added the app to the real-app EXE boundary probe so direct EXE remains
  pinned at the known typed-object blocker.

## Verification

```bash
apps/allocator-stress/test.sh
tools/smokes/v2/run.sh --profile integration --suite real-apps --skip-preflight
tools/smokes/v2/run.sh --profile integration --suite real-apps-exe-boundary --skip-preflight
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
