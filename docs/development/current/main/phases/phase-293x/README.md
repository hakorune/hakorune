# Phase 293x: real-app bringup

- Status: Paused / parent lane while Phase 294x grows exact `usize` semantics
- Purpose: use real applications to expose compiler/runtime seams after the
  Program(JSON v0) cleanup lane, without adding `.hako` workarounds for real
  compiler blockers.
- Parent lane token: `phase-293x real-app bringup`
- Current successor:
  `docs/development/current/main/phases/phase-294x/README.md`
- Current successor blocker token:
  `phase-294x exact usize semantics before mimalloc migration`
- Mimalloc purpose SSOT:
  `docs/development/current/main/design/mimalloc-hako-port-purpose-ssot.md`

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
- Treat mimalloc as `.hako` / `hako_alloc` completeness work in this lane.
  Allocator-provider M104+ is optional future host-replacement support.

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
- `293x-133`: M81 allocator provider activation safety gate landed, fixing the
  reserved activation evidence bundle and gate-closed diagnostics while keeping
  activation gate opening, hook activation, and allocator replacement inactive.
- `293x-134`: M82 allocator provider activation safety diagnostic owner
  landed, naming `src/runtime/allocator_provider_registry.rs` as the diagnostic
  owner and making past provider guards future-compatible with owner-file and
  diagnostic type names while keeping activation implementation inactive.
- `293x-135`: M83 allocator provider activation safety diagnostic report
  landed, adding the runtime-owned gate-closed diagnostic report over
  caller-provided activation safety TOML text while keeping gate opening, hook
  activation, and allocator replacement inactive.
- `293x-136`: M84 allocator provider activation safety CLI surface landed,
  exposing the gate-closed runtime report through an explicit caller-provided
  TOML path while keeping environment discovery, implicit discovery, gate
  opening, hook activation, and allocator replacement inactive.
- `293x-137`: M85 allocator provider activation safety closeout inventory
  landed, locking M76-M84 SSOT/fixture/card/guard coverage before any later
  activation decision row while keeping runtime activation inactive.
- `293x-138`: M86 allocator provider activation decision surface proposal
  landed, defining the future explicit-input activation decision contract while
  keeping runtime parsing, CLI routing, provider selection, proof consumption,
  rollback preparation, hook activation, and allocator replacement inactive.
- `293x-139`: M86b allocator provider lightweight doc sync policy landed,
  reserving heavy mirror updates for closeout rows or durable lane-policy
  changes while keeping all activation behavior inactive.
- `293x-140`: M87 allocator provider activation decision fixture contract
  landed, fixing the reserved explicit decision bundle while keeping runtime
  parsing, CLI routing, provider selection, proof consumption, rollback, gate
  opening, hook activation, and replacement inactive.
- `293x-141`: M88 allocator provider activation decision diagnostic owner
  landed, naming the runtime diagnostic owner while keeping activation
  implementation inactive.
- `293x-142`: M89 allocator provider activation decision diagnostic report
  landed, adding the blocked activation decision runtime report with all
  activation outputs fixed false.
- `293x-143`: M90 allocator provider activation decision CLI surface landed,
  exposing the blocked activation decision report through an explicit TOML path
  while keeping activation inactive.
- `293x-144`: M91 allocator provider activation decision closeout inventory
  landed, locking M86-M90 coverage before any later activation implementation.
- `293x-145`: M92 allocator provider activation implementation entry contract
  landed, naming the single future activation owner/entry while keeping
  activation behavior inactive.
- `293x-146`: M93 allocator provider registry snapshot diagnostic report
  landed, adding the inactive registry snapshot runtime report over
  caller-provided TOML text.
- `293x-147`: M93B allocator provider diagnostic inactive actions landed,
  centralizing false diagnostic outputs in one code-side SSOT.
- `293x-148`: M94 allocator provider registry snapshot CLI surface landed,
  exposing the inactive registry snapshot report through an explicit TOML path.
- `293x-149`: M95 allocator provider activation diagnostic closeout inventory
  landed, locking M92-M94/M93B coverage while keeping active registry
  construction, provider selection, proof consumption, rollback preparation,
  gate opening, hook activation, native activation, and replacement inactive.
- `293x-150`: M96 allocator provider selection decision diagnostic report
  landed, adding runtime parsing/reporting over caller-provided selection
  decision TOML text while keeping provider selection and activation inactive.
