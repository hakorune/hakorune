# Phase 293x Real-App Taskboard

- Status: Active
- Lane: `phase-293x real-app bringup`
- Current blocker token: `phase-293x mimalloc substrate capability ladder after real-app EXE parity`

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
- [x] `293x-017` typed object birth-param storage EXE route
- [x] `293x-018` typed void side-effect global route for BoxTorrent allocator seeding
- [x] `293x-019` typed object call-param / global-return storage flow
- [x] `293x-020` BoxTorrentChunker.ingest route expansion to BoxTorrentStore.put visibility
- [x] `293x-021` BoxTorrent module-generic prepass seam
- [x] `293x-022` BoxTorrent string field return EXE parity
- [x] `293x-023` json-stream-aggregator EXE runtime parity boundary
- [x] `293x-024` binary-trees EXE parity
- [x] `293x-025` same-module body-shape cleanup
- [x] `293x-026` mimalloc capability taskboard lock
- [x] `293x-027` mimalloc-lite / allocator-stress EXE parity
- [x] `293x-073` M21 mimalloc size-class table EXE proof
- [x] `293x-074` M22 mimalloc two-class page EXE proof
- [x] `293x-075` M23 mimalloc dynamic bin EXE proof
- [x] `293x-076` M24 mimalloc size_to_bin inline EXE proof
- [x] `293x-077` M25 mimalloc OSVM page EXE proof
- [x] `293x-078` M26 mimalloc TLS cache-slot EXE proof
- [x] `293x-079` M27 mimalloc atomic CAS slot EXE proof
- [x] `293x-080` M28 mimalloc atomic load slot EXE proof
- [x] `293x-081` M29 mimalloc atomic store slot EXE proof
- [x] `293x-082` M30 mimalloc atomic fetch-add slot EXE proof
- [x] `293x-083` M31 mimalloc remote-free i64 sketch EXE proof
- [x] `293x-084` M32 mimalloc post-M31 task-order lock
- [x] `293x-085` M33 atomic memory-order args docs/route vocabulary lock
- [x] `293x-086` M34 pointer atomic vocabulary docs lock
- [x] `293x-087` M35 native pointer atomic store route proof
- [x] `293x-088` M36 TLS pointer remote-free composition proof
- [x] `293x-089` M37 allocator remote-free policy integration proof
- [x] `293x-090` M38 mimalloc allocator app closeout guard

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
