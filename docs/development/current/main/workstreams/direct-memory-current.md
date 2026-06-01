---
Status: Active
Date: 2026-06-01
Scope: direct memory / DirectArray language substrate workstream.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/reference/language/low-level-capabilities.md
  - docs/development/current/main/design/typed-numeric-memory-substrate-task-order-ssot.md
  - docs/development/current/main/design/span-no-escape-ssot.md
  - docs/development/current/main/workstreams/mimalloc-current.md
---

# Direct Memory Current Workstream

## Goal

Pause mimalloc source tweaking long enough to make the direct-memory substrate
tasks readable and reusable.

The target is not C pointer syntax. The target is a small verified view system
that lets allocator code stay ordinary `.hako` while compiler plans choose
direct storage routes.

## Stop Line

- no `RawPtr<T>` source type
- no `&`, `*`, `->`, or pointer arithmetic
- no `NativePtr` dereference or indexing
- no untyped `DirectArray` source type in v0
- no implicit `Array` <-> `DirectArrayI64` conversion
- no public ArrayBox semantics change
- no broad `unsafe` block
- no new numbered row or row-specific `.sh` guard by default

## Task Order

### Active Slice

- [x] LANG-DM-001: reference policy lock
  - output: `docs/reference/language/low-level-capabilities.md` says
    RawPtr/pointer operators are not accepted, NativePtr is opaque, and
    `direct` / `unsafe memory` / `unchecked` are distinct
  - no code unless a reference mismatch is discovered

- [x] LANG-DM-002: DirectArrayAccessPlan cleanup
  - output: plan schema carries `element_type` and `proof_ids`
  - prepare `region_id` / `view_id` as metadata only if it keeps code simpler
  - no Span, Bytes, LayoutSpan, or source syntax

- [x] LANG-DM-003: proof fact normalization
  - output: current DirectArray proof consumers can read stable fact names:
    `RangeIndexFact`, `DirectArrayExtentFact`, `RegionStabilityFact`
  - do not widen the accepted loop forms unless the fixture/smoke is included

### Next Slice

- [x] LANG-DM-004: Span no-escape SSOT
  - output: exact rules for `SpanI64` / `SpanMutI64`
  - no return, field store, capture, publish, provider boundary crossing, or
    owner resize/free inside the span lifetime

- [x] LANG-DM-005: SpanI64 / SpanMutI64 minimal pilot
  - output: borrow only from `DirectArrayI64`
  - access plans reuse DirectArray proof vocabulary
  - no NativePtr / Bytes yet
  - current slice: metadata-only `SpanBorrowFact` carrier lands first; access
    planning remains the next implementation slice
  - current slice: metadata-only `SpanAccessPlan` carrier lands next; planner
    and lowering remain closed until fixture selection
  - landed: fact-only `SpanAccessPlan` planner consumes `SpanBorrowFact`,
    `RangeIndexFact`, `DirectArrayExtentFact`, and `RegionStabilityFact` for
    one read and one mutable write fixture; source syntax and lowering remain
    closed

- [x] LANG-DM-006A: Direct FastPath Required Diagnostic Contract
  - output: `RequiredFastPathRegion` / `FastPathObligation` diagnostic contract
  - no `direct {}` source syntax in v0
  - direct is a route contract, not unsafe memory and not unchecked
  - checked direct routes are allowed; generic helper / boxed fallback /
    dynamic route are rejected when the obligation is required
  - landed: metadata/json contract and source-syntax-free obligation refresh
    from existing DirectArray/Span plans; missing plan reports `DM006001`

- [x] LANG-DM-006B: future `direct {}` syntax parking lot
  - output: direct block remains a thin future syntax over
    `RequiredFastPathRegion`
  - do not implement parser/AST/MIR scope until diagnostics are stable
  - parked: no parser, AST, MIR scope, or source syntax in this wave; reopen
    only after `RequiredFastPathRegion` diagnostics are used by a real keeper
    expectation and the syntax is only a thin source span carrier