- `293x-151`: M97 allocator provider selection decision CLI surface landed,
  exposing the inactive selection decision report through an explicit TOML path.
- `293x-152`: M97B allocator provider diagnostic helper cleanup landed,
  sharing TOML helper and fact-check ownership without changing report output.
- `293x-153`: M98 allocator provider proof bundle consumption diagnostic
  report landed, adding runtime parsing/reporting over caller-provided
  proof-bundle consumption TOML text while keeping proof consumption inactive.
- `293x-154`: M98B allocator provider runtime diagnostic module boundaries
  landed, splitting diagnostic report owners into focused modules behind the
  historical registry facade without behavior change.
- `293x-155`: M99 allocator provider proof bundle consumption CLI surface
  landed, exposing the inactive proof-bundle consumption report through an
  explicit TOML path while keeping proof consumption, provider selection,
  rollback, gate opening, hook activation, native activation, and replacement
  inactive.
- `293x-156`: M100 allocator provider proof bundle consumption entry contract
  landed, reserving the future behavior owner/entry under the activation owner
  while keeping proof consumption and activation inactive.
- `293x-157`: M101 allocator provider proof consumption fail-fast entry landed,
  creating the reserved runtime attempt report and blocking when a real selected
  provider is absent.
- `293x-158`: M102 allocator provider selected-provider precondition landed,
  validating only a caller-provided selected provider while keeping provider
  selection and proof consumption inactive.
- `293x-159`: M103 allocator provider selected-provider proof validation
  landed, validating proof operation coverage while keeping proof consumption
  token creation inactive.
- `293x-160`: mimalloc `.hako` port purpose realignment landed, fixing that
  mimalloc is current `.hako` / `hako_alloc` completeness work while
  allocator-provider M104+ is optional future host-replacement support.
- `293x-161`: low-level capability language reference sync landed, reflecting
  static tables, Rune contracts/profiles, capability modules, RawBuf/RawArray,
  `hako_alloc`, and the no-host-replacement stop line in `docs/reference`.
- `293x-162`: mimalloc upstream analysis and `.hako` port plan landed, fixing
  `microsoft/mimalloc` `v3.3.2` as the primary reference and `M163` as the next
  pure size-class policy row under `hako_alloc`.
- `293x-163`: M163 mimalloc size-class policy owner landed, adding
  `SizeClassBox` and keeping `LayoutBox` as the current small/medium
  compatibility facade.
- `293x-164`: M164-M170 mimalloc port granularity lock landed, fixing write
  sets, proof targets, and stop lines for layout closeout, page model, page
  queue, fast path, OSVM composition, local free, and remote-free integration.
- `293x-165`: M164 mimalloc layout migration closeout landed, fixing
  `LayoutBox` as the legacy two-class compatibility facade over `SizeClassBox`
  and preserving current VM/EXE allocator proof behavior.
- `293x-166`: M165 mimalloc page model split landed, adding
  `HakoAllocPageModel` as a heap-independent page-local owner for `free`,
  `local_free`, `used`, `capacity`, and `reserved` invariants.
- `293x-167`: M166 mimalloc page queue/direct-page cache landed, adding
  `HakoAllocPageQueue` as the page-selection owner while keeping block pops in
  the M167 lane.
- `293x-168`: M166B mimalloc unified-member style cleanup landed, converting
  the new page model and page queue boxes from legacy `init` slot lists to
  `field: Type` stored declarations.
- `293x-169`: Box field syntax reference sync landed, documenting the
  simple/explicit/legacy split as `field`, `field: Type`, and
  `init { field }`.
- `293x-175`: M167 mimalloc alloc fast path landed, composing page queue
  selection with page-local free-list pop and deterministic modeled fallback.
- `293x-176`: M168 mimalloc OSVM page-source composition landed, backing fresh
  modeled pages with existing reserve/commit/decommit rows through a separate
  adapter while keeping the M167 heap OSVM-free.
- `293x-177`: M169 mimalloc local-free retire landed, moving same-thread
  `local_free` entries back to reusable page-local free blocks and exposing
  empty-page retire state without remote-free atomics or abandoned reclaim.
- `293x-178`: M170 mimalloc remote-free integration landed, composing existing
  pointer load/store/CAS remote-free policy with page-owned `releaseLocal(...)`
  state through a caller-provided block-id proof seam.
