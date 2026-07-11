# 293x-008 BoxTorrent Allocator-Backed Store

- Status: Landed
- Lane: `phase-293x real-app bringup`
- Scope: wire BoxTorrent mini chunk lifecycle to the `hako_alloc` page/free-list
  seam.

## Decision

Make `BoxTorrentStore` allocator-backed without changing its content-owner
model. New chunks allocate a `HakoAllocHeap` handle, duplicate chunks retain the
existing content chunk without allocating, and final chunk release returns the
handle to the allocator seam.

## Non-Goals

- No P2P transport.
- No native allocator backend migration.
- No EXE parity claim.

## Changes

- Added `HakoAllocHeap` to `BoxTorrentStore`.
- Stored allocator handles on `ContentChunk`.
- Added deterministic allocator lifecycle output to BoxTorrent mini.
- Updated app-local and integration smoke expectations.

## Verification

```bash
apps/boxtorrent-mini/test.sh
tools/smokes/v2/run.sh --profile integration --suite real-apps --skip-preflight
tools/smokes/v2/run.sh --profile integration --suite real-apps-exe-boundary --skip-preflight
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
