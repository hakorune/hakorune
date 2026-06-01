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
  2. keep allocator-provider activation, host allocator replacement, hooks, and
     `#[global_allocator]` out of scope
  3. migrate hako_alloc non-negative fields only by explicit field-group rows
  4. keep sentinel-bearing indexes signed
  5. keep BoxShape cleanup separate from BoxCount feature rows
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
- LANG-CFG build conditional syntax is the active short language slice before
  returning to mimalloc. The source surface is `when Build... { ... }`, not
  C-style `#if`; read `CURRENT_STATE.toml` and
  `latest_workstream_card` for the active blocker.
- Mimalloc direct-exact optimization is paused after MIM-069 nonkeeper. Return
  to MIM-070 only after the LANG-CFG slice is closed or explicitly parked.
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
8. Park `unsafe memory` / `Bytes`, `LayoutSpan`, and bulk memory patterns until
   the DirectArray/Span proof system is stable.

Mimalloc optimization is active now. Read `CURRENT_STATE.toml` for the next
blocker; the current post-MIM-056 checkpoint is a fresh owner refresh.

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