- `293x-179`: M171 mimalloc page-map model landed, adding
  `HakoAllocPageMap` as the pointer-to-page/block ownership lookup owner.
- `293x-180`: M172 mimalloc page-map-backed release seam landed, adding
  `HakoAllocPageMapReleaseSeam` as the narrow owner that composes
  `lookup(...)`, `releaseLocal(...)`, and `unregister(...)` without taking over
  pointer registration.
- `293x-181`: the M173-M190 mimalloc roadmap was refreshed and corrected so
  completed `usize` facade-stat work is not scheduled again.
- `293x-182`: the M172 proof app cleanup replaced the giant summary
  conjunction with an app-local `ProofCheck` helper. Proof `check` block,
  `+=`, multiline condition, and `guard else` surfaces remain separate
  compiler rows.
- `293x-183`: M173 pre-realloc release invariant freeze landed, adding
  `HakoAllocPageMapReleaseObserver` so successful releases expire handles with a
  frozen release/unregister delta while reject paths keep live ownership.
- `293x-184`: M174 realloc same-class/no-move path landed, adding
  `HakoAllocPageMapReallocSameClassPath` so live pointers can be reused when the
  request still fits the current page block without release or unregister side
  effects.
- `293x-185`: M175 realloc alloc-copy-release fallback landed, adding
  `HakoAllocPageMapReallocAllocCopyReleasePath` so grow requests allocate a
  replacement ptr, model copy count, and release the old ptr only after
  replacement allocation succeeds.
- `293x-186`: M176 realloc negative matrix / failure contract landed, adding
  `HakoAllocPageMapReallocFailureContract` so zero, oversized, unknown, stale,
  released, and alloc-fail outcomes stay explicit while same-class and grow
  execution remain owned by M174/M175.
- `293x-187`: M177 alignment policy object landed, adding
  `HakoAllocAlignmentPolicy` so alignment normalization, power-of-two
  validation, and padded-size policy are fixed before aligned allocation
  execution begins.
- `293x-188`: M178 aligned allocation small path landed, adding
  `HakoAllocPageMapAlignedSmallPath` so normal page-map-backed small
  allocations carry live alignment metadata without reopening huge routing.
- `293x-189`: Record + packed-array lowering SSOT landed, fixing `record` as
  the user-facing identity-free aggregate surface while ordinary `box` keeps
  identity-capable semantics. C201-C205 are now the compiler/runtime follow-on
  lane for field-index fast paths, record scalar replacement, packed
  `ArrayBox` residence, and allocator metadata migration.
- `293x-190`: M179 huge threshold/routing landed, adding
  `HakoAllocHugeThresholdRouter` so padded requests above the last regular
  size-class route to an explicit huge-unsupported fail-fast result instead of
  entering the M178 small path.
- `293x-191`: M180 huge page model landed, adding
  `HakoAllocHugePageModel` so huge handles can be registered in the page map
  while requested/committed/live state stays separate from small page free
  lists.
- `293x-192`: M181 huge release seam landed, adding
  `HakoAllocHugeReleaseSeam` so huge handles are retired through the huge model
  and page map without entering small page `releaseLocal(...)`.
- `293x-193`: M182 secure free-list policy inventory landed, fixing the split
  between page-local block identity, diagnostics-only observers, and future
  encode/decode policy before secure-list code is introduced.
- `293x-194`: M183 secure-list diagnostics landed, adding
  `HakoAllocSecureFreeListDiagnostics` to observe out-of-range, duplicate,
  live-block, and count-mismatch free-list states without encode/decode policy.
- `293x-195`: M184 secure-list encode/decode small path landed, adding
  `HakoAllocSecureFreeListPolicy` for reversible encoded-next policy and
  capacity validation with caller-provided cookies only.
- `293x-196`: D195 hako_alloc SSOT refresh landed, confirming the post-M184
  ownership split before numeric field inventory resumes.
- `293x-197`: M185 hako_alloc field inventory delta landed, reconciling
  `NUMERIC_FIELDS.md`. The current live stored numeric field count is 220 after
  the C205c/C205d metadata-store counters; C205a record declaration fields are
  excluded because they are metadata shapes, not runtime state.
- `293x-198`: M187 exact `usize` size-class policy landed, adding `usize`
  input facades to `SizeClassBox` while keeping invalid/oversized sentinels in
  the signed result lane.
