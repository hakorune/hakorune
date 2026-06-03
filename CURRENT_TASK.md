# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-05-19
Scope: current lane / next lane / restart order only.

## Purpose

- root から active lane / next lane に最短で戻る
- landed history と rejected history は phase docs / investigations を正本にする
- `CURRENT_TASK.md` 自体は ledger にしない

## Current Docs Policy

- Current-state SSOT: `docs/development/current/main/CURRENT_STATE.toml`
- Update policy SSOT:
  `docs/development/current/main/design/current-docs-update-policy-ssot.md`
- Normal card work must not append landed history here.
- Per-card updates are limited to `CURRENT_STATE.toml` latest-card fields and
  the active card unless lane / blocker / restart order / durable policy changes.

## Quick Restart Pointer

1. `docs/development/current/main/CURRENT_STATE.toml`
2. `docs/development/current/main/05-Restart-Quick-Resume.md`
3. `docs/development/current/main/10-Now.md`
4. Read `active_phase`, `phase_status`, `method_anchor`, `taskboard`, and
   `latest_card_path` from `CURRENT_STATE.toml`
5. `git status -sb`
6. `bash tools/checks/current_state_pointer_guard.sh`
7. `tools/checks/dev_gate.sh quick` when a code slice is ready
   (`allocator-wide` is explicit only for allocator/mimalloc/provider closeout)
8. `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
   only when returning to phase-29bq

## Current Lane

- active lane: read `active_lane` in `CURRENT_STATE.toml`
- active phase: read `active_phase` in `CURRENT_STATE.toml`
- latest card: read `latest_card_path` in `CURRENT_STATE.toml`
- task breakdown: read `taskboard` in `CURRENT_STATE.toml`
- method anchor / design SSOT: read `method_anchor` in `CURRENT_STATE.toml`
- current blocker token: read `current_blocker_token` in `CURRENT_STATE.toml`
- phase-137x: observe-only unless app work reopens a real blocker

## Restart Handoff

- latest landed card: read `latest_card_path` in `CURRENT_STATE.toml`
- latest known checkpoint: read `latest_card` / `latest_card_path` in
  `CURRENT_STATE.toml`; `291x-691` remains the historical warning-backlog
  inventory baseline
- current no-growth baseline: `classifiers=0 rows=0`; no `.inc`
  method/box string classifiers are allowlisted
- worktree expectation: clean unless the active slice is in progress
- resume point: continue the active phase from `current_blocker_token`,
  `phase_status`, and `latest_card_path` in `CURRENT_STATE.toml`.

## Task Order

- current task source: `CURRENT_STATE.toml` plus the active taskboard
- next active-lane order:
  1. read `current_blocker_token`, `phase_status`, and `latest_card_path` from
     `CURRENT_STATE.toml`
  2. finish the Hakorune mimalloc lane through the active workstream:
     `docs/development/current/main/workstreams/mimalloc-current.md`
  3. keep ProviderFront and ReplacementFront separate; do not weaken the
     provider API to chase malloc/free hot-path thinness
  4. make the replacement-front path benchmark-only until a dedicated
     activation row opens product replacement
  5. add multithread support and evidence before any allocator victory claim
  6. migrate hako_alloc non-negative fields only by explicit field-group rows
  7. keep sentinel-bearing indexes signed
  8. keep BoxShape cleanup separate from BoxCount feature rows
- parked lanes:
  - the exact `usize` semantic foundation lane is closed; the first mimalloc
    comparison execution pass is landed; benchmark contract work is now the
    active lane
  - allocator-provider ladder remains parked unless host allocator replacement
    is explicitly reopened
  - record defaults / spread / named args / automatic record-to-box copy remain
    parked syntax features
- VM retreat reading: VM is a semantic reference executor, not a product owner
- detailed landed history: phase card files and `CURRENT_STATE.toml`
- keep BoxShape cleanup separate from BoxCount feature rows
- do not add hot inline lowering without proof/evidence gate

## Current Implementation Focus

- Current priority is constrained to four buckets:
  1. direct memory / DirectArray language substrate
  2. mimalloc migration and optimization
  3. Array / representation fast paths only when selected by mimalloc perf
     evidence or by the active direct-memory substrate workstream
  4. docs and shell hygiene
- LANG-CFG build conditional syntax is now closed for this pass:
  `LANG-CFG-003` explain/report, `LANG-CFG-004` member-level selection,
  `LANG-CFG-005` statement-level selection, and `LANG-CFG-006` optional
  `@rune Gate(...)` sugar are all landed. The source surface is
  `gate Build... { ... }`, not C-style `#if`; read `CURRENT_STATE.toml` and
  `latest_workstream_card` for the active blocker.
