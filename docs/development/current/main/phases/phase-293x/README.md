# Phase 293x: real-app bringup

- Status: Active
- Purpose: use real applications to expose compiler/runtime seams after the
  Program(JSON v0) cleanup lane, without adding `.hako` workarounds for real
  compiler blockers.
- Active lane token: `phase-293x real-app bringup`
- Current blocker token: `phase-293x mimalloc substrate capability ladder after real-app EXE parity`

## Order

1. BoxTorrent mini
2. binary-trees
3. mimalloc-lite
4. real allocator port

## Policy

- Real app code should stay simple and idiomatic.
- If an app needs a compiler expressivity improvement, fix the compiler seam
  first instead of hiding the issue in the app.
- Keep BoxShape cleanup separate from BoxCount acceptance expansion.
- Keep `phase-137x` observe-only unless app evidence reopens a real blocker.
- Do not start allocator optimization work before the preceding apps provide
  concrete ownership / allocation evidence.

## Smoke Entry

```bash
tools/smokes/v2/run.sh --profile integration --suite real-apps --skip-preflight
```

## EXE Boundary Entry

```bash
tools/smokes/v2/run.sh --profile integration --suite real-apps-exe-boundary --skip-preflight
```

This is a mixed parity/boundary suite. TypedObjectPlan now covers declared i64
fields, init-only untyped fields, handle storage, observed empty user boxes,
observed `newbox` argument storage flowing into same-module `birth` parameters,
same-module call argument storage propagation, and same-module string-like
global return storage. Conservative same-module `birth` and scalar user-box
method routes are available for the minimal typed-object fixtures. BoxTorrent
allocator page seeding now lowers through a MIR-owned void side-effect global
route, `BoxTorrentManifest` now has a typed object plan, and
`BoxTorrentChunker.ingest` exposes the nested `BoxTorrentStore.put` route. The
BoxTorrent module-generic prepass seam for `firstChunkId` / `refCount` is
lowered, `ContentChunk` is plan-backed through nullable-handle storage flow,
user-box string field returns now flow through MIR route facts, and
global-call/string-substring handle metadata now flows into downstream
same-module params. BoxTorrent mini, binary-trees, JSON stream aggregator,
mimalloc-lite, and allocator-stress direct EXE parity pass. The real-app EXE
boundary probe currently has no remaining unsupported-shape app pins.
`HakoAllocHandle` typed-object planning is now covered by MIR-owned param origin
inference for the allocator release path.

## Current Status

- `293x-001`: BoxTorrent mini local content store landed.
- `293x-002`: binary-trees allocation/shape benchmark app landed.
- `293x-003`: mimalloc-lite allocator-shaped app landed.
- `293x-004`: real-app EXE boundary probe landed.
- `293x-005`: pure-first general-newbox owner decision landed.
- `293x-006`: `hako_alloc` VM-only page/free-list policy-state port landed.
- `293x-007`: allocator-stress app landed.
- `293x-008`: BoxTorrent allocator-backed store landed.
- `293x-009`: JSON stream aggregator app landed.
- `293x-010`: smoke env Hako alias cleanup landed.
- `293x-011`: config env Hako root/bin alias cleanup landed.
- `293x-012`: typed object EXE plan for general user-box `newbox` landed.
- `293x-013`: declared-i64 typed object EXE route for `newbox` plus
  `field_set` / `field_get` landed.
- `293x-014`: init-only untyped fields, handle storage, and observed empty
  user-box allocation landed.
- `293x-015`: typed user-box `birth` same-module EXE route landed for the
  conservative single-block body shape.
- `293x-016`: typed user-box scalar method same-module EXE route landed for the
  conservative single-block body shape.
- `293x-017`: typed-object birth-param storage inference landed for untyped
  fields initialized from observed `newbox` constructor arguments.
- `293x-018`: typed void side-effect global route landed for the BoxTorrent
  allocator page seeding chain.
- `293x-019`: typed object call-param / global-return storage flow landed,
  making `BoxTorrentManifest` plan-backed.
- `293x-020`: BoxTorrent ingest user-box method body route expansion landed,
  making `BoxTorrentStore.put` visible from `ingest` and routing Store Map/Array
  field operations.
- `293x-021`: BoxTorrent module-generic prepass seam landed for
  `firstChunkId` / `refCount`, including `ContentChunk` typed-object planning
  and same-module typed object PHI propagation in the EXE shim.
- `293x-022`: BoxTorrent string field return EXE parity landed, making
  `BoxTorrentStore.readData/1` return a `string_handle` through route-fact
  placeholder refinement and allowing BoxTorrent mini direct EXE to exit 0.
- `293x-023`: JSON stream aggregator EXE parity landed, publishing global-call
  handle argument types to target params and substring handle results to
  downstream global calls.
- `293x-024`: binary-trees EXE parity landed, expanding same-module recursive
  user-box method body routes and typed-object handle global-call returns.
- `293x-025`: same-module body-shape cleanup landed, moving shared body-shape
  facts to a neutral MIR owner and tightening allocator-stress boundary pins.
- `293x-026`: mimalloc capability taskboard lock landed; mimalloc-grade work
  proceeds through capability modules plus `@rune Contract(...)` verifier rows,
  with manual updates required per implementation row.