- `293x-199`: M188 exact `usize` request path landed, adding typed request-size
  and alignment facades across alignment policy, page acquire, aligned
  small-path, and huge-router entries without migrating stored ids or result
  sentinels.
- `293x-200`: M189 object-return allocator API parity landed, adding
  `HakoAllocHeap.realloc(...) -> HakoAllocHandle` and proving allocate/realloc
  object handles under VM and pure-first EXE proof-line parity.
- `293x-201`: M190 nullable/failure handle contract landed, adding
  `HakoAllocHandleResult` plus `allocateResult(...)` / `reallocResult(...)`
  proof-line parity without changing the M189 compatibility APIs.
- `293x-202`: C197-C200 proof/application surface order locked, separating
  ordinary short-circuit boolean chains from eager proof `check` blocks.
- `293x-203`: C197 logical condition surface hardening landed, adding a
  proof app and parser regression for parenthesized multiline `&&` / `||`
  conditions while keeping `check` as a separate eager proof-list row.
- `293x-204`: C198 check block surface landed, adding
  `check "name" { "label": expr }` as an eager proof-list expression with
  scalar pass/fail result.
- `293x-205`: C199 compound assignment surface landed, accepting `+=`, `-=`,
  `*=`, and `/=` for local, field, and index targets as canonical assignment
  sugar.
- `293x-206`: C200 guard else surface landed, accepting
  `guard expr else { ... }` as canonical `if !(expr) { ... }` early-exit sugar
  without adding a new control-flow AST node.
- `293x-207`: C201 ordinary user-box field-index fast path landed, exposing
  `layout_id + field_index + storage` metadata for legal typed fields in MIR
  JSON while keeping ordinary `box` identity semantics unchanged.
- `293x-208`: C202 record surface landed, accepting `record Name { field:
  Type }` declarations as the explicit identity-free aggregate surface while
  rejecting weak/untyped/method-bearing record bodies.
- `293x-209`: C203a record declaration metadata transport landed, carrying
  `record_decls` through Program JSON v0, JSON bridge, MIR metadata, and MIR
  JSON without treating records as ordinary user boxes.
- `293x-210`: C203b record layout plans landed, deriving metadata-only
  `record_layout_plans` for concrete records while keeping the lane separate
  from typed-object/user-box layout plans.
- `293x-211`: C203c record local scalar replacement metadata landed, exposing
  concrete record layouts as `record_local_layout` rows in folded `agg_local`
  and placement/effect metadata without rewriting MIR.
- `293x-212`: C204a ArrayBox inline-record storage descriptors landed, deriving
  metadata-only `array_record_storage_plans` from record layout plans without
  changing runtime ArrayBox storage.
- `293x-213`: C204b ArrayBox inline-record storage vocabulary landed, adding
  private `ArrayStorage::InlineRecord` columnar storage with stable
  unmaterialized boundaries and no compiler auto-use or hako_alloc migration.
- `293x-214`: C205a allocator metadata record declarations landed, adding
  declaration-only `HakoAllocAlignedSmallMeta` and `HakoAllocHugePageMeta`
  records while keeping current M178/M180 scalar metadata columns authoritative.
- `293x-215`: C205b allocator record construction/read lowering landed,
  scalarizing direct record field reads in the MIR builder while keeping record
  values out of ordinary `NewBox`, typed-object, backend, and ArrayBox packed
  storage lanes.
- `293x-216`: C205c aligned-small metadata record store landed, moving M178
  aligned metadata columns behind `HakoAllocAlignedSmallMetaStore` and using
  record construction/read at the append boundary without enabling packed
  ArrayBox compiler auto-use.
- `293x-217`: C205d huge-page metadata record store landed, moving M180 huge
  metadata columns behind `HakoAllocHugePageMetaStore` and using record
  construction/read at the append boundary without enabling packed ArrayBox
  compiler auto-use.
- `293x-218`: C206a aligned metadata store lookup cleanup landed, adding a
  single `HakoAllocAlignedSmallMetaStore.findIndex(ptr)` seam for aligned
  metadata reads without changing allocation behavior or enabling packed
  ArrayBox compiler auto-use.
- `293x-219`: C206b ArrayBox inline-record probe landed, adding a `#[cfg(test)]`
  `ArrayInlineRecordProbe` owner for explicit runtime probe arrays without
  compiler auto-use, public ArrayBox API exposure, or hako_alloc migration.
