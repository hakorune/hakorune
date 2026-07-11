# 293x-001 BoxTorrent Mini Local Store

- Status: Landed
- Lane: `phase-293x real-app bringup`
- Scope: first real-app slice for content-addressed chunks, dedupe, manifest
  materialization, and explicit refcount release.

## Changes

- Added `apps/boxtorrent-mini/main.hako`.
- Added app-local `apps/boxtorrent-mini/test.sh`.
- Added `tools/smokes/v2/profiles/integration/apps/boxtorrent_mini_vm.sh`.
- Added the `real-apps` integration suite.
- Updated `apps/README.md` to mark BoxTorrent mini as implemented.

## Contract

- A repeated ingest of identical payload chunks reuses the same chunk IDs.
- Duplicate chunks increment store refcounts.
- Manifest materialization reconstructs the original payload.
- Releasing the duplicate manifest lowers the first chunk refcount from `2`
  to `1`.

## Verification

```bash
apps/boxtorrent-mini/test.sh
tools/smokes/v2/profiles/integration/apps/boxtorrent_mini_vm.sh
tools/smokes/v2/run.sh --profile integration --suite real-apps --skip-preflight
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