- [x] LANG-DM-006C: hako_check FastPath explain adapter
  - output: add a read-only developer diagnostic for existing direct-memory MIR
    metadata
  - tool: `tools/hako_check/fastpath_explain.py`
  - contract: `hako-check-fastpath-explain-v0`
  - consumes caller-provided MIR JSON only; it does not emit MIR, rewrite
    source, select keepers, or own lowering policy
  - reports `DirectArrayAccessPlan`, `SpanAccessPlan`,
    `RequiredFastPathRegion`, and `FastPathObligation` counts, plus
    `DM006001` missing FastPathPlan failures
  - optional `--require-clean` returns non-zero only when existing
    FastPath obligations failed

### Parked

- [ ] LANG-DM-007: unsafe memory / Bytes parked design
  - output: NativePtr remains opaque; Bytes owns byte-offset load/store methods

- [ ] LANG-DM-008: LayoutSpan / bulk memory pattern parking lot
  - output: future layout view and fill/zero/iota/copy recognition remain
    separate from existing enum record terminology

## Decision Log

- 2026-06-01: The active task focus moves from mimalloc source-level
  micro-optimization to direct-memory language substrate cleanup. Mimalloc is
  paused after MIM-054, not abandoned. The next language tasks should make
  DirectArray / Span / Bytes planning explicit without adding C-style pointers.
- 2026-06-01: LANG-DM-001 reference policy is locked in
  `docs/reference/language/low-level-capabilities.md`. Next active task is
  LANG-DM-002: make `DirectArrayAccessPlan` carry the element/proof shape that
  later Span/Bytes work can reuse.
- 2026-06-01: LANG-DM-002 is implemented without widening `.hako` syntax.
  `DirectArrayAccessPlan` now carries `proof_ids` beside the existing
  `element_type`, and Rust JSON / Python / C shim consumers all validate the
  proof carrier. `region_id` / `view_id` remain deferred metadata until
  LANG-DM-003 decides the fact vocabulary.
- 2026-06-01: LANG-DM-003 normalized the DirectArray proof vocabulary into
  `RangeIndexFact` + `DirectArrayExtentFact` + `RegionStabilityFact`.
  Unchecked DirectArray planning now requires the extent fact to reference a
  matching stability fact, and MIR JSON emits all three fact families for later
  Span work.
- 2026-06-01: LANG-DM-004 accepted
  `docs/development/current/main/design/span-no-escape-ssot.md`. Span v0 is a
  no-escape `SpanI64` / `SpanMutI64` view over `DirectArrayI64`; it is not
  pointer syntax, not unsafe memory, and not unchecked by itself.
- 2026-06-01: LANG-DM-005 started with the metadata carrier slice:
  `SpanBorrowFact` records no-escape borrow shape over a stable region without
  adding source syntax or lowering behavior yet. Next slice should select the
  first read/write Span access plan.
- 2026-06-01: LANG-DM-005 added a metadata-only `SpanAccessPlan` carrier.
  The plan reuses the DirectArray proof vocabulary (`range_index`,
  `direct_array_extent`, `region_stability`) and keeps Span planner/lowering
  closed for the next selected fixture.
- 2026-06-01: LANG-DM-005 landed the minimal fact-only Span planner. It creates
  `SpanAccessPlan` rows only when a no-escape Span borrow, range index, direct
  array extent, and region stability proof all line up. No source syntax or
  lowering was opened.
- 2026-06-01: LANG-DM-006 was split. v0 will not add `direct {}` syntax.
  The active next slice is `LANG-DM-006A`: a diagnostic/report contract for
  required FastPath regions. Future `direct {}` may only become syntax sugar
  over that contract.
- 2026-06-01: LANG-DM-006A landed the metadata and refresh path for
  `RequiredFastPathRegion` / `FastPathObligation`. Existing DirectArray and
  Span plans satisfy obligations; missing plans fail with `DM006001`. No
  parser, source syntax, or lowering contract was opened.
- 2026-06-01: LANG-DM-006B parked future `direct {}` syntax. The current wave
  keeps source unchanged; `direct {}` may reopen only as syntax sugar over
  `RequiredFastPathRegion` after diagnostic/report usage proves it is needed.
- 2026-06-01: LANG-DM-006C added `hako_check fastpath-explain` as a read-only
  MIR JSON diagnostic adapter. This gives DirectArray / Span /
  RequiredFastPath metadata one developer-facing report without widening
  `.hako` syntax or moving keeper selection into hako_check.