- `293x-220`: C206c ArrayBox inline-record probe negative landed, fixing ragged
  column rejection before an `ArrayBox` is built.
- `293x-221`: C206d ArrayBox inline-record plan probe landed, connecting
  `ArrayRecordStoragePlan` metadata to the explicit test-only runtime probe for
  integer-lane columns while rejecting handle columns.
- `293x-222`: D196 stop-the-line guard refresh landed, fixing `C206+`
  cleanup/probe guards as local-run and index-listed by default unless a later
  card names the production stop line that needs gate promotion.
- `293x-223`: C206e metadata-store indexed read cleanup landed, adding
  `alignmentAt` / `paddedSizeAt` and huge-page `*At` read seams so callers with
  a resolved metadata index do not repeat pointer lookup.
- `293x-224`: C207 packed ArrayBox compiler auto-use eligibility landed,
  emitting conservative `array_record_autouse_eligibility_plans` metadata while
  leaving production runtime auto-use disabled.
- `293x-225`: C208 inline-record materialization / escape boundary landed,
  emitting `array_record_materialization_boundary_plans` metadata while keeping
  visible record materialization and runtime auto-use disabled.
- `293x-226`: C209 non-escaping packed ArrayBox auto-use pilot landed, adding
  `array_record_packed_autouse_pilot_plans` and a crate-private i64 column
  construction/read seam while keeping public materialization, hako_alloc
  migration, and backend lowering disabled.
- `293x-227`: C210 aligned-small metadata packed-store pilot landed, adding
  `hako_alloc_aligned_small_packed_store_pilot_plans` for
  `HakoAllocAlignedSmallMeta` and a runtime proof that reads aligned metadata
  through the private C209 i64-column seam. The `.hako` source keeps its
  record-shaped scalar-column compatibility and does not mention compiler
  internals.
- `293x-228`: C211 huge-page metadata packed-store pilot landed, adding
  `hako_alloc_huge_page_packed_store_pilot_plans` for `HakoAllocHugePageMeta`
  and a runtime proof that reads huge metadata through the private C209
  i64-column seam while preserving live-flag and released-sentinel contracts.
- `293x-229`: C212 packed record backend fail-fast hardening landed, adding the
  shared `enforce_mir_backend_supported(...)` gate and a packed record backend
  checker that rejects future required packed-record routes on unsupported
  backends without enabling backend lowering today.
- Next: C194 verifier-owned allocation invariants. No M191 allocator API row is
  scheduled yet; M186 facade stats already landed as `294x-19e`. M104 is next
  only if the optional allocator-provider host-replacement ladder is explicitly
  reopened.

## Mimalloc Port Roadmap Snapshot

SSOT:
`docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md`

Current execution order:

1. Allocator API completion rows M185-M190 are complete. M185 inventory, M187
   size-class facades, M188 request-path facades, M189 object-return API parity,
   and M190 explicit result contract are complete, and facade stats are already
   exact `usize` via `294x-19e`, so those rows must not be repeated.
2. `C198-C200`: improve proof/application syntax only as separate language rows
   after docs/reference decisions. Do not fold them into allocator rows.
   `C197`, `C198`, `C199`, and `C200` are complete.
3. `C201-C205`: add record/packed-array compiler-runtime support before moving
   allocator metadata off the current M178 scalar columns. `C201`, `C202`,
   `C203a`, `C203b`, `C203c`, `C204a`, `C204b`, `C205a`, `C205b`, `C205c`, and
   `C205d` are complete. Packed ArrayBox compiler auto-use remains future work
   and should not be implied by the C205 store migrations.
4. `C207-C212`: open packed ArrayBox compiler auto-use in stages:
   eligibility gate, materialization/escape boundary, non-escaping auto-use
   pilot, aligned-small metadata packed-store pilot, huge-page packed-store
   pilot, and backend fail-fast hardening. `C207-C212` are complete.
5. `C191-C194`: run compiler/backend hardening only when it does not collide
   with the active `.hako` row.
6. `D195-D196`: refresh SSOT/guards at milestones, not after every tiny row.
   `D195` and `D196` are complete; `C206+` cleanup/probe guards stay
   local-run/index-listed unless a card names a production stop line for
   promotion.
