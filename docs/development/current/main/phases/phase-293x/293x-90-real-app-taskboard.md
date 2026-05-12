# Phase 293x Real-App Taskboard

- Status: Active
- Lane: `phase-293x real-app bringup`
- Current blocker token: `phase-293x mimalloc substrate capability ladder after real-app EXE parity`
- Mimalloc purpose SSOT:
  `docs/development/current/main/design/mimalloc-hako-port-purpose-ssot.md`

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
- [x] `293x-091` M39 native pointer atomic load route proof
- [x] `293x-092` M40 native pointer atomic CAS route proof
- [x] `293x-093` M41 pointer CAS remote-free list proof
- [x] `293x-094` M42 allocator remote-free list policy integration proof
- [x] `293x-095` M43 allocator remote-free retry-loop proof
- [x] `293x-096` M44 mimalloc allocator substrate closeout guard
- [x] `293x-097` M45 production allocator port entry plan
- [x] `293x-098` M46 hako_alloc production facade boundary
- [x] `293x-099` M47 allocator local page policy proof
- [x] `293x-100` M48 allocator remote-free policy proof
- [x] `293x-101` M49 allocator OSVM page-source proof
- [x] `293x-102` M50 allocator stress production-facade parity
- [x] `293x-103` M51 production allocator port closeout guard
- [x] `293x-104` M52 allocator replacement hook boundary
- [x] `293x-105` M53 allocator HookPlan vocabulary lock
- [x] `293x-106` M54 allocator hook runtime dry-run boundary
- [x] `293x-107` M55 allocator hook activation proof
- [x] `293x-108` M56 allocator hook runtime owner row
- [x] `293x-109` M57 allocator hook runtime dry-run code
- [x] `293x-110` M58 allocator hook dry-run manifest callsite
- [x] `293x-111` M59 allocator hook dry-run test surface
- [x] `293x-112` M60 allocator hook activation proof validator
- [x] `293x-113` M61 allocator hook dry-run CLI surface
- [x] `293x-114` M62 allocator hook activation preflight boundary
- [x] `293x-115` M63 allocator hook activation preflight shape
- [x] `293x-116` M64 allocator provider boundary vocabulary
- [x] `293x-117` M65 allocator provider manifest vocabulary
- [x] `293x-118` M66 allocator provider task breakdown
- [x] `293x-119` M67 allocator provider manifest parser
- [x] `293x-120` M68 allocator provider manifest CLI surface
- [x] `293x-121` M69 allocator provider readiness preflight shape
- [x] `293x-122` M70 combined hook/provider dry-run report
- [x] `293x-123` M71 allocator provider registry boundary
- [x] `293x-124` M72 hako model provider proof fixture
- [x] `293x-125` M73 debug guarded provider proof fixture
- [x] `293x-126` M74 native system provider proof boundary
- [x] `293x-127` M75 native mimalloc provider proof boundary
- [x] `293x-128` M76 allocator provider activation entry contract
- [x] `293x-129` M77 allocator provider registry snapshot
- [x] `293x-130` M78 allocator provider selection decision
- [x] `293x-131` M79 allocator provider proof bundle consumption
- [x] `293x-132` M80 allocator provider rollback preflight
- [x] `293x-133` M81 allocator provider activation safety gate
- [x] `293x-134` M82 allocator provider activation safety diagnostic owner
- [x] `293x-135` M83 allocator provider activation safety diagnostic report
- [x] `293x-136` M84 allocator provider activation safety CLI surface
- [x] `293x-137` M85 allocator provider activation safety closeout inventory
- [x] `293x-138` M86 allocator provider activation decision surface proposal
- [x] `293x-139` M86b allocator provider lightweight doc sync policy
- [x] `293x-140` M87 allocator provider activation decision fixture contract
- [x] `293x-141` M88 allocator provider activation decision diagnostic owner
- [x] `293x-142` M89 allocator provider activation decision diagnostic report
- [x] `293x-143` M90 allocator provider activation decision CLI surface
- [x] `293x-144` M91 allocator provider activation decision closeout inventory
- [x] `293x-145` M92 allocator provider activation implementation entry contract
- [x] `293x-146` M93 allocator provider registry snapshot diagnostic report
- [x] `293x-147` M93B allocator provider diagnostic inactive actions
- [x] `293x-148` M94 allocator provider registry snapshot CLI surface
- [x] `293x-149` M95 allocator provider activation diagnostic closeout inventory
- [x] `293x-150` M96 allocator provider selection decision diagnostic report
- [x] `293x-151` M97 allocator provider selection decision CLI surface
- [x] `293x-152` M97B allocator provider diagnostic helper cleanup
- [x] `293x-153` M98 allocator provider proof bundle consumption diagnostic report
- [x] `293x-154` M98B allocator provider runtime diagnostic module boundaries
- [x] `293x-155` M99 allocator provider proof bundle consumption CLI surface
- [x] `293x-156` M100 allocator provider proof bundle consumption entry contract
- [x] `293x-157` M101 allocator provider proof consumption fail-fast entry
- [x] `293x-158` M102 allocator provider selected-provider precondition
- [x] `293x-159` M103 allocator provider selected-provider proof validation
- [x] `293x-160` mimalloc `.hako` port purpose realignment
- [x] `293x-161` low-level capability language reference sync
- [x] `293x-162` mimalloc upstream analysis and `.hako` port plan
- [x] `293x-163` M163 mimalloc size-class policy owner
- [x] `293x-164` M164-M170 mimalloc port granularity lock
- [x] `293x-165` M164 mimalloc layout migration closeout
- [x] `293x-166` M165 mimalloc page model split
- [x] `293x-167` M166 mimalloc page queue/direct-page cache
- [x] `293x-168` M166B mimalloc unified-member style cleanup
- [x] `293x-169` Box field syntax reference sync
- [x] `293x-170` JoinIR loop field-bound expression acceptance
- [x] `293x-171` box stored field initializer support
- [x] `293x-172` mimalloc field initializer convergence
- [x] `293x-173` hako_alloc scalar numeric fields
- [x] `293x-174` usize mimalloc syntax decision
- [x] `293x-175` M167 mimalloc alloc fast path
- [x] `293x-176` M168 mimalloc OSVM page-source composition
- [x] `293x-177` M169 mimalloc local-free retire
- [x] `293x-178` M170 mimalloc remote-free integration

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
