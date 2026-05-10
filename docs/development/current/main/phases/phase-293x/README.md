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
- `293x-097`: M45 production allocator port entry plan landed, fixing the
  first production allocator port order as facade boundary, local page policy,
  remote-free policy, OSVM page-source, then stress parity.
- `293x-098`: M46 hako_alloc production facade boundary landed, adding
  `HakoAllocProductionFacade` as the public allocator port seam over existing
  page/free-list policy state.
- `293x-099`: M47 allocator local page policy proof landed, validating
  small/medium allocate/free/reject behavior through the production facade.
- `293x-100`: M48 allocator remote-free policy proof landed, composing the
  M43 bounded CAS retry-loop shape behind the production facade while keeping
  pointer atomics in substrate.
- `293x-101`: M49 allocator OSVM page-source proof landed, composing
  reserve/commit/decommit through `HakoAllocPageSourcePolicy` behind the
  production facade while keeping OSVM metal in substrate/native keep.
- `293x-102`: M50 allocator stress production-facade parity landed, adding
  production-facade stress coverage while keeping `apps/allocator-stress` as
  lower-seam regression coverage.
- `293x-103`: M51 production allocator port closeout guard landed, inventorying
  M46-M50 production allocator port proof coverage and keeping allocator
  replacement hooks inactive.
- `293x-104`: M52 allocator replacement hook boundary landed, adding the
  allocator replacement hook SSOT and guard while keeping process allocator
  replacement inactive.
- `293x-105`: M53 allocator HookPlan vocabulary lock landed, adding reserved
  HookPlan v0 docs/TOML vocabulary while keeping runtime hook activation
  inactive.
- `293x-106`: M54 allocator hook runtime dry-run boundary landed, fixing the
  diagnostic-only runtime seam shape while keeping hook implementation inactive.
- `293x-107`: M55 allocator hook activation proof landed, adding reserved
  activation proof vocabulary while keeping hook activation inactive.
- `293x-108`: M56 allocator hook runtime owner row landed, naming the future
  `src/runtime/allocator_hook_dry_run.rs` owner while keeping implementation
  absent.
- `293x-109`: M57 allocator hook runtime dry-run code landed, adding
  diagnostic-only runtime validation that never installs or replaces the process
  allocator.
- `293x-110`: M58 allocator hook dry-run manifest callsite landed, feeding
  reserved HookPlan/proof TOML text into the diagnostic-only runtime validator.
- `293x-111`: M59 allocator hook dry-run test surface landed, adding a
  `#[cfg(test)]` reserved-fixture observation helper without CLI/env exposure.
- `293x-112`: M60 allocator hook activation proof validator landed, validating
  reserved activation-proof TOML text while keeping activation and CLI/env/file
  discovery inactive.
- `293x-113`: M61 allocator hook dry-run CLI surface landed, exposing explicit
  plan/proof file diagnostics without env toggles, implicit discovery, runner
  ownership, or activation.
- `293x-114`: M62 allocator hook activation preflight boundary landed, naming
  the reentrancy/bootstrap/no-alloc/rollback/fail-fast handoff required before
  any activation row.
- `293x-115`: M63 allocator hook activation preflight shape landed, adding
  diagnostic-only runtime facts/report and stable missing-fact names while
  keeping `would_activate=false`.
- `293x-116`: M64 allocator provider boundary vocabulary landed, reserving
  provider ids for system allocator, mimalloc, hako model, and guarded debug
  providers while keeping provider registry/selection/replacement inactive.
- `293x-117`: M65 allocator provider manifest vocabulary landed, adding a
  reserved provider manifest TOML fixture for the M64 provider ids while
  keeping runtime parser/registry/selection/replacement inactive.
- `293x-118`: M66 allocator provider task breakdown landed, adding a readable
  M52-M65 checkpoint and M67-M75 task ladder.
- `293x-119`: M67 allocator provider manifest parser landed, adding a
  diagnostic-only runtime parser/report for caller-provided provider manifest
  TOML text while keeping provider selection and allocator replacement inactive.
- `293x-120`: M68 allocator provider manifest CLI surface landed, exposing an
  explicit provider manifest file diagnostic without env toggles, implicit
  discovery, runner ownership, provider selection, or replacement.
- `293x-121`: M69 allocator provider readiness preflight shape landed, tying
  provider manifest readiness to hook activation preflight diagnostics while
  keeping provider selection and activation false.
- `293x-122`: M70 combined hook/provider dry-run report landed, composing
  explicit hook plan, activation proof, and provider manifest diagnostics while
  keeping install, provider selection, and activation false.
- `293x-123`: M71 allocator provider registry boundary landed, naming the
  future registry owner/API shape while keeping active registry implementation,
  provider selection, and allocator replacement absent.
- `293x-124`: M72 hako model provider proof fixture landed, reserving the
  `.hako` policy/model provider proof shape while keeping provider selection,
  native metal activation, and allocator replacement inactive.
- `293x-125`: M73 debug guarded provider proof fixture landed, reserving the
  guarded-provider diagnostic proof shape while keeping provider selection,
  hook activation, and allocator replacement inactive.
- `293x-126`: M74 native system provider proof boundary landed, reserving the
  system allocator ABI proof shape while keeping `#[global_allocator]`,
  provider selection, hook activation, and allocator replacement inactive.
- `293x-127`: M75 native mimalloc provider proof boundary landed, reserving the
  mimalloc provider ABI/page lifecycle proof shape while keeping production
  activation, provider selection, hook activation, and allocator replacement
  inactive.
- `293x-128`: M76 allocator provider activation entry contract landed, naming
  future registry/selection ownership, fail-fast selection diagnostics,
  activation proof consumption, provider proof consumption, rollback behavior,
  and a dedicated guard while keeping runtime registry code, provider
  selection, hook activation, and allocator replacement inactive.
- `293x-129`: M77 allocator provider registry snapshot landed, fixing the
  reserved provider-entry snapshot shape and registry missing diagnostics while
  keeping runtime registry code, provider selection, hook activation, and
  allocator replacement inactive.
- `293x-130`: M78 allocator provider selection decision landed, fixing the
  reserved caller-provided request/decision shape with no selected provider
  while keeping selection implementation, hook activation, and allocator
  replacement inactive.
- `293x-131`: M79 allocator provider proof bundle consumption landed, fixing
  the reserved proof bundle input/diagnostic shape while keeping runtime proof
  consumption, hook activation, and allocator replacement inactive.
- `293x-132`: M80 allocator provider rollback preflight landed, fixing the
  reserved rollback target facts and activation-blocked diagnostics while
  keeping rollback preparation, hook activation, and allocator replacement
  inactive.
- Next: M81 activation safety gate contract. It must stay diagnostic-only: no
  environment discovery, implicit manifest discovery, hook activation,
  `#[global_allocator]`, or process allocator replacement.