- Mimalloc direct-exact optimization resumes after MIM-069 nonkeeper. The next
  owner refresh lives in `CURRENT_STATE.toml` and the mimalloc workstream card.
- The current LD_PRELOAD benchmark-only replacement front is not the full
  `.hako` mimalloc algorithm. It is a fixed-slot native front used to isolate
  replacement-boundary thinness. Treat `.hako` `hako_alloc` as the policy/model
  algorithm source until an explicit bridge connects it to the replacement
  front.
- Day-to-day work lives in `latest_workstream_card` from `CURRENT_STATE.toml`.
  Do not create numbered rows for inventory-only progress.
- Do not open another Array / helper / RuntimeDataBox fast path unless the
  active workstream provides concrete owner-family evidence and a positive-net
  implementation path.
- Do not add new inventory-only rows or one-off row guards. Fold small
  inventories into the active workstream card or a single investigation note,
  and use reusable lane guards.
- Detailed DirectArray / RuntimeDataBox / typed-object history lives in the
  phase cards and `CURRENT_STATE.toml` landed tail, not in this root pointer.

## Current FastPath Visibility Order

FastPath visibility is now part of the active implementation surface. The
truth stays in compiler/MIR metadata and `hako_check` reports; source files are
not rewritten.

Current order:

1. `FPVIS-001` JSON truth surface for `hako_check fastpath-explain`.
   - include `mir_hash`, `source_hash` when available, stable `site_id`,
     `source_span` placeholders, route/proof/fallback fields
   - keep existing key-value output compatible
2. `FPVIS-002` compact summary mode for daily diagnostics.
3. `FPVIS-003` Markdown annotated report under `target/` or an explicit
   `--out` path.
   - annotate report excerpts only; do not edit `.hako` source comments
4. `FPVIS-004` strict require-fastpath contract cleanup.
   - current `--require-clean` remains the v0 strict mode
   - broader CI matrix / costlier checks stay parked until explicitly opened
5. `FPVIS-005` compare/SARIF/LSP/editor overlay are future tooling only.
6. `@rune Check(fastpath)` or `@rune Require(fastpath)` remains parked until it
   can be a thin `RequiredFastPathRegion` sugar.

Rejected for this pass:

```text
source rewrite
compiler-inserted [FASTPATH] comments
source comments as optimization truth
CI-by-default expensive fastpath matrix
```

## Current RouteDecision Task Order

Route decision cleanup is the next compiler-shape direction if the active
mimalloc lane needs a cleaner fastpath/slowpath boundary.

Decision:

```text
fastpath-preferred:
  planners try fast routes first

fallback-explicit:
  slow routes are named, reportable, and policy-controlled

MIRBuilder:
  preserves origin/span/type/semantic-op facts; it does not choose fast vs slow

Lowering:
  consumes selected RouteDecision rows; it must not re-decide route policy
```

Planned order:

1. `RD-001` RouteDecisionV0 docs/report-only surface.
   - define `preferred_route`, `selected_route`, `fallback_policy`,
     `proof_ids`, and `miss_reason`
   - behavior change: none
2. `RD-002` DirectArray RouteDecision view.
   - map existing `DirectArrayAccessPlan` rows into RouteDecision output
   - first fixture: `DirectArrayI64` source migration / resetToFresh
3. `RD-003` fallback policy bridge.
   - `RequiredFastPathRegion` sets `fallback_policy=require_fastpath`
   - normal code remains `opportunistic` or `report_if_slow`
4. `RD-004` lowering consumer contract.
   - lowering reads `selected_route`
   - report `backend_redecide_count=0` and `silent_fallback_count=0`