- `293x-027`: mimalloc-lite and allocator-stress EXE parity landed. MIR-owned
  param-origin inference clears `HakoAllocHeap.release/1`, and same-module
  pure-first PHI refinement now respects explicit `dst_type`.
- `293x-073`: M21 mimalloc size-class table EXE proof landed, composing M11b
  static `u16` size-class tables with the M14-M20 raw-page pure-first route
  surface through a narrow MIR-owned `static_data_load` reader, without adding
  new source syntax or allocator policy.
- `293x-074`: M22 mimalloc two-class page EXE proof landed, composing the M21
  static table seam with two M14-M20 raw pages for small/medium reject,
  release, and reuse, without adding new compiler vocabulary.
- `293x-075`: M23 mimalloc dynamic bin EXE proof landed, proving non-constant
  `static_data_load` indices for `u16` size-class tables under pure-first EXE.
- `293x-076`: M24 mimalloc size_to_bin inline EXE proof landed, composing
  `Profile(allocator.fast)` verified inline with dynamic static table loads.
- `293x-077`: M25 mimalloc OSVM page EXE proof landed, routing
  `OsVmCoreBox.reserve_bytes_i64/commit_bytes_i64/decommit_bytes_i64` through
  MIR-owned extern route facts into pure-first EXE.
- `293x-078`: M26 mimalloc TLS cache-slot EXE proof landed, routing
  `TlsCoreBox.cache_slot_get_i64/1` and `cache_slot_set_i64/2` through
  MIR-owned extern route facts into pure-first EXE.
- `293x-079`: M27 mimalloc atomic CAS slot EXE proof landed, routing
  `AtomicCoreBox.cas_i64/3` through MIR-owned extern route facts into
  pure-first EXE.
- `293x-080`: M28 mimalloc atomic load slot EXE proof landed, routing
  `AtomicCoreBox.load_i64/1` through MIR-owned extern route facts into
  pure-first EXE.
- `293x-081`: M29 mimalloc atomic store slot EXE proof landed, routing
  `AtomicCoreBox.store_i64/2` through MIR-owned extern route facts into
  pure-first EXE.
- `293x-082`: M30 mimalloc atomic fetch-add slot EXE proof landed, routing
  `AtomicCoreBox.fetch_add_i64/2` through MIR-owned extern route facts into
  pure-first EXE.
- `293x-083`: M31 mimalloc remote-free i64 sketch EXE proof landed, composing
  the existing CAS/load/store/fetch_add route facts into a pure-first EXE
  remote-free push sketch without adding a new backend route.
- `293x-084`: M32 mimalloc post-M31 task-order lock landed, fixing the next
  M33-M38 order and syncing stale atomic/TLS taskboard wording.
- `293x-085`: M33 atomic memory-order args docs/route vocabulary lock landed,
  reserving ordered fixed-slot i64 atomic facade/route names while keeping
  implementation rows inactive.
- `293x-086`: M34 pointer atomic vocabulary docs lock landed, reserving
  native-pointer atomic load/store/CAS facade/route names while keeping
  implementation rows inactive.
- `293x-087`: M35 native pointer atomic store route proof landed, routing
  `hako_atomic_ptr_store_ordered/3` through MIR-owned extern facts, NyRT export,
  and pure-first native pointer argument lowering without activating pointer
  load/CAS or native pointer attrs.
- `293x-088`: M36 TLS pointer remote-free composition proof landed, composing
  M26 TLS cache-slot rows with the M35 pointer-store row in pure-first EXE
  without adding new route rows or allocator policy.
- `293x-089`: M37 allocator remote-free policy integration proof landed,
  routing `AllocatorRemoteFreePolicy` as same-module generic-i64 bodies over
  the existing M26/M35 mailbox seam without adding backend-specific matchers.
- `293x-090`: M38 mimalloc allocator app closeout guard landed, locking
  M20-M37 mimalloc proof coverage in docs index and `dev_gate.sh quick` without
  adding another app, route row, or backend matcher.
- `293x-091`: M39 native pointer atomic load route proof landed, routing
  `hako_atomic_ptr_load_ordered/2` through MIR-owned extern facts, NyRT export,
  and pure-first native pointer return lowering while leaving CAS to M40 and
  keeping pointer fetch_add and native pointer attrs inactive.
- `293x-092`: M40 native pointer atomic CAS route proof landed, routing
  `hako_atomic_ptr_cas_ordered/5` through MIR-owned extern facts, NyRT export,
  and pure-first native pointer return lowering while keeping pointer
  fetch_add and native pointer attrs inactive.
- `293x-093`: M41 pointer CAS remote-free list proof landed, composing the
  existing M35/M39/M40 pointer store/load/CAS routes into a two-node remote-free
  list push fixture without adding a route row or backend matcher.
- `293x-094`: M42 allocator remote-free list policy integration proof landed,
  moving the M41 list push shape behind `AllocatorRemoteFreeListPolicy` through
  same-module generic-i64 routes without adding a route row or backend matcher.
- `293x-095`: M43 allocator remote-free retry-loop proof landed, proving a
  bounded CAS retry loop inside `AllocatorRemoteFreeRetryPolicy` over existing
  pointer store/load/CAS routes.
- `293x-096`: M44 mimalloc allocator substrate closeout guard landed, locking
  the M20-M43 substrate proof ladder before production allocator port work.
- Next: M45 production allocator port entry plan; keep future blockers as
  compiler/runtime seams and do not hide them in app code.
