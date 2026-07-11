# 293x-009 JSON Stream Aggregator App

- Status: Landed
- Lane: `phase-293x real-app bringup`
- Scope: add a streaming JSONL-style aggregation real app.

## Decision

Add `apps/json-stream-aggregator` as the next real-app slice. The app keeps a
narrow deterministic scanner for the local fixture shape and exercises
string-scanning, map-backed state, per-user aggregation, and stable reporting.

## Non-Goals

- No general JSON parser.
- No language feature expansion.
- No EXE parity claim.

## Changes

- Added `apps/json-stream-aggregator`.
- Added app-local and integration smoke entries.
- Added the app to the real-app EXE boundary probe.
- Updated phase/current pointers.

## Verification

```bash
apps/json-stream-aggregator/test.sh
tools/smokes/v2/run.sh --profile integration --suite real-apps --skip-preflight
tools/smokes/v2/run.sh --profile integration --suite real-apps-exe-boundary --skip-preflight
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
