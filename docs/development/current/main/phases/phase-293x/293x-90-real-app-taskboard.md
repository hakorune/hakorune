# Phase 293x Real-App Taskboard

- Status: Active
- Lane: `phase-293x real-app bringup`
- Current blocker token: `phase-293x birth/method call route expansion before real-app EXE parity`

## Tasks

- [x] `293x-001` BoxTorrent mini local content store
- [x] `293x-002` binary-trees allocation/GC benchmark
- [x] `293x-003` mimalloc-lite allocator-shaped app
- [x] `293x-004` real-app EXE boundary probe
- [x] `293x-005` pure-first general-newbox owner decision
- [x] `293x-006` real allocator port
- [x] `293x-007` allocator-stress app
- [x] `293x-008` BoxTorrent allocator-backed store
- [x] `293x-009` JSON stream aggregator
- [x] `293x-010` smoke env Hako alias cleanup
- [x] `293x-011` config env Hako root/bin alias cleanup
- [x] `293x-012` typed object EXE plan for general user-box `newbox`
- [x] `293x-013` i64 field typed-object EXE route for `newbox` + `field_set` / `field_get`
- [x] `293x-014` typed object route expansion for init-only untyped fields, handle storage, and empty user-box allocation
- [x] `293x-015` typed user-box `birth` same-module EXE route
- [x] `293x-016` typed user-box scalar method same-module EXE route
- [ ] `293x-017` next real-app method / birth-body route expansion before real-app EXE parity

## Gates

- App-local test:
  `apps/<app>/test.sh`
- Real-app suite:
  `tools/smokes/v2/run.sh --profile integration --suite real-apps --skip-preflight`
- EXE boundary suite:
  `tools/smokes/v2/run.sh --profile integration --suite real-apps-exe-boundary --skip-preflight`
- Current pointer guard:
  `bash tools/checks/current_state_pointer_guard.sh`

## Notes

- Add one runnable app slice at a time.
- Add compiler acceptance only when a real app exposes a blocker.
- Keep app smoke output deterministic and small.