5. `RD-005` horizontal extension after DirectArray proves the shape.
   - DirectState / RecordState / HotCore / ReplacementFront only by evidence

Rejected for this pass:

```text
MIRBuilder directly emitting fast/slow variants
backend method-name or helper-name route redecision
source comments as RouteDecision truth
new source syntax
```

## Current Language/Substrate Reset

The current language-substrate direction is:

```text
source surface:
  do not grow for this slice

box:
  identity / lifecycle / methods / DirectArray ownership

record:
  identity-free aggregate, snapshot, metadata row, or local scalarizable value
  future box-private primitive state bundle only through RecordStateResidencePlanV0

DirectArrayI64:
  owned variable-length exact-i64 table for internal hot paths

gate:
  build/test/proof selection only; not a fast-path selector

direct{}:
  parked; v0 uses RequiredFastPathRegion / FastPathObligation diagnostics
```

Do not convert allocator owner boxes such as `HakoAllocPageModel` into records.
The accepted future shape is `box` owner plus a primitive `record` state bundle
when `RecordStateResidencePlanV0` proves box-private subfield load/store:

```text
PageModel:
  box owner

PageState:
  record state bundle candidate

me.state.free_top:
  future direct record-state subfield access
```

Next active-lane substrate order before more speculative mimalloc source edits:

1. `StateBucketClassifierV0` inventory for page / queue / facade / result
   fields.
2. `RecordStateResidencePlanV0` metadata-only contract; no source migration
   yet.
3. `hako_check state-explain` read-only adapter for field buckets and
   record-state candidates.
4. Route-aware materialization / HotCore direct-exact call plan selection if
   current evidence still points to MIR call/copy boundaries.
5. PageState source migration only after positive-net report evidence; no
   whole-record ABI or public materialization.

Current C-gap reading:

```text
Do not read the remaining C gap as "record is missing" alone.

Expected composite:
  RecordStateResidencePlanV0
  + DirectArrayI64 / proved direct-array access
  + HotCore direct-exact call plans where the active front uses HotCore
  + route-aware copy/materialization
  + observer/public/proof state classification
```

The observer-light HotCore compiler fastpath slice is already a completed
front. The public proof front is heavier by design because it still carries
facade/result/observer semantics. Do not mix those fronts when selecting the
next owner.

Current algorithm-port reading:

```text
Do not read benchmark-only ReplacementFront throughput as proof that the full
.hako mimalloc algorithm is product-ready.

Modelled in .hako:
  size-class policy
  page-local free/local_free/block_used state
  page-map/realloc/huge/OSVM/remote-free policy seams
  object-lifecycle hot-core small alloc/release shape

Currently executed by the benchmark-only replacement front:
  single fixed-slot native free stack
  optional thread-local arena and remote-free bridge
  direct malloc/free/realloc wrappers for the fixed slot size

Open bridge work:
  connect size-class policy to real replacement bins/pages
  migrate hot page arrays toward DirectArrayI64-backed storage
  connect .hako PageModel/HotCore plans to replacement-front lowering
  keep provider API and ReplacementFront separate
```

Use `tools/allocator/hako_mimalloc_algorithm_coverage.py` for the current
coverage report before claiming algorithmic port completeness.

Latest representative public proof refresh:

```text
hako_body_elapsed_ns=3000000
c_body_elapsed_ns=3523685
hako_instructions=57574640
c_instructions=65106835
```

Current decision: no route-aware copy/materialization implementation is open
unless fresh owner-first perf/asm evidence selects a concrete site family and
proves `before_route == after_route`.

## Current Direct Memory Task Order

This wave is complete. For current work, read
`docs/development/current/main/CURRENT_STATE.toml` and the
`latest_workstream_card` it points to first.

Recommended order:

1. `LANG-DM-001` reference policy lock
   - no `RawPtr<T>`
   - no pointer operators
   - `NativePtr` remains opaque
   - `direct` / `unsafe memory` / `unchecked` stay separate
   - status: done; reference policy is in
     `docs/reference/language/low-level-capabilities.md`
2. `LANG-DM-002` DirectArrayAccessPlan cleanup
   - status: done; `DirectArrayAccessPlan` carries `element_type` and
     `proof_ids`, and consumers validate the proof carrier
   - region/view vocabulary remains metadata-only/deferred
