# Phase 293x Real-App Taskboard

- Status: Active
- Lane: `phase-293x real-app bringup`
- Current blocker token: `phase-293x real-app bringup order: BoxTorrent mini -> binary-trees -> mimalloc-lite -> allocator port`

## Tasks

- [x] `293x-001` BoxTorrent mini local content store
- [ ] `293x-002` binary-trees allocation/GC benchmark
- [ ] `293x-003` mimalloc-lite allocator-shaped app
- [ ] `293x-004` real allocator port

## Gates

- App-local test:
  `apps/<app>/test.sh`
- Real-app suite:
  `tools/smokes/v2/run.sh --profile integration --suite real-apps --skip-preflight`
- Current pointer guard:
  `bash tools/checks/current_state_pointer_guard.sh`

## Notes

- Add one runnable app slice at a time.
- Add compiler acceptance only when a real app exposes a blocker.
- Keep app smoke output deterministic and small.