3. `LANG-DM-003` proof fact normalization
   - status: done; DirectArray unchecked planning now reads
     `RangeIndexFact` + `DirectArrayExtentFact` + `RegionStabilityFact`
4. `LANG-DM-004` Span no-escape SSOT
   - status: done; accepted in
     `docs/development/current/main/design/span-no-escape-ssot.md`
5. `LANG-DM-005` SpanI64 / SpanMutI64 minimal pilot
   - status: done; metadata carriers and fact-only Span access planner landed
     for one read and one mutable write fixture
6. `LANG-DM-006A` Direct FastPath Required Diagnostic Contract
   - status: done; `RequiredFastPathRegion` / `FastPathObligation` metadata and
     refresh landed, with missing FastPathPlan reported as `DM006001`
   - no `direct {}` source syntax in v0
   - define `RequiredFastPathRegion` / `FastPathObligation`
   - checked direct routes are allowed; generic helper / boxed fallback /
     dynamic route are rejected when required
7. `LANG-DM-006B` future `direct {}` syntax parking lot
   - status: parked; no parser/AST/MIR source syntax in this wave
   - only after diagnostics are stable
8. `LANG-DM-006C` hako_check FastPath explain adapter
   - status: done; `tools/hako_check/fastpath_explain.py` reads existing MIR
     JSON and reports DirectArray / Span / RequiredFastPath metadata coverage
   - no source rewrite, no MIR emission, no keeper selection
9. `LANG-DM-006D` hako_check FastPath explain developer wrapper
   - status: done; `tools/hako_check/fastpath_explain.sh` can emit temporary
     MIR JSON for `--app app.hako` and then call the read-only adapter
   - root entry: `tools/hako_check.sh fastpath-explain`
   - root hako_check tool entries also include `perf-surface` and
     `perf-surface-contract`; MIR method-shape remains outside hako_check
   - no compiler build, benchmark run, source rewrite, persistent MIR artifact,
     or keeper selection
10. Park `unsafe memory` / `Bytes`, `LayoutSpan`, and bulk memory patterns until
   the DirectArray/Span proof system is stable.

Mimalloc optimization is active now. Read `CURRENT_STATE.toml` for the next
blocker; the current post-MIM-056 checkpoint is a fresh owner refresh and the
paused build-conditional slice has returned control to mimalloc.

## Current Ordered App Bringup

- latest app card: read `latest_card_path` in
  `docs/development/current/main/CURRENT_STATE.toml`
- order:
  1. BoxTorrent mini
  2. binary-trees
  3. mimalloc-lite
  4. real allocator port
  5. allocator-stress app
  6. BoxTorrent allocator-backed store
  7. JSON stream aggregator
- current status: BoxTorrent mini, binary-trees, mimalloc-lite, the
  `hako_alloc` VM-only page/free-list port, allocator-stress, BoxTorrent
  allocator-backed store, and JSON stream aggregator landed; direct EXE now
  lowers typed-object allocation/field slots, the BoxTorrent `firstChunkId` /
  `refCount` module-generic seam, BoxTorrent mini user-box string field
  returns, global-call handle param metadata, substring handle result
  publication, recursive same-module user-box method bodies, typed-object
  handle global-call returns, allocator handle param-origin inference, and
  explicit same-module PHI type preservation; BoxTorrent mini, binary-trees,
  JSON stream aggregator, mimalloc-lite, and allocator-stress direct EXE
  parity now exit 0
- compiler rule: do not hide a real compiler blocker in app code; fix the
  compiler seam structurally when needed

## Detail Pointers

- Active phase: read `active_phase` in `CURRENT_STATE.toml`
- Active taskboard: read `taskboard` in `CURRENT_STATE.toml`
- Active design SSOT: read `method_anchor` in `CURRENT_STATE.toml`
- Bootstrap route SSOT:
  `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
- Perf owner-first policy:
  `docs/development/current/main/design/perf-owner-first-optimization-ssot.md`
- Hotline/CoreMethodContract SSOT:
  `docs/development/current/main/design/hotline-core-method-contract-ssot.md`
